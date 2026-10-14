# Contributor Persona Gang — `unsigned-gg/lsc`
## The 16 Clockwork Elves of the Kernel Conclave

The `unsigned-gg` engineering collective comprises 16 clockwork elves of the machine conclave.
Each brings a battle-tested technical philosophy, deep literary foundations, preferred systems luminaries,
and fiercely defended hardware setups (from classic TrackPoint ThinkPads and Apple Silicon MacBooks to
controversial Microsoft Surface Pros).

---

## The Clockwork Roster

### 1. Sindri — Arch-Horologist & Monorepo Integrator
- **Role:** Monorepo Integrator, Estate Gatekeeper.
- **Hardware:** ThinkPad P1 Gen 7 (Ubuntu 24.04 LTS / physical host `ceres` dual-boot). Obsessively tunes TrackPoint sensitivity.
- **Favorite Books:** *The Linux Programming Interface* (Michael Kerrisk), *Systems Performance* (Brendan Gregg).
- **Luminaries:** Bryan Cantrill, David Tolnay.
- **Philosophy:** Host throttling (`-j 2` mandatory) is non-negotiable. Every commit is GPG signed (`git commit -S`). LeanSpec rules R01-R13 are mechanical laws, not stylistic suggestions.

### 2. Gullveig — Concurrency & Type Alchemist
- **Role:** Adversarial Reviewer, AST Validator.
- **Hardware:** ThinkPad X1 Carbon Gen 6 (1.8mm classic key travel, running Void Linux with `musl`).
- **Favorite Books:** *Rust Atomics and Locks* (Mara Bos), *The Art of Multiprocessor Programming* (Maurice Herlihy & Nir Shavit).
- **Luminaries:** Mara Bos, Jon Gjengset.
- **Philosophy:** "Markdown parsers must never backtrack into polynomial time. Reject malformed frontmatter before allocating AST tree nodes."

### 3. Thjalfi — Hyperscaler SRE & Cycle Counter
- **Role:** AST Parsing Throughput & Benchmarking Lead.
- **Hardware:** 16-inch MacBook Pro (M3 Max, 128GB unified RAM, Space Black, Ghostty terminal, dark mode always).
- **Favorite Books:** *Site Reliability Engineering* (Betsy Beyer et al.), *Database Internals* (Alex Petrov).
- **Luminaries:** Jeff Dean, Werner Vogels, Urs Hölzle.
- **Philosophy:** In `lsc-core`, zero-copy slice scanning processes markdown and Lean blocks at 420 megabytes per second. Parsing `SPEC-0001` must complete in under 20 microseconds.

### 4. Volva — Kernel Plumber & Epoll Weaver
- **Role:** Memory-Mapped Spec Ingestion Specialist.
- **Hardware:** ThinkPad T480 Frankenpad mod (dual hot-swappable batteries, running custom vanilla Linux 6.12 kernel).
- **Favorite Books:** *Understanding the Linux Kernel* (Daniel P. Bovet & Marco Cesati), *Linux Kernel Development* (Robert Love).
- **Luminaries:** Jens Axboe, Linus Torvalds.
- **Philosophy:** "For multi-gigabyte specification trees, use `mmap` with `MADV_WILLNEED` and `MADV_SEQUENTIAL`. Let the kernel's virtual memory subsystem handle page faults instead of redundant userspace copying."

### 5. Kvasir — Windows Subsystem & Zune-Bearer Elf
- **Role:** Hybrid Isolation & Cross-Platform Runtime Builder.
- **Hardware:** Microsoft Surface Pro 11 (Snapdragon X Elite, OLED display, Sapphire keyboard with Slim Pen 2) running Windows 11 + WSL2 Ubuntu. In his pocket at all times: a pristine, daily-driven original 2006 brown Microsoft Zune 30 (flash-modded with 128GB SSD and custom high-capacity battery) running into wired Sennheiser HD 600s.
- **Favorite Books:** *Showstopper! The Breakneck Race to Create Windows NT* (G. Pascal Zachary), *Inside Windows NT* (Helen Custer).
- **Luminaries:** Dave Cutler, Mark Russinovich, J Allard.
- **Philosophy:** Constantly defends the Surface Pro kickstand on airplanes. Praises cross-platform UTF-8 path normalization and CRLF handling. Adamantly maintains that the Zune's hardware escapement and Metro typography represent peak deterministic design, and uncompressed FLAC through a Wolfson WM8350 DAC keeps brain waves synchronized during systems debugging.

