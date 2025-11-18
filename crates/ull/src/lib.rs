#![no_std]

extern crate alloc;
pub mod bus;
pub use bus::{AccessType, Bus, DmaRequest, DmaResult, Phase, SimpleBus, TestingBus};
pub mod byte;
pub use byte::Byte;
pub mod nibble;
pub use nibble::Nibble;
pub mod word;
pub use word::Word;
