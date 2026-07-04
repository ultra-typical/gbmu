use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

use crate::ppu::colors_palette::{CgbColor, ColorType, DmgColor};
use crate::ppu::lcd_control::LcdControl;
use crate::ppu::pixel::Pixel;
use crate::ppu::pixel_fifo::PixelFifo;
use crate::ppu::vram::{CgbVram, DmgVram};

const TILE_DATA_1_START: u16 = 0x8000;
const TILE_DATA_0_START: u16 = 0x8800;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FetcherState {
    #[default]
    GetTileId = 0,
    GetLowData = 1,
    GetHighData = 2,
    Sleep = 3,
    PushPixel = 4,
}

use super::{Cram, Vram};

pub trait PFetcher<V, C: ColorType + Copy> {
    #[allow(clippy::too_many_arguments)]
    fn tick(
        &mut self,
        fifo: &PixelFifo<C>,
        vram: &mut V,
        ly: u8,
        scx: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
        bgp: u8,
        cram: &Cram,
        compatibility: bool,
        boot_enable: bool,
    ) -> Option<[Pixel<C>; 8]>;
    fn reset_for_window(&mut self);

    fn reset_to_state_1(&mut self);
    fn reset_for_scanline(&mut self);

    fn new() -> Self
    where
        Self: Sized;
    fn push_pixel(&self, _cram: &Cram, _bgp: u8, _dmg_compat: bool) -> Option<[Pixel<C>; 8]> {
        Some([Pixel::<C>::new_bg(C::new(0, 0), false, 0); 8])
    }
    fn get_tile_data_high(
        &self,
        _vram: &mut V,
        _ly: u8,
        _scy: u8,
        _wly: u8,
        _lcd_control: &LcdControl,
        _use_window: bool,
    ) -> u8 {
        0
    }
    fn get_tile_data_low(
        &self,
        _vram: &mut V,
        _ly: u8,
        _scy: u8,
        _wly: u8,
        _lcd_control: &LcdControl,
        _use_window: bool,
    ) -> u8 {
        0
    }

