# MMU (Memory Management Unit) Implementation

## Overview
The MMU is the central memory router for the Game Boy / Game Boy Color emulator. It intercepts every memory read and write coming from the CPU and forwards it to the hardware component or memory region that owns the target address. Think of it as a switchboard operator directing calls to the right department.

## Why the MMU Exists
The Game Boy has a 64KB address space (0x0000-0xFFFF), divided among many hardware components:
- ROM cartridge (via the MBC)
- Video RAM (managed by the PPU)
- Work RAM
- Object Attribute Memory (OAM, sprite data, managed by the PPU)
- I/O registers (joypad, sound, LCD control, timers, DMA/HDMA, etc.)
- High RAM (HRAM)

The MMU's job is to:
1. Determine which component owns a given address.
2. Route reads/writes to that component.
3. Handle special cases (mirrors, unusable regions, read-only areas, register side effects).

## Memory Map
```
0x0000-0x3FFF: ROM Bank 0 (fixed)
0x4000-0x7FFF: ROM Bank N (switchable via MBC)
0x8000-0x9FFF: VRAM (managed by the PPU)
0xA000-0xBFFF: External RAM (cartridge RAM/RTC, via MBC)
0xC000-0xDFFF: Work RAM (WRAM)
0xE000-0xFDFF: Echo RAM (mirror of 0xC000-0xDDFF)
0xFE00-0xFE9F: OAM (Object Attribute Memory, managed by the PPU)
0xFEA0-0xFEFF: Unusable (reads return 0xFF, writes ignored)
0xFF00       : Joypad register (special handling)
0xFF04-0xFF07: Timer registers (delegated to the timer component)
0xFF0F       : Interrupt Flag (IF)
0xFF10-0xFF26: Audio registers (delegated to the APU)
0xFF30-0xFF3F: Wave RAM (delegated to the APU)
0xFF40-0xFF6B: PPU / LCD registers (except 0xFF46 and 0xFF55)
0xFF46       : OAM DMA transfer trigger
0xFF50       : Boot ROM disable latch
0xFF55       : HDMA5 (CGB HBlank/General-purpose DMA)
0xFF00-0xFF7F: Remaining I/O registers (default storage)
0xFF80-0xFFFE: High RAM (HRAM)
0xFFFF       : Interrupt Enable (IE)
```
The boot ROM, when enabled, overlays part of this map at the very start of the address space (see *Boot ROM Handling*).

## Architecture

### Two hardware models, one trait
The emulator supports both the original Game Boy (DMG) and the Game Boy Color (CGB). Rather than duplicating the routing logic, both share a single trait:

```rust
pub trait MemoryMapper {
    // ... accessors, routing, and tick methods
}
```

- `DmgMmu<M, T, P>`, DMG implementation.
- `CgbMmu<M, T, P>`, CGB implementation, which additionally implements HDMA/GDMA and a colour-aware PPU tick.

Both are generic over three type parameters:
- `M: Mbc`, the cartridge / memory bank controller.
- `T: TimingComponent`, the timer implementation (`DmgTimers` / `CgbTimer`).
- `P: PixelProcessor`, the PPU implementation (`DmgPpu` / `CgbPpu`).

This lets the same MMU code be reused with different concrete components, and keeps hardware-specific behaviour (boot ROM layout, HDMA, colour PPU) in the appropriate implementation.

Most of the routing (`read_byte`, `write_byte`, the joypad logic, `tick_timers`, `tick_apu`, `update_keys`, the interrupt helpers) lives as **default methods on the trait**, so both models inherit it. The two implementations only provide the per-model accessors and override the methods that genuinely differ (`tick_ppu`, `tick_dma`, `new`, HDMA handling, `addr_is_in_boot_rom`).

### HardwareKind
```rust
pub enum HardwareKind {
    Dmg,
    Cgb,
}
```
Returned by `hardware_kind()`. Used elsewhere in the emulator to branch on the console model at runtime without downcasting.

