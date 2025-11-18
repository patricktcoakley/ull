## ull

`ull` is the core of the other crates in this project as it contains the type-safe numeric
wrappers (`Nibble`, `Byte`, `Word`), the `Bus` trait, and some convenience implementations like `SimpleBus`.
Other crates leverage these fundamental building blocks so they can share the same semantics for addresses, DMA, and
memory I/O. While not every system will use every aspect of this crate, they will all still use this as the foundation.

### Bus trait

The `Bus` trait models a synchronous, byte-addressed data bus:

```rust
pub trait Bus {
    fn read<A>(&mut self, addr: A, access: AccessType) -> Byte
    where
        A: Into<Word>;

    fn write<A, V>(&mut self, addr: A, value: V, access: AccessType)
    where
        A: Into<Word>,
        V: Into<Byte>;

    fn on_tick(&mut self, cycles: u8) { … }
    fn request_dma(&mut self, request: DmaRequest) -> DmaResult { … }
    fn poll_dma_cycle(&mut self) -> Option<u8> { … }
}
```

- `AccessType` tags accesses (opcode fetch, stack read/write, DMA, etc.) so a
  bus can differentiate them.
- `on_tick` lets peripherals run “in parallel” with the CPU by giving the bus a
  chance to advance its own notion of time each time the CPU consumes cycles.
- `request_dma`/`poll_dma_cycle` allow the bus to enqueue DMA work that should
  be factored into the CPU’s total cycles.

### Included buses

- `SimpleBus` – a flat 64KiB RAM array with helpers to load buffers and update
  the reset vector.
- `TestingBus` – a 64KiB RAM array backed by `Box<[u8]>` that records total
  cycles, DMA cycles, and lets you enqueue DMA bursts up front.

They’re deliberately minimal so you can embed them into examples or as a
starting point for a richer memory map.

### Quick start

```rust
use ull::{AccessType, Bus, Byte, SimpleBus, Word};

fn main() {
    let mut bus = SimpleBus::default();

    bus.write_block(Word(0x8000), &[0xAA, 0xBB], AccessType::DataWrite);
    assert_eq!(bus.read(Word(0x8001), AccessType::DataRead), Byte(0xBB));
}

/// Example of embedding `SimpleBus` inside a richer memory map.
struct MirrorBus(SimpleBus);

impl Bus for MirrorBus {
    fn read<A>(&mut self, addr: A, access: AccessType) -> Byte
    where
        A: Into<Word>,
    {
        self.0.read(addr, access)
    }

    fn write<A, V>(&mut self, addr: A, value: V, access: AccessType)
    where
        A: Into<Word>,
        V: Into<Byte>,
    {
        // Mirror writes into two halves of RAM.
        let addr = addr.into();
        self.0.write(addr, value, access);
        self.0.write(addr + 0x8000, value, access);
    }
}
```