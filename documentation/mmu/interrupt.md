# Interrupt System Implementation

## Overview
Interrupts let hardware components signal the CPU that an important event has occurred. Instead of constantly polling for events and wasting cycles, a component can flag an interrupt, and the CPU services it between instructions. This module owns the two interrupt registers and decides which pending interrupt has priority. The actual servicing (saving PC, jumping to the vector, toggling IME) lives in the CPU, which drives this controller through a small API.

## The Five Interrupt Types
Listed in priority order, highest first:
1. **V-Blank (bit 0)**: raised when the PPU finishes drawing a frame (scanline 144). The safe window to touch VRAM.
2. **LCD STAT (bit 1)**: raised by various PPU conditions (mode changes, scanline coincidence).
3. **Timer (bit 2)**: raised when TIMA overflows.
4. **Serial (bit 3)**: raised when a serial transfer completes.
5. **Joypad (bit 4)**: raised when a button is pressed.

## Hardware Registers
Two registers drive the system, both using the same bit layout:
- **IE (Interrupt Enable, 0xFFFF)**: each bit enables one interrupt type.
- **IF (Interrupt Flag, 0xFF0F)**: each bit marks one interrupt as requested.

```
Bit 4: Joypad
Bit 3: Serial
Bit 2: Timer
Bit 1: LCD STAT
Bit 0: V-Blank
Bits 5-7: unused (read back as 1 on IF, as 0 on IE)
```

## Vector Addresses
When an interrupt is serviced, the CPU jumps to a fixed address:
- V-Blank: 0x0040
- LCD STAT: 0x0048
- Timer: 0x0050
- Serial: 0x0058
- Joypad: 0x0060

## Servicing Flow
This controller covers steps 1, 2, and part of 3 below. The rest is CPU work.
1. **Request**: a component sets its IF bit (via `request`).
2. **Select**: the CPU asks the controller for the highest-priority interrupt that is both enabled and requested (via `next_request`).
3. **Handle**: if IME is on and an interrupt is selected, the CPU pushes PC, clears IME, clears the IF bit (via `clear_request`), and jumps to the vector.
4. **Execute**: game code runs the handler.
5. **Return**: RETI restores PC and re-enables IME.

Note that IME (the Interrupt Master Enable flag) is held by the CPU, not by this controller. The controller only tracks IE and IF and resolves priority.

## Interrupt Enum
```rust
#[repr(u8)]
pub enum Interrupt {
    VBlank  = 0b00000001,
    LcdStat = 0b00000010,
    Timer   = 0b00000100,
    Serial  = 0b00001000,
    Joypad  = 0b00010000,
}
```
A type-safe representation of the five interrupt types.

Design choices:
- `#[repr(u8)]` forces a `u8` layout so the enum can be cast straight to its bit value.
- Each variant is a single set bit, matching its position in IE and IF. `Interrupt::Timer as u8` yields `0b00000100`, ready for bitwise use.
- Derives `Copy, Clone` (cheap to pass around) and `Debug, PartialEq` (printing and comparison in tests).

### vector()
```rust
pub fn vector(self) -> u16 {
    match self {
        Interrupt::VBlank  => 0x40,
        Interrupt::LcdStat => 0x48,
        Interrupt::Timer   => 0x50,
        Interrupt::Serial  => 0x58,
        Interrupt::Joypad  => 0x60,
    }
}
```
Returns the address the CPU jumps to when servicing this interrupt. These addresses are fixed by the hardware.

## InterruptController Structure
```rust
pub struct InterruptController {
    ienable: u8, // IE register (0xFFFF)
    iflag: u8,   // IF register (0xFF0F)
}
```
- `ienable`: the IE register, one enable bit per interrupt type.
- `iflag`: the IF register, one request bit per interrupt type.

Only the lower 5 bits of each are meaningful. The struct derives `Debug, Clone, Serialize, Deserialize`, so interrupt state is captured in save states.