### MemoryRegion Enum
```rust
pub enum MemoryRegion {
    Mbc,             // 0x0000-0x7FFF: read-only ROM (+ RAM via MBC)
    Vram,            // 0x8000-0x9FFF
    ERam,            // 0xA000-0xBFFF
    Wram,            // 0xC000-0xDFFF
    Mram,            // 0xE000-0xFDFF: mirror of 0xC000-0xDDFF
    Oam,             // 0xFE00-0xFE9F
    Unusable,        // 0xFEA0-0xFEFF
    InterruptFlag,   // 0xFF0F
    Timers,          // 0xFF04-0xFF07
    Audio,           // 0xFF10-0xFF26
    WaveRam,         // 0xFF30-0xFF3F
    Io,              // 0xFF00-0xFF7F (catch-all for I/O)
    HRam,            // 0xFF80-0xFFFE
    InterruptEnable, // 0xFFFF
}
```
**Purpose**: Type-safe classification of an address into a region.

**Design choice**: `InterruptFlag`, `Timers`, `Audio`, and `WaveRam` are technically part of the I/O range, but are given their own variants so routing can delegate them directly to the owning component.

**Derivations**: `PartialEq, Eq` (comparison in tests) and `Debug` (printing).

### MemoryRegion::from()
Converts a 16-bit address to its region using pattern matching with inclusive ranges. **Pattern order matters**, specific patterns must precede the general `Io` range, otherwise addresses such as 0xFF0F, 0xFF06, 0xFF10 or 0xFF30 would be swallowed by `Io` and never reach their dedicated variant.

The order is: the memory blocks (0x0000-0xFEFF), then `Timers` (0xFF04-0xFF07), `InterruptFlag` (0xFF0F), `Audio` (0xFF10-0xFF26), `WaveRam` (0xFF30-0xFF3F), then the catch-all `Io` (0xFF00-0xFF7F), then `HRam` and `InterruptEnable`. The union of the ranges covers the entire `u16` space, so the match is exhaustive with no wildcard arm.

**Examples**:
- 0x8123 → `Vram`
- 0xFF06 → `Timers` (not `Io`)
- 0xFF15 → `Audio`
- 0xFF31 → `WaveRam`
- 0xFF41 → `Io` (routed to the PPU inside `read_byte`/`write_byte`, see below)

### MemoryRegion::to_address()
Returns the starting address of a region. Used to avoid magic numbers, for instance `tick_timers` writes to `MemoryRegion::InterruptFlag.to_address()` instead of a hard-coded `0xFF0F`.

## MMU Structures

### DmgMmu
```rust
pub struct DmgMmu<M: Mbc, T: TimingComponent, P: PixelProcessor> {
    data: Vec<u8>,            // full 0x10000 address image, initialised to 0xFF
    cart: M,                  // MBC (ROM + external RAM/RTC)
    interrupts: InterruptController,
    timers: T,
    apu: Apu,
    pub ppu: P,
    boot_enable: bool,        // is the boot ROM currently mapped?
    boot_rom: Vec<u8>,
    dpad_state: u8,           // joypad: direction keys (active-low)
    button_state: u8,         // joypad: action buttons (active-low)
    dma_source: u16,          // OAM DMA source base address
    pub dma_index: u8,        // OAM DMA progress (0xFF = idle)
    dma_last_byte: u8,        // last value written to 0xFF46
    dma_delay: u8,
}
```

### CgbMmu
The CGB struct has the same core fields, minus `dma_delay`, plus the HDMA state machine:
```rust
    hdma_source: u16,
    hdma_dest: u16,
    hdma_length: u16,
    hdma_active: bool,
    hdma_mode: HdmaMode,       // Gdma | Hblank
    was_hblank: bool,          // edge detection for HBlank DMA
    hdma_blocks_remaining: u16,
    hdma_pending_this_period: bool,
```

**Field notes**
- `data: Vec<u8>`: a 64KB image used as backing storage for the regions the MMU still owns directly (WRAM, HRAM, most I/O registers, echo RAM target). VRAM and OAM are **not** stored here anymore, they live in the PPU. ROM/external RAM live in the MBC. It is initialised to `0xFF` (matching the power-on read value of unmapped I/O).
- `cart: M`: the MBC. Handles 0x0000-0x7FFF (ROM + bank switching) and 0xA000-0xBFFF (external RAM / RTC).
- `interrupts`: manages IE (0xFFFF) and IF (0xFF0F).
- `timers: T`: DIV/TIMA/TMA/TAC (0xFF04-0xFF07).
- `apu: Apu`: sound channels and wave RAM (0xFF10-0xFF26, 0xFF30-0xFF3F).
- `ppu: P`: VRAM, OAM, and LCD registers.
- `boot_enable` / `boot_rom`: boot ROM overlay (see below).
- `dpad_state` / `button_state`: current joypad input, stored active-low (0 = pressed).
- `dma_source` / `dma_index` / `dma_last_byte`: OAM DMA state.
- HDMA fields (CGB only): HBlank / general-purpose VRAM DMA state.

