//! The WDC 65C02 instruction set implementation.

use crate::byte::Byte;
use crate::instruction::mos6502::{illegal, illegal_a, Mos6502};
use crate::instruction::{Instruction, InstructionSet, InstructionTable};
use crate::processor::addressing_mode::{
    Absolute, AbsoluteIndirectCorrect, AbsoluteIndirectX, AbsoluteX, Immediate, ZeroPage,
    ZeroPageIndirect, ZeroPageX,
};
use crate::processor::flags::Flags;
use crate::word::Word;
use crate::{word, AccessType, AddressingMode, Bus, Cpu, RunState, IRQ_VECTOR_HI, IRQ_VECTOR_LO};

pub struct Wdc65c02s;

impl Wdc65c02s {
    /// Builds the canonical WDC 65C02S instruction table.
    #[must_use] 
    pub const fn base_table<B: Bus + 'static>() -> InstructionTable<B> {
        Mos6502::base_table()
            // BRK with decimal clear
            .with(
                0x00,
                Instruction {
                    cycles: 7,
                    execute: brk::<B>,
                },
            )
            // BRA (branch always)
            .with(
                0x80,
                Instruction {
                    cycles: 2,
                    execute: bra::<Immediate, B>,
                },
            )
            // STZ variants
            .with(
                0x64,
                Instruction {
                    cycles: 3,
                    execute: stz::<ZeroPage, B>,
                },
            )
            .with(
                0x74,
                Instruction {
                    cycles: 4,
                    execute: stz::<ZeroPageX, B>,
                },
            )
            .with(
                0x9C,
                Instruction {
                    cycles: 4,
                    execute: stz::<Absolute, B>,
                },
            )
            .with(
                0x9E,
                Instruction {
                    cycles: 5,
                    execute: stz::<AbsoluteX, B>,
                },
            )
            // TSB/TRB
            .with(
                0x04,
                Instruction {
                    cycles: 5,
                    execute: tsb::<ZeroPage, B>,
                },
            )
            .with(
                0x0C,
                Instruction {
                    cycles: 6,
                    execute: tsb::<Absolute, B>,
                },
            )
            .with(
                0x14,
                Instruction {
                    cycles: 5,
                    execute: trb::<ZeroPage, B>,
                },
            )
            .with(
                0x1C,
                Instruction {
                    cycles: 6,
                    execute: trb::<Absolute, B>,
                },
            )
            // INC/DEC accumulator
            .with(
                0x1A,
                Instruction {
                    cycles: 2,
                    execute: inc_a::<B>,
                },
            )
            .with(
                0x3A,
                Instruction {
                    cycles: 2,
                    execute: dec_a::<B>,
                },
            )
            // Stack transfers
            .with(
                0x5A,
                Instruction {
                    cycles: 3,
                    execute: phy::<B>,
                },
            )
            .with(
                0x7A,
                Instruction {
                    cycles: 4,
                    execute: ply::<B>,
                },
            )
            .with(
                0xDA,
                Instruction {
                    cycles: 3,
                    execute: phx::<B>,
                },
            )
            .with(
                0xFA,
                Instruction {
                    cycles: 4,
                    execute: plx::<B>,
                },
            )
            // BIT immediate
            .with(
                0x89,
                Instruction {
                    cycles: 2,
                    execute: bit::<B>,
                },
            )
            // BIT indexed variants
            .with(
                0x34,
                Instruction {
                    cycles: 4,
                    execute: super::mos6502::bit::<ZeroPageX, B>,
                },
            )
            .with(
                0x3C,
                Instruction {
                    cycles: 4,
                    execute: super::mos6502::bit::<AbsoluteX, B>,
                },
            )
            // JMP absolute indirect fixed
            .with(
                0x6C,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::jmp::<AbsoluteIndirectCorrect, B>,
                },
            )
            // JMP (abs,X)
            .with(
                0x7C,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::jmp::<AbsoluteIndirectX, B>,
                },
            )
            // Official 65C02 NOPs promoted from illegal opcodes
            .with(
                0x02,
                Instruction {
                    cycles: 2,
                    execute: illegal::<Immediate, B>,
                },
            )
            // (Zero Page) indirect addressing variants
            .with(
                0x12,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::ora::<ZeroPageIndirect, B>,
                },
            )
            .with(
                0x32,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::and::<ZeroPageIndirect, B>,
                },
            )
            .with(
                0x52,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::eor::<ZeroPageIndirect, B>,
                },
            )
            .with(
                0x72,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::adc::<Mos6502, ZeroPageIndirect, B>,
                },
            )
            .with(
                0x92,
                Instruction {
                    cycles: 6,
                    execute: super::mos6502::sta::<ZeroPageIndirect, B>,
                },
            )
            .with(
                0xB2,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::lda::<ZeroPageIndirect, B>,
                },
            )
            .with(
                0xD2,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::cmp::<ZeroPageIndirect, B>,
                },
            )
            .with(
                0xF2,
                Instruction {
                    cycles: 5,
                    execute: super::mos6502::sbc::<Mos6502, ZeroPageIndirect, B>,
                },
            )
            .with(
                0x22,
                Instruction {
                    cycles: 2,
                    execute: illegal::<Immediate, B>,
                },
            )
            .with(
                0x42,
                Instruction {
                    cycles: 2,
                    execute: illegal::<Immediate, B>,
                },
            )
            .with(
                0x62,
                Instruction {
                    cycles: 2,
                    execute: illegal::<Immediate, B>,
                },
            )
            .with(
                0x82,
                Instruction {
                    cycles: 2,
                    execute: illegal::<Immediate, B>,
                },
            )
            .with(
                0xC2,
                Instruction {
                    cycles: 2,
                    execute: illegal::<Immediate, B>,
                },
            )
            .with(
                0xE2,
                Instruction {
                    cycles: 2,
                    execute: illegal::<Immediate, B>,
                },
            )
            .with(
                0x44,
                Instruction {
                    cycles: 3,
                    execute: illegal::<ZeroPage, B>,
                },
            )
            .with(
                0x54,
                Instruction {
                    cycles: 4,
                    execute: illegal::<ZeroPageX, B>,
                },
            )
            .with(
                0xD4,
                Instruction {
                    cycles: 4,
                    execute: illegal::<ZeroPageX, B>,
                },
            )
            .with(
                0xF4,
                Instruction {
                    cycles: 4,
                    execute: illegal::<ZeroPageX, B>,
                },
            )
            .with(
                0x5C,
                Instruction {
                    cycles: 8,
                    execute: illegal::<Absolute, B>,
                },
            )
            .with(
                0xDC,
                Instruction {
                    cycles: 4,
                    execute: illegal::<AbsoluteX, B>,
                },
            )
            .with(
                0xFC,
                Instruction {
                    cycles: 4,
                    execute: illegal::<AbsoluteX, B>,
                },
            )
            .with(
                0x03,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x0B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x13,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x1B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x23,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x2B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x33,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x3B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x43,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x4B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x53,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x5B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x63,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x6B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x73,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x7B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x83,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x8B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x93,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0x9B,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xA3,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xAB,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xB3,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xBB,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xC3,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xD3,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xE3,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xEB,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xF3,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            .with(
                0xFB,
                Instruction {
                    cycles: 1,
                    execute: illegal_a::<B>,
                },
            )
            // RMB/SMB bit operations
            .with(
                0x07,
                Instruction {
                    cycles: 5,
                    execute: rmb0::<B>,
                },
            )
            .with(
                0x17,
                Instruction {
                    cycles: 5,
                    execute: rmb1::<B>,
                },
            )
            .with(
                0x27,
                Instruction {
                    cycles: 5,
                    execute: rmb2::<B>,
                },
            )
            .with(
                0x37,
                Instruction {
                    cycles: 5,
                    execute: rmb3::<B>,
                },
            )
            .with(
                0x47,
                Instruction {
                    cycles: 5,
                    execute: rmb4::<B>,
                },
            )
            .with(
                0x57,
                Instruction {
                    cycles: 5,
                    execute: rmb5::<B>,
                },
            )
            .with(
                0x67,
                Instruction {
                    cycles: 5,
                    execute: rmb6::<B>,
                },
            )
            .with(
                0x77,
                Instruction {
                    cycles: 5,
                    execute: rmb7::<B>,
                },
            )
            .with(
                0x87,
                Instruction {
                    cycles: 5,
                    execute: smb0::<B>,
                },
            )
            .with(
                0x97,
                Instruction {
                    cycles: 5,
                    execute: smb1::<B>,
                },
            )
            .with(
                0xA7,
                Instruction {
                    cycles: 5,
                    execute: smb2::<B>,
                },
            )
            .with(
                0xB7,
                Instruction {
                    cycles: 5,
                    execute: smb3::<B>,
                },
            )
            .with(
                0xC7,
                Instruction {
                    cycles: 5,
                    execute: smb4::<B>,
                },
            )
            .with(
                0xD7,
                Instruction {
                    cycles: 5,
                    execute: smb5::<B>,
                },
            )
            .with(
                0xE7,
                Instruction {
                    cycles: 5,
                    execute: smb6::<B>,
                },
            )
            .with(
                0xF7,
                Instruction {
                    cycles: 5,
                    execute: smb7::<B>,
                },
            )
            // BBR/BBS bit-branch operations
            .with(
                0x0F,
                Instruction {
                    cycles: 5,
                    execute: bbr0::<B>,
                },
            )
            .with(
                0x1F,
                Instruction {
                    cycles: 5,
                    execute: bbr1::<B>,
                },
            )
            .with(
                0x2F,
                Instruction {
                    cycles: 5,
                    execute: bbr2::<B>,
                },
            )
            .with(
                0x3F,
                Instruction {
                    cycles: 5,
                    execute: bbr3::<B>,
                },
            )
            .with(
                0x4F,
                Instruction {
                    cycles: 5,
                    execute: bbr4::<B>,
                },
            )
            .with(
                0x5F,
                Instruction {
                    cycles: 5,
                    execute: bbr5::<B>,
                },
            )
            .with(
                0x6F,
                Instruction {
                    cycles: 5,
                    execute: bbr6::<B>,
                },
            )
            .with(
                0x7F,
                Instruction {
                    cycles: 5,
                    execute: bbr7::<B>,
                },
            )
            .with(
                0x8F,
                Instruction {
                    cycles: 5,
                    execute: bbs0::<B>,
                },
            )
            .with(
                0x9F,
                Instruction {
                    cycles: 5,
                    execute: bbs1::<B>,
                },
            )
            .with(
                0xAF,
                Instruction {
                    cycles: 5,
                    execute: bbs2::<B>,
                },
            )
            .with(
                0xBF,
                Instruction {
                    cycles: 5,
                    execute: bbs3::<B>,
                },
            )
            .with(
                0xCF,
                Instruction {
                    cycles: 5,
                    execute: bbs4::<B>,
                },
            )
            .with(
                0xDF,
                Instruction {
                    cycles: 5,
                    execute: bbs5::<B>,
                },
            )
            .with(
                0xEF,
                Instruction {
                    cycles: 5,
                    execute: bbs6::<B>,
                },
            )
            .with(
                0xFF,
                Instruction {
                    cycles: 5,
                    execute: bbs7::<B>,
                },
            )
            // STP/WAI
            .with(
                0xCB,
                Instruction {
                    cycles: 3,
                    execute: wai::<B>,
                },
            )
            .with(
                0xDB,
                Instruction {
                    cycles: 3,
                    execute: stp::<B>,
                },
            )
    }
}

