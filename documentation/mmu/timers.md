# Timers Implementation

## Overview
The Game Boy timer system provides hardware-based timing for games. It consists of a constantly incrementing divider (DIV) and a configurable counter (TIMA) that can trigger interrupts at several frequencies. This drives game timing, animation, music tempo, sound effects, and any time-based mechanic.

## Timer Registers
Four hardware registers, mapped at 0xFF04-0xFF07 and routed to this component by the MMU.

### DIV (0xFF04), Divider Register
- **Read**: returns the upper 8 bits of an internal 16-bit counter.
- **Write**: any write resets the entire 16-bit counter to 0 (the written value is ignored).
- **Frequency**: the internal counter increments every M-cycle; the exposed upper byte therefore increments at 16384 Hz.
- **Use cases**: RNG seeding, basic timing, frame pacing.

### TIMA (0xFF05), Timer Counter
- **Read/Write**: 8-bit counter incrementing at a configurable frequency.
- **Behaviour**: on overflow (0xFF → 0x00) it is reloaded with TMA and a Timer interrupt is requested.
- **Use cases**: precise timing events, music tempo, synchronised events.

### TMA (0xFF06), Timer Modulo
- **Read/Write**: 8-bit reload value for TIMA.
- **Behaviour**: when TIMA overflows it is set to TMA (not to 0).

### TAC (0xFF07), Timer Control
- **Bit 2**: Timer Enable (1 = enabled).
- **Bits 0-1**: Clock Select (frequency).
    - `00`: 4096 Hz (CPU clock / 1024)
    - `01`: 262144 Hz (CPU clock / 16)
    - `10`: 65536 Hz (CPU clock / 64)
    - `11`: 16384 Hz (CPU clock / 256)
- **Bits 3-7**: unused.

## Frequency Selection
The timer frequencies are derived by watching a specific bit of the internal 16-bit DIV counter. TIMA increments on the **falling edge** (1 → 0) of the selected bit while the timer is enabled, this is why the implementation tracks `previous_and_result`.

| TAC bits 0-1 | Frequency | Cycles per TIMA increment | DIV bit watched |
|--------------|-----------|---------------------------|-----------------|
| 00 | 4096 Hz | 1024 | Bit 9 |
| 01 | 262144 Hz | 16 | Bit 3 |
| 10 | 65536 Hz | 64 | Bit 5 |
| 11 | 16384 Hz | 256 | Bit 7 |

**Example** (TAC = 0b101 → enabled, frequency `01`): DIV bit 3 is watched; every 16 M-cycles it falls from 1 to 0, incrementing TIMA at 262144 Hz.

## Overflow and Interrupts
When TIMA overflows:
1. TIMA is reloaded with TMA.
2. `tick()` returns `true`, signalling the MMU to raise the Timer interrupt (IF bit 2). The MMU wrapper (`tick_timers`) is responsible for setting IF; the timer itself only reports the overflow.

TMA controls the interrupt period: TMA = 0x00 → interrupt every 256 increments; TMA = 0xFF → every increment; TMA = 0xF0 → every 16.

## Architecture

### TimingComponent trait
```rust
pub trait TimingComponent {
    fn new() -> Self where Self: Sized;

    fn div(&self) -> u16;
    fn set_div(&mut self, value: u8);
    fn inc_div(&mut self);

    fn tac(&self) -> u8;      fn set_tac(&mut self, value: u8);
    fn tma(&self) -> u8;      fn set_tma(&mut self, value: u8);
    fn tima(&self) -> u8;     fn set_tima(&mut self, value: u8);
    fn inc_tima(&mut self);

    fn previous_and_result(&self) -> bool;
    fn set_next_and_result(&mut self, and_result: bool);

    // default methods:
    fn and_result(&self) -> bool { /* ... */ }
    fn tick(&mut self) -> bool   { /* ... */ }
    fn write(&mut self, addr: u16, value: u8) { /* ... */ }
    fn read(&self, addr: u16) -> u8           { /* ... */ }
}
```
The trait splits into two layers:
- **Per-implementation accessors** (`div`, `set_div`, `inc_tima`, …): trivial field getters/setters each concrete timer must provide.
- **Shared logic as default methods** (`and_result`, `tick`, `write`, `read`): the actual timer behaviour, written once in terms of the accessors and inherited by every implementation.

### Concrete implementations
```rust
pub struct DmgTimers {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    previous_and_result: bool,
}

pub struct CgbTimer { /* identical fields */ }
```
Both derive `Debug, Default, Serialize, Deserialize` and implement `TimingComponent` with straightforward accessors.

> **Current status of `CgbTimer`**: it is presently identical to `DmgTimers`. The trait exists so CGB-specific timing (notably double-speed mode, which changes how fast the internal counter runs) can be introduced later without touching the DMG path. Until that logic is added, the two behave the same.

### Fields
- `div: u16`, internal 16-bit divider. Increments every M-cycle. Only the upper 8 bits are exposed via the DIV register (0xFF04); the lower 8 bits drive falling-edge detection, which is why a `u16` is needed even though the visible register is 8-bit.
- `tima: u8`, timer counter (TIMA), increments at the TAC-selected frequency, triggers an interrupt on overflow.
- `tma: u8`, reload value applied to TIMA on overflow.
- `tac: u8`, control register (bit 2 enable, bits 0-1 frequency).
- `previous_and_result: bool`, the previous cycle's `(selected DIV bit) AND (timer enabled)` value, used to detect the true→false transition (falling edge) that increments TIMA.

