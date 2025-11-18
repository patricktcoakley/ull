//! The original MOS 6502 instruction set implementation.

use crate::instruction::{Instruction, InstructionSet, InstructionTable};
use crate::processor::addressing_mode::{
    Absolute, AbsoluteIndirect, AbsoluteX, AbsoluteY, AddressingMode, Immediate, ZeroPage,
    ZeroPageIndirectY, ZeroPageX, ZeroPageXIndirect, ZeroPageY,
};
use crate::processor::flags::Flags;
use crate::{Cpu, IRQ_VECTOR_HI, IRQ_VECTOR_LO, RunState};
use ull::{AccessType, Bus, Byte, Nibble};
use ull::{byte, word};

pub struct Mos6502;

impl Mos6502 {
    #[must_use]
    pub const fn base_table<B: Bus + 'static>() -> InstructionTable<B> {
        InstructionTable([
            // 0x00
            Instruction {
                cycles: 7,
                execute: brk::<B>,
            },
            // 0x01
            Instruction {
                cycles: 6,
                execute: ora::<ZeroPageXIndirect, B>,
            },
            // 0x02
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x03
            Instruction {
                cycles: 8,
                execute: slo::<ZeroPageXIndirect, B>,
            },
            // 0x04
            Instruction {
                cycles: 3,
                execute: illegal::<ZeroPage, B>,
            },
            // 0x05
            Instruction {
                cycles: 3,
                execute: ora::<ZeroPage, B>,
            },
            // 0x06
            Instruction {
                cycles: 5,
                execute: asl::<ZeroPage, B>,
            },
            // 0x07
            Instruction {
                cycles: 5,
                execute: slo::<ZeroPage, B>,
            },
            // 0x08
            Instruction {
                cycles: 3,
                execute: php::<B>,
            },
            // 0x09
            Instruction {
                cycles: 2,
                execute: ora::<Immediate, B>,
            },
            // 0x0A
            Instruction {
                cycles: 2,
                execute: asl_a::<B>,
            },
            // 0x0B
            Instruction {
                cycles: 2,
                execute: anc::<Immediate, B>,
            },
            // 0x0C
            Instruction {
                cycles: 4,
                execute: illegal::<Absolute, B>,
            },
            // 0x0D
            Instruction {
                cycles: 4,
                execute: ora::<Absolute, B>,
            },
            // 0x0E
            Instruction {
                cycles: 6,
                execute: asl::<Absolute, B>,
            },
            // 0x0F
            Instruction {
                cycles: 6,
                execute: slo::<Absolute, B>,
            },
            // 0x10
            Instruction {
                cycles: 2,
                execute: bpl::<B>,
            },
            // 0x11
            Instruction {
                cycles: 5,
                execute: ora::<ZeroPageIndirectY, B>,
            },
            // 0x12
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x13
            Instruction {
                cycles: 8,
                execute: slo::<ZeroPageIndirectY, B>,
            },
            // 0x14
            Instruction {
                cycles: 4,
                execute: illegal::<ZeroPageX, B>,
            },
            // 0x15
            Instruction {
                cycles: 4,
                execute: ora::<ZeroPageX, B>,
            },
            // 0x16
            Instruction {
                cycles: 6,
                execute: asl::<ZeroPageX, B>,
            },
            // 0x17
            Instruction {
                cycles: 6,
                execute: slo::<ZeroPageX, B>,
            },
            // 0x18
            Instruction {
                cycles: 2,
                execute: clc::<B>,
            },
            // 0x19
            Instruction {
                cycles: 4,
                execute: ora::<AbsoluteY, B>,
            },
            // 0x1A
            Instruction {
                cycles: 2,
                execute: illegal_a::<B>,
            },
            // 0x1B
            Instruction {
                cycles: 7,
                execute: slo::<AbsoluteY, B>,
            },
            // 0x1C
            Instruction {
                cycles: 4,
                execute: illegal::<AbsoluteX, B>,
            },
            // 0x1D
            Instruction {
                cycles: 4,
                execute: ora::<AbsoluteX, B>,
            },
            // 0x1E
            Instruction {
                cycles: 7,
                execute: asl::<AbsoluteX, B>,
            },
            // 0x1F
            Instruction {
                cycles: 7,
                execute: slo::<AbsoluteX, B>,
            },
            // 0x20
            Instruction {
                cycles: 6,
                execute: jsr::<Absolute, B>,
            },
            // 0x21
            Instruction {
                cycles: 6,
                execute: and::<ZeroPageXIndirect, B>,
            },
            // 0x22
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x23
            Instruction {
                cycles: 8,
                execute: rla::<ZeroPageXIndirect, B>,
            },
            // 0x24
            Instruction {
                cycles: 3,
                execute: bit::<ZeroPage, B>,
            },
            // 0x25
            Instruction {
                cycles: 3,
                execute: and::<ZeroPage, B>,
            },
            // 0x26
            Instruction {
                cycles: 5,
                execute: rol::<ZeroPage, B>,
            },
            // 0x27
            Instruction {
                cycles: 5,
                execute: rla::<ZeroPage, B>,
            },
            // 0x28
            Instruction {
                cycles: 4,
                execute: plp::<B>,
            },
            // 0x29
            Instruction {
                cycles: 2,
                execute: and::<Immediate, B>,
            },
            // 0x2A
            Instruction {
                cycles: 2,
                execute: rol_a::<B>,
            },
            // 0x2B
            Instruction {
                cycles: 2,
                execute: anc::<Immediate, B>,
            },
            // 0x2C
            Instruction {
                cycles: 4,
                execute: bit::<Absolute, B>,
            },
            // 0x2D
            Instruction {
                cycles: 4,
                execute: and::<Absolute, B>,
            },
            // 0x2E
            Instruction {
                cycles: 6,
                execute: rol::<Absolute, B>,
            },
            // 0x2F
            Instruction {
                cycles: 6,
                execute: rla::<Absolute, B>,
            },
            // 0x30
            Instruction {
                cycles: 2,
                execute: bmi::<B>,
            },
            // 0x31
            Instruction {
                cycles: 5,
                execute: and::<ZeroPageIndirectY, B>,
            },
            // 0x32
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x33
            Instruction {
                cycles: 8,
                execute: rla::<ZeroPageIndirectY, B>,
            },
            // 0x34
            Instruction {
                cycles: 4,
                execute: illegal::<ZeroPageX, B>,
            },
            // 0x35
            Instruction {
                cycles: 4,
                execute: and::<ZeroPageX, B>,
            },
            // 0x36
            Instruction {
                cycles: 6,
                execute: rol::<ZeroPageX, B>,
            },
            // 0x37
            Instruction {
                cycles: 6,
                execute: rla::<ZeroPageX, B>,
            },
            // 0x38
            Instruction {
                cycles: 2,
                execute: sec::<B>,
            },
            // 0x39
            Instruction {
                cycles: 4,
                execute: and::<AbsoluteY, B>,
            },
            // 0x3A
            Instruction {
                cycles: 2,
                execute: illegal_a::<B>,
            },
            // 0x3B
            Instruction {
                cycles: 7,
                execute: rla::<AbsoluteY, B>,
            },
            // 0x3C
            Instruction {
                cycles: 4,
                execute: illegal::<AbsoluteX, B>,
            },
            // 0x3D
            Instruction {
                cycles: 4,
                execute: and::<AbsoluteX, B>,
            },
            // 0x3E
            Instruction {
                cycles: 7,
                execute: rol::<AbsoluteX, B>,
            },
            // 0x3F
            Instruction {
                cycles: 7,
                execute: rla::<AbsoluteX, B>,
            },
            // 0x40
            Instruction {
                cycles: 6,
                execute: rti::<B>,
            },
            // 0x41
            Instruction {
                cycles: 6,
                execute: eor::<ZeroPageXIndirect, B>,
            },
            // 0x42
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x43
            Instruction {
                cycles: 8,
                execute: sre::<ZeroPageXIndirect, B>,
            },
            // 0x44
            Instruction {
                cycles: 3,
                execute: illegal::<ZeroPage, B>,
            },
            // 0x45
            Instruction {
                cycles: 3,
                execute: eor::<ZeroPage, B>,
            },
            // 0x46
            Instruction {
                cycles: 5,
                execute: lsr::<ZeroPage, B>,
            },
            // 0x47
            Instruction {
                cycles: 5,
                execute: sre::<ZeroPage, B>,
            },
            // 0x48
            Instruction {
                cycles: 3,
                execute: pha::<B>,
            },
            // 0x49
            Instruction {
                cycles: 2,
                execute: eor::<Immediate, B>,
            },
            // 0x4A
            Instruction {
                cycles: 2,
                execute: lsr_a::<B>,
            },
            // 0x4B
            Instruction {
                cycles: 2,
                execute: asr::<Immediate, B>,
            },
            // 0x4C
            Instruction {
                cycles: 3,
                execute: jmp::<Absolute, B>,
            },
            // 0x4D
            Instruction {
                cycles: 4,
                execute: eor::<Absolute, B>,
            },
            // 0x4E
            Instruction {
                cycles: 6,
                execute: lsr::<Absolute, B>,
            },
            // 0x4F
            Instruction {
                cycles: 6,
                execute: sre::<Absolute, B>,
            },
            // 0x50
            Instruction {
                cycles: 2,
                execute: bvc::<B>,
            },
            // 0x51
            Instruction {
                cycles: 5,
                execute: eor::<ZeroPageIndirectY, B>,
            },
            // 0x52
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x53
            Instruction {
                cycles: 8,
                execute: sre::<ZeroPageIndirectY, B>,
            },
            // 0x54
            Instruction {
                cycles: 4,
                execute: illegal::<ZeroPageX, B>,
            },
            // 0x55
            Instruction {
                cycles: 4,
                execute: eor::<ZeroPageX, B>,
            },
            // 0x56
            Instruction {
                cycles: 6,
                execute: lsr::<ZeroPageX, B>,
            },
            // 0x57
            Instruction {
                cycles: 6,
                execute: sre::<ZeroPageX, B>,
            },
            // 0x58
            Instruction {
                cycles: 2,
                execute: cli::<B>,
            },
            // 0x59
            Instruction {
                cycles: 4,
                execute: eor::<AbsoluteY, B>,
            },
            // 0x5A
            Instruction {
                cycles: 2,
                execute: illegal_a::<B>,
            },
            // 0x5B
            Instruction {
                cycles: 7,
                execute: sre::<AbsoluteY, B>,
            },
            // 0x5C
            Instruction {
                cycles: 4,
                execute: illegal::<AbsoluteX, B>,
            },
            // 0x5D
            Instruction {
                cycles: 4,
                execute: eor::<AbsoluteX, B>,
            },
            // 0x5E
            Instruction {
                cycles: 7,
                execute: lsr::<AbsoluteX, B>,
            },
            // 0x5F
            Instruction {
                cycles: 7,
                execute: sre::<AbsoluteX, B>,
            },
            // 0x60
            Instruction {
                cycles: 6,
                execute: rts::<B>,
            },
            // 0x61
            Instruction {
                cycles: 6,
                execute: adc::<Mos6502, ZeroPageXIndirect, B>,
            },
            // 0x62
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x63
            Instruction {
                cycles: 8,
                execute: rra::<Mos6502, ZeroPageXIndirect, B>,
            },
            // 0x64
            Instruction {
                cycles: 3,
                execute: illegal::<ZeroPage, B>,
            },
            // 0x65
            Instruction {
                cycles: 3,
                execute: adc::<Mos6502, ZeroPage, B>,
            },
            // 0x66
            Instruction {
                cycles: 5,
                execute: ror::<ZeroPage, B>,
            },
            // 0x67
            Instruction {
                cycles: 5,
                execute: rra::<Mos6502, ZeroPage, B>,
            },
            // 0x68
            Instruction {
                cycles: 4,
                execute: pla::<B>,
            },
            // 0x69
            Instruction {
                cycles: 2,
                execute: adc::<Mos6502, Immediate, B>,
            },
            // 0x6A
            Instruction {
                cycles: 2,
                execute: ror_a::<B>,
            },
            // 0x6B
            Instruction {
                cycles: 2,
                execute: arr::<Immediate, B>,
            },
            // 0x6C
            Instruction {
                cycles: 5,
                execute: jmp::<AbsoluteIndirect, B>,
            },
            // 0x6D
            Instruction {
                cycles: 4,
                execute: adc::<Mos6502, Absolute, B>,
            },
            // 0x6E
            Instruction {
                cycles: 6,
                execute: ror::<Absolute, B>,
            },
            // 0x6F
            Instruction {
                cycles: 6,
                execute: rra::<Mos6502, Absolute, B>,
            },
            // 0x70
            Instruction {
                cycles: 2,
                execute: bvs::<B>,
            },
            // 0x71
            Instruction {
                cycles: 5,
                execute: adc::<Mos6502, ZeroPageIndirectY, B>,
            },
            // 0x72
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x73
            Instruction {
                cycles: 8,
                execute: rra::<Mos6502, ZeroPageIndirectY, B>,
            },
            // 0x74
            Instruction {
                cycles: 4,
                execute: illegal::<ZeroPageX, B>,
            },
            // 0x75
            Instruction {
                cycles: 4,
                execute: adc::<Mos6502, ZeroPageX, B>,
            },
            // 0x76
            Instruction {
                cycles: 6,
                execute: ror::<ZeroPageX, B>,
            },
            // 0x77
            Instruction {
                cycles: 6,
                execute: rra::<Mos6502, ZeroPageX, B>,
            },
            // 0x78
            Instruction {
                cycles: 2,
                execute: sei::<B>,
            },
            // 0x79
            Instruction {
                cycles: 4,
                execute: adc::<Mos6502, AbsoluteY, B>,
            },
            // 0x7A
            Instruction {
                cycles: 2,
                execute: illegal_a::<B>,
            },
            // 0x7B
            Instruction {
                cycles: 7,
                execute: rra::<Mos6502, AbsoluteY, B>,
            },
            // 0x7C
            Instruction {
                cycles: 4,
                execute: illegal::<AbsoluteX, B>,
            },
            // 0x7D
            Instruction {
                cycles: 4,
                execute: adc::<Mos6502, AbsoluteX, B>,
            },
            // 0x7E
            Instruction {
                cycles: 7,
                execute: ror::<AbsoluteX, B>,
            },
            // 0x7F
            Instruction {
                cycles: 7,
                execute: rra::<Mos6502, AbsoluteX, B>,
            },
            // 0x80
            Instruction {
                cycles: 2,
                execute: illegal::<Immediate, B>,
            },
            // 0x81
            Instruction {
                cycles: 6,
                execute: sta::<ZeroPageXIndirect, B>,
            },
            // 0x82
            Instruction {
                cycles: 2,
                execute: illegal::<Immediate, B>,
            },
            // 0x83
            Instruction {
                cycles: 6,
                execute: sax::<ZeroPageXIndirect, B>,
            },
            // 0x84
            Instruction {
                cycles: 3,
                execute: sty::<ZeroPage, B>,
            },
            // 0x85
            Instruction {
                cycles: 3,
                execute: sta::<ZeroPage, B>,
            },
            // 0x86
            Instruction {
                cycles: 3,
                execute: stx::<ZeroPage, B>,
            },
            // 0x87
            Instruction {
                cycles: 3,
                execute: sax::<ZeroPage, B>,
            },
            // 0x88
            Instruction {
                cycles: 2,
                execute: dey::<B>,
            },
            // 0x89
            Instruction {
                cycles: 2,
                execute: illegal::<Immediate, B>,
            },
            // 0x8A
            Instruction {
                cycles: 2,
                execute: txa::<B>,
            },
            // 0x8B
            Instruction {
                cycles: 2,
                execute: xaa::<Immediate, B>,
            },
            // 0x8C
            Instruction {
                cycles: 4,
                execute: sty::<Absolute, B>,
            },
            // 0x8D
            Instruction {
                cycles: 4,
                execute: sta::<Absolute, B>,
            },
            // 0x8E
            Instruction {
                cycles: 4,
                execute: stx::<Absolute, B>,
            },
            // 0x8F
            Instruction {
                cycles: 4,
                execute: sax::<Absolute, B>,
            },
            // 0x90
            Instruction {
                cycles: 2,
                execute: bcc::<B>,
            },
            // 0x91
            Instruction {
                cycles: 6,
                execute: sta::<ZeroPageIndirectY, B>,
            },
            // 0x92
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0x93
            Instruction {
                cycles: 6,
                execute: sha::<ZeroPageIndirectY, B>,
            },
            // 0x94
            Instruction {
                cycles: 4,
                execute: sty::<ZeroPageX, B>,
            },
            // 0x95
            Instruction {
                cycles: 4,
                execute: sta::<ZeroPageX, B>,
            },
            // 0x96
            Instruction {
                cycles: 4,
                execute: stx::<ZeroPageY, B>,
            },
            // 0x97
            Instruction {
                cycles: 4,
                execute: sax::<ZeroPageY, B>,
            },
            // 0x98
            Instruction {
                cycles: 2,
                execute: tya::<B>,
            },
            // 0x99
            Instruction {
                cycles: 5,
                execute: sta::<AbsoluteY, B>,
            },
            // 0x9A
            Instruction {
                cycles: 2,
                execute: txs::<B>,
            },
            // 0x9B
            Instruction {
                cycles: 5,
                execute: shs::<AbsoluteY, B>,
            },
            // 0x9C
            Instruction {
                cycles: 5,
                execute: shy::<AbsoluteX, B>,
            },
            // 0x9D
            Instruction {
                cycles: 5,
                execute: sta::<AbsoluteX, B>,
            },
            // 0x9E
            Instruction {
                cycles: 5,
                execute: shx::<AbsoluteY, B>,
            },
            // 0x9F
            Instruction {
                cycles: 5,
                execute: sha::<AbsoluteY, B>,
            },
            // 0xA0
            Instruction {
                cycles: 2,
                execute: ldy::<Immediate, B>,
            },
            // 0xA1
            Instruction {
                cycles: 6,
                execute: lda::<ZeroPageXIndirect, B>,
            },
            // 0xA2
            Instruction {
                cycles: 2,
                execute: ldx::<Immediate, B>,
            },
            // 0xA3
            Instruction {
                cycles: 6,
                execute: lax::<ZeroPageXIndirect, B>,
            },
            // 0xA4
            Instruction {
                cycles: 3,
                execute: ldy::<ZeroPage, B>,
            },
            // 0xA5
            Instruction {
                cycles: 3,
                execute: lda::<ZeroPage, B>,
            },
            // 0xA6
            Instruction {
                cycles: 3,
                execute: ldx::<ZeroPage, B>,
            },
            // 0xA7
            Instruction {
                cycles: 3,
                execute: lax::<ZeroPage, B>,
            },
            // 0xA8
            Instruction {
                cycles: 2,
                execute: tay::<B>,
            },
            // 0xA9
            Instruction {
                cycles: 2,
                execute: lda::<Immediate, B>,
            },
            // 0xAA
            Instruction {
                cycles: 2,
                execute: tax::<B>,
            },
            // 0xAB
            Instruction {
                cycles: 2,
                execute: illegal::<Immediate, B>,
            },
            // 0xAC
            Instruction {
                cycles: 4,
                execute: ldy::<Absolute, B>,
            },
            // 0xAD
            Instruction {
                cycles: 4,
                execute: lda::<Absolute, B>,
            },
            // 0xAE
            Instruction {
                cycles: 4,
                execute: ldx::<Absolute, B>,
            },
            // 0xAF
            Instruction {
                cycles: 4,
                execute: lax::<Absolute, B>,
            },
            // 0xB0
            Instruction {
                cycles: 2,
                execute: bcs::<B>,
            },
            // 0xB1
            Instruction {
                cycles: 5,
                execute: lda::<ZeroPageIndirectY, B>,
            },
            // 0xB2
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0xB3
            Instruction {
                cycles: 5,
                execute: lax::<ZeroPageIndirectY, B>,
            },
            // 0xB4
            Instruction {
                cycles: 4,
                execute: ldy::<ZeroPageX, B>,
            },
            // 0xB5
            Instruction {
                cycles: 4,
                execute: lda::<ZeroPageX, B>,
            },
            // 0xB6
            Instruction {
                cycles: 4,
                execute: ldx::<ZeroPageY, B>,
            },
            // 0xB7
            Instruction {
                cycles: 4,
                execute: lax::<ZeroPageY, B>,
            },
            // 0xB8
            Instruction {
                cycles: 2,
                execute: clv::<B>,
            },
            // 0xB9
            Instruction {
                cycles: 4,
                execute: lda::<AbsoluteY, B>,
            },
            // 0xBA
            Instruction {
                cycles: 2,
                execute: tsx::<B>,
            },
            // 0xBB
            Instruction {
                cycles: 4,
                execute: las::<AbsoluteY, B>,
            },
            // 0xBC
            Instruction {
                cycles: 4,
                execute: ldy::<AbsoluteX, B>,
            },
            // 0xBD
            Instruction {
                cycles: 4,
                execute: lda::<AbsoluteX, B>,
            },
            // 0xBE
            Instruction {
                cycles: 4,
                execute: ldx::<AbsoluteY, B>,
            },
            // 0xBF
            Instruction {
                cycles: 4,
                execute: lax::<AbsoluteY, B>,
            },
            // 0xC0
            Instruction {
                cycles: 2,
                execute: cpy::<Immediate, B>,
            },
            // 0xC1
            Instruction {
                cycles: 6,
                execute: cmp::<ZeroPageXIndirect, B>,
            },
            // 0xC2
            Instruction {
                cycles: 2,
                execute: illegal::<Immediate, B>,
            },
            // 0xC3
            Instruction {
                cycles: 8,
                execute: dcp::<ZeroPageXIndirect, B>,
            },
            // 0xC4
            Instruction {
                cycles: 3,
                execute: cpy::<ZeroPage, B>,
            },
            // 0xC5
            Instruction {
                cycles: 3,
                execute: cmp::<ZeroPage, B>,
            },
            // 0xC6
            Instruction {
                cycles: 5,
                execute: dec::<ZeroPage, B>,
            },
            // 0xC7
            Instruction {
                cycles: 5,
                execute: dcp::<ZeroPage, B>,
            },
            // 0xC8
            Instruction {
                cycles: 2,
                execute: iny::<B>,
            },
            // 0xC9
            Instruction {
                cycles: 2,
                execute: cmp::<Immediate, B>,
            },
            // 0xCA
            Instruction {
                cycles: 2,
                execute: dex::<B>,
            },
            // 0xCB
            Instruction {
                cycles: 2,
                execute: sbx::<Mos6502, Immediate, B>,
            },
            // 0xCC
            Instruction {
                cycles: 4,
                execute: cpy::<Absolute, B>,
            },
            // 0xCD
            Instruction {
                cycles: 4,
                execute: cmp::<Absolute, B>,
            },
            // 0xCE
            Instruction {
                cycles: 6,
                execute: dec::<Absolute, B>,
            },
            // 0xCF
            Instruction {
                cycles: 6,
                execute: dcp::<Absolute, B>,
            },
            // 0xD0
            Instruction {
                cycles: 2,
                execute: bne::<B>,
            },
            // 0xD1
            Instruction {
                cycles: 5,
                execute: cmp::<ZeroPageIndirectY, B>,
            },
            // 0xD2
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0xD3
            Instruction {
                cycles: 8,
                execute: dcp::<ZeroPageIndirectY, B>,
            },
            // 0xD4
            Instruction {
                cycles: 4,
                execute: illegal::<ZeroPageX, B>,
            },
            // 0xD5
            Instruction {
                cycles: 4,
                execute: cmp::<ZeroPageX, B>,
            },
            // 0xD6
            Instruction {
                cycles: 6,
                execute: dec::<ZeroPageX, B>,
            },
            // 0xD7
            Instruction {
                cycles: 6,
                execute: dcp::<ZeroPageX, B>,
            },
            // 0xD8
            Instruction {
                cycles: 2,
                execute: cld::<B>,
            },
            // 0xD9
            Instruction {
                cycles: 4,
                execute: cmp::<AbsoluteY, B>,
            },
            // 0xDA
            Instruction {
                cycles: 2,
                execute: illegal_a::<B>,
            },
            // 0xDB
            Instruction {
                cycles: 7,
                execute: dcp::<AbsoluteY, B>,
            },
            // 0xDC
            Instruction {
                cycles: 4,
                execute: illegal::<AbsoluteX, B>,
            },
            // 0xDD
            Instruction {
                cycles: 4,
                execute: cmp::<AbsoluteX, B>,
            },
            // 0xDE
            Instruction {
                cycles: 7,
                execute: dec::<AbsoluteX, B>,
            },
            // 0xDF
            Instruction {
                cycles: 7,
                execute: dcp::<AbsoluteX, B>,
            },
            // 0xE0
            Instruction {
                cycles: 2,
                execute: cpx::<Immediate, B>,
            },
            // 0xE1
            Instruction {
                cycles: 6,
                execute: sbc::<Mos6502, ZeroPageXIndirect, B>,
            },
            // 0xE2
            Instruction {
                cycles: 2,
                execute: illegal::<Immediate, B>,
            },
            // 0xE3
            Instruction {
                cycles: 8,
                execute: isc::<Mos6502, ZeroPageXIndirect, B>,
            },
            // 0xE4
            Instruction {
                cycles: 3,
                execute: cpx::<ZeroPage, B>,
            },
            // 0xE5
            Instruction {
                cycles: 3,
                execute: sbc::<Mos6502, ZeroPage, B>,
            },
            // 0xE6
            Instruction {
                cycles: 5,
                execute: inc::<ZeroPage, B>,
            },
            // 0xE7
            Instruction {
                cycles: 5,
                execute: isc::<Mos6502, ZeroPage, B>,
            },
            // 0xE8
            Instruction {
                cycles: 2,
                execute: inx::<B>,
            },
            // 0xE9
            Instruction {
                cycles: 2,
                execute: sbc::<Mos6502, Immediate, B>,
            },
            // 0xEA
            Instruction {
                cycles: 2,
                execute: nop::<B>,
            },
            // 0xEB
            Instruction {
                cycles: 2,
                execute: sbc::<Mos6502, Immediate, B>,
            },
            // 0xEC
            Instruction {
                cycles: 4,
                execute: cpx::<Absolute, B>,
            },
            // 0xED
            Instruction {
                cycles: 4,
                execute: sbc::<Mos6502, Absolute, B>,
            },
            // 0xEE
            Instruction {
                cycles: 6,
                execute: inc::<Absolute, B>,
            },
            // 0xEF
            Instruction {
                cycles: 6,
                execute: isc::<Mos6502, Absolute, B>,
            },
            // 0xF0
            Instruction {
                cycles: 2,
                execute: beq::<B>,
            },
            // 0xF1
            Instruction {
                cycles: 5,
                execute: sbc::<Mos6502, ZeroPageIndirectY, B>,
            },
            // 0xF2
            Instruction {
                cycles: 0,
                execute: jam::<B>,
            },
            // 0xF3
            Instruction {
                cycles: 8,
                execute: isc::<Mos6502, ZeroPageIndirectY, B>,
            },
            // 0xF4
            Instruction {
                cycles: 4,
                execute: illegal::<ZeroPageX, B>,
            },
            // 0xF5
            Instruction {
                cycles: 4,
                execute: sbc::<Mos6502, ZeroPageX, B>,
            },
            // 0xF6
            Instruction {
                cycles: 6,
                execute: inc::<ZeroPageX, B>,
            },
            // 0xF7
            Instruction {
                cycles: 6,
                execute: isc::<Mos6502, ZeroPageX, B>,
            },
            // 0xF8
            Instruction {
                cycles: 2,
                execute: sed::<B>,
            },
            // 0xF9
            Instruction {
                cycles: 4,
                execute: sbc::<Mos6502, AbsoluteY, B>,
            },
            // 0xFA
            Instruction {
                cycles: 2,
                execute: illegal_a::<B>,
            },
            // 0xFB
            Instruction {
                cycles: 7,
                execute: isc::<Mos6502, AbsoluteY, B>,
            },
            // 0xFC
            Instruction {
                cycles: 4,
                execute: illegal::<AbsoluteX, B>,
            },
            // 0xFD
            Instruction {
                cycles: 4,
                execute: sbc::<Mos6502, AbsoluteX, B>,
            },
            // 0xFE
            Instruction {
                cycles: 7,
                execute: inc::<AbsoluteX, B>,
            },
            // 0xFF
            Instruction {
                cycles: 7,
                execute: isc::<Mos6502, AbsoluteX, B>,
            },
        ])
    }
}

