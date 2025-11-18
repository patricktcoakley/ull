use std::collections::VecDeque;
use ull::{AccessType, Bus, Byte, DmaRequest, DmaResult, Word};
use ull65::instruction::mos6502::Mos6502;
use ull65::processor::cpu::Cpu;

struct TestBus {
    mem: [u8; 0x10000],
    ticks: u64,
    instr_ticks: u64,
    dma_ticks: u64,
    dma_queue: VecDeque<u8>,
}

impl Default for TestBus {
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

impl Bus for TestBus {
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
            self.dma_ticks += u64::from(cycles);
            Some(cycles)
        } else {
            None
        }
    }
}

#[test]
fn dma_cycles_are_accounted() {
    let mut bus = TestBus::default();
    let mut cpu: Cpu<TestBus> = Cpu::with_instruction_set::<Mos6502>();

    bus.write_block(Word(0x8000), &[0xEA, 0xEA, 0xEA], AccessType::DataWrite);
    bus.set_reset_vector(Word(0x8000));
    cpu.reset(&mut bus);

    for step in 0..3 {
        let cycles = cpu.step(&mut bus);
        bus.instr_ticks += u64::from(cycles);
        bus.on_tick(cycles);

        if step == 1 {
            let _ = bus.request_dma(DmaRequest {
                source: Word(0),
                destination: Word(0),
                length: 4,
            });
        }

        while let Some(dma_cycles) = bus.poll_dma_cycle() {
            bus.on_tick(dma_cycles);
        }
    }

    assert_eq!(bus.ticks, bus.instr_ticks + bus.dma_ticks);
    assert_eq!(bus.dma_ticks, 4);
}

#[test]
fn read_block_fetches_consecutive_bytes() {
    let mut bus = TestBus::default();
    bus.write_block(Word(0x8000), &[1, 2, 3, 4], AccessType::DataWrite);
    let mut buf = [0u8; 4];
    bus.read_block(Word(0x8000), &mut buf, AccessType::DataRead);
    assert_eq!(buf, [1, 2, 3, 4]);
}

#[test]
fn write_block_stores_consecutive_bytes() {
    let mut bus = TestBus::default();
    bus.write_block(Word(0x9000), &[0xAA, 0xBB, 0xCC], AccessType::DataWrite);
    assert_eq!(bus.read(Word(0x9000), AccessType::DataRead).0, 0xAA);
    assert_eq!(bus.read(Word(0x9001), AccessType::DataRead).0, 0xBB);
    assert_eq!(bus.read(Word(0x9002), AccessType::DataRead).0, 0xCC);
}
