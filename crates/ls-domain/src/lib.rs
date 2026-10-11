//! Zero-I/O domain models for `ls`.
//!
//! # Invariants
//! This crate contains pure data structures, deterministic state transitions,
//! and verification assertions. It does not perform I/O, network calls, or system operations.

use serde::{Deserialize, Serialize};

/// Core state representation for `ls`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LsState {
    /// Protocol version.
    pub version: u32,
    /// Identifier.
    pub id: String,
}

impl Default for LsState {
    fn default() -> Self {
        Self {
            version: 1,
            id: String::from("initial"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = LsState::default();
        assert_eq!(state.version, 1);
        assert_eq!(state.id, "initial");
    }
}
