//! Basic 64KB flat memory implementation.

use crate::bus::AccessType;
use crate::word::Word;
use crate::{Bus, Byte};
use alloc::{boxed::Box, vec};

/// Simple 64KB flat memory implementation.
///
/// Provides a basic, contiguous 64KB memory space (0x0000-0xFFFF) with no banking
/// or special behavior. This is the simplest [`Bus`] implementation and
/// is suitable for testing or bootstrapping a very basic system.
///
/// # Examples
///
/// ```
/// use ull65::bus::SimpleBus;
/// use ull65::{AccessType, Bus, Byte, Word};
///
/// let mut bus = SimpleBus::default();
///
/// // Load data at a specific address
/// bus.write_block(Word(0x8000), &[0xA9, 0x42], AccessType::DataWrite);
/// ```
#[derive(Debug)]
pub struct SimpleBus {
    mem: Box<[u8]>,
}

impl SimpleBus {
    const ADDR_MASK: usize = 0xFFFF;
    const MEM_SIZE: usize = 0x10000;
}

impl Default for SimpleBus {
    fn default() -> Self {
        Self {
            mem: vec![0; Self::MEM_SIZE].into_boxed_slice(),
        }
    }
}

impl SimpleBus {
    /// Load bytes starting at `start`.
    pub fn load(&mut self, start: Word, bytes: &[u8]) {
        self.write_block(start, bytes, AccessType::DataWrite);
    }

    /// Load a program and update the reset vector in one step.
    pub fn load_with_reset(&mut self, start: Word, bytes: &[u8], reset: Word) {
        self.load(start, bytes);
        self.set_reset_vector(reset);
    }
}

impl Bus for SimpleBus {
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

    fn read_block<A>(&mut self, start: A, dst: &mut [u8], _access: AccessType)
    where
        A: Into<Word>,
    {
        let mut idx = start.into().as_usize() & Self::ADDR_MASK;
        for byte in dst {
            *byte = self.mem[idx];
            idx = (idx + 1) & Self::ADDR_MASK;
        }
    }

    fn write_block<A>(&mut self, start: A, data: &[u8], _access: AccessType)
    where
        A: Into<Word>,
    {
        let mut idx = start.into().as_usize() & Self::ADDR_MASK;
        for &byte in data {
            self.mem[idx] = byte;
            idx = (idx + 1) & Self::ADDR_MASK;
        }
    }
}
