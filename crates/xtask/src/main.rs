// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Repo automation, run with `cargo xtask <command>`.
//!
//! - `spec-check`: run `lsc check specs/` on this repository's own specifications.
//! - `toolchain-check`: verify the active Rust matches `rust-toolchain.toml`.
//! - `verify-signed <owner/repo> <pr>`: require GitHub-verified signatures on every PR commit.
//! - `image-tags <ref> <sha>`: print the image tags a publish from that ref gets.
//! - `package`: build the static musl `lsc` into `dist/lsc`.

mod release;
mod toolchain;

use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn spec_check(root: &std::path::Path) -> ExitCode {
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

fn main() -> ExitCode {
    let root = root();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest = args.get(1..).unwrap_or_default().to_vec();
    match args.first().map(String::as_str) {
        Some("spec-check") => spec_check(&root),
        Some("toolchain-check") => toolchain::run(&root),
        Some("verify-signed") => release::run_verify_signed(&rest),
        Some("image-tags") => release::run_image_tags(&root, &rest),
        Some("package") => release::run_package(&root),
        _ => {
            eprintln!(
                "usage: cargo xtask <spec-check|toolchain-check|verify-signed|image-tags|package>"
            );
            ExitCode::from(2)
        }
    }
}
