//! NES nestest ROM runner.
//!
//! Loads the nestest ROM (iNES, mapper 0) and runs until the success PC `$C66E`
//! is reached, verifying our CPU implementation against the well-known test ROM.

use std::fs;
use std::path::Path;

use ull::{AccessType, Bus, Byte, Word};
use ull65::instruction::mos6502::Mos6502;
use ull65::instruction::{InstructionSet, InstructionTable};
use ull65::{Cpu, RESET_VECTOR_HI, RESET_VECTOR_LO};

/// Minimal NES memory map that satisfies nestest.
struct NesBus {
    ram: [u8; 0x800],
    sram: [u8; 0x2000],
    rom: Vec<u8>,
    ppu: [u8; 8],
}

impl NesBus {
    /// Mirror PRG addresses depending on whether the ROM is 16 KB or 32 KB.
    fn prg_addr(&self, addr: u16) -> usize {
        if self.rom.len() <= 0x4000 {
            (addr & 0x3FFF) as usize
        } else {
            (addr & 0x7FFF) as usize
        }
    }

    /// Parse an iNES file (mapper 0 only) and extract the PRG ROM.
    fn from_ines<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let data = fs::read(path).map_err(|e| format!("Failed to read ROM: {e}"))?;
        if data.len() < 16 {
            return Err("ROM file too small".into());
        }
        if &data[0..4] != b"NES\x1A" {
            return Err("Invalid iNES header magic".into());
        }

        let flags6 = data[6];
        let flags7 = data[7];
        if flags6 & 0x04 != 0 {
            return Err("Trainer data not supported".into());
        }
        let mapper = ((flags7 & 0xF0) | (flags6 >> 4)) & 0xFF;
        if mapper != 0 {
            return Err(format!("Mapper {mapper} not supported (expected 0)"));
        }

        let prg_banks = data[4] as usize;
        if prg_banks == 0 {
            return Err("ROM contains zero PRG banks".into());
        }
        let prg_size = prg_banks * 16_384;
        let prg_start = 16;
        let prg_end = prg_start + prg_size;
        if data.len() < prg_end {
            return Err("ROM truncated (PRG)".into());
        }
        let prg_rom = data[prg_start..prg_end].to_vec();

        Ok(Self {
            ram: [0; 0x800],
            sram: [0; 0x2000],
            rom: prg_rom,
            ppu: [0; 8],
        })
    }
}

impl Bus for NesBus {
    fn read<A>(&mut self, addr: A, _access: AccessType) -> Byte
    where
        A: Into<Word>,
    {
        let addr = addr.into();
        let raw = addr.0;
        match raw {
            // $0000-$1FFF: internal RAM mirrored every 2 KB.
            0x0000..=0x1FFF => Byte::from(self.ram[(addr.as_usize()) & 0x07FF]),
            // $2000-$3FFF: PPU registers mirrored every 8 bytes.
            0x2000..=0x3FFF => Byte::from(self.ppu[(raw & 0x0007) as usize]),
            // $4000-$4017: APU + I/O (unused for nestest, return 0).
            0x4000..=0x4017 => Byte::from(0x00),
            // $6000-$7FFF: cartridge SRAM.
            0x6000..=0x7FFF => Byte::from(self.sram[(addr.as_usize()) & 0x1FFF]),
            // $8000-$FFFF: PRG ROM (mirrored if only 16 KB present).
            0x8000..=0xFFFF => Byte::from(self.rom[self.prg_addr(raw)]),
            _ => Byte::from(0xFF),
        }
    }

    fn write<A, V>(&mut self, addr: A, value: V, _access: AccessType)
    where
        A: Into<Word>,
        V: Into<Byte>,
    {
        let addr = addr.into();
        let raw = addr.0;
        let value = value.into();
        match raw {
            // Writes follow the same mirroring scheme as reads.
            0x0000..=0x1FFF => {
                self.ram[(addr.as_usize()) & 0x07FF] = u8::from(value);
            }
            0x2000..=0x3FFF => {
                self.ppu[(raw & 0x0007) as usize] = u8::from(value);
            }
            0x4000..=0x4017 => {}
            0x6000..=0x7FFF => {
                self.sram[(addr.as_usize()) & 0x1FFF] = u8::from(value);
            }
            // Allow the test harness to poke the reset vector inside PRG ROM.
            _ if raw == u16::from(RESET_VECTOR_LO) || raw == u16::from(RESET_VECTOR_HI) => {
                let offset = self.prg_addr(raw);
                if offset < self.rom.len() {
                    self.rom[offset] = u8::from(value);
                }
            }
            0x8000..=0xFFFF => {}
            _ => {}
        }
    }
}

struct Ricoh2a03;

impl InstructionSet for Ricoh2a03 {
    fn instruction_table<B: Bus + 'static>() -> InstructionTable<B> {
        Mos6502::base_table()
    }
    const SUPPORTS_DECIMAL_MODE: bool = false;
}

fn main() -> Result<(), String> {
    let rom_path = "thirdparty/nestest/nestest.nes";
    let mut bus = NesBus::from_ines(rom_path)?;

    // nestest expects the reset vector to point to $C000.
    let mut cpu: Cpu<NesBus> = Cpu::with_reset_vector::<Ricoh2a03>(&mut bus, Word(0xC000));

    let success_pc = Word(0xC66E);
    // The ROM reports success by executing at $C66E; keep stepping until we land there.
    let mut instruction_count = 0;
    let max_instructions = 100_000;
    let mut last_pc = Word(0);
    let mut loop_count = 0;

    while instruction_count < max_instructions {
        let pc = cpu.pc;
        if pc == last_pc {
            loop_count += 1;
            if loop_count > 5 {
                println!("\n⚠ Stuck in infinite loop at PC=${:04X}", pc);
                let opcode = bus.read(pc, AccessType::DataRead);
                println!("Opcode at PC: ${:02X}", opcode.0);
                break;
            }
        } else {
            loop_count = 0;
        }
        last_pc = pc;

        if pc == success_pc {
            println!("\n✓ Test PASSED!");
            println!("Reached success point at PC=${:04X}", pc);
            println!("Instructions executed: {instruction_count}");
            println!("Total cycles: {}", cpu.cycles);
            return Ok(());
        }

        if cpu.tick(&mut bus) == 0 {
            println!("\n⚠ CPU made no progress.");
            break;
        }

        instruction_count += 1;
    }

    println!("\n⚠ Test reached instruction limit");
    println!("Final PC=${:04X}", cpu.pc);
    println!("Instructions executed: {instruction_count}");
    Err("Execution limit reached".into())
}
