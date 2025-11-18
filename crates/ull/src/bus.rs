//! Memory and I/O bus abstraction.

pub mod simple_bus;
pub mod testing_bus;

use crate::{Byte, Word};
pub use simple_bus::SimpleBus;
pub use testing_bus::TestingBus;

/// Phase within a bus cycle. Some hardware (e.g., NES DMA) cares whether we're in
/// the read (GET) or write (PUT) half of a cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Bus is in the read/GET half-cycle.
    Read,
    /// Bus is in the write/PUT half-cycle.
    Write,
}

/// High-level purpose of a bus access combined with its direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AccessType {
    /// Normal data read.
    #[default]
    DataRead,
    /// Normal data write.
    DataWrite,
    /// Instruction opcode fetch (read).
    OpcodeFetch,
    /// Stack read (pull).
    StackRead,
    /// Stack write (push).
    StackWrite,
    /// Read of an interrupt vector entry ($FFFA-$FFFF).
    InterruptVectorRead,
    /// Bus cycle triggered by DMA hardware when reading.
    DmaRead,
    /// Bus cycle triggered by DMA hardware when writing.
    DmaWrite,
    /// Dummy/throw-away read cycle (e.g., page-cross penalties).
    DummyRead,
}

impl AccessType {
    /// Returns `true` if this access reads from the bus.
    #[must_use]
    pub const fn is_read(self) -> bool {
        matches!(
            self,
            AccessType::DataRead
                | AccessType::OpcodeFetch
                | AccessType::StackRead
                | AccessType::InterruptVectorRead
                | AccessType::DmaRead
                | AccessType::DummyRead
        )
    }

    /// Returns `true` if this access writes to the bus.
    #[must_use]
    pub const fn is_write(self) -> bool {
        matches!(
            self,
            AccessType::DataWrite | AccessType::StackWrite | AccessType::DmaWrite
        )
    }

    /// Convenience helper to derive a [`Phase`] from the access type.
    #[must_use]
    pub const fn phase(self) -> Phase {
        if self.is_write() {
            Phase::Write
        } else {
            Phase::Read
        }
    }
}

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
    fn read<A>(&mut self, addr: A, access: AccessType) -> Byte
    where
        A: Into<Word>;

    fn write<A, V>(&mut self, addr: A, value: V, access: AccessType)
    where
        A: Into<Word>,
        V: Into<Byte>;

    /// Read a contiguous block of memory starting at `start` into `dst`.
    ///
    /// Default implementation issues repeated [`read`](Self::read) calls and wraps addresses at
    /// 16-bit boundaries. Override this if your bus can service burst reads more efficiently.
    fn read_block<A>(&mut self, start: A, dst: &mut [u8], access: AccessType)
    where
        A: Into<Word>,
    {
        let mut addr = start.into();
        for byte in dst {
            *byte = u8::from(self.read(addr, access));
            addr += 1;
        }
    }

    /// Write a contiguous block of bytes starting at `start`.
    ///
    /// Default implementation issues repeated [`write`](Self::write) calls and wraps addresses at
    /// 16-bit boundaries. Override this when the bus can push larger buffers directly.
    fn write_block<A>(&mut self, start: A, data: &[u8], access: AccessType)
    where
        A: Into<Word>,
    {
        let mut addr = start.into();
        for &byte in data {
            self.write(addr, Byte(byte), access);
            addr += 1;
        }
    }

    /// Convenience helper to set the CPU reset vector.
    fn set_reset_vector<A>(&mut self, target: A)
    where
        A: Into<Word>,
    {
        let target = target.into();
        self.write(Word(0xFFFC), target.lo(), AccessType::DataWrite);
        self.write(Word(0xFFFD), target.hi(), AccessType::DataWrite);
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
