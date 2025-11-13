//! CPU core, status flags, and addressing modes.

pub mod addressing_mode;
pub mod cpu;
pub mod flags;
pub mod run;

pub use addressing_mode::AddressingMode;
pub use cpu::{Cpu, RunState};
pub use run::{RunConfig, RunOutcome, RunPredicate, RunSummary};