impl InstructionSet for Mos6502 {
    fn instruction_table<B: Bus + 'static>() -> InstructionTable<B> {
        Self::base_table()
    }
}

pub fn lda<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    cpu.a = val;
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn ldx<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    cpu.x = val;
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn ldy<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    cpu.y = val;
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn sta<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    bus.write(addr, cpu.a, AccessType::DataWrite);
    cpu.pc += AM::BYTES;
}

pub fn stx<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    bus.write(addr, cpu.x, AccessType::DataWrite);
    cpu.pc += AM::BYTES;
}

pub fn sty<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    bus.write(addr, cpu.y, AccessType::DataWrite);
    cpu.pc += AM::BYTES;
}

pub fn tax<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.x = cpu.a;
    cpu.p.set_zero(cpu.x == 0);
    cpu.p.set_signed(cpu.x.is_signed());
    cpu.pc += 1;
}

pub fn tay<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.y = cpu.a;
    cpu.p.set_zero(cpu.y == 0);
    cpu.p.set_signed(cpu.y.is_signed());
    cpu.pc += 1;
}

pub fn tsx<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.x = cpu.sp;
    cpu.p.set_zero(cpu.x == 0);
    cpu.p.set_signed(cpu.x.is_signed());
    cpu.pc += 1;
}

