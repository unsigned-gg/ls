# ls

> ls (Live Stream & Object Listing Engine)  
> Memory-mapped high-speed POSIX directory, CAS object, and AST structural listing engine with byte-exact ANSI/JSON output.  
> Dual-licensed under `Apache-2.0 OR MIT`.

## Philosophical Foundations
- **Champion Inspiration:** Manish Goregaokar & David Tolnay
- **Zero Days Open Source Initiative (ZOSI):** Machine sympathy, low-level efficiency, and verified contracts.
- **Architecture:** Zero-I/O domain model in `ls-domain` separated from systems runtime in `ls`.

## Crates
- `crates/ls`: Systems implementation and runtime interfaces.
- `crates/ls-domain`: Pure zero-I/O domain models, validation, and algebraic state machines.

## Building
All builds must specify `-j 2` to prevent memory exhaustion on shared multi-agent hosts:
```bash
cargo build -j 2
cargo test -j 2
```
