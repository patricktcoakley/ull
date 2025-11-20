## ull

`ull` is the core of the other crates in this project as it contains the type-safe numeric
wrappers (`Nibble`, `Byte`, `Word`) plus the `Bus` trait. Higher-level crates supply ready-to-use
bus implementations (e.g., `SimpleBus` in `ull65`) while sharing the same address and DMA semantics.
Other crates leverage these fundamental building blocks so they can share the same semantics for addresses, DMA, and
memory I/O. While not every system will use every aspect of this crate, they will all still use this as the foundation.

### Bus trait

The `Bus` trait models a synchronous, byte-addressed data bus:

```rust
pub trait Bus {
    type Access: Copy;
    type Data: Copy;

    fn read<A>(&mut self, addr: A, access: Self::Access) -> Self::Data
    where
        A: Address;

    fn write<A, V>(&mut self, addr: A, value: V, access: Self::Access)
    where
        A: Address,
        V: Into<Self::Data>;

    fn on_tick(&mut self, cycles: u8) { … }
    fn request_dma(&mut self, request: DmaRequest) -> DmaResult { … }
    fn poll_dma_cycle(&mut self) -> Option<u8> { … }
}
```

- Each bus chooses its own `Access` type (or `()` if it doesn’t care) so
  higher-level CPUs can tag reads/writes however they see fit.
- `on_tick` lets peripherals run “in parallel” with the CPU by giving the bus a
  chance to advance its own notion of time each time the CPU consumes cycles.
- `request_dma`/`poll_dma_cycle` allow the bus to enqueue DMA work that should
  be factored into the CPU’s total cycles.

### Reference buses

The companion `ull65` crate ships two simple 8-bit implementations built on this trait:

- `SimpleBus` – a flat 64KiB RAM array with helpers to load buffers and update
  the reset vector.
- `TestingBus` – a 64KiB RAM array backed by `Box<[u8]>` that records total
  cycles, DMA cycles, and lets you enqueue DMA bursts up front.

They’re deliberately minimal so you can embed them into examples or as a
starting point for a richer memory map.

### Quick start

```rust
use ull::{Address, Bus, Byte, Word};
use ull65::{AccessType, SimpleBus};

fn main() {
    let mut bus = SimpleBus::default();

    bus.write_block(Word(0x8000), &[0xAA, 0xBB], AccessType::DataWrite);
    assert_eq!(bus.read(Word(0x8001), AccessType::DataRead), Byte(0xBB));
}

/// Example of embedding `SimpleBus` inside a richer memory map.
struct MirrorBus(SimpleBus);

impl Bus for MirrorBus {
    type Access = AccessType;
    type Data = Byte;

    fn read<A>(&mut self, addr: A, access: AccessType) -> Byte
    where
        A: Address,
    {
        self.0.read(addr, access)
    }

    fn write<A, V>(&mut self, addr: A, value: V, access: AccessType)
    where
        A: Address,
        V: Into<Self::Data>,
    {
        // Mirror writes into two halves of RAM.
        let addr_mirror = addr + 0x8000;
        let byte = value.into();
        self.0.write(addr, byte, access);
        self.0.write(addr_mirror, byte, access);
    }
}
```
