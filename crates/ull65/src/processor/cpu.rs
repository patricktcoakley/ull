//! 6502 CPU implementation with registers and execution loop.

use crate::instruction::{InstructionSet, InstructionTable, mos6502::Mos6502};
use crate::processor::flags::Flags;
use crate::processor::run::{RunConfig, RunOutcome, RunSummary};
use core::fmt;
use ull::{AccessType, Bus, Byte, Word};
use ull::{byte, word};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    Reset,
    Nmi,
    Irq,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RunState {
    Running,
    Waiting,
    Halted,
}
/// IRQ/BRK vector low byte address.
pub const IRQ_VECTOR_LO: Word = Word(0xFFFE);
/// IRQ/BRK vector high byte address.
pub const IRQ_VECTOR_HI: Word = Word(0xFFFF);
/// NMI vector low byte address.
pub const NMI_VECTOR_LO: Word = Word(0xFFFA);
/// NMI vector high byte address.
pub const NMI_VECTOR_HI: Word = Word(0xFFFB);
/// RESET vector low byte address.
pub const RESET_VECTOR_LO: Word = Word(0xFFFC);
/// RESET vector high byte address.
pub const RESET_VECTOR_HI: Word = Word(0xFFFD);
/// Start of stack space (the 6502 stack grows downward from 0x01FF to 0x0100).
pub const STACK_SPACE_START: Word = Word(0x0100);

/// The 6502 CPU with registers and instruction table.
///
/// Maintains the CPU state over a generic [`Bus`] implementation to allow custom memory/I/O.
///
/// # Examples
///
/// ```
/// use ull::{AccessType, Bus, SimpleBus, Word};
/// use ull65::{Cpu, RESET_VECTOR_HI, RESET_VECTOR_LO};
/// use ull65::instruction::mos6502::Mos6502;
///
/// // Create CPU with MOS 6502 instruction set
/// let mut cpu: Cpu<SimpleBus> = Cpu::with_instruction_set::<Mos6502>();
/// let mut bus = SimpleBus::default();
///
/// // Set up reset vector
/// bus.write(RESET_VECTOR_LO, 0x00, AccessType::DataWrite);
/// bus.write(RESET_VECTOR_HI, 0x80, AccessType::DataWrite);
///
/// // Reset the CPU (loads PC from reset vector)
/// cpu.reset(&mut bus);
/// assert_eq!(cpu.pc, Word(0x8000));
///
/// // Execute instruction
/// bus.write_block(0x8000u16, &[0xA9, 0x42], AccessType::DataWrite); // LDA #$42
/// cpu.tick(&mut bus);
/// assert_eq!(cpu.a.0, 0x42);
/// ```
pub struct Cpu<B: Bus> {
    /// Accumulator register.
    pub a: Byte,
    /// X index register.
    pub x: Byte,
    /// Y index register.
    pub y: Byte,
    /// Processor status flags.
    pub p: Flags,
    /// Stack pointer (0x00-0xFF, actual stack is at 0x0100 + sp).
    pub sp: Byte,
    /// Program counter.
    pub pc: Word,
    /// Total cycles executed.
    pub cycles: u64,
    last_step_cycles: u8,
    /// Opcode executed by the most recent successful [`step`](Self::step) call.
    pub last_opcode: Byte,
    /// Instruction dispatch table.
    pub table: InstructionTable<B>,
    pub run_state: RunState,
    irq_pending: bool,
    nmi_pending: bool,
    reset_pending: bool,
}

impl<B: Bus> fmt::Debug for Cpu<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Here only because the jump table is too noisy
        f.debug_struct("Cpu")
            .field("a", &self.a)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("p", &self.p)
            .field("sp", &self.sp)
            .field("pc", &self.pc)
            .field("cycles", &self.cycles)
            .field("last_step_cycles", &self.last_step_cycles)
            .field("last_opcode", &self.last_opcode)
            .field("run_state", &self.run_state)
            .field("irq_pending", &self.irq_pending)
            .field("nmi_pending", &self.nmi_pending)
            .field("reset_pending", &self.reset_pending)
            .finish_non_exhaustive()
    }
}

