# OAM (Object Attribute Memory) Implementation

## Overview
OAM is the region at 0xFE00-0xFE9F that stores sprite data (also called objects). It holds 40 sprites, each described by 4 bytes, for a total of 160 bytes. The PPU reads OAM during its scan phase to decide which sprites appear on the current scanline. In this emulator OAM is owned by the PPU and reached through the `ObjectManager` trait, so the MMU routes reads and writes in the 0xFE00-0xFE9F range to it.

## Sprite Layout in Memory
Each sprite occupies 4 consecutive bytes:

| Byte | Meaning |
|------|---------|
| 0 | Y position |
| 1 | X position |
| 2 | Tile index |
| 3 | Attributes / flags |

The attribute byte holds:
- Bit 7: priority (behind background or in front)
- Bit 6: Y flip
- Bit 5: X flip
- Bit 4: palette selection (DMG)
- Bits 3-0: unused on DMG, used for VRAM bank and CGB palette on Game Boy Color

## Sprite Structure
```rust
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile: u8,
    pub oam_index: u8,
    pub attributes: u8,
}
```
The first three fields and `attributes` map directly onto the 4 memory bytes. `oam_index` is bookkeeping used by the PPU, it is not one of the 4 stored bytes and is not exposed through raw reads.

Every field defaults to `0xFF`. A default sprite therefore sits fully off screen with all flags set, which is the desired inactive state.

### is_visible()
```rust
pub fn is_visible(&self, ly: u8, height: u8) -> bool {
    if ly >= 144 {
        return false;
    }
    let sprite_top: i16 = self.y as i16 - 16;
    let sprite_bottom: i16 = sprite_top + height as i16;
    let ly: i16 = ly as i16;
    ly >= sprite_top && ly < sprite_bottom
}
```
Determines whether the sprite intersects scanline `ly`, given the current sprite height (8 in 8x8 mode, 16 in 8x16 mode).

Key points:
- The stored Y value is offset by 16, so the real top edge is `y - 16`. A sprite with `y = 16` starts at screen line 0, and a sprite with `y < 16` is partially or fully above the screen.
- The visible span is `[top, bottom)`, that is, inclusive of the top line and exclusive of the bottom line.
- Any `ly` of 144 or more returns false, since the visible screen is lines 0 to 143.
- Signed arithmetic (`i16`) is used so sprites straddling the top edge (negative top) are handled correctly.

## Oam Structure
```rust
pub struct Oam {
    pub sprites: Vec<Sprite>,
    accessed_oam_row: u8,
}
```
- `sprites`: 40 sprites, initialised to defaults.
- `accessed_oam_row`: the OAM row currently being touched by the PPU during its scan, or `0xFF` when none. This drives the OAM corruption bug (see below). One row is 8 bytes, meaning 2 sprites.

Both `Sprite` and `Oam` derive `Serialize`/`Deserialize` so sprite state is captured in save states.

## ObjectManager Interface
`Oam` implements the `ObjectManager` trait, the surface the PPU and MMU use:

```rust
fn read(&mut self, addr: u16) -> u8 {
    if self.accessed_oam_row != 0xFF {
        self.trigger_oam_bug_read(self.accessed_oam_row);
    }
    self.read_raw((addr.wrapping_sub(OAM_BEGINNING)) as u8)
}
fn write(&mut self, addr: u16, val: u8) {
    self.write_raw((addr.wrapping_sub(OAM_BEGINNING)) as u8, val);
}
```
- `read` first applies the read corruption bug if a row is currently being accessed by the PPU, then converts the absolute address to a 0-159 offset (`addr - 0xFE00`) and reads the byte.
- `write` converts the address the same way and stores the byte.
- `set_accessed_oam_row`, `update_accessed_oam_row` (adds to the current value), and `accessed_oam_row` manage the state that gates the corruption bug.
- `sprite(index)` returns a mutable reference to a sprite, letting the PPU update bookkeeping such as `oam_index` directly.

## Raw Byte Access
```rust
pub fn read_raw(&self, offset: u8) -> u8 {
    let sprite = (offset / 4) as usize;
    let byte = (offset % 4) as usize;
    match byte {
        0 => self.sprites[sprite].y,
        1 => self.sprites[sprite].x,
        2 => self.sprites[sprite].tile,
        3 => self.sprites[sprite].attributes,
        _ => 0,
    }
}
```
The offset (0-159) is split into a sprite index (`offset / 4`) and a byte within that sprite (`offset % 4`). `write_raw` mirrors this mapping. Word helpers `read_word_raw` and `write_word_raw` combine or split two consecutive bytes in big-endian order (`(byte_1 << 8) | byte_2`), which the corruption routines rely on since the hardware glitch operates on 16-bit words.

## OAM Corruption Bug
On real DMG hardware, certain 16-bit address-register operations that land inside OAM while the PPU is scanning it (mode 2) corrupt whole rows of OAM. A row is 8 bytes (2 sprites). This implementation reproduces the documented glitch, working on 16-bit words within the affected row and the row immediately before it.

Three variants are modelled:

### Write corruption
```rust
pub fn trigger_oam_bug_write(&mut self, offset: u8)
```
Rows below offset 8 (the first row) are immune and return early. Otherwise the current row and the previous row are read as four words each. The first word of the current row is replaced by the glitch formula `((cur ^ prev2) & (prev0 ^ prev2)) ^ prev2`, where `prev0` and `prev2` are the first and third words of the previous row. The remaining three words of the row are overwritten with the previous row's words. The result is written back into the current row.

### Read corruption
```rust
pub fn trigger_oam_bug_read(&mut self, offset: u8)
```
Same structure, with a different formula for the first word: `prev0 | (cur & prev2)`. The corrupted words are written into the current row, and the first corrupted word is also written into the previous row, matching hardware.

### Read corruption with increment
```rust
pub fn trigger_oam_bug_read_increase(&mut self, offset: u8)
```
A stronger variant that applies only to offsets in the range 32 to 151 (it does not affect the first four rows or the last row). It involves three rows: the current one, the previous one, and the one before that. The first word of the previous row is recomputed with a formula combining all three, then the rows are written back. Finally it also invokes the plain read corruption. The guard `(32..152).contains(&offset)` encodes exactly which rows are affected.

## Integration
- The MMU classifies 0xFE00-0xFE9F as `MemoryRegion::Oam` and forwards reads to `ppu.read_oam` and writes to `ppu.write_oam`, which reach this component.
- OAM DMA (triggered by writing to 0xFF46) fills OAM by writing one byte at a time into this region.
- The PPU sets `accessed_oam_row` during its scan so that CPU reads at the wrong moment reproduce the corruption bug.

## Implementation Status
### Completed
- ✅ 40 sprites, 4 bytes each, addressed by offset
- ✅ Sprite field mapping for raw byte and word access
- ✅ Sprite visibility test with 8x8 and 8x16 heights, including top-edge straddling
- ✅ Address translation from absolute (0xFE00-0xFE9F) to offset
- ✅ OAM corruption bug: write, read, and read-with-increment variants
- ✅ Save-state (serde) support

### TODO
- ❌ Full OAM access blocking during PPU modes 2 and 3 (beyond the corruption bug itself)
- ❌ Cycle-accurate coupling between the corruption bug and the exact scan timing

### Last Modification
This document was last updated on **2026/07/04**, based on the state at commit `653854a1d657d2c499f7f261228352ead4feceff`.

Some changes may have been made to the code since then.
