//! Apple I WOZMON demo using the `Bus` trait.
//!
//! Loads WOZMON + BASIC ROMs, pipes host stdin into the Apple I keyboard MMIO,
//! and prints characters written to the display register.

use std::collections::VecDeque;
use std::io::{self, Write};

use ull65::bus::{AccessType, Bus};
use ull65::{Byte, Cpu, IRQ_VECTOR_LO, NMI_VECTOR_LO, RESET_VECTOR_LO, Word};

const MEMORY_SIZE: usize = 0x10000;
const BASIC_START: usize = 0xE000;
const WOZMON_START: usize = 0xFF00;

const KBD_DATA: u16 = 0xD010;
const KBD_STATUS: u16 = 0xD011;
const DISPLAY_DATA: u16 = 0xD012;

const BASIC_ROM: &[u8] = include_bytes!("../../../thirdparty/applei/BASIC.ROM");
const WOZMON_ROM: &[u8] = include_bytes!("../../../thirdparty/applei/WOZMON.ROM");
const PUMP_CYCLES: usize = 1_000_000;

struct Apple1Bus {
    mem: Box<[u8]>,
    rom_mask: Box<[bool]>,
    keyboard_data: u8,
    keyboard_ready: bool,
    pending_keys: VecDeque<u8>,
    display_buffer: Vec<u8>,
}

impl Apple1Bus {
    fn new() -> Self {
        let mut bus = Self {
            mem: vec![0; MEMORY_SIZE].into_boxed_slice(),
            rom_mask: vec![false; MEMORY_SIZE].into_boxed_slice(),
            keyboard_data: 0,
            keyboard_ready: false,
            pending_keys: VecDeque::new(),
            display_buffer: Vec::new(),
        };
        bus.load_basic();
        bus.load_wozmon();
        bus
    }

    fn load_basic(&mut self) {
        let end = BASIC_START + BASIC_ROM.len();
        self.mem[BASIC_START..end].copy_from_slice(BASIC_ROM);
        for idx in BASIC_START..end {
            self.rom_mask[idx] = true;
        }
    }

    fn load_wozmon(&mut self) {
        let end = WOZMON_START + WOZMON_ROM.len();
        self.mem[WOZMON_START..end].copy_from_slice(WOZMON_ROM);
        for idx in WOZMON_START..end {
            self.rom_mask[idx] = true;
        }

        let entry = u16::try_from(WOZMON_START).expect("WOZMON start fits in 16 bits");
        self.write_vector(RESET_VECTOR_LO, entry);
        self.write_vector(NMI_VECTOR_LO, entry);
        self.write_vector(IRQ_VECTOR_LO, entry);
    }

    fn write_vector(&mut self, addr: Word, value: u16) {
        let lo = (value & 0x00FF) as u8;
        let hi = (value >> 8) as u8;
        let idx = addr.as_usize();
        self.mem[idx] = lo;
        self.mem[idx + 1] = hi;
        self.rom_mask[idx] = true;
        self.rom_mask[idx + 1] = true;
    }

    fn push_key(&mut self, ascii: u8) {
        let value = ascii & 0x7F;
        if self.keyboard_ready {
            self.pending_keys.push_back(value);
        } else {
            self.keyboard_data = value;
            self.keyboard_ready = true;
        }
    }

    fn load_next_key(&mut self) {
        if !self.keyboard_ready {
            if let Some(next) = self.pending_keys.pop_front() {
                self.keyboard_data = next;
                self.keyboard_ready = true;
            }
        }
    }

    fn read_keyboard(&mut self, addr: u16) -> Byte {
        match addr {
            KBD_DATA => {
                let mut value = self.keyboard_data;
                if self.keyboard_ready {
                    value |= 0x80;
                    self.keyboard_ready = false;
                    self.load_next_key();
                }
                Byte(value)
            }
            KBD_STATUS => {
                if self.keyboard_ready {
                    Byte(0x80)
                } else {
                    Byte(0x00)
                }
            }
            _ => Byte(0x00),
        }
    }

    fn write_display(&mut self, value: Byte) {
        let ch = value.0 & 0x7F;
        if ch == 0x7F {
            // Ignore RUBOUT control characters WOZMON emits as part of its handshake.
            return;
        }
        if ch == 0x9B {
            // Apple I uses 0x9B as newline.
            self.display_buffer.extend_from_slice(b"\r\n");
        } else if ch == b'\r' {
            self.display_buffer.extend_from_slice(b"\r\n");
        } else {
            self.display_buffer.push(ch);
        }
    }

    fn take_display(&mut self) -> String {
        let out = String::from_utf8_lossy(&self.display_buffer).into_owned();
        self.display_buffer.clear();
        out
    }
}

impl Bus for Apple1Bus {
    fn read<A>(&mut self, addr: A, _access: AccessType) -> Byte
    where
        A: Into<Word>,
    {
        let addr = addr.into();
        match addr.0 {
            KBD_DATA | KBD_STATUS => self.read_keyboard(addr.0),
            _ => Byte(self.mem[addr.as_usize()]),
        }
    }

    fn write<A, V>(&mut self, addr: A, value: V, _access: AccessType)
    where
        A: Into<Word>,
        V: Into<Byte>,
    {
        let addr = addr.into();
        let value = value.into();
        match addr.0 {
            DISPLAY_DATA => self.write_display(value),
            KBD_STATUS => {
                // Writing any value clears the ready flag on real hardware.
                self.keyboard_ready = false;
                self.load_next_key();
            }
            KBD_DATA => {}
            _ => {
                let idx = addr.as_usize();
                if !self.rom_mask[idx] {
                    self.mem[idx] = value.0;
                }
            }
        }
    }
}

fn pump(cpu: &mut Cpu<Apple1Bus>, bus: &mut Apple1Bus) {
    for _ in 0..PUMP_CYCLES {
        if cpu.tick(bus) == 0 {
            break;
        }
    }
}

fn flush_display(bus: &mut Apple1Bus) {
    let out = bus.take_display();
    if !out.is_empty() {
        print!("{out}");
        let _ = io::stdout().flush();
    }
}

fn main() -> io::Result<()> {
    let mut bus = Apple1Bus::new();
    let mut cpu: Cpu<Apple1Bus> = Cpu::default();
    cpu.reset(&mut bus);

    println!("Apple I WOZMON demo. Type 'E000R' to enter BASIC mode and ':quit' to exit.\n");

    // Give WOZMON time to print its banner and prompt before accepting input.
    pump(&mut cpu, &mut bus);
    flush_display(&mut bus);

    let stdin = io::stdin();
    loop {
        pump(&mut cpu, &mut bus);
        flush_display(&mut bus);

        let mut line = String::new();
        let bytes = stdin.read_line(&mut line)?;
        if bytes == 0 || line.trim_end() == ":quit" {
            println!("\nExiting Apple I demo.");
            break;
        }

        for ch in line.chars() {
            match ch {
                '\r' => {}
                '\n' => bus.push_key(b'\r'),
                _ => bus.push_key(ch.to_ascii_uppercase() as u8),
            }
        }

        if !line.ends_with('\n') {
            bus.push_key(b'\r');
        }

        pump(&mut cpu, &mut bus);
        flush_display(&mut bus);
    }

    Ok(())
}
