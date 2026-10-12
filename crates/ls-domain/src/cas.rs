//! Content-Addressed Storage (CAS) entry and tree listing representations.
//!
//! Pure data structures for directory hashes and CAS structural listings.

use serde::{Deserialize, Serialize};

/// Type classification of an entry in a content-addressed tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CasEntryType {
    /// Standard file / binary blob.
    Blob,
    /// Executable binary or script.
    Executable,
    /// Directory tree.
    Tree,
    /// Symbolic link target.
    Symlink,
}

/// A content-addressed entry within a storage manifest or tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CasEntry {
    /// Cryptographic digest (SHA-256 or BLAKE3 hex string).
    pub hash: String,
    /// Relative path within the scanned root.
    pub rel_path: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Entry type classification.
    pub entry_type: CasEntryType,
}

impl CasEntry {
    /// Create a new CAS entry.
    #[must_use]
    pub fn new(hash: impl Into<String>, rel_path: impl Into<String>, size: u64, kind: CasEntryType) -> Self {
        Self {
            hash: hash.into(),
            rel_path: rel_path.into(),
            size_bytes: size,
            entry_type: kind,
        }
    }
}

/// Aggregated listing of a content-addressed tree.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CasListing {
    /// Root hash representing the collective tree state.
    pub root_hash: String,
    /// All indexed entries ordered by path.
    pub entries: Vec<CasEntry>,
    /// Total aggregated size in bytes.
    pub total_bytes: u64,
}
