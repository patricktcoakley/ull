# úll

úll (Irish for 🍎and written as `ull` when referring to the code) is a family of Rust crates aimed at emulating machines
built around the MOS 6502 lineage. The long-term goal is to successfully emulate various models of 8-bit Apple
computers, including the Apple II, as well as be suitable for emulating other 6502-based systems, such as the NES, Atari
800, and Commodore 64.

## Getting started

### `ull65`

[`ull65`](crates/ull65) is a `no_std` 6502/65C02 CPU core with pluggable buses and instruction sets. More info can be
found in [the README](crates/ull65/README.md).

Examples can be run using `cargo run -p ull65 --example <name>`.

- [`hello_world.rs`](crates/ull65/examples/hello_world.rs) copies
  `"Hello, World!"` into zero-page RAM, halts on `BRK`, then inspects memory.
  ```
  cargo run -p ull65 --example hello_world
  ```
- [`custom_instruction_set.rs`](crates/ull65/examples/custom_instruction_set.rs)
  starts from the MOS table, patches opcode `$00`, and stops once the custom
  handler advances the program counter.
  ```
  cargo run -p ull65 --example custom_instruction_set
  ```
- [`dma_loop.rs`](crates/ull65/examples/dma_loop.rs) exercises `Cpu::tick`
  alongside a bus that keeps DMA bursts in sync with instruction timing.
  ```
  cargo run -p ull65 --example dma_loop
  ```
- [`apple1.rs`](crates/ull65/examples/apple1.rs) wires the core up to a minimal
  Apple I bus so you can boot WOZMON and interact over the terminal to run BASIC.
  ```
  cargo run -p ull65 --example apple1
  ```
- [`nestest.rs`](crates/ull65/examples/nestest.rs) runs the well-known NES CPU
  validation ROM using a Ricoh 2A03-style instruction set with a minimal NES bus.
  ```
  cargo run -p ull65 --example nestest
  ```