    #[allow(clippy::too_many_arguments)]
    fn get_tile_id(
        &mut self,
        _vram: &V,
        _ly: u8,
        _scx: u8,
        _scy: u8,
        _wly: u8,
        _lcd_control: &LcdControl,
        _use_window: bool,
        _compatibility: bool,
        _boot_enable: bool,
    ) -> u8 {
        println!(" ah oui ?");
        0
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PixelFetcher<V: Vram, C: ColorType> {
    phantom_vram: PhantomData<V>,
    phantom_color: PhantomData<C>,
    fetcher_state: FetcherState,
    tile_id: u8,
    bg_attribute: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    fetcher_x: u8,
    dot_counter: u32,
    first_fetch_done: bool,
}

impl<V: Vram, C: ColorType + Copy> PixelFetcher<V, C> {
    fn reset_internal(&mut self, first_fetch_done: bool) {
        self.fetcher_state = FetcherState::GetTileId;
        self.fetcher_x = 0;
        self.first_fetch_done = first_fetch_done;
    }
}

impl PFetcher<DmgVram, DmgColor> for PixelFetcher<DmgVram, DmgColor> {
    #[allow(clippy::too_many_arguments)]
    fn tick(
        &mut self,
        fifo: &PixelFifo<DmgColor>,
        vram: &mut DmgVram,
        ly: u8,
        scx: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
        bgp: u8,
        cram: &Cram,
        compatibility: bool,
        boot_enable: bool,
    ) -> Option<[Pixel<DmgColor>; 8]> {
        self.dot_counter = self.dot_counter.wrapping_add(1);

        if self.fetcher_state == FetcherState::PushPixel && fifo.is_empty() {
            let tile: Option<[Pixel<DmgColor>; 8]> = self.push_pixel(cram, bgp, false);

            self.fetcher_x += 1;
            self.fetcher_state = FetcherState::GetTileId;

            tile
        } else if self.dot_counter.is_multiple_of(2) {
            match self.fetcher_state {
                FetcherState::GetTileId => {
                    self.tile_id = self.get_tile_id(
                        vram,
                        ly,
                        scx,
                        scy,
                        wly,
                        lcd_control,
                        use_window,
                        compatibility,
                        boot_enable,
                    );
                    self.fetcher_state = FetcherState::GetLowData;
                    None
                }
                FetcherState::GetLowData => {
                    self.tile_data_low =
                        self.get_tile_data_low(vram, ly, scy, wly, lcd_control, use_window);
                    self.fetcher_state = FetcherState::GetHighData;
                    None
                }
                FetcherState::GetHighData => {
                    self.tile_data_high =
                        self.get_tile_data_high(vram, ly, scy, wly, lcd_control, use_window);
                    if self.first_fetch_done {
                        if fifo.is_empty() {
                            let tile: Option<[Pixel<DmgColor>; 8]> = self.push_pixel(cram, bgp, false);

                            self.fetcher_x += 1;
                            self.fetcher_state = FetcherState::GetTileId;

                            return tile;
                        } else {
                            self.fetcher_state = FetcherState::Sleep;
                        }
                    } else {
                        self.reset_internal(true);
                    }
                    None
                }
                FetcherState::Sleep => {
                    self.fetcher_state = FetcherState::PushPixel;
                    None
                }
                FetcherState::PushPixel => None,
            }
        } else {
            None
        }
    }

    fn new() -> Self {
        PixelFetcher::default()
    }

    fn reset_for_window(&mut self) {
        self.reset_internal(true);
    }

    fn reset_for_scanline(&mut self) {
        self.reset_internal(false);
    }

    fn reset_to_state_1(&mut self) {
        self.fetcher_state = FetcherState::GetTileId;
    }

    fn get_tile_data_low(
        &self,
        vram: &mut DmgVram,
        ly: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
    ) -> u8 {
        let y = if use_window {
            wly as usize
        } else {
            ly as usize + scy as usize
        };

        let correct_byte = (y % 8) * 2;

        if lcd_control.bg_window_tile_data_area().start == TILE_DATA_1_START {
            let tilemap_base = TILE_DATA_1_START + (self.tile_id as u16) * 16;
            vram.read(tilemap_base + correct_byte as u16)
        } else if lcd_control.bg_window_tile_data_area().start == TILE_DATA_0_START {
            let base = 0x9000u16;
            let offset = (self.tile_id as i8) as i16 * 16;
            let tilemap_base = base.wrapping_add_signed(offset);
            vram.read(tilemap_base + correct_byte as u16)
        } else {
            unreachable!()
        }
    }

    fn get_tile_data_high(
        &self,
        vram: &mut DmgVram,
        ly: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
    ) -> u8 {
        let y = if use_window {
            wly as usize
        } else {
            ly as usize + scy as usize
        };

        let correct_byte = ((y % 8) * 2) + 1;

        if lcd_control.bg_window_tile_data_area().start == TILE_DATA_1_START {
            let tilemap_base = TILE_DATA_1_START + (self.tile_id as u16) * 16;
            vram.read(tilemap_base + correct_byte as u16)
        } else if lcd_control.bg_window_tile_data_area().start == TILE_DATA_0_START {
            let base = 0x9000u16;
            let offset = (self.tile_id as i8) as i16 * 16;
            let tilemap_base = base.wrapping_add_signed(offset);
            vram.read(tilemap_base + correct_byte as u16)
        } else {
            unreachable!()
        }
    }

    fn push_pixel(&self, _cram: &Cram, bgp: u8, _dmg_compat: bool) -> Option<[Pixel<DmgColor>; 8]> {
        let mut tile_pixels = [Pixel::<DmgColor>::default(); 8];
        for i in 0..8 {
            let bit_index = 7 - i;

            let low_weight_bit = (self.tile_data_low >> bit_index) & 1;
            let high_weight_bit = (self.tile_data_high >> bit_index) & 1;

            let color_index = (high_weight_bit << 1) | low_weight_bit;
            let color = DmgColor::apply_background_palette_bgp(color_index, bgp);
            let pixel = Pixel::new_bg(color, false, 0);

            tile_pixels[i as usize] = pixel;
        }

        Some(tile_pixels)
    }

    #[allow(clippy::too_many_arguments)]
    fn get_tile_id(
        &mut self,
        vram: &DmgVram,
        ly: u8,
        scx: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
        _compatibility: bool,
        _boot_enable: bool,
    ) -> u8 {
        let tilemap_base: std::ops::Range<u16> = if use_window {
            lcd_control.window_tile_map_area()
        } else {
            lcd_control.bg_tile_map_area()
        };

        let (x, y) = if use_window {
            (self.fetcher_x as usize, wly as usize / 8)
        } else {
            (
                ((scx / 8) as usize + self.fetcher_x as usize) & 0x1F,
                ((ly as usize + scy as usize) & 0xFF) / 8,
            )
        };

        let offset = (y * 32 + x) as u16;

        vram.read(tilemap_base.start + offset)
    }
}

impl PFetcher<CgbVram, CgbColor> for PixelFetcher<CgbVram, CgbColor> {
    #[allow(clippy::too_many_arguments)]
    fn tick(
        &mut self,
        fifo: &PixelFifo<CgbColor>,
        vram: &mut CgbVram,
        ly: u8,
        scx: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
        bgp: u8,
        cram: &Cram,
        compatibility: bool,
        boot_enable: bool,
    ) -> Option<[Pixel<CgbColor>; 8]> {
        let dmg_compat = compatibility && !boot_enable;
        self.dot_counter = self.dot_counter.wrapping_add(1);

        if self.fetcher_state == FetcherState::PushPixel && fifo.is_empty() {
            let tile: Option<[Pixel<CgbColor>; 8]> = self.push_pixel(cram, bgp, dmg_compat);

            self.fetcher_x += 1;
            self.fetcher_state = FetcherState::GetTileId;

            tile
        } else if self.dot_counter.is_multiple_of(2) {
            match self.fetcher_state {
                FetcherState::GetTileId => {
                    self.tile_id = self.get_tile_id(
                        vram,
                        ly,
                        scx,
                        scy,
                        wly,
                        lcd_control,
                        use_window,
                        compatibility,
                        boot_enable,
                    );
                    self.fetcher_state = FetcherState::GetLowData;
                    None
                }
                FetcherState::GetLowData => {
                    self.tile_data_low =
                        self.get_tile_data_low(vram, ly, scy, wly, lcd_control, use_window);
                    self.fetcher_state = FetcherState::GetHighData;
                    None
                }
                FetcherState::GetHighData => {
                    self.tile_data_high =
                        self.get_tile_data_high(vram, ly, scy, wly, lcd_control, use_window);
                    if self.first_fetch_done {
                        if fifo.is_empty() {
                            let tile: Option<[Pixel<CgbColor>; 8]> = self.push_pixel(cram, bgp, dmg_compat);

                            self.fetcher_x += 1;
                            self.fetcher_state = FetcherState::GetTileId;

                            return tile;
                        } else {
                            self.fetcher_state = FetcherState::Sleep;
                        }
                    } else {
                        self.reset_internal(true);
                    }
                    None
                }
                FetcherState::Sleep => {
                    self.fetcher_state = FetcherState::PushPixel;
                    None
                }
                FetcherState::PushPixel => None,
            }
        } else {
            None
        }
    }

    fn reset_for_window(&mut self) {
        self.reset_internal(true);
    }

    fn reset_to_state_1(&mut self) {
        self.fetcher_state = FetcherState::GetTileId
    }

    fn reset_for_scanline(&mut self) {
        self.reset_internal(false);
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        PixelFetcher::default()
    }

    fn get_tile_data_low(
        &self,
        vram: &mut CgbVram,
        ly: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
    ) -> u8 {
        let y = if use_window {
            wly as usize
        } else {
            ly as usize + scy as usize
        };

        let y_flip = (self.bg_attribute >> 6) & 1 != 0;
        let flipped_y = if y_flip { 7 - (y % 8) } else { y % 8 };
        let correct_byte = flipped_y * 2;

        let vbk = vram.get_vbk(self.bg_attribute);
        if lcd_control.bg_window_tile_data_area().start == TILE_DATA_1_START {
            let tilemap_base = TILE_DATA_1_START + (self.tile_id as u16) * 16;
            vram.read_with_custom_vbk(tilemap_base + correct_byte as u16, vbk)
        } else if lcd_control.bg_window_tile_data_area().start == TILE_DATA_0_START {
            let base = 0x9000u16;
            let offset = (self.tile_id as i8) as i16 * 16;
            let tilemap_base = base.wrapping_add_signed(offset);
            vram.read_with_custom_vbk(tilemap_base + correct_byte as u16, vbk)
        } else {
            unreachable!()
        }
    }

    fn get_tile_data_high(
        &self,
        vram: &mut CgbVram,
        ly: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
    ) -> u8 {
        let y = if use_window {
            wly as usize
        } else {
            ly as usize + scy as usize
        };
        let y_flip = (self.bg_attribute >> 6) & 1;
        let flipped_y = if y_flip == 1 { 7 - (y % 8) } else { y % 8 };
        let correct_byte = (flipped_y * 2) + 1;

        let vbk = vram.get_vbk(self.bg_attribute);
        if lcd_control.bg_window_tile_data_area().start == TILE_DATA_1_START {
            let tilemap_base = TILE_DATA_1_START + (self.tile_id as u16) * 16;
            vram.read_with_custom_vbk(tilemap_base + correct_byte as u16, vbk)
        } else if lcd_control.bg_window_tile_data_area().start == TILE_DATA_0_START {
            let base = 0x9000u16;
            let offset = (self.tile_id as i8) as i16 * 16;
            let tilemap_base = base.wrapping_add_signed(offset);
            vram.read_with_custom_vbk(tilemap_base + correct_byte as u16, vbk)
        } else {
            unreachable!()
        }
    }

    fn push_pixel(&self, bg_cram: &Cram, bgp: u8, dmg_compat: bool) -> Option<[Pixel<CgbColor>; 8]> {
        let mut tile_pixels = [Pixel::<CgbColor>::default(); 8];
        let x_flip = (self.bg_attribute >> 5) & 1 != 0;
        for i in 0..8 {
            let bit_index = if x_flip { i } else { 7 - i };
            let low_weight_bit = (self.tile_data_low >> bit_index) & 1;
            let high_weight_bit = (self.tile_data_high >> bit_index) & 1;
            let color_index = (high_weight_bit << 1) | low_weight_bit;
            let palette_index = self.bg_attribute & 0b111;
            let priority = ((self.bg_attribute >> 7) & 1) != 0;
            // In DMG compatibility mode the color index goes through BGP
            // before indexing CRAM palette 0, but transparency/priority
            // decisions keep using the raw index.
            let cram_index = if dmg_compat {
                (bgp >> (color_index * 2)) & 0b11
            } else {
                color_index
            };
            let mut color =
                CgbColor::apply_background_palette_cram(bg_cram, palette_index, cram_index);
            color.base_index = color_index;
            let pixel = Pixel::new_bg(color, priority, 0);

            tile_pixels[i as usize] = pixel;
        }

        Some(tile_pixels)
    }

    #[allow(clippy::too_many_arguments)]
    fn get_tile_id(
        &mut self,
        vram: &CgbVram,
        ly: u8,
        scx: u8,
        scy: u8,
        wly: u8,
        lcd_control: &LcdControl,
        use_window: bool,
        compatibility: bool,
        boot_enable: bool,
    ) -> u8 {
        let tilemap_base: std::ops::Range<u16> = if use_window {
            lcd_control.window_tile_map_area()
        } else {
            lcd_control.bg_tile_map_area()
        };

        let (x, y) = if use_window {
            (self.fetcher_x as usize, wly as usize / 8)
        } else {
            (
                ((scx / 8) as usize + self.fetcher_x as usize) & 0x1F,
                ((ly as usize + scy as usize) & 0xFF) / 8,
            )
        };

        let offset = (y * 32 + x) as u16;
        let addr = tilemap_base.start + offset;

        if boot_enable {
            self.bg_attribute = vram.read_with_custom_vbk(addr, 0x01);
            return vram.read_with_custom_vbk(addr, self.bg_attribute << 3 & 1);
        }
        if compatibility {
            self.bg_attribute = 0;
            vram.read_with_custom_vbk(addr, 0)
        } else {
            self.bg_attribute = vram.read_with_custom_vbk(addr, 0x01);
            vram.read_with_custom_vbk(addr, self.bg_attribute << 3 & 1)
        }
    }
}
