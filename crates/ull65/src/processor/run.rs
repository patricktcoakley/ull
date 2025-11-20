use crate::bus::Mos6502CompatibleBus;
use crate::Cpu;

/// Reason why [`Cpu::run_until`](crate::processor::cpu::Cpu::run_until) stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RunOutcome {
    /// Execution is still in progress (default before stopping condition hit).
    #[default]
    InProgress,
    /// Execution hit a BRK instruction and `stop_on_brk` was enabled.
    HitBrk,
    /// User-supplied predicate returned `true`.
    HitPredicate,
    /// [`RunConfig::instruction_limit`] was reached.
    HitInstructionLimit,
    /// CPU failed to make forward progress (halted, waiting, etc.).
    Stalled,
}

/// Summary produced by [`Cpu::run_until`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RunSummary {
    /// Total instructions executed.
    pub instructions_executed: u64,
    /// Total CPU cycles spent on those instructions.
    pub cycles: u64,
    /// Outcome describing why execution stopped.
    pub outcome: RunOutcome,
}

impl RunSummary {
    /// Record that the run stopped for the provided outcome.
    pub fn mark(&mut self, outcome: RunOutcome) {
        self.outcome = outcome;
    }

    /// Returns `true` if execution halted because a BRK was executed.
    #[must_use]
    pub fn hit_brk(&self) -> bool {
        self.outcome == RunOutcome::HitBrk
    }

    /// Returns `true` if the user predicate stopped execution.
    #[must_use]
    pub fn hit_predicate(&self) -> bool {
        self.outcome == RunOutcome::HitPredicate
    }

    /// Returns `true` if execution reached the configured instruction limit.
    #[must_use]
    pub fn hit_instruction_limit(&self) -> bool {
        self.outcome == RunOutcome::HitInstructionLimit
    }

    /// Returns `true` if the CPU stalled (e.g., waiting, halted).
    #[must_use]
    pub fn stalled(&self) -> bool {
        self.outcome == RunOutcome::Stalled
    }
}

/// Wrapper around a predicate callback used by [`RunConfig`].
pub struct RunPredicate<'a, B: Mos6502CompatibleBus> {
    callback: &'a mut dyn FnMut(&Cpu<B>, &mut B) -> bool,
}

impl<'a, B: Mos6502CompatibleBus> RunPredicate<'a, B> {
    /// Create a new predicate wrapper.
    pub fn new(callback: &'a mut dyn FnMut(&Cpu<B>, &mut B) -> bool) -> Self {
        Self { callback }
    }

    pub fn should_stop(&mut self, cpu: &Cpu<B>, bus: &mut B) -> bool {
        (self.callback)(cpu, bus)
    }
}

/// Configuration for [`Cpu::run_until`].
pub struct RunConfig<'a, B: Mos6502CompatibleBus> {
    /// Maximum number of instructions to execute before stopping.
    pub instruction_limit: Option<u64>,
    /// Stop automatically when a BRK (opcode 0x00) executes.
    pub stop_on_brk: bool,
    /// Optional predicate invoked after each instruction; returning `true` stops the run.
    pub predicate: Option<RunPredicate<'a, B>>,
}

impl<B: Mos6502CompatibleBus> Default for RunConfig<'_, B> {
    fn default() -> Self {
        Self {
            instruction_limit: None,
            stop_on_brk: false,
            predicate: None,
        }
    }
}
