//! Pure domain models and validation rules for `ls`.
//!
//! # Architecture & Invariants
//! Contains zero system call wrappers, zero filesystem operations, and zero network calls.
//! Provides pure LeanSpec AST parsing, formal structural validation rules (R01-R12),
//! and Content-Addressed Storage (CAS) listing types.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod cas;
pub mod document;
pub mod rules;

pub use cas::{CasEntry, CasEntryType, CasListing};
pub use document::{
    SpecDocument, SpecFrontmatter, SpecPriority, SpecSection, SpecStatus, ValidationMessage,
    ValidationReport,
};
pub use rules::{parse_file_name, parse_spec, LEGACY_SECTIONS, MODERN_SECTIONS};
