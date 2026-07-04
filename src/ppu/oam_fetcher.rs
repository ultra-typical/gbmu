use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

use crate::mmu::oam::Sprite;
use crate::ppu::Cram;
use crate::ppu::colors_palette::{CgbColor, ColorType, DmgColor};
use crate::ppu::obj_piso::ObjPiso;
use crate::ppu::vram::{CgbVram, DmgVram, Vram};

const VRAM_START: u16 = 0x8000;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FetcherState {
    #[default]
    GetTileId = 0,
    GetLowData = 1,
    GetHighData = 2,
    PushPixel = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OamFetcher<V: Vram, C: ColorType> {
    phantom_vram: PhantomData<V>,
    phantom_color: PhantomData<C>,
    fetcher_state: FetcherState,
    tile_id: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    dot_counter: u32,
    actual_sprite_line: usize,
    attributes: u8,
}

impl<V: Vram, C: ColorType> Default for OamFetcher<V, C> {
    fn default() -> Self {
        Self {
            phantom_vram: PhantomData,
            phantom_color: PhantomData,
            fetcher_state: FetcherState::default(),
            tile_id: 0,
            tile_data_low: 0,
            tile_data_high: 0,
            dot_counter: 0,
            actual_sprite_line: 0,
            attributes: 0,
        }
    }
}

impl OamFetcher<DmgVram, DmgColor> {
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        vram: &DmgVram,
        sprite: &Sprite,
        piso: &mut ObjPiso<DmgColor>,
        ly: u8,
        height: u8,
        scanline_x: usize,
        obp0: u8,
        obp1: u8,
        _is_dmg_mode: bool,
    ) -> bool {
        self.dot_counter = self.dot_counter.wrapping_add(1);

        if self.dot_counter.is_multiple_of(2) {
            match self.fetcher_state {
                FetcherState::GetTileId => {
                    self.tile_id = self.get_tile_id(sprite, ly, height);
                    self.fetcher_state = FetcherState::GetLowData;

                    return false;
                }
                FetcherState::GetLowData => {
                    self.tile_data_low = self.get_tile_data_low(vram);
                    self.fetcher_state = FetcherState::GetHighData;

                    return false;
                }
                FetcherState::GetHighData => {
                    self.tile_data_high = self.get_tile_data_high(vram);
                    self.fetcher_state = FetcherState::PushPixel;

                    return false;
                }
                FetcherState::PushPixel => {
                    self.push_pixel(piso, sprite, scanline_x, obp0, obp1);
                    self.fetcher_state = FetcherState::GetTileId;

                    return true;
                }
            }
        }

        false
    }

    fn get_tile_id(&mut self, sprite: &Sprite, ly: u8, height: u8) -> u8 {
        let sprite_top: i16 = sprite.y as i16 - 16;
        let sprite_line = (ly as i16 - sprite_top) as usize;

        let y_flip = ((sprite.attributes >> 6) & 1) != 0;
        let actual_sprite_line = if y_flip {
            (height as usize - 1) - sprite_line
        } else {
            sprite_line
        };

        let tile_always_pair = if height == 16 {
            sprite.tile & 0xFE
        } else {
            sprite.tile
        };
        let tile_index = if height == 16 && actual_sprite_line >= 8 {
            tile_always_pair + 1
        } else {
            tile_always_pair
        };

        self.actual_sprite_line = actual_sprite_line;
        tile_index
    }

    fn get_tile_data_low(&mut self, vram: &DmgVram) -> u8 {
        let tile_address =
            VRAM_START + (self.tile_id as u16 * 16) + (self.actual_sprite_line % 8 * 2) as u16;
        vram.read(tile_address)
    }

    fn get_tile_data_high(&mut self, vram: &DmgVram) -> u8 {
        let tile_address =
            VRAM_START + (self.tile_id as u16 * 16) + (self.actual_sprite_line % 8 * 2) as u16;
        vram.read(tile_address + 1)
    }

    fn extract_attributes(&self, attributes: u8) -> (bool, bool, bool, bool) {
        (
            ((attributes >> 7) & 1) != 0,
            ((attributes >> 6) & 1) != 0,
            ((attributes >> 5) & 1) != 0,
            ((attributes >> 4) & 1) != 0,
        )
    }

    fn push_pixel(
        &mut self,
        piso: &mut ObjPiso<DmgColor>,
        sprite: &Sprite,
        scanline_x: usize,
        obp0: u8,
        obp1: u8,
    ) {
        let (priority, _, x_flip, palette_attribute) = self.extract_attributes(sprite.attributes);

        let palette = if palette_attribute { obp1 } else { obp0 };
        let oam_index = sprite.oam_index;
        piso.merge(
            self.tile_data_low,
            self.tile_data_high,
            sprite.x,
            x_flip,
            palette,
            priority,
            scanline_x,
            oam_index,
        );
    }
}

