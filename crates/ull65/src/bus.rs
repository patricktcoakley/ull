//! Memory and I/O bus abstraction.

pub mod simple_bus;
pub mod testing_bus;

use crate::{Byte, RESET_VECTOR_HI, RESET_VECTOR_LO, Word};
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
        self.write(RESET_VECTOR_LO, target.lo(), AccessType::DataWrite);
        self.write(RESET_VECTOR_HI, target.hi(), AccessType::DataWrite);
    }

    fn on_tick(&mut self, cycles: u8) {
        let _ = cycles;
    }

    fn request_dma(&mut self, request: DmaRequest) -> DmaResult {
        let _ = request;
        DmaResult::Denied
    }

    /// Retrieve the next pending DMA transfer, if any.
    ///
    /// Default implementations report no pending DMA. Bus implementations that queue DMA work
    /// should override this method and return the number of CPU cycles consumed each time
    /// a transfer chunk completes.
    fn poll_dma_cycle(&mut self) -> Option<u8> {
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::Cpu;
    use crate::instruction::mos6502::Mos6502;
    use alloc::collections::VecDeque;

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
}
