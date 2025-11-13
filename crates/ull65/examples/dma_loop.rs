//! Demonstrates how [`Cpu::tick`](crate::Cpu::tick) handles the
//! `step → on_tick → poll_dma_cycle` synchronization pattern for you.
//!
//! Writing to $D002 on this demo bus schedules an 8-cycle DMA burst. Each call
//! to `tick` returns the instruction's cycle count while internally advancing
//! the bus clock and draining DMA so elapsed time stays in sync.

use std::collections::VecDeque;
use ull65::bus::Bus;
use ull65::instruction::mos6502::Mos6502;
use ull65::{AccessType, Byte, Cpu, DmaRequest, DmaResult, Word};

const DMA_TRIGGER_ADDR: u16 = 0xD002;
const DMA_LENGTH: u8 = 8;

struct DemoBus {
    mem: [u8; 0x10000],
    ticks: u64,
    instr_ticks: u64,
    dma_ticks: u64,
    dma_queue: VecDeque<u8>,
}

impl Default for DemoBus {
    fn default() -> Self {
        Self {
            mem: [0; 0x10000],
            ticks: 0,
            instr_ticks: 0,
            dma_ticks: 0,
            dma_queue: VecDeque::new(),
        }
    }
}

impl Bus for DemoBus {
    fn read<A>(&mut self, addr: A, _access: AccessType) -> Byte
    where
        A: Into<Word>,
    {
        let addr = addr.into();
        Byte(self.mem[addr.as_usize()])
    }

    fn write<A, V>(&mut self, addr: A, value: V, _access: AccessType)
    where
        A: Into<Word>,
        V: Into<Byte>,
    {
        let addr = addr.into();
        let value = value.into();
        self.mem[addr.as_usize()] = value.0;
        if addr.0 == DMA_TRIGGER_ADDR {
            println!("DMA triggered ({} cycles)", DMA_LENGTH);
            self.dma_queue.push_back(DMA_LENGTH);
        }
    }

    fn on_tick(&mut self, cycles: u8) {
        self.ticks += u64::from(cycles);
    }

    fn request_dma(&mut self, request: DmaRequest) -> DmaResult {
        self.dma_queue.push_back(request.length as u8);
        DmaResult::Pending
    }

    fn poll_dma_cycle(&mut self) -> Option<u8> {
        if let Some(cycles) = self.dma_queue.pop_front() {
            println!("  -> DMA consuming {} cycles", cycles);
            self.dma_ticks += u64::from(cycles);
            return Some(cycles);
        }
        None
    }
}

fn main() {
    let mut bus = DemoBus::default();

    // Program loops forever, writing to $0200, triggering DMA, then incrementing.
    let program = [
        0xA9,
        0x01, // LDA #$01
        0x8D,
        0x00,
        0x02, // STA $0200
        0x8D,
        (DMA_TRIGGER_ADDR & 0xFF) as u8,
        (DMA_TRIGGER_ADDR >> 8) as u8, // STA $D002 (DMA)
        0xEE,
        0x00,
        0x02, // INC $0200
        0x4C,
        0x00,
        0x80, // JMP $8000
    ];
    let mut cpu: Cpu<DemoBus> =
        Cpu::with_program::<Mos6502>(&mut bus, Word(0x8000), &program, Word(0x8000));

    for iteration in 0..4 {
        println!("\nIteration {}", iteration + 1);
        let cycles = cpu.tick(&mut bus);
        if cycles == 0 {
            println!("CPU reported no progress; stopping early.");
            break;
        }
        bus.instr_ticks += u64::from(cycles);
        let mem_0200 = bus.read(Word(0x0200), AccessType::DataRead).0;
        println!(
            "Instruction consumed {} cycles; $0200 now contains {:02X}",
            cycles, mem_0200
        );
    }

    println!(
        "Instruction cycles: {}, DMA cycles: {}, total ticks: {}",
        bus.instr_ticks, bus.dma_ticks, bus.ticks
    );
    println!(
        "Final Memory[$0200] = {:02X}",
        bus.read(Word(0x0200), AccessType::DataRead).0
    );
}
