# ls

[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://github.com/unsigned-gg/.github)

> **LeanSpec AST Parser, Structural Conformance Checker, and CAS Listing Engine**  
> Fast, deterministic specification parsing, Lean 4 theorem AST extraction, validation rules (R01–R16), and BLAKE3 Content-Addressed Storage listing.  
> Dual-licensed under `Apache-2.0 OR MIT`.

## Architectural Invariants & Machine Sympathy
- **Substantive Systems Implementation:** Fully functional LeanSpec parser and validation rule engine with zero vanity stubs.
- **Zero-I/O Domain Isolation:** `ls-domain` has zero dependencies on `std::fs`, `std::net`, `std::process`, or async runtimes. All AST models, frontmatter parsing, theorem parsing, and rule validations operate on pure memory buffers.
- **Controlled Language & Verification:** Built for ASD-STE100 technical precision and deterministic build verification.
- **Content-Addressed Storage (CAS):** 256-bit BLAKE3 cryptographic hashing for specification documents, theorem bodies, and directory trees.

## Crate Layout
| Crate | Responsibility |
|---|---|
| `crates/ls-domain` | Pure zero-I/O LeanSpec AST types, Lean 4 theorem statements, frontmatter parser, BLAKE3 CAS algorithms, and rule validators (R01–R16). |
| `crates/ls` | CLI binary (`lsc`) and filesystem scanner for specification suites (`lsc check`, `lsc hash`, `lsc list`, `lsc parse`, `lsc theorems`). |

## CLI Usage

### Check Specification Suite
```bash
# Validate LeanSpec specifications against rules R01-R16
lsc check specs/
```

### Compute CAS Root Hash
```bash
# Compute deterministic BLAKE3 digest of file or directory tree
lsc hash specs/
```

### List Specifications
```bash
# Display tabular summary of specifications with status, priority, theorem count, lines, and BLAKE3 hash
lsc list specs/
```

### Parse Specification to JSON AST
```bash
# Parse a single specification into structured JSON AST with theorems
lsc parse specs/SPEC-0001-leanspec-ast.md
```

### List Formal Theorems
```bash
# List all formal Lean 4 theorems, verification statuses, and tactic counts
lsc theorems specs/
```

### Projectfile CLI Operations
```bash
# Interpolate projectfile string references (§3.8)
lsc pf interp projectfile.yaml

# Resolve org.projectfile.ci intent DAG and print execution plan
lsc pf ci resolve projectfile.yaml

# Lower CI intent DAG into a Rust xtask (stdout) or a deterministic Justfile
lsc pf ci generate projectfile.yaml --target xtask
lsc pf ci generate projectfile.yaml --target just

# Dispatch mode: emit a runner that hands each stage to your own xtask instead of spawning
# it. This repository's `crates/xtask/src/stages.rs` is generated this way
# (`cargo xtask stages-gen`), and `cargo xtask stages-check` fails if it drifts.
lsc pf ci generate projectfile.yaml --target xtask --dispatch

# Derive container image variant tags from org.projectfile.image
lsc pf image tags projectfile.yaml
```

## Building
All builds must specify `-j 2` to prevent memory exhaustion on shared hosts:
```bash
cargo check -j 2
cargo test -j 2
cargo xtask xwork   # full gate: toolchain, specs, projectfile, zero-I/O, fmt, clippy, test, doc-test
make check
make smoke
```

## Specifications & Sessions
- Architectural Roadmap: [ROADMAP.md](ROADMAP.md) · [SPEC-0002-initial-roadmap.md](specs/SPEC-0002-initial-roadmap.md)
- Formal Architecture Specification: [SPEC-0001-leanspec-ast.md](specs/SPEC-0001-leanspec-ast.md)
- Projectfile Engine & CI Lowering: [SPEC-0003-projectfile-ci-lowering.md](specs/SPEC-0003-projectfile-ci-lowering.md)
- Elf Conclave Transcript: [2026-10-11-ls-formal-ast.irc](sessions/2026-10-11-ls-formal-ast.irc)

## License
Dual-licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

## Principles

See the [unsigned-gg principles](https://github.com/unsigned-gg/.github) for the architectural doctrine and community standards.
