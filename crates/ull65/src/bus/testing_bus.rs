//! Utility bus for deterministic unit testing.

use alloc::{boxed::Box, collections::VecDeque, vec};
use ull::{Address, Byte};
use ull::{Bus, DmaRequest, DmaResult};
use crate::AccessType;

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
    type Access = AccessType;
    type Data = Byte;

    fn read<A>(&mut self, addr: A, _access: Self::Access) -> Self::Data
    where
        A: Address,
    {
        Byte(self.mem[addr.as_usize()])
    }

    fn write<A, V>(&mut self, addr: A, value: V, _access: Self::Access)
    where
        A: Address,
        V: Into<Self::Data>,
    {
        let byte: Byte = value.into();
        self.mem[addr.as_usize()] = byte.0;
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