pub fn txa<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.a = cpu.x;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());
    cpu.pc += 1;
}

pub fn txs<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.sp = cpu.x;
    cpu.pc += 1;
}

pub fn tya<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.a = cpu.y;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());
    cpu.pc += 1;
}

pub fn pha<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.push(bus, cpu.a);
    cpu.pc += 1;
}

pub fn php<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let bits = cpu.p | Flags::Break | Flags::Expansion;
    cpu.push(bus, byte!(bits));
    cpu.pc += 1;
}

pub fn pla<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let val = cpu.pop(bus);
    cpu.a = val;
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());
    cpu.pc += 1;
}

pub fn plp<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let val = cpu.pop(bus) | Flags::Expansion;
    let masked = val & !Flags::Break;
    cpu.p = Flags::from_bits_truncate(u8::from(masked));
    cpu.pc += 1;
}

pub fn asl<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let mut val = bus.read(addr, AccessType::DataRead);

    cpu.p.set_carry(val & Flags::Sign != 0);
    val <<= 1;
    bus.write(addr, val, AccessType::DataWrite);
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn asl_a<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_carry(cpu.a & Flags::Sign != 0);
    cpu.a <<= 1;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());
    cpu.pc += 1;
}

pub fn lsr<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let mut val = bus.read(addr, AccessType::DataRead);

    cpu.p.set_carry(val & Flags::Carry != 0);
    val >>= 1;
    bus.write(addr, val, AccessType::DataWrite);
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn lsr_a<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_carry(cpu.a & Flags::Carry != 0);
    cpu.a >>= 1;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());
    cpu.pc += 1;
}

