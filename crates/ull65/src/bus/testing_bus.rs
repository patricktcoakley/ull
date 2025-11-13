use crate::{AccessType, Bus, Byte, DmaRequest, DmaResult, Word};
use alloc::{boxed::Box, collections::VecDeque, vec};

pub struct TestingBus {
    mem: Box<[u8]>,
    pub ticks: u64,
    pub dma_ticks: u64,
    dma_queue: VecDeque<u8>,
}

impl TestingBus {
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn queue_dma(&mut self, cycles: u8) {
        self.dma_queue.push_back(cycles);
    }
}

impl Default for TestingBus {
    fn default() -> Self {
        Self {
            mem: vec![0; 0x10000].into_boxed_slice(),
            ticks: 0,
            dma_ticks: 0,
            dma_queue: VecDeque::new(),
        }
    }
}

impl Bus for TestingBus {
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