All structs derive `Serialize`/`Deserialize` to support save states.

## Construction (`new`)
```rust
fn new(
    wrapped_boot_rom: Option<[u8; 0x900]>,
    rom_data: Vec<u8>,
    ram_data: Option<Vec<u8>>,
    rom_compatibility: bool,
) -> Result<Self, String>
```
1. `boot_enable` is set to `true` iff a boot ROM was supplied. When present, it is copied into `boot_rom`; otherwise `boot_rom` is filled with `0xFF`.
2. The 64KB `data` image is allocated and filled with `0xFF`.
3. The MBC is built from `rom_data` (+ optional `ram_data` for battery-backed saves). Construction can fail (e.g. an inconsistent cartridge header), which is why `new` returns `Result`.
4. Interrupt controller, timer, APU, and PPU are created. `rom_compatibility` is forwarded to the PPU (DMG-on-CGB compatibility handling).
5. Joypad state defaults to `0x0F` (nothing pressed), and the OAM DMA index defaults to `0xFF` (idle).

`DmgMmu` also implements `Default`, which builds an MMU with no boot ROM and an empty ROM, used by tests.

## Boot ROM Handling
When `boot_enable` is true, `read_byte` returns bytes from `boot_rom` for addresses that fall inside the boot ROM region **before** consulting the normal memory map:

```rust
if self.get_boot_enable() && Self::addr_is_in_boot_rom(addr) {
    return self.get_boot_rom()[addr as usize];
}
```

The overlaid region differs by model (`addr_is_in_boot_rom`):
- **DMG**: 0x0000-0x00FF.
- **CGB**: 0x0000-0x00FF **and** 0x0200-0x08FF. The gap at 0x0100-0x01FF is where the cartridge header remains visible during boot.

The boot ROM is disabled by writing a non-zero value to **0xFF50**. `write_byte` intercepts this first, records the write, clears `boot_enable`, and returns, from that point on the cartridge ROM is visible at 0x0000 and the overlay never fires again.

## read_byte(), Read Router
```rust
fn read_byte(&mut self, addr: u16) -> u8
```
Note it now takes `&mut self`: some reads reach through to components whose read path needs mutable access (PPU/APU internals, HDMA5 status). Routing, after the boot-ROM check:

- **`Mbc` | `ERam`** → `cart.read(addr)`. The same call handles both ROM (0x0000-0x7FFF) and external RAM (0xA000-0xBFFF); the MBC decides internally.
- **`Vram`** → `ppu.read_vram(addr)`.
- **`Mram`** (Echo RAM) → reads `data[addr - 0x2000]`, mirroring WRAM.
- **`Timers`** → `read_timers(addr)` → timer component.
- **`Audio`** / **`WaveRam`** → `apu.read(addr)`.
- **`Io`**, sub-dispatched:
  - **0xFF00 (Joypad)**: builds the register value from the current selection bits and the active input state (see *Joypad Register*).
  - **0xFF40-0xFF6B, except 0xFF46 and 0xFF55**: `ppu.read_register(addr)` (LCDC, STAT, scroll, palettes, CGB colour registers, etc.).
  - **0xFF55**: `handle_read_hdma5()` (CGB HDMA status; DMG returns the default).
  - anything else in the I/O range → raw `data[addr]`.
- **`Oam`** → `ppu.read_oam(addr)`.
- **`Unusable`** → `0xFF` (hardware-accurate).
- **`InterruptFlag`** / **`InterruptEnable`** → interrupt controller, which applies the correct bit masking.
- **default** (`Wram`, `HRam`, remaining I/O) → raw `data[addr]`.

