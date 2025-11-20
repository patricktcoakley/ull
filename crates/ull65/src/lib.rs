//! A `no_std` emulator library for MOS 6502 and WDC 65C02 microprocessors.
//!
//! `ull65` provides a flexible, trait-based architecture for emulating 6502-family CPUs.
//! The library is designed for extensibility, allowing custom memory implementations,
//! instruction sets, and addressing modes.
//!
//! # Architecture
//!
//! The emulator is built around three core abstractions:
//!
//! - **[`Bus`]**: Memory and I/O abstraction. Implement this trait to provide custom
//!   memory-mapped hardware, banking, or I/O devices.
//! - **[`instruction::InstructionSet`]**: Defines the instruction table for a CPU variant (e.g., MOS 6502,
//!   WDC 65C02, or custom extensions).
//! - **[`Cpu`]**: The CPU core that executes instruction using a specified bus and
//!   instruction set.
//!
//! # Quick Start
//!
//! ```
//! use ull::{Byte, Word};
//! use ull65::{AccessType, Cpu, SimpleBus, instruction::mos6502::Mos6502};
//!
//! let mut bus = SimpleBus::default();
//! let program = [0xA9, 0x42, 0x00]; // LDA #$42; BRK
//! let mut cpu: Cpu<SimpleBus> =
//!     Cpu::with_program::<Mos6502>(&mut bus, Word(0x8000), &program, Word(0x8000));
//!
//! let cycles = cpu.tick(&mut bus);
//! assert!(cycles > 0);
//! assert_eq!(cpu.a, Byte(0x42));
//! ```
//!
//! # Type-Safe Primitives
//!
//! The library uses newtype wrappers ([`Byte`], [`Word`]) for type safety and to provide
//! convenient operator overloads with wrapping arithmetic, matching 6502 behavior.
//!
//! # Examples
//!
//! - See [`hello_world`](https://github.com/patricktcoakley/ull/blob/main/crates/ull65/examples/hello_world.rs)
//!   for the quickest possible demo using [`Cpu::run_until`].
//! - See [`custom_instruction_set`](https://github.com/patricktcoakley/ull/blob/main/crates/ull65/examples/custom_instruction_set.rs)
//!   for how to extend or patch the instruction table.
//! - See [`custom_bus`](https://github.com/patricktcoakley/ull/blob/main/crates/ull65/examples/custom_bus.rs)
//!   for implementing memory-mapped I/O.
//!
//! For a broader architectural overview and onboarding guide, read
//! [`crates/ull65/README.md`](https://github.com/patricktcoakley/ull/blob/main/crates/ull65/README.md).

#![no_std]
// 6502 math wraps and uses a sign bit
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
// Instruction table
#![allow(clippy::too_many_lines)]

extern crate alloc;

pub mod access;
pub mod bus;
pub mod instruction;
pub mod processor;

/// Re-export core primitives/bus for convenience so downstream users can depend on `ull65`
/// only, while internal modules still import them explicitly from `ull`.
pub use access::{AccessType, Phase, ResetVectorExt};
pub use bus::{Mos6502CompatibleBus, simple_bus::SimpleBus, testing_bus::TestingBus};
pub use instruction::{Instruction, InstructionSet, InstructionTable};
pub use processor::addressing_mode::{self, AddressingMode};
pub use processor::run::{RunConfig, RunOutcome, RunPredicate, RunSummary};
pub use processor::{
    cpu::{
        IRQ_VECTOR_HI, IRQ_VECTOR_LO, NMI_VECTOR_HI, NMI_VECTOR_LO, RESET_VECTOR_HI,
        RESET_VECTOR_LO, STACK_SPACE_START,
    }, Cpu,
    RunState,
};
