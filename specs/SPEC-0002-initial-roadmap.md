---
status: in-progress
created: 2026-10-12
tags: [leanspec, roadmap, milestones, cas, verification]
priority: high
scope: ls-roadmap
depends_on: [SPEC-0001-leanspec-ast]
---

# SPEC-0002-initial-roadmap

## Requirements

1. The roadmap shall specify the architectural development phases for `ls`.
2. Phase 1 shall establish the zero-I/O LeanSpec AST parser and BLAKE3 CAS listing foundation.
3. Phase 2 shall implement advanced conformance rules, ASD-STE100 linting, and EARS syntax verification.
4. Phase 3 shall provide automated SMT-LIB2 proof discharge into Z3 and coinductive stream analysis.
5. Phase 4 shall deliver contiguous arena allocation for mass multi-tenant specification scanning.
6. The roadmap shall maintain strict zero-I/O isolation in `lsc-core`.
7. Every architectural phase shall define testable completion criteria.

```lean
namespace Unsigned.LeanSpec.Roadmap

/-- Architectural maturity phases for the LeanSpec listing engine. -/
inductive RoadmapPhase : Type where
  | phase1_ast_and_cas : RoadmapPhase
  | phase2_conformance_rules : RoadmapPhase
  | phase3_smt_discharge : RoadmapPhase
  | phase4_arena_allocation : RoadmapPhase
  | phase5_coinductive_streams : RoadmapPhase

/-- Ordinal mapping for architectural phases. -/
def phase_ordinal (p : RoadmapPhase) : Nat :=
  match p with
  | RoadmapPhase.phase1_ast_and_cas => 1
  | RoadmapPhase.phase2_conformance_rules => 2
  | RoadmapPhase.phase3_smt_discharge => 3
  | RoadmapPhase.phase4_arena_allocation => 4
  | RoadmapPhase.phase5_coinductive_streams => 5

/-- Theorem: All roadmap phases possess a positive ordinal index. -/
theorem phase_ordinal_positive (p : RoadmapPhase) : phase_ordinal p >= 1 := by
  omega
```

## Design

### Architecture Evolution Horizons

The development of `ls` proceeds across four phased horizons. Each horizon expands verification rigor without sacrificing microsecond execution speed.

```
Development Horizons
├── Phase 1: Core AST & CAS Foundation (Complete)
│   ├── Pure domain model in crates/lsc-core
│   ├── YAML frontmatter and section parser
│   ├── Architectural validation rules R01-R16
│   └── BLAKE3 Content-Addressed Storage listing
├── Phase 2: Advanced Conformance & Linting (Active)
│   ├── ASD-STE100 sentence and terminology linter
│   ├── EARS requirement keyword checker
│   ├── Tarjan dependency cycle graph analysis
│   └── Source code snippet diagnostic reporting
├── Phase 3: Formal Lean 4 & SMT Discharge (Next)
│   ├── Lake project package integration
│   ├── SMT-LIB2 automated proof obligations for Z3
│   ├── Coinductive stream guardedness checking
│   └── Cross-specification lemma index by BLAKE3 hash
└── Phase 4: High-Throughput Engine & IDE Bridge (Later)
    ├── Chunked arena allocators for 10,000+ specifications
    ├── CAS Merkle tree diffing (ls diff)
    ├── Zed Agent Communication Protocol (ACP) server
    └── Dolt/Beads and Reverie memory synchronization
```

### Phase Breakdown & Acceptance Criteria

#### Phase 1: Core AST & CAS Foundation (Status: Complete)
- Implement pure `lsc-core` without `std::fs`, `std::net`, or async runtimes.
- Extract YAML frontmatter, canonical H2 sections, and embedded Lean 4 theorem blocks.
- Compute deterministic 256-bit BLAKE3 hashes for documents and directory trees.
- Provide CLI subcommands: `check`, `hash`, `list`, `parse`, and `theorems`.
- Enforce host physical throttling (`-j 2`) across all compilation steps.

#### Phase 2: Advanced Conformance & Linting (Status: In-Progress)
- Implement ASD-STE100 sentence length validator (<= 20 words for procedural steps).
- Implement Easy Approach to Requirements Syntax (EARS) clause validation.
- Detect cross-file dependency cycles using Tarjan strongly connected components.
- Enhance CLI diagnostics with source snippets and column pointers.
- Maintain zero-I/O purity in `lsc-core`.

#### Phase 3: Formal Lean 4 & SMT Discharge (Status: Planned)
- Support bidirectional Lean 4 Lake manifest extraction.
- Translate linear arithmetic theorem goals into SMT-LIB2 queries for Z3.
- Formalize greatest fixpoint coinductive stream invariants for reactive specs.
- Index proven lemmas in estate-wide content-addressed store.

#### Phase 4: High-Throughput Engine & IDE Bridge (Status: Planned)
- Replace per-document heap allocations with contiguous arena buffers.
- Benchmark mass parsing throughput exceeding 500 MB/s across 10,000 specs.
- Implement Merkle tree difference engine (`ls diff tree_a tree_b`).
- Expose JSON-RPC and Zed ACP language server endpoints.

## Tasks

- [x] Phase 1: Implement zero-I/O `lsc-core` AST models and parser.
- [x] Phase 1: Implement BLAKE3 CAS hashing and Merkle tree generation.
- [x] Phase 1: Deliver CLI tools: `ls check`, `ls hash`, `ls list`, `ls parse`, `ls theorems`.
- [x] Phase 1: Author formal grammar specification [SPEC-0001-leanspec-ast.md](./SPEC-0001-leanspec-ast.md).
- [ ] Phase 2: Add ASD-STE100 sentence length checker to `ls check`.
- [ ] Phase 2: Implement EARS clause prefix validation in `lsc-core`.
- [ ] Phase 2: Implement Tarjan topological cycle detector for spec dependencies.
- [ ] Phase 2: Add terminal color diagnostics with source snippet context.
- [ ] Phase 3: Implement SMT-LIB2 arithmetic proof translation.
- [ ] Phase 3: Add coinductive stream guardedness checking.
- [ ] Phase 4: Implement bump arena allocator for batch directory scanning.
- [ ] Phase 4: Add `ls diff` command for CAS Merkle tree comparison.

## Verification

Execute test suites and verify specification conformance:

```bash
cargo test -j 2
cargo run -p lsc -j 2 -- check specs
cargo run -p lsc -j 2 -- hash specs
cargo run -p lsc -j 2 -- list specs
cargo run -p lsc -j 2 -- theorems specs
```

## Notes

- Specifications adhere to ASD-STE100 controlled language rules.
- Pure domain models remain in `crates/lsc-core` with zero I/O imports.
- Host physical throttling mandates `-j 2` across all compilation steps.
