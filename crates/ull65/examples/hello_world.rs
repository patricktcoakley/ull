use std::string::String;
use ull65::processor::run::RunConfig;
use ull65::{AccessType, Bus, Byte, Cpu, Word, bus::SimpleBus, instruction::mos6502::Mos6502};

const PROGRAM_START: Word = Word(0x8000);
const TEXT_ADDR: u16 = 0x0200;
const MESSAGE: &[u8] = b"Hello, World!";

fn main() {
    let mut bus = SimpleBus::default();

    // Assemble a tiny program that copies "Hello, World!" into zero-page RAM.
    let mut program = Vec::with_capacity(MESSAGE.len() * 5 + 1);
    for (offset, &byte) in MESSAGE.iter().enumerate() {
        let addr = Word(TEXT_ADDR + offset as u16);
        program.extend_from_slice(&[
            0xA9, // LDA #imm
            byte,
            0x8D, // STA $nnnn
            u8::from(addr.lo()),
            u8::from(addr.hi()),
        ]);
    }
    program.push(0x00); // BRK

    let mut cpu: Cpu<SimpleBus> =
        Cpu::with_program::<Mos6502>(&mut bus, PROGRAM_START, &program, PROGRAM_START);

    let summary = cpu.run_until(
        &mut bus,
        RunConfig {
            stop_on_brk: true,
            ..RunConfig::default()
        },
    );

    let mut rendered = String::new();
    for offset in 0..MESSAGE.len() {
        let addr = Word(TEXT_ADDR + offset as u16);
        let byte: Byte = bus.read(addr, AccessType::DataRead);
        rendered.push(u8::from(byte) as char);
    }

    println!("Program finished after {summary:?}");
    println!("Memory at ${TEXT_ADDR:04X}: {rendered}");
}
