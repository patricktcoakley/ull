//! Basic flat memory implementation for 8-bit 65xx CPUs.

use alloc::{boxed::Box, vec};
use ull::{Address, Byte, Word};
use ull::Bus;
use crate::AccessType;

/// Simple contiguous memory without mirroring or bank logic.
#[derive(Debug)]
pub struct SimpleBus {
    mem: Box<[u8]>,
}

impl SimpleBus {
    const ADDR_MASK: usize = 0xFFFF;
    const MEM_SIZE: usize = 0x10000;

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Load raw bytes directly into memory starting at `start`.
    pub fn load(&mut self, start: Word, bytes: &[u8]) {
        let mut idx = start.as_usize() & Self::ADDR_MASK;
        for &byte in bytes {
            self.mem[idx] = byte;
            idx = (idx + 1) & Self::ADDR_MASK;
        }
    }
}

impl Default for SimpleBus {
    fn default() -> Self {
        Self {
            mem: vec![0; Self::MEM_SIZE].into_boxed_slice(),
        }
    }
}

impl Bus for SimpleBus {
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

    fn read_block<A>(&mut self, start: A, dst: &mut [u8], _access: Self::Access)
    where
        A: Address,
    {
        let mut idx = start.as_usize() & Self::ADDR_MASK;
        for byte in dst {
            *byte = self.mem[idx];
            idx = (idx + 1) & Self::ADDR_MASK;
        }
    }

    fn write_block<A>(&mut self, start: A, data: &[u8], _access: Self::Access)
    where
        A: Address,
    {
        let mut idx = start.as_usize() & Self::ADDR_MASK;
        for &byte in data {
            self.mem[idx] = byte;
            idx = (idx + 1) & Self::ADDR_MASK;
        }
    }
}
