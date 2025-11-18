use ull::bus::SimpleBus;
use ull::{AccessType, Bus, Word};
use ull65::Cpu;
use ull65::instruction::{InstructionSet, mos6502::Mos6502};

const MAX_STEPS: u64 = 50_000_000;
const LOOP_THRESHOLD: u32 = 10;

#[derive(Clone, Copy)]
pub struct Fixture<'a> {
    pub name: &'a str,
    pub rom: &'a [u8],
    pub load_addr: Word,
    pub reset_vector: Word,
    pub success_pc: Word,
}

pub const MOS_FIXTURES: &[Fixture<'_>] = &[
    Fixture {
        name: "AllSuiteA",
        rom: include_bytes!("../../../thirdparty/AllSuiteA/AllSuiteA.bin"),
        load_addr: Word(0x4000),
        reset_vector: Word(0x4000),
        success_pc: Word(0x45C0),
    },
    Fixture {
        name: "Klaus",
        rom: include_bytes!("../../../thirdparty/Klaus2m5/6502_functional_test.bin"),
        load_addr: Word(0x0000),
        reset_vector: Word(0x0400),
        success_pc: Word(0x3469),
    },
];

pub const WDC65C02_FIXTURES: &[Fixture<'_>] = &[Fixture {
    name: "Klaus65C02",
    rom: include_bytes!("../../../thirdparty/Klaus2m5/65C02_extended_opcodes_test.bin"),
    load_addr: Word(0x0000),
    reset_vector: Word(0x0400),
    success_pc: Word(0x24F1),
}];

pub fn run_fixture_with<S>(fixture: &Fixture)
where
    S: InstructionSet,
{
    let mut bus = SimpleBus::default();
    bus.write_block(fixture.load_addr, fixture.rom, AccessType::DataWrite);
    bus.set_reset_vector(fixture.reset_vector);

    let mut cpu: Cpu<SimpleBus> = Cpu::with_instruction_set::<S>();
    cpu.reset(&mut bus);

    let mut last_pc = cpu.pc;
    let mut repeat_count = 0u32;

    for step in 1..=MAX_STEPS {
        cpu.step(&mut bus);

        let pc = cpu.pc;
        if pc == fixture.success_pc {
            return;
        }

        if pc == last_pc {
            repeat_count += 1;
            if repeat_count >= LOOP_THRESHOLD {
                panic_trapped(fixture, step, pc, &cpu, &mut bus);
            }
        } else {
            repeat_count = 0;
            last_pc = pc;
        }
    }

    panic_hung(fixture, &cpu);
}

#[allow(dead_code)]
pub fn run_fixture(fixture: &Fixture) {
    run_fixture_with::<Mos6502>(fixture);
}

pub fn run_fixtures_with<S>(fixtures: &[Fixture])
where
    S: InstructionSet,
{
    let _ = env_logger::builder().is_test(true).try_init();

    for fixture in fixtures {
        run_fixture_with::<S>(fixture);
    }
}

fn panic_trapped(
    fixture: &Fixture<'_>,
    steps: u64,
    pc: Word,
    cpu: &Cpu<SimpleBus>,
    bus: &mut SimpleBus,
) -> ! {
    let test_case = bus.read(Word(0x0200), AccessType::DataRead);
    panic!(
        "{name} trapped at {pc:04X} after {steps} steps (test_case {test_case:02X}); processor={cpu:?}",
        name = fixture.name,
        pc = pc,
        steps = steps,
        test_case = u8::from(test_case),
        cpu = cpu
    );
}

fn panic_hung(fixture: &Fixture<'_>, cpu: &Cpu<SimpleBus>) -> ! {
    panic!(
        "{name} exceeded {MAX_STEPS} steps (pc {pc:04X}); processor={cpu:?}",
        name = fixture.name,
        pc = cpu.pc,
        cpu = cpu
    );
}