impl<B: Bus + 'static> Cpu<B> {
    /// Create a new CPU with the specified instruction set.
    ///
    /// Initializes all registers to their power-on state:
    /// - A, X, Y = 0
    /// - SP = 0xFD
    /// - P = Interrupt Disabled
    /// - PC = 0 (call [`reset`](Self::reset) to load from reset vector)
    ///
    /// # Examples
    ///
    /// ```
    /// use ull::SimpleBus;
    /// use ull65::instruction::mos6502::Mos6502;
    /// use ull65::Cpu;
    ///
    /// let cpu: Cpu<SimpleBus> = Cpu::with_instruction_set::<Mos6502>();
    /// ```
    #[must_use]
    pub fn with_instruction_set<S: InstructionSet>() -> Self {
        Self {
            a: byte!(0),
            x: byte!(0),
            y: byte!(0),
            p: Flags::InterruptDisabled | Flags::Expansion,
            sp: byte!(0xFD),
            pc: word!(0u16),
            cycles: 0,
            last_step_cycles: 0,
            last_opcode: byte!(0),
            table: S::instruction_table::<B>(),
            run_state: RunState::Running,
            irq_pending: false,
            nmi_pending: false,
            reset_pending: false,
        }
    }

    /// Convenience constructor that sets the reset vector and resets the CPU in one call.
    ///
    /// # Examples
    ///
    /// ```
    /// use ull::{SimpleBus, Word};
    /// use ull65::instruction::mos6502::Mos6502;
    /// use ull65::Cpu;
    ///
    /// let mut bus = SimpleBus::default();
    /// let cpu: Cpu<SimpleBus> = Cpu::with_reset_vector::<Mos6502>(&mut bus, Word(0x9000));
    /// assert_eq!(cpu.pc, Word(0x9000));
    /// ```
    pub fn with_reset_vector<S: InstructionSet>(bus: &mut B, reset_vector: Word) -> Self {
        bus.set_reset_vector(reset_vector);
        let mut cpu = Self::with_instruction_set::<S>();
        cpu.reset(bus);
        cpu
    }

    /// Convenience constructor that loads a program, sets the reset vector, and resets the CPU.
    ///
    /// # Examples
    ///
    /// ```
    /// use ull::{SimpleBus, Word};
    /// use ull65::instruction::mos6502::Mos6502;
    /// use ull65::Cpu;
    ///
    /// let mut bus = SimpleBus::default();
    /// let program = [0xEA, 0x00]; // NOP; BRK
    /// let cpu: Cpu<SimpleBus> =
    ///     Cpu::with_program::<Mos6502>(&mut bus, Word(0x8000), &program, Word(0x8000));
    /// assert_eq!(cpu.pc, Word(0x8000));
    /// ```
    pub fn with_program<S: InstructionSet>(
        bus: &mut B,
        load_address: Word,
        program: &[u8],
        reset_vector: Word,
    ) -> Self {
        bus.write_block(load_address, program, AccessType::DataWrite);
        Self::with_reset_vector::<S>(bus, reset_vector)
    }

    /// Reset the CPU to initial state and load PC from reset vector.
    ///
    /// Clears all registers (except loads PC from $FFFC/FFFD) and resets cycle count.
    /// This mimics the hardware RESET line behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use ull::{AccessType, Bus, SimpleBus, Word};
    /// use ull65::instruction::mos6502::Mos6502;
    /// use ull65::{Cpu, RESET_VECTOR_HI, RESET_VECTOR_LO};
    ///
    /// let mut cpu: Cpu<SimpleBus> = Cpu::default();
    /// let mut bus = SimpleBus::default();
    ///
    /// // Set reset vector to 0x8000
    /// bus.write(RESET_VECTOR_LO, 0x00, AccessType::DataWrite);
    /// bus.write(RESET_VECTOR_HI, 0x80, AccessType::DataWrite);
    ///
    /// cpu.reset(&mut bus);
    /// assert_eq!(cpu.pc, Word(0x8000));
    /// ```
    pub fn reset(&mut self, bus: &mut B) {
        self.a = byte!(0);
        self.x = byte!(0);
        self.y = byte!(0);
        self.sp = byte!(0xFD);
        self.p = Flags::InterruptDisabled | Flags::Expansion;
        self.cycles = 0;
        self.last_step_cycles = 0;
        self.last_opcode = byte!(0);
        self.run_state = RunState::Running;
        let lo = bus.read(RESET_VECTOR_LO, AccessType::InterruptVectorRead);
        let hi = bus.read(RESET_VECTOR_HI, AccessType::InterruptVectorRead);
        self.pc = word!((lo, hi));
    }

    /// Execute one instruction.
    ///
    /// Reads the opcode at PC, dispatches to the corresponding instruction function,
    /// and increments the cycle counter. The instruction function is responsible for
    /// advancing PC.
    ///
    /// # Examples
    ///
    /// ```
    /// use ull::{AccessType, Bus, SimpleBus, Word};
    /// use ull65::instruction::mos6502::Mos6502;
    /// use ull65::Cpu;
    ///
    /// let mut cpu: Cpu<SimpleBus> = Cpu::default();
    /// let mut bus = SimpleBus::default();
    ///
    /// cpu.pc = Word(0x8000);
    /// bus.write_block(0x8000u16, &[0xEA], AccessType::DataWrite); // NOP
    ///
    /// let cycles = cpu.step(&mut bus);
    /// assert!(cycles > 0);
    /// ```
    pub fn step(&mut self, bus: &mut B) -> u8 {
        if self.run_state == RunState::Halted {
            self.last_step_cycles = 0;
            return 0;
        }

        if self.reset_pending {
            self.reset(bus);
            self.reset_pending = false;
            return 0;
        }

        if self.nmi_pending {
            self.enter_interrupt(bus, Interrupt::Nmi);
            self.nmi_pending = false;
            self.last_step_cycles = 0;
            return 0;
        }

        if self.irq_pending && !self.p.contains(Flags::InterruptDisabled) {
            self.enter_interrupt(bus, Interrupt::Irq);
            self.irq_pending = false;
            self.last_step_cycles = 0;
            return 0;
        }

        if self.run_state == RunState::Waiting {
            self.last_step_cycles = 0;
            return 0;
        }

        let next_opcode = bus.read(self.pc, AccessType::OpcodeFetch);
        self.last_opcode = next_opcode;
        let instruction = &self.table[u8::from(next_opcode) as usize];
        let execute = instruction.execute;
        let cycles = instruction.cycles;

        let before = self.cycles;
        execute(self, bus);
        self.cycles += u64::from(cycles);
        let consumed = (self.cycles - before) as u8;
        self.last_step_cycles = consumed;
        consumed
    }

    /// Execute one instruction and synchronize the bus.
    ///
    /// This method wraps [`step`](Self::step) and automatically advances the attached bus via
    /// [`Bus::on_tick`] while also draining any pending DMA work reported by [`Bus::poll_dma_cycle`].
    pub fn tick(&mut self, bus: &mut B) -> u8 {
        let cycles = self.step(bus);
        if cycles > 0 {
            bus.on_tick(cycles);
        }

        while let Some(dma_cycles) = bus.poll_dma_cycle() {
            bus.on_tick(dma_cycles);
        }

        cycles
    }

    /// Drive the CPU until a configured stop condition occurs and return a summary.
    pub fn run_until(&mut self, bus: &mut B, config: RunConfig<'_, B>) -> RunSummary {
        let RunConfig {
            instruction_limit,
            stop_on_brk,
            mut predicate,
        } = config;

        let mut summary = RunSummary::default();

        loop {
            if let Some(limit) = instruction_limit
                && summary.instructions_executed >= limit
            {
                summary.mark(RunOutcome::HitInstructionLimit);
                break;
            }

            let cycles = self.tick(bus);
            if cycles == 0 {
                summary.mark(RunOutcome::Stalled);
                break;
            }

            summary.instructions_executed += 1;
            summary.cycles += u64::from(cycles);

            if stop_on_brk && self.last_opcode == byte!(0x00) {
                summary.mark(RunOutcome::HitBrk);
                break;
            }

            if let Some(predicate_cb) = predicate.as_mut()
                && predicate_cb.should_stop(self, bus)
            {
                summary.mark(RunOutcome::HitPredicate);
                break;
            }
        }

        summary
    }

    /// Push a byte onto the stack.
    ///
    /// The stack grows downward from 0x01FF. Stack pointer is decremented after the write.
    pub fn push(&mut self, bus: &mut B, val: Byte) {
        let addr = STACK_SPACE_START + self.sp;
        bus.write(addr, val, AccessType::StackWrite);
        self.sp -= 1;
    }

    /// Pop a byte from the stack.
    ///
    /// Stack pointer is incremented before the read.
    pub fn pop(&mut self, bus: &mut B) -> Byte {
        self.sp += 1;
        let addr = STACK_SPACE_START + self.sp;
        bus.read(addr, AccessType::StackRead)
    }

    /// Check if two addresses are on different pages.
    ///
    /// Some instruction take an extra cycle when crossing page boundaries (when the
    /// high byte changes). This is used internally for cycle-accurate timing.
    #[inline]
    #[must_use]
    pub fn crosses_page(&self, from: Word, to: Word) -> bool {
        from.hi() != to.hi()
    }

    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::Reset => self.reset_pending = true,
            Interrupt::Nmi => self.nmi_pending = true,
            Interrupt::Irq => self.irq_pending = true,
        }
    }

    fn enter_interrupt(&mut self, bus: &mut B, interrupt: Interrupt) {
        self.run_state = RunState::Running;

        let (vector_lo, vector_hi) = match interrupt {
            Interrupt::Nmi => (NMI_VECTOR_LO, NMI_VECTOR_HI),
            Interrupt::Irq => (IRQ_VECTOR_LO, IRQ_VECTOR_HI),
            Interrupt::Reset => unreachable!("enter_interrupt should not be called with Reset"),
        };

        self.push(bus, self.pc.hi());
        self.push(bus, self.pc.lo());
        let mut flags = self.p;
        flags.remove(Flags::Break);
        flags.remove(Flags::DecimalMode);
        self.push(bus, flags.into());

        self.p.set_decimal_mode(false);
        self.p.set_interrupt_disabled(true);

        let lo = bus.read(vector_lo, AccessType::InterruptVectorRead);
        let hi = bus.read(vector_hi, AccessType::InterruptVectorRead);
        self.pc = word!((lo, hi));
    }

    /// Cycles consumed by the most recent [`step`](Self::step) call.
    #[must_use]
    pub fn last_step_cycles(&self) -> u8 {
        self.last_step_cycles
    }
}

