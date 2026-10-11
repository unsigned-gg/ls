//! ls (Live Stream & Object Listing Engine) CLI
use anyhow::Result;

fn main() -> Result<()> {
    println!("ls v0.1.0 - Memory-mapped high-speed POSIX directory, CAS object, and AST structural listing engine with byte-exact ANSI/JSON output.");
    let state = ls::init()?;
    println!("Initialized: {:?}", state);
    Ok(())
}