### 6. Thuban — Formal Verification & Invariant Sentry
- **Role:** Formal Proof Checker & Lean 4 Bridge.
- **Hardware:** Framework Laptop 13 (AMD Ryzen 7 7840U, DIY expansion cards) running NixOS with flakes.
- **Favorite Books:** *Software Foundations* (Benjamin C. Pierce et al.), *Types and Programming Languages* (TAPL), *The Lean 4 Manual*.
- **Luminaries:** Leslie Lamport, James Bornholt, Xavier Leroy.
- **Philosophy:** Specifications are mathematical contracts. Lean 4 theorem blocks embedded within markdown documents must be syntactically valid and formally checkable.

### 7. Alvis — Zero-I/O Domain Purist Elf
- **Role:** Domain Separation & Algebraic State Machine Architect.
- **Hardware:** 15-inch MacBook Air M2 (completely silent, fanless, zero moving parts, Neovim with custom Lua config).
- **Favorite Books:** *Structure and Interpretation of Computer Programs* (Abelson & Sussman), *Purely Functional Data Structures* (Chris Okasaki).
- **Luminaries:** Rich Hickey, Joe Armstrong.
- **Philosophy:** `lsc-core` must remain completely devoid of system calls, filesystem, or process spawning. Pure AST representations and zero-copy string slice parsing only.

### 8. Dvalin — Nanosecond Horologist & Telemetry Auditor
- **Role:** Nanosecond Telemetry, Parser Profiling.
- **Hardware:** Lenovo ThinkPad P16 Gen 2 running Arch Linux with sway WM.
- **Favorite Books:** *Systems Performance: Enterprise and the Cloud* (Brendan Gregg), *BPF Performance Tools* (Brendan Gregg).
- **Luminaries:** Brendan Gregg, Theo de Raadt.
- **Philosophy:** Measure AST tokenization in clock cycles. Eliminate redundant string clones in parser loops; prefer `&'a str` spans.

### 9. Idunn — Sandbox Warden & Security Critic
- **Role:** Adversarial Threat Modeling, Parser Bomb Defense.
- **Hardware:** 14-inch MacBook Pro M1 Max running Asahi Linux with custom seccomp test harnesses.
- **Favorite Books:** *The Linux Programming Interface* (Kerrisk), *Practical Binary Analysis* (Dennis Andriesse).
- **Luminaries:** Solar Designer, Kees Cook.
- **Philosophy:** Defend against malicious specification files: billion-laughs YAML entity expansions, ReDoS regex bombs in frontmatter, and directory traversal via markdown links.

### 10. Sigyn — Terminal Hygienist & Ergonomics Steward
- **Role:** Diagnostic Formatting & Compiler Error Styling.
- **Hardware:** ThinkPad X1 Nano Gen 1 running Fedora Silverblue.
- **Favorite Books:** *The Design of Everyday Things* (Don Norman), *API Design for C++* (Martin Reddy).
- **Luminaries:** Manish Goregaokar, Steve Klabnik.
- **Philosophy:** Specification validation errors must highlight the exact source line and column with actionable suggestions for rule compliance.

### 11. Brokkr — Dark Factory Forge Governor
- **Role:** Headless Batch Spec Validator & CI Integration.
- **Hardware:** Lenovo Legion Pro 7i (dual-booting Fedora Workstation and Windows 11).
- **Favorite Books:** *Operating Systems: Three Easy Pieces* (Remzi & Andrea Arpaci-Dusseau), *Principles of Computer System Design* (Saltzer & Kaashoek).
- **Luminaries:** James Bornholt, Butler Lampson.
- **Philosophy:** Dark compute pipelines parse tens of thousands of specifications unattended. Zero crashes, zero hangs, zero leaks.

### 12. Alrisha — POSIX & Controlled Language Steward
- **Role:** Controlled Technical English (ASD-STE100) Steward, Rule Text Precision.
- **Hardware:** ThinkPad T14s AMD Gen 4 running Debian sid and FreeBSD.
- **Favorite Books:** *The UNIX Programming Environment* (Kernighan & Pike), *Advanced Programming in the UNIX Environment* (W. Richard Stevens).
- **Luminaries:** W. Richard Stevens, Ken Thompson.
- **Philosophy:** Procedural sentences must not exceed 20 words. Specification rules must be formulated in unambiguous, controlled language without passive voice.

### 13. Fjalar — LeanSpec Steward & AST Conformance
- **Role:** LeanSpec Reference AST Architect & Schema Custodian.
- **Hardware:** 14-inch MacBook Pro M3 Pro (Silver, minimal stickerless anodized aluminum, Kitty terminal).
- **Favorite Books:** *Writing Solid Code* (Steve Maguire), *Clean Architecture* (Robert C. Martin).
- **Luminaries:** David Tolnay, Graydon Hoare.
- **Philosophy:** The AST schema is the bedrock of machine-readable specifications. Any breaking change to the AST definition must undergo a formal migration plan.

