use ull::{Bus, Byte, Word};

/// Phase within a bus cycle. Some hardware (e.g., NES DMA) cares whether we're in
/// the read (GET) or write (PUT) half of a cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Read,
    Write,
}

/// High-level purpose of a bus access combined with its direction (6502-specific).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AccessType {
    #[default]
    DataRead,
    DataWrite,
    OpcodeFetch,
    StackRead,
    StackWrite,
    InterruptVectorRead,
    DmaRead,
    DmaWrite,
    DummyRead,
}

impl AccessType {
    #[must_use]
    pub const fn is_write(self) -> bool {
        matches!(
            self,
            AccessType::DataWrite | AccessType::StackWrite | AccessType::DmaWrite
        )
    }

    #[must_use]
    pub const fn phase(self) -> Phase {
        if self.is_write() {
            Phase::Write
        } else {
            Phase::Read
        }
    }
}

pub trait ResetVectorExt {
    fn set_reset_vector(&mut self, target: Word);
}

impl<B> ResetVectorExt for B
where
    B: Bus<Access = AccessType, Data = Byte>,
{
    fn set_reset_vector(&mut self, target: Word) {
        self.write(Word(0xFFFC), target.lo(), AccessType::DataWrite);
        self.write(Word(0xFFFD), target.hi(), AccessType::DataWrite);
    }
}