impl InstructionSet for Wdc65c02s {
    fn instruction_table<B: Bus + 'static>() -> InstructionTable<B> {
        Self::base_table()
    }
}

// Here for clarity on the bit operations since passing in `true` or `false` is a bit ambiguous,
// and you can't currently use an enum in this way.
const RESET: bool = false;
const SET: bool = true;

pub fn bra<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + AM::BYTES;
    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;

    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

#[inline]
pub fn stz<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    bus.write(addr, Byte::ZERO, AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn trb<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);

    cpu.p.set_zero(cpu.a & val == 0);
    bus.write(addr, !cpu.a & val, AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn tsb<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let result = val | cpu.a;

    cpu.p.set_zero((cpu.a & val) == Byte(0));
    bus.write(addr, result, AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn inc_a<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.a += 1;

    cpu.p.set_signed(cpu.a.is_signed());
    cpu.p.set_zero(cpu.a == 0);

    cpu.pc += 1;
}
pub fn dec_a<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.a -= 1;

    cpu.p.set_signed(cpu.a.is_signed());
    cpu.p.set_zero(cpu.a == 0);

    cpu.pc += 1;
}

pub fn phx<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.push(bus, cpu.x);

    cpu.pc += 1;
}

pub fn phy<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.push(bus, cpu.y);

    cpu.pc += 1;
}

