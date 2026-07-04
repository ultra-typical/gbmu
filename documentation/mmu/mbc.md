# MBC (Memory Bank Controller) Implementation

## Overview
The Game Boy CPU can only address 32KB of ROM at once (0x0000-0x7FFF), but cartridges are often much larger. A Memory Bank Controller sits on the cartridge and swaps which slice of ROM (and RAM) is visible in the address space, letting games reach several megabytes through a 32KB window. This module provides the `Mbc` trait and five concrete controllers: `RomOnly`, `Mbc1`, `Mbc2`, `Mbc3`, and `Mbc5`.

## Address Space Seen by an MBC
- **0x0000-0x3FFF**: ROM bank 0 (usually fixed).
- **0x4000-0x7FFF**: switchable ROM bank.
- **0xA000-0xBFFF**: external cartridge RAM (or RTC registers on MBC3), when present.

Writes into the ROM range do not modify ROM. They are commands interpreted by the controller (RAM enable, bank select, mode select).

## The Mbc Trait
```rust
pub trait Mbc {
    fn new(rom_image: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Result<Self, String>
        where Self: Sized;
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
    fn dump(&self) -> Option<Vec<u8>>;
    fn kind(&self) -> MbcType;
}
```
- `new`: builds the controller from the raw ROM image and optional battery-backed RAM. Returns `Result` because construction validates the cartridge header and can fail.
- `read` / `write`: handle the 0x0000-0x7FFF and 0xA000-0xBFFF ranges.
- `dump`: returns the current external RAM for saving, or `None` when the cartridge has no battery RAM.
- `kind`: reports which controller this is (see `MbcType`).

The MMU holds the controller as a generic parameter `M: Mbc`, so the concrete type is chosen by the caller (based on the cartridge header byte 0x0147) and the MMU code stays controller-agnostic.

```rust
pub enum MbcType {
    RomOnly,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
}
```

## Cartridge Header Parsing
Three constants define the geometry:
```rust
const ONLY_ROM_SIZE: usize = 0xC000;
const ROM_BANK_SIZE: usize = 0x4000; // 16KB
const RAM_BANK_SIZE: usize = 0x2000; // 8KB
```

Two helpers read the header:
- `get_rom_bank_size` reads byte 0x148 and maps the code to a bank count (0 gives 2 banks, 1 gives 4, up to 8 giving 512). An unknown code is an error.
- `get_ram_bank_size` reads byte 0x149 and maps to a RAM bank count (0 and 1 give 0 banks, 2 gives 1, 3 gives 4, 4 gives 16, 5 gives 8).

Two helpers build the banks:
- `map_rom_into_bank` splits the ROM image into exact 16KB chunks and checks that the number of banks matches the header. A mismatch is an error, which catches truncated or inconsistent images.
- `map_ram_banks` allocates the RAM banks. When `saved_ram` is provided, it is split into `RAM_BANK_SIZE` chunks (each chunk validated for size, so a corrupted save is rejected). Otherwise the banks are zero-initialised. `map_n_ram_banks` is the shared building block.

## RomOnly
The simplest cartridge, up to 32KB with optional RAM and no banking.
```rust
pub struct RomOnly {
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Vec<Vec<u8>>,
}
```
- `new` copies the ROM image into a single buffer of `ONLY_ROM_SIZE` and builds any RAM banks from the header.
- `read`: 0x0000-0x7FFF reads straight from the single ROM buffer, 0xA000-0xBFFF reads external RAM if present, otherwise returns 0xFF.
- `write`: only external RAM is writable, and only when RAM banks exist. Writes to the ROM range are ignored.
- `dump` returns `None`, since a plain ROM cartridge has no battery save.

