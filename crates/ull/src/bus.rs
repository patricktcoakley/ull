//! Memory and I/O bus abstraction.

use crate::{Address, Byte, Word};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DmaRequest {
    pub source: Word,
    pub destination: Word,
    pub length: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaResult {
    Accepted { cycles: u32 },
    Pending,
    Denied,
}

pub trait Bus {
    type Access: Copy;
    type Data: Copy;

    fn read<A>(&mut self, addr: A, access: Self::Access) -> Self::Data
    where
        A: Address;

    fn write<A, V>(&mut self, addr: A, value: V, access: Self::Access)
    where
        A: Address,
        V: Into<Self::Data>;

    /// Read a contiguous block of memory starting at `start` into `dst`.
    ///
    /// Default implementation issues repeated [`read`](Self::read) calls and wraps addresses using
    /// the `Address` type's wrapping semantics. Override this if your bus can service burst reads
    /// more efficiently.
    fn read_block<A>(&mut self, start: A, dst: &mut [u8], access: Self::Access)
    where
        A: Address,
        Self::Data: Into<Byte>,
    {
        let mut addr = start;
        for byte in dst {
            let value: Byte = self.read(addr, access).into();
            *byte = u8::from(value);
            addr += 1;
        }
    }

    /// Write a contiguous block of bytes starting at `start`.
    ///
    /// Default implementation issues repeated [`write`](Self::write) calls and wraps addresses using
    /// the `Address` type's wrapping semantics. Override this when the bus can push larger buffers
    /// directly.
    fn write_block<A>(&mut self, start: A, data: &[u8], access: Self::Access)
    where
        A: Address,
        Self::Data: From<u8>,
    {
        let mut addr = start;
        for &byte in data {
            self.write(addr, Self::Data::from(byte), access);
            addr += 1;
        }
    }

    fn on_tick(&mut self, cycles: u8) {
        let _ = cycles;
    }

    fn request_dma(&mut self, request: DmaRequest) -> DmaResult {
        let _ = request;
        DmaResult::Denied
    }

    /// Retrieve the next pending DMA transfer, if any.
    fn poll_dma_cycle(&mut self) -> Option<u8> {
        None
    }
}
