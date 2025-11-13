//! Instruction sets and dispatch tables.
//!
//! Implement [`InstructionSet`] to define CPU variants or patch existing tables

use crate::{Bus, Cpu};
use core::ops::Index;

pub mod mos6502;
pub mod wdc65c02s;

/// A single instruction with cycle count and execution function.
#[derive(Debug, Copy, Clone)]
pub struct Instruction<B: Bus> {
    pub cycles: u8,
    pub execute: fn(&mut Cpu<B>, &mut B),
}

/// 256-entry instruction table mapping opcodes to instructions.
pub struct InstructionTable<B: Bus>([Instruction<B>; 256]);

impl<B: Bus> InstructionTable<B> {
    #[must_use]
    pub const fn with(mut self, opcode: u8, instruction: Instruction<B>) -> Self {
        self.0[opcode as usize] = instruction;
        self
    }
}

impl<B: Bus> Index<usize> for InstructionTable<B> {
    type Output = Instruction<B>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Trait for defining CPU instruction sets.
///
/// Implement this to create custom or variant instruction sets (e.g., 65C02, custom extensions).
/// The trait generates a complete 256-entry instruction table.
///
/// # Examples
///
/// ```
/// use ull65::instruction::{InstructionSet, Instruction, InstructionTable};
/// use ull65::{Bus, Cpu};
///
/// struct Custom6502;
///
/// impl InstructionSet for Custom6502 {
///     fn instruction_table<B: Bus + 'static>() -> InstructionTable<B> {
///         // Start with MOS 6502 table and customize
///         use ull65::instruction::mos6502::Mos6502;
///         Mos6502::base_table::<B>()
///         // .with(opcode, custom_instruction)...
///     }
/// }
/// ```
// Necessary because traits don't support `const fn` yet and we can't inline the generated table.
pub trait InstructionSet {
    /// Generate the 256-entry instruction table for this CPU variant.
    fn instruction_table<B: Bus + 'static>() -> InstructionTable<B>;
    const SUPPORTS_DECIMAL_MODE: bool = true;
}
