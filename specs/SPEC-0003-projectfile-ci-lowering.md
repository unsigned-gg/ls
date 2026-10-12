---
status: complete
created: 2026-10-12
tags: [projectfile, ci-compiler, interpolation, image-tags, justfile, zero-io]
priority: critical
scope: ls-projectfile
depends_on: [SPEC-0001-leanspec-ast]
---

# SPEC-0003-projectfile-ci-lowering

## Requirements

1. The projectfile engine shall operate without system input or output operations.
2. The interpolation resolver shall resolve balanced references and cartesian expansions according to §3.8.
3. The CI engine shall resolve `org.projectfile.ci` intent DAGs with strict cycle refusal.
4. The CI engine shall infer graph sinks when no explicit goal is flagged.
5. The image engine shall derive container image tags according to `org.projectfile.image`.
6. The image engine shall reject variant collisions and publishing head shadowing.
7. The compiler shall lower resolved CI intent DAGs into deterministic `Justfile` recipes.

```lean
namespace Unsigned.Projectfile

/-- CI timing classification for an execution node. -/
inductive TimingClass : Type where
  | source : TimingClass
  | cell : TimingClass
  | join : TimingClass

/-- A verified CI graph node representation. -/
structure CiNode where
  name : String
  timing : TimingClass
  is_goal : Bool

/-- Theorem stating cell multiplier is positive for non-empty matrix. -/
theorem cell_count_positive (cells : Nat) (h : cells > 0) : cells >= 1 := by
  omega
```

## Design

### Zero-I/O Domain Engine Architecture

The projectfile engine in `crates/ls-domain` executes completely in memory without filesystem, process, or network access.

```
projectfile Engine Pipeline
┌───────────────────────────┐
│ In-Memory Document (YAML) │
└─────────────┬─────────────┘
              │
      ┌───────┴───────────────────────┬────────────────────────┐
      ▼                               ▼                        ▼
┌───────────────┐             ┌───────────────┐        ┌───────────────┐
│     §3.8      │             │  org.ci DAG   │        │   org.image   │
│ Interpolation │             │  Resolution   │        │ Tag Derivation│
└───────┬───────┘             └───────┬───────┘        └───────┬───────┘
        │                             │                        │
        │                             ▼                        │
        │                     ┌───────────────┐                │
        │                     │ Justfile / GHA│                │
        │                     │ Lowering AST  │                │
        │                     └───────┬───────┘                │
        ▼                             ▼                        ▼
┌──────────────────────────────────────────────────────────────┐
│                  Deterministic Output String                 │
└──────────────────────────────────────────────────────────────┘
```

### Component Modules

1. **§3.8 Interpolation (`interp.rs`):**
   - Resolves `${address}` references with balanced braces.
   - Evaluates filter transformations (`| env`).
   - Supports Cartesian product line expansion (`interpolate_lines`).
   - Limits recursive depth to 10 iterations to prevent infinite cycles.

2. **CI Intent DAG Resolution (`ci.rs`):**
   - Merges reusable preset `includes` with base `input` documents.
   - Applies local overrides (`false` removes dependencies).
   - Detects cycles using three-color depth-first search.
   - Infers graph sinks when no explicit `goal: true` is declared.
   - Prunes unreachable nodes outside the goal closure.
   - Classifies timing semantics (`SOURCE`, `CELL`, `JOIN`).

3. **Container Image Tagging (`image.rs`):**
   - Derives version facts from semver strings or branch names.
   - Expands publishing heads (`release`, `prerelease`, `trunk`, `branch`).
   - Generates variant alias combinations from matrix axes and build args.
   - Refuses collisions across cells (`cell-collision`).
   - Refuses variants that shadow publishing heads (`head-is-variant`).

4. **Justfile Lowering Compiler (`just_target.rs`):**
   - Translates resolved tool DAGs into deterministic recipes.
   - Emits shell directives (`set shell := ["bash", "-euo", "pipefail", "-c"]`).
   - Enforces execution dependencies through recipe prerequisites.

## Tasks

- [x] Implement §3.8 interpolation engine with balanced bracket parsing in `interp.rs`.
- [x] Implement `org.projectfile.ci` DAG resolver with cycle detection in `ci.rs`.
- [x] Implement `org.projectfile.image` variant and head tag derivation in `image.rs`.
- [x] Implement deterministic `Justfile` lowering compiler in `just_target.rs`.
- [x] Expose CLI subcommands (`ls pf interp`, `ci resolve`, `ci generate`, `image tags`).
- [x] Validate 100% of specification conformance vectors (43 test fixtures).

## Verification

Execute test suites and verify specification conformance:

```bash
cargo test -j 2 --test conformance_tests
cargo run -p ls -j 2 -- pf ci resolve crates/ls-domain/tests/conformance/ci/matrix-exclude.yaml
cargo run -p ls -j 2 -- pf ci generate crates/ls-domain/tests/conformance/ci/matrix-exclude.yaml
cargo run -p ls -j 2 -- pf image tags crates/ls-domain/tests/conformance/image/two-axes.yaml
```

## Notes

- All implementations adhere strictly to the zero-I/O invariant.
- Conforms to normative specification <https://projectfile.org/spec/v1>.
- Prose follows ASD-STE100 controlled language rules.