pub fn plx<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.x = cpu.pop(bus);
    cpu.p.set_signed(cpu.x.is_signed());
    cpu.p.set_zero(cpu.x == 0);

    cpu.pc += 1;
}
pub fn ply<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.y = cpu.pop(bus);

    cpu.p.set_signed(cpu.y.is_signed());
    cpu.p.set_zero(cpu.y == 0);

    cpu.pc += 1;
}

pub fn bbr0<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<0, false, B>(cpu, bus);
}
pub fn bbr1<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<1, false, B>(cpu, bus);
}
pub fn bbr2<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<2, false, B>(cpu, bus);
}
pub fn bbr3<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<3, false, B>(cpu, bus);
}
pub fn bbr4<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<4, false, B>(cpu, bus);
}
pub fn bbr5<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<5, false, B>(cpu, bus);
}
pub fn bbr6<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<6, false, B>(cpu, bus);
}
pub fn bbr7<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<7, false, B>(cpu, bus);
}

pub fn bbs0<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<0, true, B>(cpu, bus);
}
pub fn bbs1<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<1, true, B>(cpu, bus);
}
pub fn bbs2<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<2, true, B>(cpu, bus);
}
pub fn bbs3<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<3, true, B>(cpu, bus);
}
pub fn bbs4<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<4, true, B>(cpu, bus);
}
pub fn bbs5<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<5, true, B>(cpu, bus);
}
pub fn bbs6<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<6, true, B>(cpu, bus);
}
pub fn bbs7<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    branch_on_zero_page_bit::<7, true, B>(cpu, bus);
}