pub fn rol<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let rotated = rotate_left(cpu, val);
    bus.write(addr, rotated, AccessType::DataWrite);
    cpu.pc += AM::BYTES;
}

pub fn rol_a<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.a = rotate_left(cpu, cpu.a);
    cpu.pc += 1;
}

pub fn ror<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let rotated = rotate_right(cpu, val);
    bus.write(addr, rotated, AccessType::DataWrite);
    cpu.pc += AM::BYTES;
}

pub fn ror_a<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.a = rotate_right(cpu, cpu.a);
    cpu.pc += 1;
}

fn rotate_left<B: Bus + 'static>(cpu: &mut Cpu<B>, mut val: Byte) -> Byte {
    let old_carry = cpu.p.bit(Flags::Carry);
    cpu.p.set_carry(val & Flags::Sign != 0);
    val <<= 1;
    val |= old_carry;
    cpu.p.set_zero(val == Byte(0));
    cpu.p.set_signed(val.is_signed());
    val
}

fn rotate_right<B: Bus + 'static>(cpu: &mut Cpu<B>, mut val: Byte) -> Byte {
    let old_carry = (cpu.p.bit(Flags::Carry)) << 7;
    cpu.p.set_carry(val & Flags::Carry != 0);
    val >>= 1;
    val |= old_carry;
    cpu.p.set_zero(val == Byte(0));
    cpu.p.set_signed(val.is_signed());
    val
}

pub fn and<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);

    cpu.a &= val;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn bit<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let test = val & cpu.a;

    cpu.p.set_zero(test == 0);
    cpu.p.set_signed(val.is_signed());
    cpu.p.set_overflow(val & Flags::Overflow != 0);

    cpu.pc += AM::BYTES;
}

pub fn eor<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);

    cpu.a ^= val;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn ora<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);

    cpu.a |= val;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn adc<S: InstructionSet, AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let value = bus.read(addr, AccessType::DataRead);
    add_with_carry::<S, _>(cpu, value);
    cpu.pc += AM::BYTES;
}

pub fn cmp<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let (raw, overflow) = u8::from(cpu.a).overflowing_sub(u8::from(val));
    let result = byte!(raw);

    cpu.p.set_zero(result == Byte(0));
    cpu.p.set_signed(result.is_signed());
    cpu.p.set_carry(!overflow);

    cpu.pc += AM::BYTES;
}

pub fn cpx<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let (raw, overflow) = u8::from(cpu.x).overflowing_sub(u8::from(val));
    let result = byte!(raw);

    cpu.p.set_zero(result == Byte(0));
    cpu.p.set_signed(result.is_signed());
    cpu.p.set_carry(!overflow);

    cpu.pc += AM::BYTES;
}

pub fn cpy<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let (raw, overflow) = u8::from(cpu.y).overflowing_sub(u8::from(val));
    let result = byte!(raw);

    cpu.p.set_zero(result == Byte(0));
    cpu.p.set_signed(result.is_signed());
    cpu.p.set_carry(!overflow);

    cpu.pc += AM::BYTES;
}

pub fn sbc<S: InstructionSet, AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let value = bus.read(addr, AccessType::DataRead);
    sub_with_borrow::<S, _>(cpu, value);
    cpu.pc += AM::BYTES;
}

pub(crate) fn add_with_carry<S: InstructionSet, B: Bus + 'static>(cpu: &mut Cpu<B>, value: Byte) {
    let carry_in = u16::from(cpu.p.contains(Flags::Carry));
    let decimal = S::SUPPORTS_DECIMAL_MODE && cpu.p.contains(Flags::DecimalMode);

    let sum = u16::from(cpu.a) + u16::from(value) + carry_in;
    let mut result = byte!((sum & 0x00FF) as u8);

    cpu.p
        .set_overflow(((cpu.a ^ result) & (value ^ result) & Byte(0x80)) != Byte(0)); // overflow when operands had same sign but result differs

    if decimal {
        // In BCD mode each nibble represents a base-10 digit. Add the ones place
        // and manually correct if it overflowed past 9, then fold the carry into
        // the tens place and apply the same correction. Pack the resulting digits
        // back into a byte instead of relying on binary addition overflow.
        let mut lo = i16::from(cpu.a.lo()) + i16::from(value.lo()) + i16::from(carry_in as u8); // add ones digit
        let mut carry_10 = 0i16;
        if lo > 9 {
            lo -= 10;
            carry_10 = 1;
        }

        let mut hi = i16::from(cpu.a.hi()) + i16::from(value.hi()) + carry_10; // add tens digit
        let mut carry_out = false;
        if hi > 9 {
            hi -= 10;
            carry_out = true;
        }

        result = Byte::from((Nibble::from(lo as u8), Nibble::from(hi as u8)));
        cpu.p.set_carry(carry_out);
    } else {
        cpu.p.set_carry(sum > 0xFF);
    }

    cpu.p.set_zero(result == Byte(0));
    cpu.p.set_signed(result.is_signed());
    cpu.a = result;
}

pub(crate) fn sub_with_borrow<S: InstructionSet, B: Bus + 'static>(cpu: &mut Cpu<B>, value: Byte) {
    let carry_in = u16::from(cpu.p.contains(Flags::Carry));
    let decimal = S::SUPPORTS_DECIMAL_MODE && cpu.p.contains(Flags::DecimalMode);

    // subtract via addition of the complement
    let sum = u16::from(cpu.a) + (u16::from(value) ^ 0x00FF) + carry_in;
    let mut result = byte!((sum & 0x00FF) as u8);

    cpu.p
        .set_overflow(((cpu.a ^ result) & ((value ^ Byte(0xFF)) ^ result) & Byte(0x80)) != Byte(0)); // same-sign check adjusted for subtraction form

    if decimal {
        // Decimal subtraction is performed per digit. Subtract the low nibble,
        // borrowing from the high nibble when the result would go negative. Then
        // subtract the high nibble with the propagated borrow and re-pack the result.
        let borrow_lo = i16::from(carry_in != 1); // carry==1 means no borrow
        let a_lo = i16::from(cpu.a.lo());
        let b_lo = i16::from(value.lo());

        let (lo, borrow_hi) = if a_lo >= b_lo + borrow_lo {
            (a_lo - (b_lo + borrow_lo), 0i16)
        } else {
            (a_lo + 10 - (b_lo + borrow_lo), 1i16) // borrow 10 from tens digit
        };

        let a_hi = i16::from(cpu.a.hi());
        let b_hi = i16::from(value.hi());

        let (hi, carry_out) = if a_hi >= b_hi + borrow_hi {
            (a_hi - (b_hi + borrow_hi), true)
        } else {
            (a_hi + 10 - (b_hi + borrow_hi), false) // final borrow clears carry flag
        };

        result = Byte::from((Nibble::from(lo as u8), Nibble::from(hi as u8)));
        cpu.p.set_carry(carry_out);
    } else {
        cpu.p.set_carry(sum > 0xFF);
    }

    cpu.p.set_zero(result == Byte(0));
    cpu.p.set_signed(result.is_signed());
    cpu.a = result;
}

pub fn dec<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead) - 1;
    bus.write(addr, val, AccessType::DataWrite);
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn dex<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.x -= 1;
    cpu.p.set_zero(cpu.x == 0);
    cpu.p.set_signed(cpu.x.is_signed());

    cpu.pc += 1;
}

pub fn dey<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.y -= 1;
    cpu.p.set_zero(cpu.y == 0);
    cpu.p.set_signed(cpu.y.is_signed());
    cpu.pc += 1;
}

pub fn inc<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead) + 1;
    bus.write(addr, val, AccessType::DataWrite);
    cpu.p.set_zero(val == 0);
    cpu.p.set_signed(val.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn inx<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.x += 1;
    cpu.p.set_zero(cpu.x == 0);
    cpu.p.set_signed(cpu.x.is_signed());

    cpu.pc += 1;
}

pub fn iny<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.y += 1;
    cpu.p.set_zero(cpu.y == 0);
    cpu.p.set_signed(cpu.y.is_signed());

    cpu.pc += 1;
}

pub fn brk<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let return_pc = word!(cpu.pc + 2);

    cpu.push(bus, return_pc.hi());
    cpu.push(bus, return_pc.lo());

    let bits = cpu.p | Flags::Break | Flags::Expansion;
    cpu.push(bus, byte!(bits));

    cpu.p.set_interrupt_disabled(true);

    let lo = bus.read(IRQ_VECTOR_LO, AccessType::InterruptVectorRead);
    let hi = bus.read(IRQ_VECTOR_HI, AccessType::InterruptVectorRead);
    cpu.pc = word!((lo, hi));
}

pub fn jmp<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.pc = AM::fetch_address(cpu, bus);
}

pub fn jsr<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let return_addr = cpu.pc + (AM::BYTES - 1);

    cpu.push(bus, return_addr.hi());
    cpu.push(bus, return_addr.lo());

    cpu.pc = addr;
}

pub fn rti<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let mut p = cpu.pop(bus);
    p |= Flags::Expansion; // force E to 1
    p &= !Flags::Break; // clear B
    cpu.p = p.into();

    let lo = cpu.pop(bus);
    let hi = cpu.pop(bus);
    cpu.pc = word!((lo, hi));
}

pub fn rts<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let lo = cpu.pop(bus);
    let hi = cpu.pop(bus);
    cpu.pc = word!((lo, hi)) + 1;
}

pub fn bcc<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if cpu.p.contains(Flags::Carry) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn bcs<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if !cpu.p.contains(Flags::Carry) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn beq<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if !cpu.p.contains(Flags::Zero) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn bmi<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if !cpu.p.contains(Flags::Sign) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn bne<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if cpu.p.contains(Flags::Zero) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn bpl<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if cpu.p.contains(Flags::Sign) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn bvc<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if cpu.p.contains(Flags::Overflow) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn bvs<B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let base = cpu.pc + 2;

    if !cpu.p.contains(Flags::Overflow) {
        cpu.pc = base;
        return;
    }

    let offset = i8::from(bus.read(cpu.pc + 1, AccessType::DataRead));
    let target = base + offset;
    cpu.cycles += 1;
    if cpu.crosses_page(base, target) {
        cpu.cycles += 1;
    }
    cpu.pc = target;
}

pub fn clc<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_carry(false);
    cpu.pc += 1;
}

pub fn cld<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_decimal_mode(false);
    cpu.pc += 1;
}

pub fn cli<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_interrupt_disabled(false);
    cpu.pc += 1;
}

pub fn clv<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_overflow(false);
    cpu.pc += 1;
}

