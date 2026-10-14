// SPDX-License-Identifier: Apache-2.0 OR MIT
//! GitHub-flow release helpers: signed-commit gate, image tags and the musl package build.

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

use anyhow::{anyhow, bail, Context, Result};

const TARGET: &str = "x86_64-unknown-linux-musl";

/// Tags for an image published from `git_ref` at `sha`.
///
/// `refs/heads/main` yields `main` and `sha-<7>`. `refs/tags/vX.Y.Z` yields `X.Y.Z` and is
/// refused unless it equals the workspace `version`. Any other ref is refused.
pub fn image_tags(git_ref: &str, sha: &str, version: &str) -> Result<Vec<String>> {
    if sha.len() < 7 || !sha.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("not a commit sha: `{sha}`");
    }
    let short = format!("sha-{}", &sha[..7]);
    if git_ref == "refs/heads/main" {
        return Ok(vec!["main".into(), short]);
    }
    if let Some(tag) = git_ref.strip_prefix("refs/tags/v") {
        if tag != version {
            bail!("tag v{tag} does not match workspace version {version}");
        }
        return Ok(vec![tag.into(), short]);
    }
    bail!("`{git_ref}` is not published (only refs/heads/main and refs/tags/v*)")
}

/// Fail unless every commit line (`{"sha","verified","reason"}` JSON, one per line) is verified.
pub fn verify_signed(ndjson: &str) -> Result<usize> {
    let mut count = 0;
    let mut bad = Vec::new();
    for line in ndjson.lines().filter(|l| !l.trim().is_empty()) {
        let v: serde_json::Value =
            serde_json::from_str(line).with_context(|| format!("not JSON: {line}"))?;
        count += 1;
        if v["verified"].as_bool() != Some(true) {
            bad.push(format!(
                "{} ({})",
                v["sha"].as_str().unwrap_or("?"),
                v["reason"].as_str().unwrap_or("unknown")
            ));
        }
    }
    if count == 0 {
        bail!("no commits to verify");
    }
    if !bad.is_empty() {
        bail!("{} unverified commit(s): {}", bad.len(), bad.join(", "));
    }
    Ok(count)
}

fn workspace_version(root: &Path) -> Result<String> {
    let manifest = fs::read_to_string(root.join("Cargo.toml")).context("cannot read Cargo.toml")?;
    manifest
        .lines()
        .find_map(|l| {
            let (k, v) = l.split_once('=')?;
            (k.trim() == "version").then(|| v.trim().trim_matches('"').to_string())
        })
        .ok_or_else(|| anyhow!("no `version` in Cargo.toml"))
}

fn report(name: &str, r: Result<String>) -> ExitCode {
    match r {
        Ok(out) => {
            if !out.is_empty() {
                println!("{out}");
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{name}: {err:#}");
            ExitCode::FAILURE
        }
    }
}

/// `cargo xtask image-tags <ref> <sha>`: print one tag per line.
pub fn run_image_tags(root: &Path, args: &[String]) -> ExitCode {
    report(
        "image-tags",
        || -> Result<String> {
            let [git_ref, sha] = args else {
                bail!("usage: cargo xtask image-tags <ref> <sha>");
            };
            Ok(image_tags(git_ref, sha, &workspace_version(root)?)?.join("\n"))
        }(),
    )
}

/// `cargo xtask verify-signed <owner/repo> <pr>`: every PR commit must be verified by GitHub.
pub fn run_verify_signed(args: &[String]) -> ExitCode {
    report(
        "verify-signed",
        || -> Result<String> {
            let [repo, pr] = args else {
                bail!("usage: cargo xtask verify-signed <owner/repo> <pr-number>");
            };
            let out = Command::new("gh")
            .args(["api", "--paginate", &format!("repos/{repo}/pulls/{pr}/commits")])
            .args(["--jq", ".[] | {sha, verified: .commit.verification.verified, reason: .commit.verification.reason}"])
            .output()
            .context("cannot run `gh`")?;
            if !out.status.success() {
                bail!(
                    "gh api failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                );
            }
            let n = verify_signed(&String::from_utf8_lossy(&out.stdout))?;
            Ok(format!("verify-signed: {n} commit(s) verified"))
        }(),
    )
}

/// `cargo xtask package`: static musl `lsc` copied to `dist/lsc`.
pub fn run_package(root: &Path) -> ExitCode {
    report(
        "package",
        || -> Result<String> {
            let status = Command::new(env!("CARGO"))
                .args([
                    "build",
                    "--release",
                    "--locked",
                    "-j",
                    "2",
                    "-p",
                    "lsc",
                    "--target",
                    TARGET,
                ])
                .current_dir(root)
                .status()
                .context("cannot run cargo")?;
            if !status.success() {
                bail!("release build for {TARGET} failed");
            }
            let built = root.join("target").join(TARGET).join("release/lsc");
            fs::create_dir_all(root.join("dist"))?;
            fs::copy(&built, root.join("dist/lsc"))
                .with_context(|| format!("cannot copy {}", built.display()))?;
            Ok("package: dist/lsc".into())
        }(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA: &str = "8ebd9c1f3a2b4c5d6e7f8091a2b3c4d5e6f70812";

    #[test]
    fn main_gets_main_and_sha_tags() {
        assert_eq!(
            image_tags("refs/heads/main", SHA, "0.1.0").unwrap(),
            ["main", "sha-8ebd9c1"]
        );
    }

    #[test]
    fn matching_version_tag_is_published() {
        assert_eq!(
            image_tags("refs/tags/v0.1.0", SHA, "0.1.0").unwrap(),
            ["0.1.0", "sha-8ebd9c1"]
        );
    }

    #[test]
    fn mismatched_version_tag_is_refused() {
        assert!(image_tags("refs/tags/v0.2.0", SHA, "0.1.0").is_err());
    }

    #[test]
    fn branches_and_pull_requests_are_refused() {
        assert!(image_tags("refs/heads/feat/x", SHA, "0.1.0").is_err());
        assert!(image_tags("refs/pull/5/merge", SHA, "0.1.0").is_err());
    }

    #[test]
    fn bad_sha_is_refused() {
        assert!(image_tags("refs/heads/main", "xyz", "0.1.0").is_err());
    }

    #[test]
    fn all_verified_passes() {
        let j = "{\"sha\":\"a\",\"verified\":true,\"reason\":\"valid\"}\n{\"sha\":\"b\",\"verified\":true,\"reason\":\"valid\"}\n";
        assert_eq!(verify_signed(j).unwrap(), 2);
    }

    #[test]
    fn one_unverified_fails_and_names_it() {
        let j = "{\"sha\":\"a\",\"verified\":true,\"reason\":\"valid\"}\n{\"sha\":\"bad1\",\"verified\":false,\"reason\":\"unsigned\"}\n";
        let e = verify_signed(j).unwrap_err().to_string();
        assert!(e.contains("bad1") && e.contains("unsigned"), "{e}");
    }

    #[test]
    fn empty_input_fails() {
        assert!(verify_signed("").is_err());
    }
}