pub fn rmb0<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<0, RESET, B>(cpu, bus);
}
pub fn rmb1<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<1, RESET, B>(cpu, bus);
}
pub fn rmb2<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<2, RESET, B>(cpu, bus);
}
pub fn rmb3<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<3, RESET, B>(cpu, bus);
}
pub fn rmb4<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<4, RESET, B>(cpu, bus);
}
pub fn rmb5<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<5, RESET, B>(cpu, bus);
}
pub fn rmb6<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<6, RESET, B>(cpu, bus);
}
pub fn rmb7<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<7, RESET, B>(cpu, bus);
}
pub fn smb0<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<0, SET, B>(cpu, bus);
}
pub fn smb1<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<1, SET, B>(cpu, bus);
}
pub fn smb2<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<2, SET, B>(cpu, bus);
}
pub fn smb3<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<3, SET, B>(cpu, bus);
}
pub fn smb4<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<4, SET, B>(cpu, bus);
}
pub fn smb5<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<5, SET, B>(cpu, bus);
}
pub fn smb6<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<6, SET, B>(cpu, bus);
}
pub fn smb7<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    modify_zero_page_bit::<7, SET, B>(cpu, bus);
}
pub fn stp<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.run_state = RunState::Halted;
}
pub fn wai<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.run_state = RunState::Waiting;
    cpu.pc += 1;
}