pub fn sec<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_carry(true);
    cpu.pc += 1;
}

pub fn sed<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_decimal_mode(true);
    cpu.pc += 1;
}

pub fn sei<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.p.set_interrupt_disabled(true);
    cpu.pc += 1;
}

pub fn nop<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.pc += 1;
}

// Undocumented instructions
pub fn las<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let result = val & cpu.sp;

    cpu.a = result;
    cpu.x = result;
    cpu.sp = result;
    cpu.p.set_zero(result == Byte(0));
    cpu.p.set_signed(result.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn lax<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);

    cpu.a = val;
    cpu.x = val;

    cpu.p.set_zero(val == Byte(0));
    cpu.p.set_signed(val.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn sax<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    bus.write(addr, cpu.a & cpu.x, AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn sha<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = addr.hi() + 1;

    bus.write(addr, val & (cpu.x & cpu.a), AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn shx<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = addr.hi() + 1;

    bus.write(addr, val & cpu.x, AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn shy<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = addr.hi() + 1;

    bus.write(addr, val & cpu.y, AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn shs<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);

    cpu.sp = cpu.x & cpu.a;
    bus.write(addr, cpu.sp & (addr.hi() + 1), AccessType::DataWrite);

    cpu.pc += AM::BYTES;
}

pub fn anc<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let result = val & cpu.a;

    cpu.a = result;
    cpu.p.set_zero(result == 0);
    cpu.p.set_signed(result.is_signed());
    cpu.p.set_carry(result.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn arr<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let anded = cpu.a & val;

    let carry_in = cpu.p.bit(Flags::Carry);
    let mut result = (anded >> 1) | (carry_in << 7);

    if cpu.p.contains(Flags::DecimalMode) {
        // BCD correction
        let high = result & 0xF0;
        let low = result & 0x0F;
        cpu.p.set_carry(high > 0x40);
        cpu.p
            .set_overflow(((high & 0x40) >> 6) ^ ((high & 0x20) >> 5) != 0);
        if low + (low & 0x01) > 5 {
            result += 0x06;
        }

        if high > 0x40 {
            result += 0x60;
        }
    } else {
        cpu.p.set_carry(anded & 0x40 != 0);
        let val = u8::from(result);
        cpu.p.set_overflow(((val >> 5) ^ (val >> 6)) & 1 != 0);
    }

    cpu.a = result;
    cpu.p.set_zero(result == 0);
    cpu.p.set_signed(result.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn asr<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let result = cpu.a & val;

    cpu.p.set_carry(result & 0x01 != 0);
    cpu.a = result >> 1;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(false);

    cpu.pc += AM::BYTES;
}

pub fn dcp<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let decremented = val - 1;
    bus.write(addr, decremented, AccessType::DataWrite);

    let result = cpu.a - decremented;

    cpu.p.set_zero(result == 0);
    cpu.p.set_signed(result.is_signed());
    cpu.p.set_carry(cpu.a >= decremented);
    cpu.pc += AM::BYTES;
}

pub fn isc<S: InstructionSet, AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let value = bus.read(addr, AccessType::DataRead) + 1;
    bus.write(addr, value, AccessType::DataWrite);

    sub_with_borrow::<S, _>(cpu, value);

    cpu.pc += AM::BYTES;
}

pub fn rla<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let rotated = rotate_left(cpu, val);
    bus.write(addr, rotated, AccessType::DataWrite);

    cpu.a &= rotated;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());

    cpu.pc += AM::BYTES;
}
pub fn rra<S: InstructionSet, AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let rotated = rotate_right(cpu, val);
    bus.write(addr, rotated, AccessType::DataWrite);
    add_with_carry::<S, _>(cpu, rotated);

    cpu.pc += AM::BYTES;
}

pub fn sbx<S: InstructionSet, AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let masked = cpu.a & cpu.x;
    let result = masked - val;
    cpu.x = result;

    cpu.p.set_carry(masked >= val);
    cpu.p.set_zero(cpu.x == 0);
    cpu.p.set_signed(cpu.x.is_signed());

    cpu.pc += AM::BYTES;
}

pub fn slo<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let result = rotate_left(cpu, val);
    bus.write(addr, result, AccessType::DataWrite);

    cpu.a |= result;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn sre<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    let addr = AM::fetch_address(cpu, bus);
    let val = bus.read(addr, AccessType::DataRead);
    let result = rotate_right(cpu, val);
    bus.write(addr, result, AccessType::DataWrite);

    cpu.a ^= result;
    cpu.p.set_zero(cpu.a == 0);
    cpu.p.set_signed(cpu.a.is_signed());
    cpu.pc += AM::BYTES;
}

pub fn xaa<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, bus: &mut B) {
    illegal_a(cpu, bus);
}

pub fn jam<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.run_state = RunState::Halted;
}

pub fn illegal<AM: AddressingMode, B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.pc += AM::BYTES;
}

