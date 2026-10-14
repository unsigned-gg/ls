---
status: in-progress
created: 2026-10-14
tags: [xtask, gates, ci, tools, toolchain]
priority: high
scope: ls-xtask
depends_on: [SPEC-0003-projectfile-ci-lowering]
---

# SPEC-0004-cargo-xtask-gates

## Requirements

Operator decision, Christian, 2026-10-13: `cargo xtask` is the one build entrypoint for every
estate repo, with nothing behind it but cargo and rustup. These gates sit beside `xwork`, the
Makefile and the existing CI jobs. Nothing is removed.

1. **XT-1 (ubiquitous):** xtask shall provide the commands `tools`, `toolchain-check`, `fmt`,
   `fmt-check`, `clippy`, `test`, `doc-test`, `deny`, `typos`, `machete`, `ci` and `install`, and
   keep every other command under its name.
2. **XT-2 (ubiquitous):** xtask shall keep the version of each tool it runs in one table:
   cargo-deny 0.19.9, typos-cli 1.49.0, cargo-machete 0.9.1 and cargo-nextest 0.9.104.
3. **XT-3 (event-driven):** When `tools` runs, xtask shall ask each tool for its version the way
   the gates run it, with `.tools/bin` first on `PATH`. Only for a tool that is missing or at
   another version shall it run `cargo install --locked --version <pin> --root .tools --force
   <crate>`, under Rust 1.94.1 (`rustup run --install 1.94.1`), because typos-cli 1.49.0 needs
   Rust 1.91 and the pinned toolchain is 1.85.0.
4. **XT-4 (unwanted behaviour):** If a tool still reports another version after its install, then
   `tools` shall fail and name the tool and both versions.
5. **XT-5 (unwanted behaviour):** If `rustc --version` differs from the `channel` in
   `rust-toolchain.toml`, then `toolchain-check` shall fail and name both.
6. **XT-6 (ubiquitous):** Each gate shall run from the repository root with `.tools/bin` first on
   `PATH`, and without the variables cargo sets for the xtask binary itself (`CARGO_PKG_*`,
   `CARGO_MANIFEST_DIR`, `CARGO_MANIFEST_PATH`, `CARGO_CRATE_NAME`, `CARGO_BIN_NAME`,
   `CARGO_PRIMARY_PACKAGE`). N is `JOBS`, else 2.
7. **XT-7 (event-driven):** When `ci` runs, xtask shall run `tools`, `toolchain-check`,
   `fmt-check`, `clippy`, `test`, `doc-test`, `deny`, `typos` and `machete` in that order. It shall
   stop at the first step that fails, print one summary line per step, and exit non-zero if a
   step failed.
8. **XT-8 (event-driven):** When `install` runs, xtask shall run `ci`, then
   `cargo build --release --workspace --exclude xtask --bins -j N`.
9. **XT-9 (event-driven):** When CI runs for a push or a pull request, the job `xtask-ci` shall run
   `cargo xtask ci` on `ubuntu-latest`, the runner the existing Rust jobs use.

## Design

| Gate | Runs |
|---|---|
| `fmt` | `cargo fmt --all` |
| `fmt-check` | `cargo fmt --all -- --check` |
| `clippy` | `cargo clippy --workspace --all-targets -j N -- -D warnings` |
| `test` | `cargo nextest run --workspace --build-jobs N`; `cargo test --workspace -j N` when nextest is missing |
| `doc-test` | `cargo test --doc --workspace -j N` |
| `deny` | `cargo deny check` |
| `typos` | `typos` |
| `machete` | `cargo machete` |

`.gitignore` keeps `Cargo.lock` out of the repository, so no gate passes `--locked`.
`cargo install --locked` for the tools uses each tool's own published lock file.

The code is `crates/xtask/src/gates.rs`. `.tools/` is gitignored.

## Tasks

- [x] Write the clauses XT-1 to XT-9.
- [ ] Add unit tests named for the clauses, red first, then the gates (XT-1 to XT-8).
- [ ] Add the CI job `xtask-ci` (XT-9).

## Verification

```bash
cargo test -p xtask -j 2 -- gates::
cargo xtask ci
```

## Notes

- `cargo xtask ci` needs only cargo and rustup.
- The make target `smoke` and the `xwork` pipeline are unchanged.
