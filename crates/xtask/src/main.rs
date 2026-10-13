// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Repo automation, run with `cargo xtask <command>`.
//!
//! - `spec-check`: run `lsc check specs/` on this repository's own specifications.
//! - `projectfile-check`: run `lsc pf ci resolve projectfile.yaml` on this repository's projectfile.
//! - `toolchain-check`: verify the active Rust matches `rust-toolchain.toml`.
//! - `xwork`: run the full verification pipeline (`toolchain-check`, `spec-check`, `projectfile-check`, `zero-io-domain`, `fmt-check`, `clippy`, `test`, `doc-test`).
//! - `verify-signed <owner/repo> <pr>`: require GitHub-verified signatures on every PR commit.
//! - `image-tags <ref> <sha>`: print the image tags a publish from that ref gets.
//! - `package`: build the static musl `lsc` into `dist/lsc`.

mod release;
mod toolchain;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn spec_check(root: &Path) -> ExitCode {
    let status = Command::new(env!("CARGO"))
        .args(["run", "-q", "-p", "ls", "-j", "2", "--", "check", "specs/"])
        .current_dir(root)
        .status();
    match status {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        Ok(_) => {
            eprintln!("spec-check: `lsc check specs/` reported errors");
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("spec-check: cannot run cargo: {err}");
            ExitCode::FAILURE
        }
    }
}

fn projectfile_check(root: &Path) -> ExitCode {
    let status = Command::new(env!("CARGO"))
        .args([
            "run",
            "-q",
            "-p",
            "ls",
            "-j",
            "2",
            "--",
            "pf",
            "ci",
            "resolve",
            "projectfile.yaml",
        ])
        .current_dir(root)
        .status();
    match status {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        Ok(_) => {
            eprintln!("projectfile-check: `lsc pf ci resolve projectfile.yaml` reported errors");
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("projectfile-check: cannot run cargo: {err}");
            ExitCode::FAILURE
        }
    }
}

fn verify_zero_io_domain(root: &Path) -> Result<(), String> {
    let domain_src = root.join("crates/ls-domain/src");
    let mut violations = Vec::new();
    scan_dir_for_io(&domain_src, &mut violations)?;
    if !violations.is_empty() {
        return Err(format!(
            "ls-domain must not use I/O, found forbidden imports:\n  {}",
            violations.join("\n  ")
        ));
    }
    Ok(())
}

fn scan_dir_for_io(dir: &Path, violations: &mut Vec<String>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|e| format!("cannot read directory {}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            scan_dir_for_io(&path, violations)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let text = fs::read_to_string(&path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            for (idx, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("").trim();
                if code.is_empty() {
                    continue;
                }
                for forbidden in ["std::net", "std::fs", "std::process", "tokio::"] {
                    if code.contains(forbidden) {
                        violations.push(format!("{}:{}: {}", path.display(), idx + 1, line.trim()));
                    }
                }
            }
        }
    }
    Ok(())
}

fn run_cargo(root: &Path, args: &[&str]) -> ExitCode {
    let status = Command::new(env!("CARGO"))
        .args(args)
        .current_dir(root)
        .status();
    match status {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(err) => {
            eprintln!("xwork: cannot run cargo {}: {err}", args.join(" "));
            ExitCode::FAILURE
        }
    }
}

fn run_xwork(root: &Path) -> ExitCode {
    let jobs = env::var("JOBS")
        .or_else(|_| env::var("CARGO_BUILD_JOBS"))
        .unwrap_or_else(|_| "2".to_string());

    println!("=== xwork: deterministic verification pipeline (jobs={jobs}) ===");

    println!("xwork: [toolchain-check] running...");
    if toolchain::run(root) != ExitCode::SUCCESS {
        return ExitCode::FAILURE;
    }

    println!("xwork: [spec-check] running...");
    if spec_check(root) != ExitCode::SUCCESS {
        return ExitCode::FAILURE;
    }

    println!("xwork: [projectfile-check] running...");
    if projectfile_check(root) != ExitCode::SUCCESS {
        return ExitCode::FAILURE;
    }

    println!("xwork: [zero-io-domain] running...");
    if let Err(err) = verify_zero_io_domain(root) {
        eprintln!("zero-io-domain: {err}");
        return ExitCode::FAILURE;
    }
    println!("zero-io-domain: ok (crates/ls-domain/src has zero I/O imports)");

    println!("xwork: [fmt-check] running...");
    if run_cargo(root, &["fmt", "--all", "--", "--check"]) != ExitCode::SUCCESS {
        return ExitCode::FAILURE;
    }

    println!("xwork: [clippy] running...");
    if run_cargo(
        root,
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "-j",
            &jobs,
            "--",
            "-D",
            "warnings",
        ],
    ) != ExitCode::SUCCESS
    {
        return ExitCode::FAILURE;
    }

    println!("xwork: [test] running...");
    if run_cargo(root, &["test", "--workspace", "-j", &jobs]) != ExitCode::SUCCESS {
        return ExitCode::FAILURE;
    }

    println!("xwork: [doc-test] running...");
    if run_cargo(root, &["test", "--doc", "--workspace", "-j", &jobs]) != ExitCode::SUCCESS {
        return ExitCode::FAILURE;
    }

    println!("=== xwork: all stages PASSED ===");
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let root = root();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest = args.get(1..).unwrap_or_default().to_vec();
    match args.first().map(String::as_str) {
        Some("spec-check") => spec_check(&root),
        Some("projectfile-check") => projectfile_check(&root),
        Some("toolchain-check") => toolchain::run(&root),
        Some("xwork") => run_xwork(&root),
        Some("verify-signed") => release::run_verify_signed(&rest),
        Some("image-tags") => release::run_image_tags(&root, &rest),
        Some("package") => release::run_package(&root),
        _ => {
            eprintln!(
                "usage: cargo xtask <spec-check|projectfile-check|toolchain-check|xwork|verify-signed|image-tags|package>"
            );
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ls_domain_is_free_of_io() {
        verify_zero_io_domain(&root()).expect("ls-domain must be free of I/O");
    }
}
