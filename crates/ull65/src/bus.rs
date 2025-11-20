pub mod simple_bus;
pub mod testing_bus;

use ull::{Bus, Byte};

use crate::AccessType;

/// Convenience bound for MOS 6502/WDC 65C02 compatible buses.
pub trait Mos6502CompatibleBus: Bus<Access = AccessType, Data = Byte> {}

impl<T> Mos6502CompatibleBus for T where T: Bus<Access = AccessType, Data = Byte> {}
