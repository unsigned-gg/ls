# Architecture Specifications (LeanSpec)

> Specifications precede implementation. If a system's invariants cannot be
> articulated in plain technical English, compiling it is merely executing an
> unvetted guess.

This directory is the LeanSpec registry of record for `ls`. Every architectural decision,
boundary rule, and verification gate is specified as an individual, versioned contract.

| Spec | Title | Status | Priority |
|---|---|---|---|
| [SPEC-0001-leanspec-ast.md](./SPEC-0001-leanspec-ast.md) | LeanSpec AST Grammar, CAS & Verification | complete | critical |
| [SPEC-0002-initial-roadmap.md](./SPEC-0002-initial-roadmap.md) | Initial Architecture Roadmap and Development Milestones | in-progress | high |
| [SPEC-0003-projectfile-ci-lowering.md](./SPEC-0003-projectfile-ci-lowering.md) | Projectfile v1 Engine, CI DAG Resolution & Justfile Lowering | complete | critical |
| [SPEC-0004-cargo-xtask-gates.md](./SPEC-0004-cargo-xtask-gates.md) | cargo xtask gates, ci and install | in-progress | high |

## Specification Lifecycle

- `planned`: Under architectural evaluation.
- `in-progress`: Implementation or formal verification active.
- `complete`: Implemented with verified test suites and theorem proofs.
- `archived`: Deprecated or superseded.
