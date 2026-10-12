# ls — Architectural Roadmap

> LeanSpec AST Parser, Structural Conformance Checker, and CAS Listing Engine  
> Repo: `unsigned-gg/ls` · Formal Spec: [SPEC-0002-initial-roadmap.md](specs/SPEC-0002-initial-roadmap.md)

## Current State (Phase 1 — Complete)

- **Formal Grammar & AST:** Fully deterministic LeanSpec document parser in `crates/ls-domain` ([SPEC-0001](specs/SPEC-0001-leanspec-ast.md)).
- **Zero-I/O Domain Isolation:** `ls-domain` enforces zero I/O imports (`std::fs`, `std::net`, `std::process` forbidden).
- **Architectural Rules R01–R16:** Enforces frontmatter integrity, line caps (<= 300 warn, <= 400 error), relative links, canonical section ordering, and theorem completeness.
- **Content-Addressed Storage (CAS):** 256-bit BLAKE3 cryptographic hashing for individual documents and directory Merkle trees.
- **CLI Commands:** `ls check`, `ls hash`, `ls list`, `ls parse`, and `ls theorems`.

## Planned Horizons

### Phase 2: Advanced Conformance & Linting (Active)
- **Controlled Language Linter:** Automated ASD-STE100 sentence length checking (<= 20 words for procedural steps).
- **EARS Requirements Validator:** Structural verification of Easy Approach to Requirements Syntax patterns.
- **Cycle Detection:** Tarjan's strongly connected components algorithm for spec dependency graphs.
- **Visual Diagnostics:** Terminal source snippet highlighting and column-accurate error diagnostics.

### Phase 3: Formal Lean 4 & SMT Integration (Next)
- **Lake Project Package Extraction:** Direct interop with Lean 4 build manifests and module namespaces.
- **SMT-LIB2 Solver Translation:** Automated export of linear integer arithmetic and bitvector proof goals into Z3.
- **Coinductive Stream Analysis:** Verification of greatest fixpoint stream specifications and bisimulation invariants.
- **Cross-Spec Lemma Index:** Estate-wide lemma lookup and citation graph addressed by `BLAKE3(name:sig:proof)`.

### Phase 4: High-Performance Engine & IDE Bridge (Later)
- **Bump Arena Allocation:** Contiguous memory buffers for high-throughput batch verification across 10,000+ specifications.
- **CAS Merkle Diff Engine:** `ls diff` command to identify structural and content deltas between two CAS Merkle roots.
- **Agent Communication Protocol (ACP):** Native integration with Zed ACP and IDE language servers.
- **State Store Synchronization:** Bidirectional synchronization with Beads/Dolt and Reverie persistent memory.

## Specifications Index
- [SPEC-0001-leanspec-ast.md](specs/SPEC-0001-leanspec-ast.md) — LeanSpec AST Grammar, CAS & Verification (complete)
- [SPEC-0002-initial-roadmap.md](specs/SPEC-0002-initial-roadmap.md) — Initial Architecture Roadmap and Development Milestones (in-progress)
