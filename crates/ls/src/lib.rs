//! ls (Live Stream & Object Listing Engine)
//!
//! Memory-mapped high-speed POSIX directory, CAS object, and AST structural listing engine with byte-exact ANSI/JSON output.
//!
//! # Architecture
//! Anchored in the systems principles of **Manish Goregaokar & David Tolnay**.

pub use ls_domain::*;

/// Initializes the `ls` systems engine.
pub fn init() -> anyhow::Result<LsState> {
    Ok(LsState::default())
}