## MBC1
Up to 2MB ROM and 32KB RAM.
```rust
pub struct Mbc1 {
    banks: Vec<Vec<u8>>,
    ram_gate_register: bool, // RAM enabled when the low nibble written is 0xA
    bank_register_1: u8,     // 5-bit low ROM bank
    bank_register_2: u8,     // 2-bit high ROM bank, or RAM bank
    mode_register: bool,     // banking mode
    ram_banks: Vec<Vec<u8>>,
}
```
Register writes:
- 0x0000-0x1FFF: RAM gate, enabled when `val & 0xF == 0xA`.
- 0x2000-0x3FFF: `bank_register_1`, the low 5 bits of the ROM bank.
- 0x4000-0x5FFF: `bank_register_2`, 2 bits used as the high ROM bank bits or the RAM bank.
- 0x6000-0x7FFF: `mode_register`, from bit 0 of the value.

Reads:
- **0x4000-0x7FFF**: bank index is `max(bank_register_1, 1) + (bank_register_2 << 5)`, taken modulo the bank count. The `max(.., 1)` reproduces the hardware quirk where bank 0 cannot be selected in this window.
- **0x0000-0x3FFF**: depends on the mode. The code reads bank 0 when `mode_register` is set and the high-bank region (`bank_register_2 << 5`) when it is clear.
- **0xA000-0xBFFF**: when the RAM gate is open, the RAM bank is `mode_register as usize * (bank_register_2 & 0b11)`, so RAM banking is only active in advanced mode. When the gate is closed, reads return 0xFF.

`new` starts `bank_register_1` at 1 and rejects cartridges declaring more than 4 RAM banks.

## MBC2
Up to 256KB ROM with 512 nibbles of built-in RAM (no external RAM).
```rust
pub struct Mbc2 {
    rom_banks: Vec<Vec<u8>>,
    ram_gate_register: bool,
    rom_bank_register: u8,
    ram_banks: Vec<u8>, // 512 bytes, only the low nibble of each is used
}
```
The distinguishing feature is that address bit 8 selects between the two control functions in the 0x0000-0x3FFF write range:
- Bit 8 clear: RAM enable (`val & 0xF == 0xA`).
- Bit 8 set: ROM bank select (`val & 0xF`, with 0 promoted to 1).

RAM behaviour:
- Only 512 half-bytes exist, so the RAM address wraps with `addr & 0x1FF`.
- Only the low nibble is stored, and reads return `0xF0 | value`, since the high nibble is not backed by memory.
- Access is gated by the RAM enable register, closed reads return 0xFF.

`dump` returns the 512-byte RAM buffer for battery saves.

## MBC3
Like MBC1 with an added Real Time Clock for timestamped saves.
```rust
pub struct Mbc3 {
    rtc: Rtc,
    ram_timer_enable: bool,
    latch_clock_data: bool,
    rom_bank_nb: u8,
    ram_rtc_select: u8,
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Vec<Vec<u8>>,
}
```
Register writes:
- 0x0000-0x1FFF: enable RAM and RTC access (`val & 0xF == 0xA`).
- 0x2000-0x3FFF: ROM bank (7 bits, 0 promoted to 1).
- 0x4000-0x5FFF: `ram_rtc_select`, which chooses a RAM bank (0x00-0x07) or an RTC register (0x08-0x0C).
- 0x6000-0x7FFF: latch clock, a 0 then 1 transition snapshots the current RTC into `latched`.

Reads at 0xA000-0xBFFF dispatch on `ram_rtc_select`: a RAM bank when the selector is a valid RAM index, an RTC register when it is in 0x08-0x0C, otherwise 0xFF. Everything is gated by `ram_timer_enable`.

### RTC internals
```rust
struct RtcRegisters {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day_counter: u16, // 9 significant bits
    halted: bool,
    day_carry: bool,
}
struct Rtc {
    base: RtcRegisters,
    base_timestamp: DateTime<Local>,
    latched: Option<RtcRegisters>,
}
```
The clock is driven by real wall-clock time rather than emulated cycles. `base` is the RTC value at the moment `base_timestamp` was recorded, and `current()` adds the real elapsed time to `base`, rolling seconds into minutes, minutes into hours, hours into days, and setting `day_carry` when the day counter passes 512. When the clock is halted, `current()` returns `base` unchanged.

`read_registers` returns the latched snapshot when one exists, otherwise the live value. `write_registers` updates `base`, resets `base_timestamp` to now, and maps selector 0x08-0x0C onto the fields (with selector 0x0C carrying the day high bit, the halt flag, and the day carry).

