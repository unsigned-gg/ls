# ls

[![License: Apache-2.0 OR MIT OR CC0-1.0](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT%20OR%20CC0--1.0-blue.svg)](LICENSE-APACHE)

> **LeanSpec AST Parser, Structural Conformance Checker, and CAS Listing Engine**  
> Fast, deterministic specification parsing, Lean 4 theorem AST extraction, validation rules (R01–R16), and BLAKE3 Content-Addressed Storage listing.  
> Tri-licensed under `Apache-2.0 OR MIT OR CC0-1.0`.

## Architectural Invariants & Machine Sympathy
- **Substantive Systems Implementation:** Fully functional LeanSpec parser and validation rule engine with zero vanity stubs.
- **Zero-I/O Domain Isolation:** `ls-domain` has zero dependencies on `std::fs`, `std::net`, `std::process`, or async runtimes. All AST models, frontmatter parsing, theorem parsing, and rule validations operate on pure memory buffers.
- **Controlled Language & Verification:** Built for ASD-STE100 technical precision and deterministic build verification.
- **Content-Addressed Storage (CAS):** 256-bit BLAKE3 cryptographic hashing for specification documents, theorem bodies, and directory trees.

## Crate Layout
| Crate | Responsibility |
|---|---|
| `crates/ls-domain` | Pure zero-I/O LeanSpec AST types, Lean 4 theorem statements, frontmatter parser, BLAKE3 CAS algorithms, and rule validators (R01–R16). |
| `crates/ls` | CLI binary (`ls`) and filesystem scanner for specification suites (`ls check`, `ls hash`, `ls list`, `ls parse`, `ls theorems`). |

## CLI Usage

### Check Specification Suite
```bash
# Validate LeanSpec specifications against rules R01-R16
ls check specs/
```

### Compute CAS Root Hash
```bash
# Compute deterministic BLAKE3 digest of file or directory tree
ls hash specs/
```

### List Specifications
```bash
# Display tabular summary of specifications with status, priority, theorem count, lines, and BLAKE3 hash
ls list specs/
```

### Parse Specification to JSON AST
```bash
# Parse a single specification into structured JSON AST with theorems
ls parse specs/SPEC-0001-leanspec-ast.md
```

### List Formal Theorems
```bash
# List all formal Lean 4 theorems, verification statuses, and tactic counts
ls theorems specs/
```

## Building
All builds must specify `-j 2` to prevent memory exhaustion on shared hosts:
```bash
cargo check -j 2
cargo test -j 2
make check
make smoke
```

## Specifications & Sessions
- Formal Architecture Specification: [SPEC-0001-leanspec-ast.md](specs/SPEC-0001-leanspec-ast.md)
- Elf Conclave Transcript: [2026-10-11-ls-formal-ast.irc](sessions/2026-10-11-ls-formal-ast.irc)

## License
Tri-licensed under any of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Creative Commons Zero 1.0 Universal ([LICENSE-CC0](LICENSE-CC0))

This repository follows the unsigned constitution: [Privacy, Freedom, Dignity](https://github.com/unsigned-gg/.github/blob/main/PRIVACY_FREEDOM_DIGNITY.md) · [Guiding Principles](https://github.com/unsigned-gg/.github/blob/main/GUIDING_PRINCIPLES.md) · [Community Standards](https://github.com/unsigned-gg/.github/blob/main/COMMUNITY_STANDARDS.md) · [Brand Voice](https://github.com/unsigned-gg/.github/blob/main/BRAND_VOICE.md).

## Principles

See the [unsigned-gg principles](https://github.com/unsigned-gg/.github) for the architectural doctrine and community standards.
