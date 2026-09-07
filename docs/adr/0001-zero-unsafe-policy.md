# 0001. `#![forbid(unsafe_code)]`, pure Rust, no FFI

- **Status:** accepted
- **Date:** 2024-10-27 (recorded 2026-09-07)

## Context

The crate reads content an author writes and writes a whole site to
disk. A memory-safety bug here corrupts published output. Nothing in
the problem needs raw pointers, and every crate it builds on is pure
Rust.

## Decision

`#![forbid(unsafe_code)]` at the crate root; no C dependencies, no FFI.
A dependency that requires `unsafe` in this crate's own code, or that
pulls in a C build, is a reason to pick a different dependency.

## Consequences

- The compiler proves the absence of unsafe blocks.
- Miri's job here is to check the interaction with dependencies that
  use `unsafe` internally, not this crate's own code.