pub fn illegal_a<B: Bus + 'static>(cpu: &mut Cpu<B>, _bus: &mut B) {
    cpu.pc += 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::addressing_mode::{
        Absolute, AbsoluteIndirect, AbsoluteX, AbsoluteY, Immediate, ZeroPage, ZeroPageIndirectY,
        ZeroPageX, ZeroPageY,
    };
    use ull::{SimpleBus, Word};

    #[test]
    fn test_lda_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);

        bus.write(cpu.pc + 1, VALUE, AccessType::DataWrite);

        lda::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, VALUE);
        assert_eq!(cpu.pc, Immediate::BYTES.into());
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
    }

    #[test]
    fn test_lda_absolute() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const TARGET_ADDRESS: u16 = 0x1234;

        let (lo, hi) = word!(TARGET_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);
        bus.write(TARGET_ADDRESS, VALUE, AccessType::DataWrite);

        lda::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, VALUE);
        assert_eq!(cpu.pc, Absolute::BYTES.into());
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const OFFSET: u8 = 0x00;
        const BASE_ADDRESS: u16 = 0x1234;
        const TARGET_ADDRESS: u16 = BASE_ADDRESS + OFFSET as u16;

        cpu.x = byte!(OFFSET);

        let (lo, hi) = word!(BASE_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);
        bus.write(TARGET_ADDRESS, VALUE, AccessType::DataWrite);

        lda::<AbsoluteX, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, VALUE);
        assert_eq!(cpu.pc, AbsoluteX::BYTES.into());
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
    }

    #[test]
    fn test_ldx_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);

        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), VALUE, AccessType::DataWrite);

        ldx::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.x, VALUE);
        assert!(!cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Word(0x0202));
    }

    #[test]
    fn test_ldx_zero_sets_flag() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x00);

        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), VALUE, AccessType::DataWrite);

        ldx::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.x, VALUE);
        assert!(cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_ldx_negative_sets_sign() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0xFF);

        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), VALUE, AccessType::DataWrite);

        ldx::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.x, VALUE);
        assert!(!cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_ldy_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);

        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), VALUE, AccessType::DataWrite);

        ldy::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.y, VALUE);
        assert!(!cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Word(0x0202));
    }

    #[test]
    fn test_ldy_zero_sets_flag() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x00);

        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), VALUE, AccessType::DataWrite);

        ldy::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.y, VALUE);
        assert!(cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_ldy_negative_sets_sign() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x80);

        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), VALUE, AccessType::DataWrite);

        ldy::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.y, VALUE);
        assert!(!cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_sta_absolute() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const TARGET_ADDRESS: u16 = 0x1234;

        cpu.a = VALUE;
        let (lo, hi) = word!(TARGET_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);

        sta::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), VALUE);
        assert_eq!(cpu.pc, Absolute::BYTES.into());
    }

    #[test]
    fn test_sta_absolute_x() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const OFFSET: u8 = 0x00;
        const BASE_ADDRESS: u16 = 0x1234;
        const TARGET_ADDRESS: u16 = BASE_ADDRESS + OFFSET as u16;

        cpu.a = VALUE;
        cpu.x = byte!(OFFSET);
        let (lo, hi) = word!(BASE_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);

        sta::<AbsoluteX, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), VALUE);
        assert_eq!(cpu.pc, AbsoluteX::BYTES.into());
    }

    #[test]
    fn test_sta_absolute_y() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const OFFSET: u8 = 0x00;
        const BASE_ADDRESS: u16 = 0x1234;
        const TARGET_ADDRESS: u16 = BASE_ADDRESS + OFFSET as u16;

        cpu.a = VALUE;
        cpu.y = byte!(OFFSET);
        let (lo, hi) = word!(BASE_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);

        sta::<AbsoluteY, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), VALUE);
        assert_eq!(cpu.pc, AbsoluteY::BYTES.into());
    }

    #[test]
    fn test_sta_zero_page() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const TARGET_ADDRESS: Byte = Byte(0x12);

        cpu.a = VALUE;
        bus.write(cpu.pc + 1, TARGET_ADDRESS, AccessType::DataWrite);

        sta::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(TARGET_ADDRESS), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_sta_zero_page_x() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const OFFSET: Byte = Byte(0x00);
        const BASE_ADDRESS: Byte = Byte(0x12);
        let target_address = BASE_ADDRESS + OFFSET;

        cpu.a = VALUE;
        cpu.x = OFFSET;
        bus.write(cpu.pc + 1, BASE_ADDRESS, AccessType::DataWrite);

        sta::<ZeroPageX, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(target_address), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.pc, ZeroPageX::BYTES.into());
    }

    #[test]
    fn test_sta_zero_page_y() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const OFFSET: Byte = Byte(0x00);
        const BASE_ADDRESS: Byte = Byte(0x12);
        let target_address = BASE_ADDRESS + OFFSET;

        cpu.a = VALUE;
        cpu.y = OFFSET;
        bus.write(cpu.pc + 1, BASE_ADDRESS, AccessType::DataWrite);

        sta::<ZeroPageY, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(target_address), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.pc, ZeroPageY::BYTES.into());
    }

    #[test]
    fn test_sta_zero_page_indirect_y() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const OFFSET: u8 = 0x05;
        const POINTER_LOCATION: Byte = Byte(0x10);
        const BASE_ADDRESS: u16 = 0x1200;
        const TARGET_ADDRESS: u16 = BASE_ADDRESS + OFFSET as u16; // 0x1205
        const VALUE: Byte = Byte(0x42);

        cpu.a = VALUE;
        cpu.y = byte!(OFFSET);

        let (lo, hi) = word!(BASE_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, POINTER_LOCATION, AccessType::DataWrite);
        bus.write(u16::from(POINTER_LOCATION), lo, AccessType::DataWrite);
        bus.write(u16::from(POINTER_LOCATION) + 1, hi, AccessType::DataRead);

        sta::<ZeroPageIndirectY, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), VALUE);
        assert_eq!(cpu.pc, ZeroPageIndirectY::BYTES.into());
    }

    #[test]
    fn test_stx_absolute() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const TARGET_ADDRESS: u16 = 0x1234;

        cpu.x = VALUE;
        let (lo, hi) = word!(TARGET_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);

        stx::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), VALUE);
        assert_eq!(cpu.pc, Absolute::BYTES.into());
    }

    #[test]
    fn test_stx_zero_page() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const TARGET_ADDRESS: Byte = Byte(0x12);

        cpu.x = VALUE;
        bus.write(cpu.pc + 1, TARGET_ADDRESS, AccessType::DataWrite);

        stx::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(TARGET_ADDRESS), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_stx_zero_page_y() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const OFFSET: Byte = Byte(0x04);
        const BASE_ADDRESS: Byte = Byte(0x12);
        let target_address = BASE_ADDRESS + OFFSET;

        cpu.x = VALUE;
        cpu.y = OFFSET;
        bus.write(cpu.pc + 1, BASE_ADDRESS, AccessType::DataWrite);

        stx::<ZeroPageY, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(target_address), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.pc, ZeroPageY::BYTES.into());
    }

    #[test]
    fn test_sty_absolute() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const TARGET_ADDRESS: u16 = 0x1234;

        cpu.y = VALUE;
        let (lo, hi) = word!(TARGET_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);

        sty::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), VALUE);
        assert_eq!(cpu.pc, Absolute::BYTES.into());
    }

    #[test]
    fn test_sty_zero_page() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const TARGET_ADDRESS: Byte = Byte(0x12);

        cpu.y = VALUE;
        bus.write(cpu.pc + 1, TARGET_ADDRESS, AccessType::DataWrite);

        sty::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(TARGET_ADDRESS), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_sty_zero_page_y() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        const OFFSET: Byte = Byte(0x04);
        const BASE_ADDRESS: Byte = Byte(0x12);
        let target_address = BASE_ADDRESS + OFFSET;

        cpu.y = VALUE;
        cpu.x = OFFSET;
        bus.write(cpu.pc + 1, BASE_ADDRESS, AccessType::DataWrite);

        sty::<ZeroPageX, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(target_address), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.pc, ZeroPageX::BYTES.into());
    }

    #[test]
    fn test_tax() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        cpu.a = VALUE;

        let initial_pc = cpu.pc;
        tax(&mut cpu, &mut bus);

        assert_eq!(cpu.x, VALUE);
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_tay() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        cpu.a = VALUE;

        let initial_pc = cpu.pc;
        tay(&mut cpu, &mut bus);

        assert_eq!(cpu.y, VALUE);
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_tsx() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0xFD);
        cpu.sp = VALUE;

        let initial_pc = cpu.pc;
        tsx(&mut cpu, &mut bus);

        assert_eq!(cpu.x, VALUE);
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_txa() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        cpu.x = VALUE;

        let initial_pc = cpu.pc;
        txa(&mut cpu, &mut bus);

        assert_eq!(cpu.a, VALUE);
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_txs() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0xFF);
        cpu.x = VALUE;

        let initial_pc = cpu.pc;
        txs(&mut cpu, &mut bus);

        assert_eq!(cpu.sp, VALUE);
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_tya() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        cpu.y = VALUE;

        let initial_pc = cpu.pc;
        tya(&mut cpu, &mut bus);

        assert_eq!(cpu.a, VALUE);
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_pha() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        cpu.a = VALUE;

        let initial_sp = cpu.sp;
        let initial_pc = cpu.pc;
        pha(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(Word(0x0100 + u16::from(initial_sp)), AccessType::DataRead),
            VALUE
        );
        assert_eq!(cpu.sp, initial_sp - 1);
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_php() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        cpu.p.insert(Flags::Carry);
        cpu.p.insert(Flags::Zero);

        let initial_sp = cpu.sp;
        let initial_pc = cpu.pc;
        php(&mut cpu, &mut bus);

        let pushed_flags = bus.read(Word(0x0100 + u16::from(initial_sp)), AccessType::DataRead);
        assert_eq!(pushed_flags & Flags::Carry, Flags::Carry);
        assert_eq!(pushed_flags & Flags::Zero, Flags::Zero);
        assert_eq!(pushed_flags & Flags::Break, Flags::Break);
        assert_eq!(pushed_flags & Flags::Expansion, Flags::Expansion);
        assert_eq!(cpu.sp, initial_sp - 1);
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_pla() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);
        cpu.sp = Byte(0xFE);
        bus.write(Word(0x0100 + 0xFF), VALUE, AccessType::DataWrite);

        let initial_pc = cpu.pc;
        pla(&mut cpu, &mut bus);

        assert_eq!(cpu.a, VALUE);
        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.p.contains(Flags::Zero), VALUE == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), VALUE.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_plp() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        let flags_value = byte!(Flags::Carry | Flags::Zero);
        cpu.sp = Byte(0xFE);
        bus.write(Word(0x0100 + 0xFF), flags_value, AccessType::DataWrite);

        let initial_pc = cpu.pc;
        plp(&mut cpu, &mut bus);

        assert!(cpu.p.contains(Flags::Carry));
        assert!(cpu.p.contains(Flags::Zero));
        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_asl_a() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b1010_0100);
        cpu.a = INPUT;

        let initial_pc = cpu.pc;
        asl_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_asl_a_with_carry() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b1101_0010);
        const EXPECTED: Byte = Byte(0b1010_0100);
        cpu.a = INPUT;

        let initial_pc = cpu.pc;
        asl_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_lsr_a() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b0010_1001);
        cpu.a = INPUT;

        let initial_pc = cpu.pc;
        lsr_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_lsr_a_with_carry() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b0101_0011);
        const EXPECTED: Byte = Byte(0b0010_1001);
        cpu.a = INPUT;

        let initial_pc = cpu.pc;
        lsr_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_rol_a() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b1010_0101);
        cpu.a = INPUT;
        cpu.p.insert(Flags::Carry);

        let initial_pc = cpu.pc;
        rol_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_rol_a_with_carry() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b1101_0010);
        const EXPECTED: Byte = Byte(0b1010_0100);
        cpu.a = INPUT;
        cpu.p.remove(Flags::Carry);

        let initial_pc = cpu.pc;
        rol_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_ror_a() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b1010_1001);
        cpu.a = INPUT;
        cpu.p.insert(Flags::Carry);

        let initial_pc = cpu.pc;
        ror_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_ror_a_with_carry() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INPUT: Byte = Byte(0b0101_0011);
        const EXPECTED: Byte = Byte(0b0010_1001);
        cpu.a = INPUT;
        cpu.p.remove(Flags::Carry);

        let initial_pc = cpu.pc;
        ror_a(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED);
        assert!(cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.p.contains(Flags::Zero), cpu.a == 0);
        assert_eq!(cpu.p.contains(Flags::Sign), cpu.a.is_signed());
        assert_eq!(cpu.pc, initial_pc + 1);
    }

    #[test]
    fn test_asl_zero_page() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const ADDRESS: Byte = Byte(0x42);
        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b1010_0100);

        bus.write(cpu.pc + 1, ADDRESS, AccessType::DataWrite);
        bus.write(ADDRESS, INPUT, AccessType::DataWrite);

        asl::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(ADDRESS, AccessType::DataRead), EXPECTED);
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_lsr_absolute() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const TARGET_ADDRESS: u16 = 0x1234;
        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b0010_1001);

        let (lo, hi) = word!(TARGET_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);
        bus.write(TARGET_ADDRESS, INPUT, AccessType::DataWrite);

        lsr::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), EXPECTED);
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.pc, Absolute::BYTES.into());
    }

    #[test]
    fn test_rol_zero_page_x() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const BASE_ADDRESS: Byte = Byte(0x40);
        const OFFSET: Byte = Byte(0x02);
        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b1010_0101);
        let target_address = BASE_ADDRESS + OFFSET;

        cpu.x = OFFSET;
        cpu.p.insert(Flags::Carry);

        bus.write(cpu.pc + 1, BASE_ADDRESS, AccessType::DataWrite);
        bus.write(u16::from(target_address), INPUT, AccessType::DataWrite);

        rol::<ZeroPageX, _>(&mut cpu, &mut bus);

        assert_eq!(
            bus.read(u16::from(target_address), AccessType::DataRead),
            EXPECTED
        );
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.pc, ZeroPageX::BYTES.into());
    }

    #[test]
    fn test_ror_absolute_x() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const BASE_ADDRESS: u16 = 0x1230;
        const OFFSET: u8 = 0x04;
        const TARGET_ADDRESS: u16 = BASE_ADDRESS + OFFSET as u16;
        const INPUT: Byte = Byte(0b0101_0010);
        const EXPECTED: Byte = Byte(0b1010_1001);

        cpu.x = byte!(OFFSET);
        cpu.p.insert(Flags::Carry);

        let (lo, hi) = word!(BASE_ADDRESS).lo_hi();
        bus.write(cpu.pc + 1, lo, AccessType::DataWrite);
        bus.write(cpu.pc + 2, hi, AccessType::DataWrite);
        bus.write(TARGET_ADDRESS, INPUT, AccessType::DataWrite);

        ror::<AbsoluteX, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(TARGET_ADDRESS, AccessType::DataRead), EXPECTED);
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.pc, AbsoluteX::BYTES.into());
    }

    #[test]
    fn test_and_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0x3C);
        const MASK_VALUE: Byte = Byte(0x0F);
        const EXPECTED_RESULT: Byte = Byte(0x0C);

        cpu.a = A_VALUE;
        bus.write(cpu.pc + 1, MASK_VALUE, AccessType::DataWrite);

        and::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED_RESULT);
        assert!(!cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Immediate::BYTES.into());
    }

    #[test]
    fn test_ora_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0x01);
        const OR_VALUE: Byte = Byte(0x80);
        const EXPECTED_RESULT: Byte = Byte(0x81);

        cpu.a = A_VALUE;
        bus.write(cpu.pc + 1, OR_VALUE, AccessType::DataWrite);

        ora::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED_RESULT);
        assert!(cpu.p.contains(Flags::Sign));
        assert!(!cpu.p.contains(Flags::Zero));
        assert_eq!(cpu.pc, Immediate::BYTES.into());
    }

    #[test]
    fn test_eor_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0xFF);
        const XOR_VALUE: Byte = Byte(0x0F);
        const EXPECTED_RESULT: Byte = Byte(0xF0);

        cpu.a = A_VALUE;
        bus.write(cpu.pc + 1, XOR_VALUE, AccessType::DataWrite);

        eor::<Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED_RESULT);
        assert!(cpu.p.contains(Flags::Sign));
        assert!(!cpu.p.contains(Flags::Zero));
        assert_eq!(cpu.pc, Immediate::BYTES.into());
    }

    #[test]
    fn test_adc_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0x70);
        const ADD_VALUE: Byte = Byte(0x10);
        const EXPECTED_RESULT: Byte = Byte(0x81);

        cpu.a = A_VALUE;
        cpu.p.insert(Flags::Carry);
        bus.write(cpu.pc + 1, ADD_VALUE, AccessType::DataWrite);

        adc::<Mos6502, Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED_RESULT);
        assert!(cpu.p.contains(Flags::Overflow));
        assert!(cpu.p.contains(Flags::Sign));
        assert!(!cpu.p.contains(Flags::Carry));
        assert_eq!(cpu.pc, Immediate::BYTES.into());
    }

    #[test]
    fn test_sbc_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0x10);
        const SUB_VALUE: Byte = Byte(0x10);
        const EXPECTED_RESULT: Byte = Byte(0x00);

        cpu.a = A_VALUE;
        cpu.p.insert(Flags::Carry);
        bus.write(cpu.pc + 1, SUB_VALUE, AccessType::DataWrite);

        sbc::<Mos6502, Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED_RESULT);
        assert!(cpu.p.contains(Flags::Carry));
        assert!(cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Immediate::BYTES.into());
    }

    #[test]
    fn test_inc_zero_page() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const ADDRESS: Byte = Byte(0x30);
        const INITIAL_VALUE: Byte = Byte(0xFF);
        const EXPECTED_VALUE: Byte = Byte(0x00);

        bus.write(cpu.pc + 1, ADDRESS, AccessType::DataWrite);
        bus.write(Word(0x0030), INITIAL_VALUE, AccessType::DataWrite);

        inc::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(Word(0x0030), AccessType::DataRead), EXPECTED_VALUE);
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_dec_zero_page() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const ADDRESS: Byte = Byte(0x31);
        const INITIAL_VALUE: Byte = Byte(0x00);
        const EXPECTED_VALUE: Byte = Byte(0xFF);

        bus.write(cpu.pc + 1, ADDRESS, AccessType::DataWrite);
        bus.write(Word(0x0031), INITIAL_VALUE, AccessType::DataWrite);

        dec::<ZeroPage, _>(&mut cpu, &mut bus);

        assert_eq!(bus.read(Word(0x0031), AccessType::DataRead), EXPECTED_VALUE);
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_inx_wraps_and_updates_pc() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INITIAL_VALUE: Byte = Byte(0xFF);
        const EXPECTED_VALUE: Byte = Byte(0x00);

        cpu.x = INITIAL_VALUE;

        inx(&mut cpu, &mut bus);

        assert_eq!(cpu.x, EXPECTED_VALUE);
        assert_eq!(cpu.pc, Word(1));
    }

    #[test]
    fn test_iny_wraps_and_updates_pc() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INITIAL_VALUE: Byte = Byte(0x7F);
        const EXPECTED_VALUE: Byte = Byte(0x80);

        cpu.y = INITIAL_VALUE;

        iny(&mut cpu, &mut bus);

        assert_eq!(cpu.y, EXPECTED_VALUE);
        assert_eq!(cpu.pc, Word(1));
    }

    #[test]
    fn test_cpx_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x10);

        cpu.x = VALUE;
        bus.write(cpu.pc + 1, VALUE, AccessType::DataWrite);

        cpx::<Immediate, _>(&mut cpu, &mut bus);

        assert!(cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Carry));
        assert!(!cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Immediate::BYTES.into());
    }

    #[test]
    fn test_cpy_immediate() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x10);

        cpu.y = VALUE;
        bus.write(cpu.pc + 1, VALUE, AccessType::DataWrite);

        cpy::<Immediate, _>(&mut cpu, &mut bus);

        assert!(cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Carry));
        assert!(!cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Immediate::BYTES.into());
    }

    #[test]
    fn test_bit_zero_page() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0b0100_0000);
        const ADDRESS: Byte = Byte(0x40);
        const MEMORY_VALUE: Byte = Byte(0b1100_0000);

        cpu.a = A_VALUE;
        bus.write(cpu.pc + 1, ADDRESS, AccessType::DataWrite);
        bus.write(Word(0x0040), MEMORY_VALUE, AccessType::DataWrite);

        bit::<ZeroPage, _>(&mut cpu, &mut bus);

        assert!(cpu.p.contains(Flags::Sign));
        assert!(cpu.p.contains(Flags::Overflow));
        assert!(!cpu.p.contains(Flags::Zero));
        assert_eq!(cpu.pc, ZeroPage::BYTES.into());
    }

    #[test]
    fn test_bne_branching() {
        const OFFSET: Byte = Byte(0x02);

        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.remove(Flags::Zero);

            bne(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0004));
        }
        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.insert(Flags::Zero);

            bne(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0002));
        }
    }

    #[test]
    fn test_beq_branching() {
        const OFFSET: Byte = Byte(0x02);

        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.insert(Flags::Zero);

            beq(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0004));
        }
        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.remove(Flags::Zero);

            beq(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0002));
        }
    }

    #[test]
    fn test_bmi_branching() {
        const OFFSET: Byte = Byte(0x02);

        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.insert(Flags::Sign);

            bmi(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0004));
        }
        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.remove(Flags::Sign);

            bmi(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0002));
        }
    }

    #[test]
    fn test_bpl_branching() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const OFFSET: Byte = Byte(0x02);

        bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
        cpu.p.remove(Flags::Sign);

        bpl(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, Word(0x0004));
    }

    #[test]
    fn test_bcc_bcs_branching() {
        const OFFSET: Byte = Byte(0x02);

        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.remove(Flags::Carry);

            bcc(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0004));
        }
        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.insert(Flags::Carry);

            bcs(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0004));
        }
    }

    #[test]
    fn test_bvc_bvs_branching() {
        const OFFSET: Byte = Byte(0x02);

        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.remove(Flags::Overflow);

            bvc(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0004));
        }
        {
            let mut bus = SimpleBus::default();
            let mut cpu = Cpu::<SimpleBus>::default();

            bus.write(cpu.pc + 1, OFFSET, AccessType::DataWrite);
            cpu.p.insert(Flags::Overflow);

            bvs(&mut cpu, &mut bus);

            assert_eq!(cpu.pc, Word(0x0004));
        }
    }

    #[test]
    fn test_jmp_absolute() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const TARGET_ADDRESS: u16 = 0x1234;

        bus.write(cpu.pc + 1, Byte(0x34), AccessType::DataWrite);
        bus.write(cpu.pc + 2, Byte(0x12), AccessType::DataWrite);

        jmp::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, Word(TARGET_ADDRESS));
    }

    #[test]
    fn test_jmp_indirect() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const POINTER_ADDRESS: u16 = 0x2000;
        const TARGET_ADDRESS: u16 = 0x5678;

        bus.write(cpu.pc + 1, Byte(0x00), AccessType::DataWrite);
        bus.write(cpu.pc + 2, Byte(0x20), AccessType::DataWrite);
        bus.write(Word(POINTER_ADDRESS), Byte(0x78), AccessType::DataWrite);
        bus.write(Word(POINTER_ADDRESS + 1), Byte(0x56), AccessType::DataWrite);

        jmp::<AbsoluteIndirect, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, Word(TARGET_ADDRESS));
    }

    #[test]
    fn test_jsr_and_rts() {
        const JSR_TARGET: u16 = 0x2000;
        const RTS_TARGET: u16 = 0x1235;

        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        bus.write(cpu.pc + 1, Byte(0x00), AccessType::DataWrite);
        bus.write(cpu.pc + 2, Byte(0x20), AccessType::DataWrite);

        jsr::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, Word(JSR_TARGET));
        assert_eq!(cpu.sp, 0xFB);
        assert_eq!(bus.read(Word(0x01FD), AccessType::DataRead), 0x00);
        assert_eq!(bus.read(Word(0x01FC), AccessType::DataRead), 0x02);

        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        cpu.sp = Byte(0xFB);
        bus.write(Word(0x01FC), Byte(0x34), AccessType::DataWrite);
        bus.write(Word(0x01FD), Byte(0x12), AccessType::DataWrite);

        rts(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, Word(RTS_TARGET));
        assert_eq!(cpu.sp, 0xFD);
    }

    #[test]
    fn test_brk_and_rti() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        cpu.pc = Word(0x0200);
        bus.write(IRQ_VECTOR_LO, Byte(0x00), AccessType::DataWrite);
        bus.write(IRQ_VECTOR_HI, Byte(0x80), AccessType::DataWrite);
        brk(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, Word(0x8000));
        assert_eq!(cpu.sp, 0xFA);
        assert_eq!(bus.read(Word(0x01FD), AccessType::DataRead), 0x02);
        assert_eq!(bus.read(Word(0x01FC), AccessType::DataRead), 0x02);

        let status = bus.read(Word(0x01FB), AccessType::DataRead);
        assert_ne!(status & Flags::Break, 0);
        assert!(cpu.p.contains(Flags::InterruptDisabled));

        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();
        cpu.sp = Byte(0xFA);
        bus.write(
            Word(0x01FB),
            byte!(Flags::Sign | Flags::Carry),
            AccessType::DataRead,
        );
        bus.write(Word(0x01FC), Byte(0x34), AccessType::DataWrite);
        bus.write(Word(0x01FD), Byte(0x12), AccessType::DataWrite);
        rti(&mut cpu, &mut bus);
        assert_eq!(cpu.pc, Word(0x1234));
        assert_eq!(cpu.sp, 0xFD);
        assert!(cpu.p.contains(Flags::Sign));
        assert!(cpu.p.contains(Flags::Carry));
    }

    #[test]
    fn test_cmp_equal() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const VALUE: Byte = Byte(0x42);

        cpu.a = VALUE;
        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), VALUE, AccessType::DataWrite);

        cmp::<Immediate, _>(&mut cpu, &mut bus);

        assert!(cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Carry));
        assert!(!cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_cmp_less_than() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0x20);
        const COMPARE_VALUE: Byte = Byte(0x42);

        cpu.a = A_VALUE;
        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), COMPARE_VALUE, AccessType::DataWrite);

        cmp::<Immediate, _>(&mut cpu, &mut bus);

        assert!(!cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Carry));
        assert!(cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_cmp_greater_than() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const A_VALUE: Byte = Byte(0x50);
        const COMPARE_VALUE: Byte = Byte(0x42);

        cpu.a = A_VALUE;
        cpu.pc = Word(0x0200);
        bus.write(Word(0x0201), COMPARE_VALUE, AccessType::DataWrite);

        cmp::<Immediate, _>(&mut cpu, &mut bus);

        assert!(!cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Carry));
        assert!(!cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_dex_wraps() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INITIAL_VALUE: Byte = Byte(0x00);
        const EXPECTED_VALUE: Byte = Byte(0xFF);

        cpu.x = INITIAL_VALUE;
        cpu.pc = Word(0x0200);

        dex(&mut cpu, &mut bus);

        assert_eq!(cpu.x, EXPECTED_VALUE);
        assert!(!cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Word(0x0201));
    }

    #[test]
    fn test_dex_sets_zero() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INITIAL_VALUE: Byte = Byte(0x01);
        const EXPECTED_VALUE: Byte = Byte(0x00);

        cpu.x = INITIAL_VALUE;
        cpu.pc = Word(0x0200);

        dex(&mut cpu, &mut bus);

        assert_eq!(cpu.x, EXPECTED_VALUE);
        assert!(cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
    }

    #[test]
    fn test_dey_wraps() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INITIAL_VALUE: Byte = Byte(0x00);
        const EXPECTED_VALUE: Byte = Byte(0xFF);

        cpu.y = INITIAL_VALUE;
        cpu.pc = Word(0x0200);

        dey(&mut cpu, &mut bus);

        assert_eq!(cpu.y, EXPECTED_VALUE);
        assert!(!cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Sign));
        assert_eq!(cpu.pc, Word(0x0201));
    }

    #[test]
    fn test_dey_sets_zero() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const INITIAL_VALUE: Byte = Byte(0x01);
        const EXPECTED_VALUE: Byte = Byte(0x00);

        cpu.y = INITIAL_VALUE;
        cpu.pc = Word(0x0200);

        dey(&mut cpu, &mut bus);

        assert_eq!(cpu.y, EXPECTED_VALUE);
        assert!(cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Sign));
    }

    // Tests for common instruction sequences and realistic operations

    #[test]
    fn test_stack_push_pop_sequence() {
        // Tests pushing multiple values and popping them in LIFO order
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();
        cpu.pc = Word(0x0200);

        // Push A, X, Y onto stack
        cpu.a = Byte(0x42);
        cpu.x = Byte(0x43);
        cpu.y = Byte(0x44);

        pha(&mut cpu, &mut bus);
        assert_eq!(cpu.sp, Byte(0xFC));

        cpu.a = cpu.x;
        pha(&mut cpu, &mut bus);
        assert_eq!(cpu.sp, Byte(0xFB));

        cpu.a = cpu.y;
        pha(&mut cpu, &mut bus);
        assert_eq!(cpu.sp, Byte(0xFA));

        // Pop values back (LIFO: Y, X, A)
        pla(&mut cpu, &mut bus);
        assert_eq!(cpu.a, Byte(0x44)); // Y value
        assert_eq!(cpu.sp, Byte(0xFB));

        pla(&mut cpu, &mut bus);
        assert_eq!(cpu.a, Byte(0x43)); // X value
        assert_eq!(cpu.sp, Byte(0xFC));

        pla(&mut cpu, &mut bus);
        assert_eq!(cpu.a, Byte(0x42)); // Original A value
        assert_eq!(cpu.sp, Byte(0xFD));
    }

    #[test]
    fn test_countdown_loop() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();
        cpu.pc = Word(0x0200);

        cpu.x = Byte(0x05);

        for expected in (0..5).rev() {
            dex(&mut cpu, &mut bus);
            assert_eq!(cpu.x, Byte(expected));

            if expected == 0 {
                assert!(cpu.p.contains(Flags::Zero));
            } else {
                assert!(!cpu.p.contains(Flags::Zero));
            }
        }
    }

    #[test]
    fn test_function_call_return_sequence() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        cpu.pc = Word(0x0800);
        bus.write(Word(0x0800), Byte(0x20), AccessType::DataWrite);
        bus.write(Word(0x0801), Byte(0x00), AccessType::DataWrite);
        bus.write(Word(0x0802), Byte(0x10), AccessType::DataWrite);

        let initial_sp = cpu.sp;
        jsr::<Absolute, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.pc, Word(0x1000));
        assert_eq!(cpu.sp, initial_sp - 2);

        // JSR pushes return address with HI byte first, then LO byte
        let base = 0x0100 + u16::from(initial_sp);
        let return_hi = bus.read(Word(base), AccessType::DataRead);
        let return_lo = bus.read(Word(base - 1), AccessType::DataRead);
        assert_eq!(return_hi, Byte(0x08));
        assert_eq!(return_lo, Byte(0x02));

        rts(&mut cpu, &mut bus);
        assert_eq!(cpu.pc, Word(0x0803));
        assert_eq!(cpu.sp, initial_sp);
    }

    #[test]
    fn test_16bit_addition() {
        // Demonstrates 16-bit addition using 8-bit ADC with carry propagation
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();
        cpu.pc = Word(0x0200);

        const FIRST_LO: Byte = Byte(0x34);
        const FIRST_HI: Byte = Byte(0x12);
        const SECOND_LO: Byte = Byte(0x56);
        const SECOND_HI: Byte = Byte(0x00);
        const EXPECTED_RESULT_LO: Byte = Byte(0x8A);
        const EXPECTED_RESULT_HI: Byte = Byte(0x12);

        cpu.p.remove(Flags::Carry);

        cpu.a = FIRST_LO;
        bus.write(Word(0x0201), SECOND_LO, AccessType::DataWrite);
        adc::<Mos6502, Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED_RESULT_LO);
        assert!(!cpu.p.contains(Flags::Carry));

        cpu.a = FIRST_HI;
        bus.write(Word(0x0202), SECOND_HI, AccessType::DataWrite);
        cpu.pc = Word(0x0201);
        adc::<Mos6502, Immediate, _>(&mut cpu, &mut bus);

        assert_eq!(cpu.a, EXPECTED_RESULT_HI);
    }

    #[test]
    fn test_multi_byte_comparison() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();
        cpu.pc = Word(0x0200);

        const FIRST_VALUE_HI: Byte = Byte(0x12);
        const FIRST_VALUE_LO: Byte = Byte(0x34);
        const SECOND_VALUE_HI: Byte = Byte(0x12);
        const SECOND_VALUE_LO: Byte = Byte(0x56);

        cpu.a = FIRST_VALUE_HI;
        bus.write(Word(0x0200), Byte(0xC9), AccessType::DataWrite);
        bus.write(Word(0x0201), SECOND_VALUE_HI, AccessType::DataWrite);
        cmp::<Immediate, _>(&mut cpu, &mut bus);

        assert!(cpu.p.contains(Flags::Zero));
        assert!(cpu.p.contains(Flags::Carry));

        cpu.a = FIRST_VALUE_LO;
        bus.write(Word(0x0202), Byte(0xC9), AccessType::DataWrite);
        bus.write(Word(0x0203), SECOND_VALUE_LO, AccessType::DataWrite);
        cpu.pc = Word(0x0202);
        cmp::<Immediate, _>(&mut cpu, &mut bus);

        assert!(!cpu.p.contains(Flags::Zero));
        assert!(!cpu.p.contains(Flags::Carry));
    }

    #[test]
    fn test_memory_copy_loop() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();

        const SOURCE_ADDR: u16 = 0x1000;
        const DEST_ADDR: u16 = 0x2000;
        let source_data = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

        for (i, &byte) in source_data.iter().enumerate() {
            bus.write(
                Word(SOURCE_ADDR + i as u16),
                Byte(byte),
                AccessType::DataWrite,
            );
        }

        for i in 0..source_data.len() as u8 {
            cpu.x = Byte(i);
            let source_val = bus.read(Word(SOURCE_ADDR + u16::from(cpu.x)), AccessType::DataRead);
            cpu.a = source_val;
            bus.write(
                Word(DEST_ADDR + u16::from(cpu.x)),
                cpu.a,
                AccessType::DataWrite,
            );
        }

        for (i, &expected) in source_data.iter().enumerate() {
            let copied = bus.read(Word(DEST_ADDR + i as u16), AccessType::DataRead);
            assert_eq!(copied, Byte(expected));
        }
    }

    #[test]
    fn test_bit_manipulation_sequence() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();
        cpu.pc = Word(0x0200);

        const INITIAL_VALUE: Byte = Byte(0b0000_1111);
        const SET_MASK: Byte = Byte(0b1111_0000);
        const CLEAR_MASK: Byte = Byte(0b0000_1111);
        const TOGGLE_MASK: Byte = Byte(0b1111_1111);

        cpu.a = INITIAL_VALUE;
        bus.write(Word(0x0201), SET_MASK, AccessType::DataWrite);
        ora::<Immediate, _>(&mut cpu, &mut bus);
        assert_eq!(cpu.a, Byte(0b1111_1111));

        cpu.pc = Word(0x0202);
        bus.write(Word(0x0203), CLEAR_MASK, AccessType::DataWrite);
        and::<Immediate, _>(&mut cpu, &mut bus);
        assert_eq!(cpu.a, Byte(0b0000_1111));

        cpu.pc = Word(0x0204);
        bus.write(Word(0x0205), TOGGLE_MASK, AccessType::DataWrite);
        eor::<Immediate, _>(&mut cpu, &mut bus);
        assert_eq!(cpu.a, Byte(0b1111_0000));
    }

    #[test]
    fn test_conditional_flag_based_logic() {
        let mut bus = SimpleBus::default();
        let mut cpu = Cpu::<SimpleBus>::default();
        cpu.pc = Word(0x0200);

        const VALUE_ABOVE_THRESHOLD: Byte = Byte(0x90);
        const THRESHOLD: Byte = Byte(0x80);

        cpu.a = VALUE_ABOVE_THRESHOLD;
        bus.write(Word(0x0201), THRESHOLD, AccessType::DataWrite);
        cmp::<Immediate, _>(&mut cpu, &mut bus);

        assert!(cpu.p.contains(Flags::Carry));

        const NEGATIVE_VALUE: Byte = Byte(0xFF);
        const POSITIVE_VALUE: Byte = Byte(0x7F);

        cpu.a = NEGATIVE_VALUE;
        cpu.p.set_signed(cpu.a.is_signed());
        assert!(cpu.p.contains(Flags::Sign));

        cpu.a = POSITIVE_VALUE;
        cpu.p.set_signed(cpu.a.is_signed());
        assert!(!cpu.p.contains(Flags::Sign));
    }
}