pub fn brk<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let return_pc = word!(cpu.pc + 2);

    cpu.push(bus, return_pc.hi());
    cpu.push(bus, return_pc.lo());

    let status = cpu.p | Flags::Break | Flags::Expansion;
    cpu.push(bus, status.into());

    cpu.p.set_decimal_mode(false);
    cpu.p.set_interrupt_disabled(true);

    let lo = bus.read(IRQ_VECTOR_LO, AccessType::InterruptVectorRead);
    let hi = bus.read(IRQ_VECTOR_HI, AccessType::InterruptVectorRead);
    cpu.pc = word!((lo, hi));
}

pub fn bit<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = Immediate::fetch_address(cpu, bus);
    let operand = bus.read(addr, AccessType::DataRead);

    cpu.p.set_zero((cpu.a & operand) == Byte(0));

    cpu.pc += Immediate::BYTES;
}

fn branch_on_zero_page_bit<const BIT: u8, const BRANCH_WHEN_SET: bool, B: Bus + 'static>(
    cpu: &mut Cpu<B>,
    bus: &mut B,
) {
    debug_assert!(BIT < 8);

    let zp_addr: Word = bus.read(cpu.pc + 1, AccessType::DataRead).into();
    let value = u8::from(bus.read(zp_addr, AccessType::DataRead));
    let rel = i8::from(bus.read(cpu.pc + 2, AccessType::DataRead));

    let base = cpu.pc + 3u16;
    cpu.pc = base;

    let mask = 1u8 << BIT;
    let bit_set = (value & mask) != 0;

    if bit_set == BRANCH_WHEN_SET {
        let target = base + rel;
        cpu.cycles += 1;
        if cpu.crosses_page(base, target) {
            cpu.cycles += 1;
        }
        cpu.pc = target;
    }
}