## MBC5
Up to 8MB ROM and 128KB RAM, and the first controller that can address a full 512 ROM banks.
```rust
pub struct Mbc5 {
    ram_gate_enable: bool,
    rom_bank_register_high: u8, // provides the 9th bank bit
    rom_bank_register_low: u8,  // low 8 bank bits
    ram_bank_register: u8,
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Vec<Vec<u8>>,
}
```
Register writes:
- 0x0000-0x1FFF: RAM gate (`val & 0xF == 0xA`).
- 0x2000-0x2FFF: low 8 bits of the ROM bank.
- 0x3000-0x3FFF: bit 0 becomes the 9th ROM bank bit.
- 0x4000-0x5FFF: RAM bank.
- 0x6000-0x7FFF: unused.

Reads at 0x4000-0x7FFF combine the two registers into a 9-bit bank number `(high & 1) * 0x100 + low`, taken modulo the bank count. RAM reads and writes at 0xA000-0xBFFF are gated by the RAM enable register and the presence of RAM banks, returning 0xFF when closed.

## Battery Saves
`dump` exposes the external RAM so the front-end can persist it. `RomOnly` returns `None`, MBC2 returns its built-in buffer, and MBC1/3/5 return their RAM banks concatenated. On startup the same bytes are passed back in through `new` as `saved_ram`.

## Integration with the MMU
The MMU routes the `Mbc` and `ERam` regions to the controller. Both ROM reads (0x0000-0x7FFF) and external RAM access (0xA000-0xBFFF) go through the same `read` and `write` calls, and the controller decides internally what each address means. Controller construction happens inside the MMU's `new`, and `ram_dump` on the MMU forwards to the controller's `dump`.

## Implementation Status
### Completed
- ✅ `Mbc` trait with a shared interface
- ✅ RomOnly, MBC1, MBC2, MBC3, MBC5 implementations
- ✅ ROM bank switching for each controller
- ✅ External RAM with enable gating
- ✅ MBC2 built-in 4-bit RAM with address wrapping
- ✅ MBC3 Real Time Clock with latch and real-time progression
- ✅ Cartridge header validation (ROM/RAM sizes)
- ✅ Battery-backed save load and dump
- ✅ Save-state (serde) support

### TODO / points to verify
- ⚠️ The MBC1 0x0000-0x3FFF read path selects bank 0 in advanced mode and the high-bank region in simple mode. This is worth checking against the reference MBC1 behaviour, where the roles are usually the other way around.
- ❌ MBC1 large-cartridge multicart wiring (alternate bank2 interpretation)
- ❌ RTC persistence across sessions (the clock currently derives from the host time at load, the latched/base values are serialised but the day-carry and halt corner cases may need review)
- ❌ Concrete-type selection from header byte 0x0147 lives outside this module, confirm every declared MBC maps to an implementation

### Unsupported cartridge types
The five controllers above cover the vast majority of the commercial library. The following are not implemented:
- ❌ **MBC6**: flash-memory mapper used by a single game (Net de Get). Very niche.
- ❌ **MBC7**: adds a two-axis accelerometer and EEPROM save (Kirby Tilt 'n' Tumble, Command Master). Needs tilt input support to be useful.
- ❌ **MMM01**: a multi-game menu mapper that remaps which ROM region acts as the base bank. Found on a handful of compilation cartridges.
- ❌ **HuC1 / HuC3**: Hudson mappers, HuC3 adds an RTC and infrared.
- ❌ **TAMA5**: Bandai mapper used by the Game Boy Tamagotchi cartridges, with its own RTC.
- ❌ **Pocket Camera (MAC-GBD)**: the Game Boy Camera mapper, exposes an image sensor.
- ❌ **M161**: rare multicart mapper.

None of these are required to run ordinary games. They would only matter for full library coverage or for the specific titles that use them.

### Last Modification
This document was last updated on **2026/07/04**, based on the state at commit `653854a1d657d2c499f7f261228352ead4feceff`.

Some changes may have been made to the code since then.