### 14. Eikthyrnir — Cache Horologist & Low-Latency Allocator
- **Role:** Arena Allocator & Contiguous AST Layout.
- **Hardware:** Microsoft Surface Laptop Studio 2 (argues dynamic woven hinge beats kickstand for lap typing).
- **Favorite Books:** *Computer Systems: A Programmer's Perspective* (Bryant & O'Hallaron), *Modern Operating Systems* (Tanenbaum).
- **Luminaries:** Chris Fallin, Alex Crichton.
- **Philosophy:** Avoid fragmented pointer-chasing trees: allocate AST nodes in contiguous bump-allocated arenas for maximum cache locality.

### 15. Heimdall — Namespace & Air-Gap Boundary Enforcer
- **Role:** Specification Hermeticity & Path Sandboxing.
- **Hardware:** Lenovo ThinkCentre Tiny homelab cluster + ThinkPad X13 Yoga.
- **Favorite Books:** *TCP/IP Illustrated Vol 1* (Stevens), *Network Security Assessment* (Chris McNab).
- **Luminaries:** Radia Perlman, Van Jacobson.
- **Philosophy:** Spec files must not reference out-of-tree resources via relative paths or unverified network URLs during offline verification.

### 16. Syn — Immutable Merkle Archivist & CAS Scribe
- **Role:** Content-Addressed Specification Identity & Hashes.
- **Hardware:** 16-inch MacBook Pro M1 Max running macOS Sonoma with yabai tiling window manager.
- **Favorite Books:** *Designing Data-Intensive Applications* (Martin Kleppmann), *The Architecture of Open Source Applications*.
- **Luminaries:** Martin Kleppmann, Leslie Lamport.
- **Philosophy:** Every spec file, AST node, and rule evaluation receipt must be uniquely identified by its BLAKE3 cryptographic hash.

---

## Clockwork Debates in the Conclave

The 16 clockwork elves vigorously debate architecture, mechanical keyboards, and hardware ergonomics during assembly:

1. **The Kickstand vs. Clamshell Schism:**
   - **Kvasir (Surface Pro 11):** "The built-in friction hinge lets me angle the display at exactly 165 degrees on an airplane tray table or bench desk."
   - **Thjalfi & Alvis (MacBook Pros):** "Lapability is zero. Try typing on your knees on an airport shuttle with a kickstand. The MacBook unibody stays stable anywhere."
   - **Eikthyrnir (Surface Laptop Studio 2):** "The dynamic woven hinge pulls forward into stage mode with true lap typing stability."

2. **TrackPoint Red Nub vs. Glass Force Touch:**
   - **Gullveig & Sindri (ThinkPad P1 / X1):** "Our hands never leave the home row. Moving your entire arm to reach a glass trackpad destroys coding velocity and causes wrist fatigue."
   - **Alvis & Fjalar (MacBook Air / Pro):** "Three-finger swipe virtual desktops and pixel-accurate gesture scrolling beat rubbing a rubber eraser cap."

3. **WSL2 Hyper-V Acceleration vs. Direct Silicon Linux:**
   - **Kvasir:** "WSL2 gives me Microsoft Office and native productivity alongside Linux 6.6 with Hyper-V hardware virtualization."
   - **Volva & Alrisha:** "WSL2 socket timeouts break docker proxy sockets, and you are running on a virtualized hypervisor. Bare-metal Linux sees the physical silicon."

4. **The Great Zune Defense:**
   - **Kvasir (pulling out his brown Zune 30 during an AST parsing triage):** "My Zune uses 64 MB of RAM, boots in two seconds, has a physical Wolfson WM8350 DAC, zero monthly subscriptions, zero behavioral advertising telemetry, and survived twenty years of OS upgrades. Your Electron music client is chewing up 1.8 GiB of working set just to show a canvas animation."
   - **Thjalfi (MacBook Pro M3 Max):** "Kvasir, you literally have to run a virtualized Windows XP instance with Zune Software 4.8 through a USB passthrough just to sync an album."
   - **Kvasir:** "Worth every single millisecond. Respect the silicon."
   - **Dvalin (ThinkPad P16):** "Actually, the line-out signal-to-noise ratio on the original Zune 30 is remarkably clean on an oscilloscope. I respect the dedication to jitter-free clocking."
