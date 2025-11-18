//! Nine addressing modes of the 6502.
//!
//! Each mode is a zero-sized type implementing [`AddressingMode`], enabling generic
//! instruction with compile-time dispatch.

use crate::processor::Cpu;
use ull::word;
use ull::{AccessType, Bus, Word};

/// Trait for computing effective addresses in different addressing modes.
///
/// Implemented as zero-sized types for compile-time dispatch. Instructions are generic
/// over this trait, allowing the same instruction logic to work with multiple addressing modes.
///
/// # Associated Items
///
/// - `BYTES`: Total instruction length (opcode + operand bytes)
/// - `fetch_address()`: Computes the effective address for the operand
pub trait AddressingMode {
    /// Compute the effective address for this addressing mode.
    ///
    /// Does not advance PC—that's the instruction's responsibility.
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word;

    /// Total bytes for an instruction using this mode (including opcode).
    const BYTES: u16;
}

pub struct Immediate;
impl AddressingMode for Immediate {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, _bus: &mut B) -> Word {
        cpu.pc + 1
    }

    const BYTES: u16 = 2;
}

pub struct Absolute;
impl AddressingMode for Absolute {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let lo = bus.read(cpu.pc + 1, AccessType::DataRead);
        let hi = bus.read(cpu.pc + 2, AccessType::DataRead);
        (lo, hi).into()
    }

    const BYTES: u16 = 3;
}

pub struct AbsoluteX;
impl AddressingMode for AbsoluteX {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let lo = bus.read(cpu.pc + 1, AccessType::DataRead);
        let hi = bus.read(cpu.pc + 2, AccessType::DataRead);
        let base: Word = (lo, hi).into();
        base + cpu.x
    }

    const BYTES: u16 = 3;
}

pub struct AbsoluteIndirectX;
impl AddressingMode for AbsoluteIndirectX {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let lo = bus.read(cpu.pc + 1, AccessType::DataRead);
        let hi = bus.read(cpu.pc + 2, AccessType::DataRead);
        let ptr = Word::from((lo, hi)) + cpu.x;

        let target_lo = bus.read(ptr, AccessType::DataRead);
        let target_hi = bus.read(ptr + 1, AccessType::DataRead);

        (target_lo, target_hi).into()
    }

    const BYTES: u16 = 3;
}

pub struct AbsoluteY;
impl AddressingMode for AbsoluteY {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let lo = bus.read(cpu.pc + 1, AccessType::DataRead);
        let hi = bus.read(cpu.pc + 2, AccessType::DataRead);
        let base: Word = (lo, hi).into();
        base + cpu.y
    }

    const BYTES: u16 = 3;
}

pub struct AbsoluteIndirect;
impl AddressingMode for AbsoluteIndirect {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let lo = bus.read(cpu.pc + 1, AccessType::DataRead);
        let hi = bus.read(cpu.pc + 2, AccessType::DataRead);
        let ptr: Word = (lo, hi).into();

        let effective_lo = bus.read(ptr, AccessType::DataRead);
        let ptr_raw: u16 = ptr.into();
        let high_addr = Word::from((ptr_raw & 0xFF00) | ((ptr_raw + 1) & 0x00FF)); // NMOS wraparound bug
        let effective_hi = bus.read(high_addr, AccessType::DataRead);
        (effective_lo, effective_hi).into()
    }

    const BYTES: u16 = 3;
}

pub struct AbsoluteIndirectCorrect;

impl AddressingMode for AbsoluteIndirectCorrect {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let ptr = Word::from((
            bus.read(cpu.pc + 1, AccessType::DataRead),
            bus.read(cpu.pc + 2, AccessType::DataRead),
        ));
        let lo = bus.read(ptr, AccessType::DataRead);
        let hi = bus.read(ptr + 1, AccessType::DataRead);
        (lo, hi).into()
    }

    const BYTES: u16 = 3;
}

pub struct ZeroPage;
impl AddressingMode for ZeroPage {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        bus.read(cpu.pc + 1, AccessType::DataRead).into()
    }

    const BYTES: u16 = 2;
}

pub struct ZeroPageX;
impl AddressingMode for ZeroPageX {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        (bus.read(cpu.pc + 1, AccessType::DataRead) + cpu.x).into()
    }

    const BYTES: u16 = 2;
}

pub struct ZeroPageIndirect;
impl AddressingMode for ZeroPageIndirect {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let ptr = bus.read(cpu.pc + 1, AccessType::DataRead);
        let lo = bus.read(ptr, AccessType::DataRead);
        let hi = bus.read(ptr + 1u8, AccessType::DataRead);
        (lo, hi).into()
    }

    const BYTES: u16 = 2;
}

pub struct ZeroPageY;
impl AddressingMode for ZeroPageY {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        (bus.read(cpu.pc + 1, AccessType::DataRead) + cpu.y).into()
    }

    const BYTES: u16 = 2;
}

pub struct ZeroPageXIndirect;
impl AddressingMode for ZeroPageXIndirect {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let addr = bus.read(cpu.pc + 1, AccessType::DataRead) + cpu.x;
        let lo = bus.read(addr, AccessType::DataRead);
        let hi = bus.read(addr + 1u8, AccessType::DataRead);
        (lo, hi).into()
    }

    const BYTES: u16 = 2;
}

pub struct ZeroPageIndirectY;
impl AddressingMode for ZeroPageIndirectY {
    fn fetch_address<B: Bus>(cpu: &Cpu<B>, bus: &mut B) -> Word {
        let addr = bus.read(cpu.pc + 1, AccessType::DataRead);
        let lo = bus.read(addr, AccessType::DataRead);
        let hi = bus.read(addr + 1u8, AccessType::DataRead);
        let ptr = word!((lo, hi));

        ptr + cpu.y
    }

    const BYTES: u16 = 2;
}
