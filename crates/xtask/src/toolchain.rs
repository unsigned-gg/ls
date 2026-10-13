// SPDX-License-Identifier: Apache-2.0 OR MIT
//! `cargo xtask toolchain-check`.
//!
//! Fails unless the active `rustc` equals the `channel` in `rust-toolchain.toml`
//! and `[workspace.package] rust-version` is not above that channel.

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

use anyhow::{anyhow, bail, Context, Result};

pub fn run(root: &Path) -> ExitCode {
    match check(root) {
        Ok(msg) => {
            println!("toolchain-check: {msg}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("toolchain-check: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn check(root: &Path) -> Result<String> {
    let toolchain = fs::read_to_string(root.join("rust-toolchain.toml"))
        .context("cannot read rust-toolchain.toml")?;
    let manifest = fs::read_to_string(root.join("Cargo.toml")).context("cannot read Cargo.toml")?;
    let channel = value(&toolchain, "channel")
        .ok_or_else(|| anyhow!("no `channel` in rust-toolchain.toml"))?;
    let rust_version = value(&manifest, "rust-version")
        .ok_or_else(|| anyhow!("no `rust-version` in Cargo.toml"))?;
    let out = Command::new("rustc")
        .arg("--version")
        .current_dir(root)
        .output()
        .context("cannot run `rustc --version`")?;
    if !out.status.success() {
        bail!("`rustc --version` failed");
    }
    let rustc = String::from_utf8_lossy(&out.stdout).trim().to_string();
    verify(&channel, &rustc, &rust_version)?;
    Ok(format!("ok ({rustc}; rust-version {rust_version})"))
}

/// First `key = "value"` line in a TOML document.
fn value(doc: &str, key: &str) -> Option<String> {
    doc.lines().find_map(|line| {
        let (k, v) = line.split_once('=')?;
        (k.trim() == key).then(|| v.trim().trim_matches('"').to_string())
    })
}

fn parse(version: &str) -> Result<Vec<u64>> {
    version
        .split('.')
        .map(|p| {
            p.parse::<u64>()
                .map_err(|_| anyhow!("not a version: `{version}`"))
        })
        .collect()
}

/// Compare the pinned channel, the `rustc --version` line and the MSRV.
pub fn verify(channel: &str, rustc_line: &str, rust_version: &str) -> Result<()> {
    let actual = rustc_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow!("unparseable rustc line `{rustc_line}`"))?;
    if actual != channel {
        bail!("active rustc is {actual}, rust-toolchain.toml pins {channel}");
    }
    if parse(rust_version)? > parse(channel)? {
        bail!("rust-version {rust_version} is above pinned channel {channel}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_toolchain_passes() {
        assert!(verify("1.85.0", "rustc 1.85.0 (4d91de4e4 2025-02-17)", "1.85").is_ok());
    }

    #[test]
    fn channel_mismatch_fails() {
        assert!(verify("1.85.0", "rustc 1.90.0 (abc 2025-09-01)", "1.85").is_err());
    }

    #[test]
    fn msrv_above_channel_fails() {
        assert!(verify("1.85.0", "rustc 1.85.0 (abc 2025-02-17)", "1.86").is_err());
    }

    #[test]
    fn reads_quoted_value() {
        assert_eq!(
            value("[toolchain]\nchannel = \"1.85.0\"\n", "channel").as_deref(),
            Some("1.85.0")
        );
    }
}
