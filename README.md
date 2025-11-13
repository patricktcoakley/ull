# úll

úll (Irish for 🍎and written as `ull` when referring to the code) is a family of Rust crates aimed at emulating machines
built around the MOS 6502 lineage. The long-term goal is to successfully emulate various models of Apple computers,
including the Apple II, as well as be usable for emulating other 6502-based systems, such as the NES, Atari 800, and
Commodore 64.

## Workspace layout

- [`crates/ull65`](crates/ull65) – A `no_std` 6502/65C02 CPU core with pluggable buses and instruction sets.

## Getting started

### `ull65`

`ull65` has [its own README](crates/ull65/README.md) and ships with runnable snippets that demonstrate the CPU core in
action.

Each example can be launched with `cargo run -p ull65 --example <name>`.

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