fn modify_zero_page_bit<const BIT: u8, const SET_BIT: bool, B: Bus + 'static>(
    cpu: &mut Cpu<B>,
    bus: &mut B,
) {
    debug_assert!(BIT < 8);

    let zp_addr: Word = bus.read(cpu.pc + 1, AccessType::DataRead).into();
    let mut value = bus.read(zp_addr, AccessType::DataRead);
    let mask = 1u8 << BIT;

    if SET_BIT {
        value |= mask;
    } else {
        value &= !mask;
    }

    bus.write(zp_addr, value, AccessType::DataWrite);
    cpu.pc += 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::SimpleBus;
    use crate::processor::addressing_mode::{AbsoluteX, Immediate, ZeroPage};
    use crate::processor::flags::Flags;
    use crate::{byte, word};
    use crate::{RunState, STACK_SPACE_START};

    #[test]
    fn test_bra_branches_forward() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        cpu.pc = word!(0x4000);
        bus.write(cpu.pc + 1, byte!(0x04), AccessType::DataWrite);

        bra::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, word!(0x4006));
    }

    #[test]
    fn test_stz_zero_page_clears_memory() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        bus.write(cpu.pc + 1, byte!(0x10), AccessType::DataWrite);
        bus.write(word!(0x0010), byte!(0xFF), AccessType::DataWrite);

        stz::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(word!(0x0010), AccessType::DataRead), byte!(0x00));
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_stz_absolute_x_clears_offset_address() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        cpu.x = byte!(0x01);
        let base = word!(0x1234);
        let target = base + cpu.x;
        let (lo, hi) = base.lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);
        bus.write(target, byte!(0x99), AccessType::DataWrite);

        stz::<AbsoluteX, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(target, AccessType::DataRead), byte!(0x00));
        assert_eq!(cpu.pc, AbsoluteX::BYTES.into());
    }

    #[test]
    fn test_trb_zero_page_clears_bits() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        cpu.a = byte!(0x0F);
        bus.write(cpu.pc + 1, byte!(0x20), AccessType::DataWrite);
        bus.write(word!(0x0020), byte!(0xF0), AccessType::DataWrite);

        trb::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(word!(0x0020), AccessType::DataRead), byte!(0xF0));
        assert!(cpu.p.contains(Flags::Zero));
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_tsb_zero_page_sets_bits() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        cpu.a = byte!(0x0F);
        bus.write(cpu.pc + 1, byte!(0x30), AccessType::DataWrite);
        bus.write(word!(0x0030), byte!(0xF1), AccessType::DataWrite);

        tsb::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(word!(0x0030), AccessType::DataRead), byte!(0xFF));
        assert!(!cpu.p.contains(Flags::Zero));

        cpu.pc = word!(0);
        cpu.p.remove(Flags::Zero);
        bus.write(cpu.pc + 1, byte!(0x30), AccessType::DataWrite);
        bus.write(word!(0x0030), byte!(0xF0), AccessType::DataWrite);

        tsb::<ZeroPage, _>(&mut cpu, &mut bus);
        assert!(cpu.p.contains(Flags::Zero));
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_inc_dec_a_update_flags() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        cpu.a = byte!(0xFF);

        inc_a(&mut cpu, &mut bus);
        assert_eq!(cpu.a, byte!(0x00));
        assert!(cpu.p.contains(Flags::Zero));

        dec_a(&mut cpu, &mut bus);
        assert_eq!(cpu.a, byte!(0xFF));
        assert!(cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_phx_phy_push_registers() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        cpu.x = byte!(0xAA);
        cpu.y = byte!(0xBB);

        phx(&mut cpu, &mut bus);
        phy(&mut cpu, &mut bus);

        let first_addr = STACK_SPACE_START + cpu.sp + byte!(1);
        let second_addr = first_addr + byte!(1);

        assert_eq!(bus.read(first_addr, AccessType::StackRead), byte!(0xBB));
        assert_eq!(bus.read(second_addr, AccessType::StackRead), byte!(0xAA));
    }

    #[test]
    fn test_plx_ply_pull_registers() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        cpu.push(&mut bus, byte!(0x12));
        cpu.push(&mut bus, byte!(0x34));

        plx(&mut cpu, &mut bus);
        ply(&mut cpu, &mut bus);

        assert_eq!(cpu.x, byte!(0x34));
        assert_eq!(cpu.y, byte!(0x12));
    }

    #[test]
    fn test_bbr_branches_per_bit() {
        for bit in 0u8..8 {
            let mut bus = SimpleBus::default();
            let mut cpu: Cpu<SimpleBus> = Cpu::default();
            cpu.pc = word!(0x2000);
            let zp = byte!(0x10 + bit);
            bus.write(cpu.pc + 1, zp, AccessType::DataWrite);
            bus.write(cpu.pc + 2, byte!(0x02), AccessType::DataWrite);
            bus.write(
                word!(u16::from(zp)),
                byte!(0xFF & !(1 << bit)),
                AccessType::DataRead,
            );

            match bit {
                0 => bbr0(&mut cpu, &mut bus),
                1 => bbr1(&mut cpu, &mut bus),
                2 => bbr2(&mut cpu, &mut bus),
                3 => bbr3(&mut cpu, &mut bus),
                4 => bbr4(&mut cpu, &mut bus),
                5 => bbr5(&mut cpu, &mut bus),
                6 => bbr6(&mut cpu, &mut bus),
                7 => bbr7(&mut cpu, &mut bus),
                _ => unreachable!(),
            }

            assert_eq!(cpu.pc, word!(0x2005));
        }
    }

    #[test]
    fn test_bbs_skips_when_bit_clear() {
        for bit in 0u8..8 {
            let mut bus = SimpleBus::default();
            let mut cpu: Cpu<SimpleBus> = Cpu::default();
            let start = cpu.pc;
            let zp = byte!(0x10 + bit);
            bus.write(cpu.pc + 1, zp, AccessType::DataWrite);
            bus.write(cpu.pc + 2, byte!(0x02), AccessType::DataWrite);
            bus.write(word!(u16::from(zp)), byte!(0x00), AccessType::DataWrite);

            match bit {
                0 => bbs0(&mut cpu, &mut bus),
                1 => bbs1(&mut cpu, &mut bus),
                2 => bbs2(&mut cpu, &mut bus),
                3 => bbs3(&mut cpu, &mut bus),
                4 => bbs4(&mut cpu, &mut bus),
                5 => bbs5(&mut cpu, &mut bus),
                6 => bbs6(&mut cpu, &mut bus),
                7 => bbs7(&mut cpu, &mut bus),
                _ => unreachable!(),
            }

            assert_eq!(cpu.pc, start + 3);
        }
    }

    #[test]
    fn test_rmb_clears_all_bits() {
        for bit in 0u8..8 {
            let mut bus = SimpleBus::default();
            let mut cpu: Cpu<SimpleBus> = Cpu::default();
            let zp = byte!(0x05 + bit);
            bus.write(cpu.pc + 1, zp, AccessType::DataWrite);
            bus.write(word!(u16::from(zp)), byte!(0xFF), AccessType::DataWrite);

            match bit {
                0 => rmb0(&mut cpu, &mut bus),
                1 => rmb1(&mut cpu, &mut bus),
                2 => rmb2(&mut cpu, &mut bus),
                3 => rmb3(&mut cpu, &mut bus),
                4 => rmb4(&mut cpu, &mut bus),
                5 => rmb5(&mut cpu, &mut bus),
                6 => rmb6(&mut cpu, &mut bus),
                7 => rmb7(&mut cpu, &mut bus),
                _ => unreachable!(),
            }

            assert_eq!(
                bus.read(word!(u16::from(zp)), AccessType::DataRead),
                byte!(0xFF & !(1 << bit))
            );
        }
    }

    #[test]
    fn test_smb_sets_all_bits() {
        for bit in 0u8..8 {
            let mut bus = SimpleBus::default();
            let mut cpu: Cpu<SimpleBus> = Cpu::default();
            let zp = byte!(0x05 + bit);
            bus.write(cpu.pc + 1, zp, AccessType::DataWrite);
            bus.write(word!(u16::from(zp)), byte!(0x00), AccessType::DataWrite);

            match bit {
                0 => smb0(&mut cpu, &mut bus),
                1 => smb1(&mut cpu, &mut bus),
                2 => smb2(&mut cpu, &mut bus),
                3 => smb3(&mut cpu, &mut bus),
                4 => smb4(&mut cpu, &mut bus),
                5 => smb5(&mut cpu, &mut bus),
                6 => smb6(&mut cpu, &mut bus),
                7 => smb7(&mut cpu, &mut bus),
                _ => unreachable!(),
            }

            assert_eq!(
                bus.read(word!(u16::from(zp)), AccessType::DataRead),
                byte!(1 << bit)
            );
        }
    }

    #[test]
    fn test_stp_sets_halted_state() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();

        stp(&mut cpu, &mut bus);

        assert_eq!(cpu.run_state, RunState::Halted);
    }

    #[test]
    fn test_wai_sets_waiting_and_advances_pc() {
        let mut bus = SimpleBus::default();
        let mut cpu: Cpu<SimpleBus> = Cpu::default();
        let start = cpu.pc;

        wai(&mut cpu, &mut bus);

        assert_eq!(cpu.run_state, RunState::Waiting);
        assert_eq!(cpu.pc, start + 1);
    }
}