impl<B: Bus + 'static> Default for Cpu<B> {
    fn default() -> Self {
        Self::with_instruction_set::<Mos6502>()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::instruction::mos6502::Mos6502;
    use crate::processor::run::RunPredicate;
    use ull::{AccessType, Bus, TestingBus};

    fn prepare_cpu(bus: &mut TestingBus) -> Cpu<TestingBus> {
        bus.write(RESET_VECTOR_LO, byte!(0x00), AccessType::DataWrite);
        bus.write(RESET_VECTOR_HI, byte!(0x80), AccessType::DataWrite);
        let mut cpu: Cpu<TestingBus> = Cpu::with_instruction_set::<Mos6502>();
        cpu.reset(bus);
        cpu
    }

    #[test]
    fn with_reset_vector_sets_pc_and_vector_bytes() {
        let mut bus = TestingBus::default();
        let reset = Word(0x9000);

        let cpu: Cpu<TestingBus> = Cpu::with_reset_vector::<Mos6502>(&mut bus, reset);

        assert_eq!(cpu.pc, reset);
        assert_eq!(bus.read(RESET_VECTOR_LO, AccessType::DataRead), reset.lo());
        assert_eq!(bus.read(RESET_VECTOR_HI, AccessType::DataRead), reset.hi());
    }

    #[test]
    fn with_program_loads_bytes_and_sets_pc() {
        let mut bus = TestingBus::default();
        let program = [0xEA, 0x00];
        let start = Word(0x8000);

        let cpu: Cpu<TestingBus> = Cpu::with_program::<Mos6502>(&mut bus, start, &program, start);

        assert_eq!(cpu.pc, start);
        assert_eq!(bus.read(start, AccessType::OpcodeFetch), Byte(program[0]));
        assert_eq!(
            bus.read(start + 1, AccessType::OpcodeFetch),
            Byte(program[1])
        );
    }

    #[test]
    fn tick_advances_bus_and_drains_dma() {
        let mut bus = TestingBus::default();
        bus.write_block(Word(0x8000), &[0xEA, 0xEA], AccessType::DataWrite);
        bus.queue_dma(3);
        bus.queue_dma(2);

        let mut cpu = prepare_cpu(&mut bus);
        let cycles = cpu.tick(&mut bus);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.last_opcode, byte!(0xEA));
        assert_eq!(bus.ticks, 2 + 3 + 2);
        assert_eq!(bus.dma_ticks, 5);
    }

    #[test]
    fn run_until_stops_on_brk() {
        let mut bus = TestingBus::default();
        bus.write_block(Word(0x8000), &[0xA9, 0x01, 0x00], AccessType::DataWrite);
        let mut cpu = prepare_cpu(&mut bus);

        let summary = cpu.run_until(
            &mut bus,
            RunConfig {
                stop_on_brk: true,
                ..RunConfig::default()
            },
        );

        assert!(summary.hit_brk());
        assert_eq!(summary.instructions_executed, 2);
        assert!(!summary.hit_instruction_limit());
        assert!(!summary.hit_predicate());
    }

    #[test]
    fn run_until_stops_on_predicate() {
        let mut bus = TestingBus::default();
        bus.write_block(Word(0x8000), &[0xE8, 0xE8, 0x00], AccessType::DataWrite);
        let mut cpu = prepare_cpu(&mut bus);

        let mut stop_when_x_is_two =
            |cpu: &Cpu<TestingBus>, _bus: &mut TestingBus| cpu.x == byte!(0x02);

        let summary = cpu.run_until(
            &mut bus,
            RunConfig {
                predicate: Some(RunPredicate::new(&mut stop_when_x_is_two)),
                ..RunConfig::default()
            },
        );

        assert!(summary.hit_predicate());
        assert_eq!(summary.instructions_executed, 2);
        assert!(!summary.hit_brk());
    }

    #[test]
    fn run_until_enforces_instruction_limit() {
        let mut bus = TestingBus::default();
        bus.write_block(
            Word(0x8000),
            &[0xA9, 0x01, 0xE8, 0x00],
            AccessType::DataWrite,
        );
        let mut cpu = prepare_cpu(&mut bus);

        let summary = cpu.run_until(
            &mut bus,
            RunConfig {
                instruction_limit: Some(1),
                stop_on_brk: true,
                ..RunConfig::default()
            },
        );

        assert!(summary.hit_instruction_limit());
        assert_eq!(summary.instructions_executed, 1);
        assert!(!summary.hit_brk());
    }

    #[test]
    fn sixteen_bit_multiply_program() {
        // Source: https://www.lysator.liu.se/~nisse/misc/6502-mul.html
        const FACTOR1_ADDR: u8 = 0x10;
        const FACTOR2_ADDR: u8 = 0x11;
        const FACTOR1_INIT: u8 = 0xB6;
        const FACTOR2_INIT: u8 = 0x4D;
        const EXPECTED_LOW: u8 = 0xBE;
        const EXPECTED_HIGH: u8 = 0x36;
        const PROGRAM: &[u8] = &[
            0xA9,
            0x00, // LDA #$00: Zero the result area (step 1).
            0xA2,
            0x08, // LDX #$08: Plan to examine every bit of A (steps 2/4/5).
            0x46,
            FACTOR1_ADDR, // LSR factor1: Examine current bit of A (steps 2 & 4).
            0x90,
            0x03, // BCC no_add: Skip add when the examined bit is clear.
            0x18, // CLC: Prepare to add B/T into the result.
            0x65,
            FACTOR2_ADDR, // ADC factor2: Add B (or shifted T) at the correct place (steps 2/4).
            // no_add:
            0x6A, // ROR A: Rotate partial result so B is effectively shifted left into scratch T (step 3).
            0x66,
            FACTOR1_ADDR, // ROR factor1: Advance A so the next bit can be examined (step 5 repeats).
            0xCA,         // DEX: Count down the remaining six iterations (step 5).
            0xD0,
            0xF5, // BNE loop: Repeat shift/examine/add until all bits handled.
            0x85,
            FACTOR2_ADDR, // STA factor2: Store the final high byte after the n^2/16 adds.
            0x00,         // BRK: Halt so the test can assert ~1.5 * n^2 work finished.
        ];

        let mut bus = TestingBus::default();
        bus.write_block(Word(0x8000), PROGRAM, AccessType::DataWrite);
        bus.write(
            Word(u16::from(FACTOR1_ADDR)),
            Byte(FACTOR1_INIT),
            AccessType::DataWrite,
        );
        bus.write(
            Word(u16::from(FACTOR2_ADDR)),
            Byte(FACTOR2_INIT),
            AccessType::DataWrite,
        );

        let mut cpu = prepare_cpu(&mut bus);
        let summary = cpu.run_until(
            &mut bus,
            RunConfig {
                stop_on_brk: true,
                instruction_limit: Some(0x200),
                ..RunConfig::default()
            },
        );

        assert!(summary.hit_brk());
        assert!(!summary.hit_instruction_limit());

        let low = bus.read(FACTOR1_ADDR, AccessType::DataRead);
        let high = bus.read(FACTOR2_ADDR, AccessType::DataRead);

        assert_eq!(low, Byte(EXPECTED_LOW));
        assert_eq!(high, Byte(EXPECTED_HIGH));
        assert_eq!(cpu.a, Byte(EXPECTED_HIGH));
        assert_eq!(cpu.last_opcode, Byte::ZERO);
    }
}
