//! Demonstrates how to build a patched instruction table on top of the
//! built‑in MOS 6502 implementation. For illustration, we replace the
//! `BRK` opcode (0x00) with a custom handler that traps into our code
//! instead of invoking the IRQ vector.

use ull::Word;
use ull65::bus::Mos6502CompatibleBus;
use ull65::instruction::{mos6502::Mos6502, Instruction, InstructionSet};
use ull65::processor::run::{RunConfig, RunPredicate};
use ull65::{Cpu, SimpleBus};

/// Custom CPU model that patches the stock MOS table.
struct Trap6502;

impl InstructionSet for Trap6502 {
    fn instruction_table<B: Mos6502CompatibleBus + 'static>()
    -> ull65::instruction::InstructionTable<B> {
        // Start from the canonical MOS table and replace opcode 0x00 (BRK).
        Mos6502::base_table::<B>().with(
            0x00,
            Instruction {
                cycles: 7,
                execute: trap_brk::<B>,
            },
        )
    }
}

/// Simple handler that records the trap and advances the program counter.
fn trap_brk<B: Mos6502CompatibleBus>(cpu: &mut Cpu<B>, _bus: &mut B) {
    println!("custom BRK handler invoked at PC={:04X}", cpu.pc);
    cpu.pc += 1;
}

fn main() {
    let mut bus = SimpleBus::default();
    // Instantiate the CPU with our patched instruction set and execute the custom BRK once.
    let mut cpu: Cpu<SimpleBus> =
        Cpu::with_program::<Trap6502>(&mut bus, Word(0x0000), &[0x00], Word(0x0000));

    println!("Patching opcode $00 to invoke trap_brk instead of the default BRK handler.");

    // Drive the CPU with run_until + a predicate so we can stop on our custom handler.
    let mut pc_advanced = |cpu: &Cpu<SimpleBus>, _bus: &mut SimpleBus| cpu.pc == Word(0x0001);
    let summary = cpu.run_until(
        &mut bus,
        RunConfig {
            predicate: Some(RunPredicate::new(&mut pc_advanced)),
            ..RunConfig::default()
        },
    );

    println!("Summary: {summary:?}");
}