`new()` starts both registers at 0 (nothing enabled, nothing pending).

## Register Access
### IE (0xFFFF)
```rust
pub fn read_interrupt_enable(&self) -> u8 {
    self.ienable & 0b00011111
}
pub fn write_interrupt_enable(&mut self, val: u8) {
    self.ienable = val & 0b00011111;
}
```
Read masks the upper 3 bits to 0, write keeps only the lower 5 bits. The mask `0x1F` reflects that only 5 interrupt types exist.

### IF (0xFF0F)
```rust
pub fn read_interrupt_flag(&self) -> u8 {
    self.iflag | 0b11100000
}
pub fn write_interrupt_flag(&mut self, val: u8) {
    self.iflag = val & 0b00011111;
}
```
Read forces the upper 3 bits to 1, matching hardware, where reading IF always returns 1 in bits 5 to 7. Write keeps only the lower 5 bits, letting games clear pending requests by writing IF directly.

## Requesting and Clearing
```rust
pub fn request(&mut self, interrupt: Interrupt) {
    self.iflag |= interrupt as u8;
}
```
Sets the interrupt's bit in IF. Multiple interrupts can be pending at once. If IF is `0b00000001` (V-Blank) and Timer is requested, IF becomes `0b00000101` (both pending).

```rust
pub fn clear_request(&mut self, interrupt: Interrupt) {
    self.iflag &= !(interrupt as u8);
}
```
Clears only that interrupt's bit, leaving other pending requests intact. Clearing Timer from `0b00000101` leaves `0b00000001` (V-Blank still pending).

## Selecting the Next Interrupt
```rust
pub fn next_request(&self) -> Option<Interrupt> {
    let pending_request = self.ienable & self.iflag;

    [
        Interrupt::VBlank,
        Interrupt::LcdStat,
        Interrupt::Timer,
        Interrupt::Serial,
        Interrupt::Joypad,
    ]
    .iter()
    .find(|&&interrupt| pending_request & (interrupt as u8) != 0)
    .copied()
}
```
Two steps:

1. **Compute the actionable set**: `ienable & iflag` keeps only the bits that are both enabled and requested. An interrupt that is requested but not enabled, or enabled but not requested, is ignored.
2. **Pick by priority**: the array is ordered from highest priority (V-Blank) to lowest (Joypad). `find` returns the first interrupt whose bit is set, and `copied` converts `Option<&Interrupt>` into `Option<Interrupt>`. Because iteration stops at the first match, the highest-priority pending interrupt wins automatically.

Returns `Some(interrupt)` when an enabled interrupt is pending, `None` otherwise.

Worked example: `ienable = 0b00010101` (V-Blank, Timer, Joypad enabled), `iflag = 0b00001110` (LCD STAT, Timer, Serial requested). The actionable set is `0b00000100`, so `next_request` returns `Some(Interrupt::Timer)`.

## Integration
- Components raise interrupts through the MMU wrappers, for example the timer overflow path sets the Timer bit, and the PPU sets V-Blank and LCD STAT.
- The CPU polls `next_request` between instructions and, when IME allows, services the result and calls `clear_request`.
- IE and IF are reachable at their normal addresses (0xFFFF and 0xFF0F) through the MMU, which delegates to `read_interrupt_enable` / `write_interrupt_enable` and `read_interrupt_flag` / `write_interrupt_flag`.

## Implementation Status
### Completed
- ✅ All five interrupt types defined with correct bit values
- ✅ Vector addresses for every interrupt
- ✅ IE read/write with correct masking (upper bits cleared)
- ✅ IF read/write with hardware-accurate upper bits (read back as 1)
- ✅ Request and clear individual interrupts
- ✅ Priority-based selection of the next actionable interrupt
- ✅ Save-state (serde) support

---

### Last Modification
This document was last updated on **2026/07/04**, based on the state at commit `653854a1d657d2c499f7f261228352ead4feceff`.

Some changes may have been made to the code since then.