impl OamFetcher<CgbVram, CgbColor> {
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        vram: &mut CgbVram,
        sprite: &Sprite,
        piso: &mut ObjPiso<CgbColor>,
        ly: u8,
        height: u8,
        scanline_x: usize,
        obj_cram: &Cram,
        opri: u8,
        obp0: u8,
        obp1: u8,
        is_dmg_mode: bool,
    ) -> bool {
        self.dot_counter = self.dot_counter.wrapping_add(1);
        if self.dot_counter.is_multiple_of(2) {
            match self.fetcher_state {
                FetcherState::GetTileId => {
                    self.tile_id = self.get_tile_id(sprite, ly, height);
                    self.fetcher_state = FetcherState::GetLowData;

                    return false;
                }
                FetcherState::GetLowData => {
                    self.tile_data_low = self.get_tile_data_low(vram, is_dmg_mode);
                    self.fetcher_state = FetcherState::GetHighData;

                    return false;
                }
                FetcherState::GetHighData => {
                    self.tile_data_high = self.get_tile_data_high(vram, is_dmg_mode);
                    self.fetcher_state = FetcherState::PushPixel;

                    return false;
                }
                FetcherState::PushPixel => {
                    self.push_pixel(piso, sprite, scanline_x, obj_cram, opri, obp0, obp1, is_dmg_mode);
                    self.fetcher_state = FetcherState::GetTileId;

                    return true;
                }
            }
        }

        false
    }

    fn get_tile_id(&mut self, sprite: &Sprite, ly: u8, height: u8) -> u8 {
        let sprite_top: i16 = sprite.y as i16 - 16;
        let sprite_line = (ly as i16 - sprite_top) as usize;

        let y_flip = ((sprite.attributes >> 6) & 1) != 0;
        let actual_sprite_line = if y_flip {
            (height as usize - 1) - sprite_line
        } else {
            sprite_line
        };

        let tile_always_pair = if height == 16 {
            sprite.tile & 0xFE
        } else {
            sprite.tile
        };
        let tile_index = if height == 16 && actual_sprite_line >= 8 {
            tile_always_pair + 1
        } else {
            tile_always_pair
        };

        self.actual_sprite_line = actual_sprite_line;
        self.attributes = sprite.attributes;
        tile_index
    }

    fn get_tile_data_low(&mut self, vram: &mut CgbVram, is_dmg_mode: bool) -> u8 {
        let vbk = if is_dmg_mode {
            0
        } else {
            vram.get_vbk(self.attributes)
        };
        let tile_address =
            VRAM_START + (self.tile_id as u16 * 16) + (self.actual_sprite_line % 8 * 2) as u16;
        vram.read_with_custom_vbk(tile_address, vbk)
    }

    fn get_tile_data_high(&mut self, vram: &mut CgbVram, is_dmg_mode: bool) -> u8 {
        let vbk = if is_dmg_mode {
            0
        } else {
            vram.get_vbk(self.attributes)
        };
        let tile_address =
            VRAM_START + (self.tile_id as u16 * 16) + (self.actual_sprite_line % 8 * 2) as u16;
        vram.read_with_custom_vbk(tile_address + 1, vbk)
    }

    fn extract_attributes(&self, attributes: u8, is_dmg_mode: bool) -> (bool, bool, bool, u8) {
        if is_dmg_mode {
            (
                ((attributes >> 7) & 1) != 0,
                ((attributes >> 6) & 1) != 0,
                ((attributes >> 5) & 1) != 0,
                (attributes >> 4) & 1,
            )
        } else {
            (
                ((attributes >> 7) & 1) != 0,
                ((attributes >> 6) & 1) != 0,
                ((attributes >> 5) & 1) != 0,
                attributes & 0b111,
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn push_pixel(
        &mut self,
        piso: &mut ObjPiso<CgbColor>,
        sprite: &Sprite,
        scanline_x: usize,
        cram: &Cram,
        opri: u8,
        obp0: u8,
        obp1: u8,
        is_dmg_mode: bool,
    ) {
        let (priority, _, x_flip, palette_index) =
            self.extract_attributes(sprite.attributes, is_dmg_mode);
        let obp = if palette_index & 1 == 1 { obp1 } else { obp0 };
        let oam_index = sprite.oam_index;
        piso.merge(
            self.tile_data_low,
            self.tile_data_high,
            sprite.x,
            x_flip,
            palette_index,
            priority,
            scanline_x,
            cram,
            oam_index,
            opri,
            obp,
            is_dmg_mode,
        );
    }
}
