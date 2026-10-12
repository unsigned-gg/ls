# ls

> **LeanSpec AST Parser, Structural Conformance Checker, and CAS Listing Engine**  
> Fast, deterministic specification parsing, AST validation rules (R01–R13), and Content-Addressed Storage listing.  
> Dual-licensed under `Apache-2.0 OR MIT`.

## Architectural Invariants & Machine Sympathy
- **Substantive Systems Implementation:** Fully functional LeanSpec parser and validation rule engine with zero vanity stubs.
- **Zero-I/O Domain Isolation:** `ls-domain` has zero dependencies on `std::fs`, `std::net`, `std::process`, or async runtimes. All AST models, frontmatter parsing, and rule validations operate on pure memory buffers.
- **Controlled Language & Verification:** Built for ASD-STE100 technical precision and deterministic build verification.

## Crate Layout
| Crate | Responsibility |
|---|---|
| `crates/ls-domain` | Pure zero-I/O LeanSpec AST types, frontmatter parser, and rule validators (R01–R13). |
| `crates/ls` | CLI binary (`ls`) and filesystem scanner for specification suites (`ls check`, `ls list`, `ls parse`). |

## CLI Usage

### Check Specification Suite
```bash
# Validate LeanSpec specifications against rules R01-R13
ls check specs/
```

### List Specifications
```bash
# Display tabular summary of specifications with status, priority, and line counts
ls list specs/
```

### Parse Specification to JSON AST
```bash
# Parse a single specification into structured JSON AST
ls parse specs/0001-template.md
```

## Building
All builds must specify `-j 2` to prevent memory exhaustion on shared hosts:
```bash
cargo check -j 2
cargo test -j 2
```

## License
Dual-licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))
