// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Repo automation, run with `cargo xtask <command>`.
//!
//! - `spec-check`: run `lsc check specs/` on this repository's own specifications.
//! - `projectfile-check`: run `lsc pf ci resolve projectfile.yaml` on this repository's projectfile.
//! - `toolchain-check`: verify the active Rust matches `rust-toolchain.toml`.
//! - `xwork`: run the verification pipeline in the order `projectfile.yaml` gives (see `stages.rs`).
//! - `stages-check`: fail if `stages.rs` differs from what `lsc` generates from `projectfile.yaml`.
//! - `stages-gen`: rewrite `stages.rs` from `projectfile.yaml`.
//! - `verify-signed <owner/repo> <pr>`: require GitHub-verified signatures on every PR commit.
//! - `image-tags <ref> <sha>`: print the image tags a publish from that ref gets.
//! - `package`: build the static musl `lsc` into `dist/lsc`.

mod release;
mod stages;
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
        .args(["run", "-q", "-p", "lsc", "-j", "2", "--", "check", "specs/"])
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
            "lsc",
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
    let domain_src = root.join("crates/lsc-core/src");
    let mut violations = Vec::new();
    scan_dir_for_io(&domain_src, &mut violations)?;
    if !violations.is_empty() {
        return Err(format!(
            "lsc-core must not use I/O, found forbidden imports:\n  {}",
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

/// How one `xwork` stage runs. Every stage the projectfile names needs an entry here.
enum Action {
    Toolchain,
    SpecCheck,
    ProjectfileCheck,
    StagesCheck,
    ZeroIo,
    /// A cargo invocation; `{jobs}` is replaced by the job limit.
    Cargo(&'static [&'static str]),
}

fn action_for(stage: &str) -> Option<Action> {
    Some(match stage {
        "toolchain-check" => Action::Toolchain,
        "spec-check" => Action::SpecCheck,
        "projectfile-check" => Action::ProjectfileCheck,
        "stages-check" => Action::StagesCheck,
        "zero-io-domain" => Action::ZeroIo,
        "cargo-fmt" => Action::Cargo(&["fmt", "--all", "--", "--check"]),
        "cargo-clippy" => Action::Cargo(&[
            "clippy",
            "--workspace",
            "--all-targets",
            "-j",
            "{jobs}",
            "--",
            "-D",
            "warnings",
        ]),
        "cargo-test" => Action::Cargo(&["test", "--workspace", "-j", "{jobs}"]),
        "cargo-doc-test" => Action::Cargo(&["test", "--doc", "--workspace", "-j", "{jobs}"]),
        _ => return None,
    })
}

fn run_stage(root: &Path, stage: &str, _args: &[&str]) -> ExitCode {
    let jobs = env::var("JOBS")
        .or_else(|_| env::var("CARGO_BUILD_JOBS"))
        .unwrap_or_else(|_| stages::DEFAULT_JOBS.to_string());
    match action_for(stage) {
        Some(Action::Toolchain) => toolchain::run(root),
        Some(Action::SpecCheck) => spec_check(root),
        Some(Action::ProjectfileCheck) => projectfile_check(root),
        Some(Action::StagesCheck) => stages_check(root),
        Some(Action::ZeroIo) => match verify_zero_io_domain(root) {
            Ok(()) => {
                println!("zero-io-domain: ok (crates/lsc-core/src has zero I/O imports)");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("zero-io-domain: {err}");
                ExitCode::FAILURE
            }
        },
        Some(Action::Cargo(args)) => {
            let args: Vec<String> = args.iter().map(|a| a.replace("{jobs}", &jobs)).collect();
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            run_cargo(root, &args)
        }
        None => {
            eprintln!("xwork: projectfile stage `{stage}` has no implementation in xtask");
            ExitCode::FAILURE
        }
    }
}

fn run_xwork(root: &Path) -> ExitCode {
    println!("=== xwork: verification pipeline in projectfile.yaml order ===");
    stages::run_xwork(root, run_stage)
}

/// What `lsc` generates from `projectfile.yaml`, or `None` after printing why it could not.
fn generated_stages(root: &Path) -> Option<Vec<u8>> {
    let out = Command::new(env!("CARGO"))
        .args([
            "run",
            "-q",
            "-p",
            "lsc",
            "-j",
            "2",
            "--",
            "pf",
            "ci",
            "generate",
            "projectfile.yaml",
            "--target",
            "xtask",
            "--dispatch",
        ])
        .current_dir(root)
        .output();
    match out {
        Ok(o) if o.status.success() => Some(o.stdout),
        Ok(o) => {
            eprintln!(
                "stages: lsc failed: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            );
            None
        }
        Err(err) => {
            eprintln!("stages: cannot run cargo: {err}");
            None
        }
    }
}

fn stages_check(root: &Path) -> ExitCode {
    let Some(generated) = generated_stages(root) else {
        return ExitCode::FAILURE;
    };
    match fs::read(root.join("crates/xtask/src/stages.rs")) {
        Ok(current) if current == generated => {
            println!("stages-check: ok (stages.rs matches projectfile.yaml)");
            ExitCode::SUCCESS
        }
        Ok(_) => {
            eprintln!(
                "stages-check: crates/xtask/src/stages.rs is out of date; run `cargo xtask stages-gen`"
            );
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("stages-check: cannot read stages.rs: {err}");
            ExitCode::FAILURE
        }
    }
}

fn stages_gen(root: &Path) -> ExitCode {
    let Some(generated) = generated_stages(root) else {
        return ExitCode::FAILURE;
    };
    match fs::write(root.join("crates/xtask/src/stages.rs"), generated) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("stages-gen: cannot write stages.rs: {err}");
            ExitCode::FAILURE
        }
    }
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
        Some("stages-check") => stages_check(&root),
        Some("stages-gen") => stages_gen(&root),
        Some("verify-signed") => release::run_verify_signed(&rest),
        Some("image-tags") => release::run_image_tags(&root, &rest),
        Some("package") => release::run_package(&root),
        _ => {
            eprintln!(
                "usage: cargo xtask <spec-check|projectfile-check|toolchain-check|xwork|stages-check|stages-gen|verify-signed|image-tags|package>"
            );
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage_names(stages: &[(&str, &[&str])]) -> Vec<String> {
        stages.iter().map(|(stage, _)| (*stage).to_string()).collect()
    }

    // SPEC-0003 requirement 9: the dispatch runner stops at the first failing stage.
    #[test]
    fn run_xwork_stops_at_the_first_failing_stage() {
        let mut calls = Vec::new();
        let code = stages::run_xwork(Path::new("."), |_, stage, _| {
            calls.push(stage.to_string());
            if calls.len() == 2 {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        });
        assert_eq!(code, ExitCode::FAILURE);
        assert_eq!(calls, stage_names(&stages::XWORK_STAGES[..2]));
    }

    // SPEC-0003 requirement 9: on success every stage runs once, in table order.
    #[test]
    fn run_xwork_runs_every_stage_in_table_order() {
        let mut calls = Vec::new();
        let code = stages::run_xwork(Path::new("."), |_, stage, _| {
            calls.push(stage.to_string());
            ExitCode::SUCCESS
        });
        assert_eq!(code, ExitCode::SUCCESS);
        assert_eq!(calls, stage_names(stages::XWORK_STAGES));
    }

    #[test]
    fn every_projectfile_stage_has_an_implementation() {
        for &(stage, _) in stages::XWORK_STAGES {
            assert!(
                action_for(stage).is_some(),
                "no implementation for `{stage}`"
            );
        }
    }

    #[test]
    fn unknown_stage_has_no_implementation() {
        assert!(action_for("no-such-stage").is_none());
    }

    #[test]
    fn lsc_core_is_free_of_io() {
        verify_zero_io_domain(&root()).expect("lsc-core must be free of I/O");
    }
}
