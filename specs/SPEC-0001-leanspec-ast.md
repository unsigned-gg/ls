---
status: complete
created: 2026-10-11
tags: [leanspec, formal-spec, ast, cas, blake3]
priority: critical
scope: ls-parser
depends_on: []
---

# SPEC-0001-leanspec-ast

## Requirements

1. The parser shall read specification documents without performing system input or output operations.
2. The parser shall extract YAML frontmatter, canonical Markdown sections, and embedded Lean 4 blocks.
3. Every specification document shall map to a deterministic Abstract Syntax Tree (AST).
4. Content-Addressed Storage (CAS) operations shall compute 256-bit BLAKE3 hashes for documents and trees.
5. The parser shall enforce architectural rules R01 through R16.
6. The verification engine shall detect unproved `sorry` placeholders in completed specifications.
7. The parser shall reject cyclic dependencies and enforce a maximum AST recursion depth of 64 levels.

```lean
namespace Unsigned.LeanSpec

/-- Proof status classification of a formal specification theorem. -/
inductive ProofStatus : Type where
  | proved : Nat -> ProofStatus
  | sorry : ProofStatus
  | axiom : ProofStatus
  | omitted : ProofStatus

/-- A verified LeanSpec theorem declaration. -/
structure LeanTheorem where
  name : String
  signature : String
  status : ProofStatus
  hash : String

/-- Theorem stating that empty data has non-negative size. -/
theorem empty_data_size_nonnegative (len : Nat) : len >= 0 := by
  omega
```

## Design

### Grammar Specification

The LeanSpec document grammar combines YAML frontmatter, Markdown structural headings, and formal Lean 4 blocks.

```ebnf
SpecDocument     ::= Frontmatter '\n' H1Heading '\n' Sections
Frontmatter      ::= '---' '\n' (YamlPair '\n')+ '---'
H1Heading        ::= '# ' Stem
Sections         ::= (H2Section)+
H2Section        ::= '## ' SectionTitle '\n' SectionBody
SectionBody      ::= (Paragraph | CodeBlock | LeanBlock)*
LeanBlock        ::= '```lean' '\n' LeanModule '\n' '```'
LeanModule       ::= (ImportDecl | NamespaceDecl | LeanDecl)*
LeanDecl         ::= TheoremDecl | DefDecl | InductiveDecl | StructDecl | AxiomDecl
TheoremDecl      ::= ('theorem' | 'lemma') Ident Binders? ':' Proposition ':=' ProofBody
```

### Structural AST Nodes

```
SpecDocument
├── stem: String
├── number: String
├── slug: String
├── h1: String
├── frontmatter: SpecFrontmatter
│   ├── status: SpecStatus
│   ├── created: String
│   ├── tags: [String]
│   ├── priority: Option<SpecPriority>
│   └── depends_on: [String]
├── sections: [SpecSection]
│   ├── title: String
│   ├── start_line: usize
│   └── body: String
├── theorems: [LeanTheorem]
│   ├── name: String
│   ├── signature: String
│   ├── proof_status: ProofStatus
│   └── hash: String
├── content_hash: String (BLAKE3)
└── line_count: usize
```

### Content-Addressed Storage (CAS)

Every file and directory tree maps to a cryptographic digest:
1. File Digest: `BLAKE3(file_bytes)` hex string.
2. Directory Merkle Hash: Hash of sorted entries `rel_path:hash:size:kind\n`.
3. Theorem Digest: `BLAKE3(name:signature:proof_body)`.

### Verification Rules

| Rule | Severity | Condition |
|---|---|---|
| R01 | Error | File name must match `NNNN-kebab-slug.md` or `SPEC-NNNN-kebab-slug.md`. |
| R02 | Error | Document must begin with valid YAML frontmatter fence `---`. |
| R03 | Error | Referenced `depends_on` stems must exist in the specification suite. |
| R04 | Error | Top-level H1 title must equal the file stem. |
| R05 | Warning | H2 sections must follow canonical order. |
| R10 | Error / Warning | Max 400 lines (Error), warning if > 300 lines. |
| R11 | Warning | Specifications must be cataloged in `specs/README.md`. |
| R12 | Error | Relative markdown links must resolve to existing files. |
| R14 | Warning | Theorems with `sorry` in `complete` specs emit verification warnings. |
| R15 | Error | Rejects self-dependency cycles and parser depth > 64. |
| R16 | Warning | Theorem names must follow Lean 4 identifier conventions. |

## Tasks

- [x] Implement pure `lsc-core` AST models and zero-I/O Lean 4 parser.
- [x] Implement BLAKE3 CAS hashing and Merkle tree computation.
- [x] Implement CLI commands: `ls check`, `ls hash`, `ls list`, `ls parse`, `ls theorems`.
- [x] Wire open-source CI/CD workflow with clippy, test, check, audit.
- [x] Create formal specification registry and catalog index.

## Verification

Execute the test suite and verify architectural conformance:

```bash
cargo test -j 2
```

Validate specification directory conformance and CAS root hash:

```bash
cargo run -p lsc -j 2 -- check specs
cargo run -p lsc -j 2 -- hash specs
```

## Notes

- Specifications follow ASD-STE100 controlled language rules.
- Pure domain models live in `crates/lsc-core` with zero I/O imports.
- Host physical throttling mandates `-j 2` across all local build steps.