### Address constants
```rust
const DIV_ADDR: u16  = 0xFF04;
const TIMA_ADDR: u16 = 0xFF05;
const TMA_ADDR: u16  = 0xFF06;
const TAC_ADDR: u16  = 0xFF07;
```
Used to avoid magic numbers in `read`/`write`.

## Core Logic

### and_result()
```rust
fn and_result(&self) -> bool {
    let enabled = (self.tac() & 0b100) > 0;
    let mask = 0b1 << match self.tac() & 0b11 {
        0b00 => 9,
        0b01 => 3,
        0b10 => 5,
        0b11 => 7,
        _ => unreachable!(),
    };
    let kept_bit = (self.div() & mask) > 0;
    kept_bit && enabled
}
```
Computes the current value of "selected DIV bit is set **and** timer is enabled":
1. `enabled` isolates TAC bit 2.
2. `mask` maps the two frequency bits to the DIV bit position (9/3/5/7) and shifts a 1 into place.
3. `kept_bit` reads that bit out of DIV.
4. The result feeds falling-edge detection in `tick`.

`unreachable!()` is safe here because `self.tac() & 0b11` can only yield 0-3.

### tick()
```rust
fn tick(&mut self) -> bool {
    self.inc_div();
    let mut overflowed = false;
    let and_result = self.and_result();
    if self.previous_and_result() && !and_result {
        self.inc_tima();
        if self.tima() == 0 {
            self.set_tima(self.tma());
            overflowed = true
        }
    }
    self.set_next_and_result(and_result);
    overflowed
}
```
Advances the timer by one M-cycle. Called once per M-cycle by the MMU. Returns `true` if TIMA overflowed.

**Step by step**
1. **Increment DIV** (`inc_div` → `wrapping_add(1)`): happens every cycle, regardless of the enable bit.
2. **Compute `and_result`** via `and_result()`.
3. **Falling-edge check** `previous_and_result() && !and_result`: true only when the condition was met last cycle and is not met this cycle. This fires both when the watched DIV bit falls 1→0 and when the timer is disabled while that bit was set, matching hardware.
4. **On a falling edge**: `inc_tima()` (wrapping). If TIMA is now 0 it wrapped from 0xFF, so reload it with TMA and flag the overflow.
5. **Store state**: save `and_result` for next cycle's edge detection and return the overflow flag.

**Overflow example**: TIMA = 0xFF, TMA = 0x53, falling edge → `inc_tima` makes TIMA 0x00 → reloaded to 0x53, returns `true`.
**No overflow**: TIMA = 0x05, falling edge → TIMA = 0x06, returns `false`.

### write()
```rust
fn write(&mut self, addr: u16, value: u8) {
    match addr {
        DIV_ADDR  => self.set_div(0),      // any write resets DIV
        TIMA_ADDR => self.set_tima(value),
        TMA_ADDR  => self.set_tma(value),
        TAC_ADDR  => self.set_tac(value),
        _ => unreachable!(),
    }
}
```
- **DIV (0xFF04)**: any write resets the internal counter to 0 (value ignored), games use this to resynchronise timing.
- **TIMA / TMA / TAC**: written directly. TAC changes to enable state and frequency take effect immediately.
- `unreachable!()`: the MMU only routes 0xFF04-0xFF07 here, so any other address is a programming error.

### read()
```rust
fn read(&self, addr: u16) -> u8 {
    match addr {
        DIV_ADDR  => (self.div() >> 8) as u8, // upper byte only
        TIMA_ADDR => self.tima(),
        TMA_ADDR  => self.tma(),
        TAC_ADDR  => self.tac(),
        _ => unreachable!(),
    }
}
```
DIV exposes only the upper 8 bits of the 16-bit internal counter (`div >> 8`); the lower byte is internal and used for edge detection. The other registers read back directly.

## Integration with the MMU
The MMU owns the timer as its generic `T: TimingComponent` and mediates all interaction:
- `write_byte` on 0xFF04-0xFF07 → `timer.write(addr, val)`.
- `read_byte` on 0xFF04-0xFF07 → `timer.read(addr)`.
- Once per M-cycle the MMU calls `timer.tick()`; if it returns `true`, the MMU sets IF bit 2 (Timer interrupt). The timer never touches the interrupt registers itself, it only reports overflow.

## Save States
`DmgTimers` and `CgbTimer` derive `serde::{Serialize, Deserialize}`, so the full timer state (including `div` and `previous_and_result`) is captured in save states.

## Implementation Status
### Completed
- ✅ Trait-based design (`TimingComponent`) shared by DMG and CGB
- ✅ DIV increments every cycle; DIV read returns the upper 8 bits
- ✅ DIV reset on write
- ✅ TIMA increments at all four selectable frequencies
- ✅ Falling-edge detection for TIMA increment (incl. disable-while-set edge)
- ✅ TIMA overflow detection and TMA reload
- ✅ Overflow reported to the MMU to raise the Timer interrupt
- ✅ Save-state (serde) support

### Known deviations / TODO
- ❌ **CGB-specific timing**: `CgbTimer` currently mirrors `DmgTimers`; double-speed mode is not yet modelled.
- ❌ **Overflow timing quirk**: real hardware delays the TMA reload and interrupt by one M-cycle after TIMA overflows, during which TIMA reads 0x00 and a write to TIMA/TMA in that window has special behaviour. This implementation reloads TMA and reports the overflow on the same cycle.
- ❌ **TIMA write during reload** and other edge-case register-write timings are not modelled.
- ❌ **DIV-write glitch**: writing to DIV can cause a spurious TIMA increment on hardware if the watched bit was high; not modelled here.

---

### Last Modification
This document was last updated **2026/07/04**

based on state at commit `653854a1d657d2c499f7f261228352ead4feceff`

Some changes may have been made to the code since then.