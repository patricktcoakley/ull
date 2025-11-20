# ull65

`ull65` is a `no_std` CPU core for the MOS 6502 and WDC 65C02 (with plans for more).

## Core concepts

The primary goal of `ull65` is to be easy to use and extend, with a secondary focus on being portable and (eventually)
cycle-accurate. To that end, the public API is mostly focused on ergonomics for the end-user, with a few key
features that make it easy to customize the core for specific use cases:

- **Pluggable buses**
    - `ull65` leverages the [`ull::Bus`](../ull/README.md) abstraction, so memory maps and peripherals live in your bus
      implementation in a consistent API across all systems.
- **Instruction-sets**
    - swap or patch opcode tables with the `InstructionSet` trait instead of duplicating the CPU core for each variant,
      making it easy to implement new CPU cores without much effort.
- **Deterministic stepping**
    - drive the CPU with `run`, `run_until`, or single-cycle `tick` calls and keep DMA/peripheral time in lockstep with
      the processor.

## Architecture overview

`ull65` is organized around a few composable building blocks:

- `Cpu<B>` is the core execution engine parameterized over a `Bus`
  implementation. It owns the registers/flags, exposes helpers like
  `with_program`, `with_reset_vector`, `run`, `run_until`, and single-cycle
  `tick`, and keeps track of elapsed cycles.
- `Bus` is a trait you implement to wire memory and peripherals. The CPU uses it
  for every instruction fetch/data access plus timing hooks:
    - `read`/`write` for memory accesses
    - `on_tick` to let the bus advance its own clocks
    - `request_dma`/`poll_dma_cycle` to model DMA bursts
- `InstructionSet` is a high-level description of a CPU flavor. Implement this
  trait to tell the core which opcode table to run, whether decimal mode is
  available, and so on.
- `InstructionTable` is a dense array of 256 `Instruction` entries. You usually
  get one by calling `Mos6502::base_table()` or `Wdc65c02s::base_table()` and
  optionally patching a few slots.
- `Instruction` is the executable payload stored in each table slot. It contains
  the cycle count and a function pointer (`fn(&mut Cpu<B>, &mut B)`) that
  performs the opcode’s work.
- `RunConfig`/`RunPredicate` are control structures for `run_until`, letting you
  stop on BRK, on predicates (e.g., “PC reached $C000”), or after a cycle limit.
- `Nibble`/`Byte`/`Word` are tiny newtypes to handle things like wrapping addition or subtraction and added conveniences
  for working with the various types of the 6502 without having to use `as` or `::from` calls everywhere.

## Error handling

The original 6502 has no concept of traps or recoverable faults because addresses wrap at
16 bits, arithmetic wraps at 8/16 bits, and undefined memory just returns
whatever the bus supplies. `ull65` mirrors that behavior by leveraging wrapped arithemtic internally, so the CPU always
continues executing unless you halt it explictly via `RunConfig` options, BRK handlers, or your own `Bus` logic.
If you need to detect error conditions (e.g., illegal opcodes, out-of-range accesses), add the checks inside your `Bus`
implementation or a patched `InstructionSet` so the behavior stays explicit.

## Basic usage

The general workflow for `ull65` is:

1. Implement a `Bus` that mirrors your target machine.
2. Choose or define an `InstructionSet`.
3. Instantiate `Cpu::<YourBus>` and load code using whichever constructor fits:
   `with_program::<YourInstructionSet>` for ad-hoc buffers, or
   `with_reset_vector::<YourInstructionSet>` when you want the ROM’s reset
   vector to run.
4. Drive it via `run`, `run_until`, or `tick`, letting the bus handle timing and optional
   DMA callbacks.

```rust
use ull::Word;
use ull65::instruction::mos6502::Mos6502;
use ull65::processor::run::RunConfig;
use ull65::{AccessType, Cpu, SimpleBus};

fn main() {
    let mut bus = SimpleBus::default();
    let program = vec![0xA9, 0x01, 0x8D, 0x00, 0x02, 0x00]; // LDA #$01; STA $0200; BRK
    let mut cpu = Cpu::with_program::<Mos6502>(&mut bus, Word(0x8000), &program, Word(0x8000));
    let summary = cpu.run_until(&mut bus, RunConfig { stop_on_brk: true, ..RunConfig::default() });
    let value: u8 = bus.read(Word(0x0200), AccessType::DataRead).into();
    println!("Summary: {summary:?}, memory[$0200]={value:02X}");
} 
```

## Customizing instruction sets

`InstructionSet` is the abstraction that tells the CPU which opcode table to
execute. Each instruction table is just an array of 256 `Instruction`s:

```rust
pub struct Instruction<B: Bus> {
    pub cycles: u8,
    pub execute: fn(&mut Cpu<B>, &mut B),
}
```

The stock tables (`Mos6502` and `Wdc65c02s`) cover the common CPU variants.
Start from whichever base table matches your target (`Mos6502::base_table()`
or `Wdc65c02s::base_table()`) and then patch it further if needed, or construct
an entirely custom ISA.

### Toggle feature flags

Many 6502 derivatives only differ by feature flags (e.g., BCD mode). Set the
associated constants on your `InstructionSet` to opt in/out without touching the
table itself:

```rust
impl InstructionSet for Ricoh2a03 {
    fn instruction_table<B: Bus + 'static>() -> InstructionTable<B> {
        Mos6502::base_table()
    }

    const SUPPORTS_DECIMAL_MODE: bool = false;
}
```

### Patch specific opcodes

When you need to replace individual opcodes you can use `with` on the instruction table to patch in a new
`Instruction`. This lets you intercept undocumented opcodes, add tracing, or emulate chips that diverge in only a
handful of instructions:

```rust
impl InstructionSet for MyCustomCpu {
    fn instruction_table<B: Bus + 'static>() -> InstructionTable<B> {
        Mos6502::base_table::<B>().with(
            0x00,
            Instruction {
                cycles: 7,
                execute: custom_brk::<B>,
            },
        )
    }
}
```

Here we keep the MOS behavior and only replace `BRK` with a custom trap handler.

## Examples

The examples directory (`crates/ull65/examples`) contains runnable snippets that
double as API demonstrations. Run them with `cargo run --example <name>`.
