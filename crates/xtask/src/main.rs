// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Repo automation, run with `cargo xtask <command>`.
//!
//! - `spec-check`: run `lsc check specs/` on this repository's own specifications.
//! - `toolchain-check`: verify the active Rust matches `rust-toolchain.toml`.

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
    match std::env::args().nth(1).as_deref() {
        Some("spec-check") => spec_check(&root),
        Some("toolchain-check") => toolchain::run(&root),
        _ => {
            eprintln!("usage: cargo xtask <spec-check|toolchain-check>");
            ExitCode::from(2)
        }
    }
}