## write_byte(), Write Router
```rust
fn write_byte(&mut self, addr: u16, val: u8)
```
First it intercepts the boot-ROM disable latch (0xFF50, non-zero). Then it routes by region:

- **`Mbc` | `ERam`** → `cart.write(addr, val)`. Writes to ROM addresses are interpreted by the MBC as bank/RAM/mode control; writes to 0xA000-0xBFFF go to external RAM (or the RTC on MBC3).
- **`Vram`** → `ppu.write_vram(addr, val)`.
- **`Mram`** (Mirror RAM or Echo RAM) → writes to `data[addr - 0x2000]`, so echo writes land in WRAM.
- **`Timers`** → timer component.
- **`Audio`** / **`WaveRam`** → `apu.write(addr, val)`.
- **`Io`**, sub-dispatched:
  - **0xFF00 (Joypad)**: stores the two selection bits and re-derives the register (see below), which may raise a Joypad interrupt.
  - **0xFF40-0xFF6B, except 0xFF46 and 0xFF55**: `ppu.write_register(addr, val)`.
  - **0xFF46 (OAM DMA)**: records the value, resets `dma_index` to 0, and sets `dma_source = (val as u16) << 8`, arming an OAM DMA transfer.
  - **0xFF55 (HDMA5)**: `handle_write_hdma5(val)` (CGB only; no-op default on DMG).
  - anything else → raw `data[addr]`.
- **`Oam`** → `ppu.write_oam(addr, val)`.
- **`Unusable`** → ignored.
- **`InterruptFlag`** / **`InterruptEnable`** → interrupt controller (masking applied).
- **default** → raw `data[addr]`.

## Joypad Register (0xFF00)
The Game Boy multiplexes eight inputs onto one register using two selection bits, and reports inputs **active-low** (0 = pressed).

- Bit 5 (`0b0010_0000`): when **clear**, action buttons (A, B, Select, Start) are selected.
- Bit 4 (`0b0001_0000`): when **clear**, the direction pad is selected.
- The low nibble reflects the selected group; upper bits 6-7 read back as 1.

**On read (0xFF00)**: the MMU starts from `0x0F`, ANDs in `dpad_state` and/or `button_state` depending on which group(s) are selected, and returns `0b1100_0000 | selection | result`.

**On write (0xFF00)**: only the selection bits are writable. The MMU stores them, keeps the current input nibble, and calls `update_joypad_register()` to refresh the low nibble.

**Joypad interrupt**: `update_joypad_register()` re-computes the selected inputs and, if any selected line transitions from high to low (a key becoming pressed), requests `Interrupt::Joypad`. `update_keys(dpad, buttons)` is the entry point the front-end calls when physical input changes; it updates both state bytes and re-runs the joypad refresh.

## OAM DMA (0xFF46)
Writing to 0xFF46 starts a 160-byte transfer from `XX00-XX9F` (where `XX` is the written value) into OAM (0xFE00-0xFE9F). This emulator performs the copy incrementally, one byte per call to `tick_dma()`:

```rust
fn tick_dma(&mut self) {
    let byte = self.read_byte(self.dma_source + self.dma_index as u16);
    let dma_index = self.dma_index;
    self.get_ppu().write_oam(0xFE00 + dma_index as u16, byte);
    self.dma_index += 1;
    if self.dma_index == 160 {
        self.dma_index = 0xFF; // mark transfer complete / idle
    }
}
```
`dma_index == 0xFF` denotes "no transfer in progress". The scheduler is expected to call `tick_dma` on the appropriate cycles while a transfer is active.

## HDMA / GDMA (CGB only, 0xFF55)
The Game Boy Color adds a VRAM DMA engine with two modes, selected by bit 7 of the value written to HDMA5:

- **General-purpose DMA (GDMA)**, bit 7 clear: the whole block is copied immediately.
- **HBlank DMA**, bit 7 set: 0x10 bytes are copied during each HBlank period until the transfer completes.

`handle_write_hdma5(val)`:
- If bit 7 is clear **and** an HBlank transfer is already active, this **terminates** it and returns.
- Otherwise it latches the source (from HDMA1/HDMA2, masked to `0xFFF0`) and destination (from HDMA3/HDMA4, masked into 0x8000-0x9FF0), computes `blocks = (val & 0x7F) + 1` and `length = blocks * 0x10`, and:
  - **GDMA**: copies `length` bytes right away (reading from the cartridge or the `data` image depending on the source address) into VRAM via `ppu.write_hdma_value`. Interrupts are temporarily masked (IE/IF saved and restored) around the burst.
  - **HBlank**: marks the transfer active and sets `hdma_mode = Hblank`; the copy is then driven from `tick_ppu`.

`handle_read_hdma5()` reports transfer status: while active it returns `blocks_remaining - 1` in the low 7 bits; when idle it returns `0xFF` (bit 7 set = "no active transfer").

The HBlank pump lives in the CGB `tick_ppu`: it detects the rising edge into HBlank (`was_hblank`), and, when a transfer is active, in HBlank mode, and the CPU is not halted, copies one 0x10-byte block, advances source/destination, decrements `hdma_blocks_remaining`, and clears `hdma_active` when the last block is done.

## Component Ticks

### tick_timers()
Advances the timer by one M-cycle. If `timer.tick()` reports an overflow, the MMU sets bit 2 of IF (the Timer interrupt) by reading, OR-ing `0b100`, and writing back to `MemoryRegion::InterruptFlag.to_address()`.

### tick_ppu()
Advances the PPU by one step and converts PPU-side pending flags into interrupt requests: `pending_vblank()` → `Interrupt::VBlank`, `pending_stat()` → `Interrupt::LcdStat` (each flag is cleared once consumed). The method returns the resulting `PpuMode`.
- The **DMG** version does only this.
- The **CGB** version additionally runs the HBlank-DMA pump described above before ticking the PPU.

### tick_apu()
Calls `apu.step()` to advance sound generation.

## Interrupt Helpers
Thin wrappers so the CPU never touches the `InterruptController` directly:
```rust
read_interrupt_enable() / read_interrupt_flag()
interrupts_next_request() -> Option<Interrupt>
interrupts_clear_request(interrupt)
interrupts_request(interrupt)
```
Typical CPU usage: after each instruction, poll `interrupts_next_request()`; when servicing, call `interrupts_clear_request(interrupt)`; when a component signals an event, call `interrupts_request(interrupt)`.

## Save States
Every MMU struct (and its sub-components) derives `serde::{Serialize, Deserialize}`, so the whole memory/hardware state can be serialised and restored as a save state.

## Implementation Status
### Completed
- ✅ Trait-based design shared by DMG and CGB (`MemoryMapper`)
- ✅ Generic over MBC / timer / PPU implementations
- ✅ Region identification and routing (with `Audio` / `WaveRam` split out)
- ✅ ROM / external RAM access via the MBC
- ✅ VRAM and OAM delegated to the PPU
- ✅ Echo RAM mirroring (0xE000-0xFDFF ↔ 0xC000-0xDDFF)
- ✅ Unusable region behaviour (reads 0xFF, writes ignored)
- ✅ Interrupt register access (IE, IF) and interrupt-request API
- ✅ Timer integration and Timer-interrupt raising
- ✅ APU register + Wave RAM routing
- ✅ Joypad register (0xFF00) with selection bits and Joypad interrupt
- ✅ Boot ROM overlay (DMG + CGB layouts) and 0xFF50 disable latch
- ✅ OAM DMA (0xFF46) via incremental `tick_dma`
- ✅ CGB HDMA/GDMA (0xFF55): general-purpose and HBlank modes
- ✅ Save-state (serde) support

### Missing / TODO
- ❌ Cycle-accurate OAM DMA timing and CPU bus conflicts during transfer
- ❌ PPU memory access restrictions (VRAM/OAM blocked in certain PPU modes)
- ❌ Full I/O register coverage / accurate power-on values for unimplemented registers
- ❌ Serial transfer registers (0xFF01-0xFF02)
- ❌ CGB double-speed and its effect on timer/DMA pacing

---
*Documentation is separated by component; the APU, MBC, timers, interrupts, and PPU each have (or will have) their own document. This file covers memory routing only.*

---
### Last Modification
This document was last updated **2026/07/04**

based on state at commit `653854a1d657d2c499f7f261228352ead4feceff`

Some changes may have been made to the code since then.