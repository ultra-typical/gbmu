pub mod colors_palette;
mod lcd_control;
mod lcd_status;
mod oam_fetcher;
mod obj_piso;
mod pixel;
mod pixel_fetcher;
mod pixel_fifo;
pub mod vram;
mod wram;

pub type DmgPpu = Ppu<DmgVram, PixelFetcher<DmgVram, DmgColor>, Oam, DmgColor>;
pub type CgbPpu = Ppu<CgbVram, PixelFetcher<CgbVram, CgbColor>, Oam, CgbColor>;

use crate::ppu::lcd_control::LcdControl;
use std::cmp::PartialEq;

use serde::Deserialize;
use serde::Serialize;

use crate::communications::GameCT;

use crate::mmu::oam::{Oam, Sprite};
use crate::ppu::PpuMode::PixelTransfer;
use crate::ppu::colors_palette::{CgbColor, ColorType, DmgColor};
use crate::ppu::lcd_status::LcdStatus;
use crate::ppu::oam_fetcher::OamFetcher;
use crate::ppu::obj_piso::ObjPiso;
use crate::ppu::pixel::Pixel;
use crate::ppu::pixel_fetcher::{PFetcher, PixelFetcher};
use crate::ppu::pixel_fifo::PixelFifo;
use crate::ppu::vram::{CgbVram, DmgVram, Vram};
use crate::ppu::wram::Wram;

pub const WIN_SIZE_X: usize = 160;
pub const WIN_SIZE_Y: usize = 144;

const OAM_DOTS: u32 = 80;
const SCANLINE_DOTS: u32 = 456;

const LCD_OFF_COLOR_DMG: DmgColor = DmgColor {
    base_index: 0,
    index_treated: 0,
    rgb: [255, 255, 255],
};
const LCD_OFF_COLOR_CGB: CgbColor = CgbColor {
    base_index: 0,
    index_treated: 0,
    rgb: [255, 255, 255],
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    #[default]
    OamSearch = 2,
    PixelTransfer = 3,
}

pub trait PixelProcessor {
    fn new(compatibility: bool) -> Self
    where
        Self: Sized;
    fn read_vram(&self, addr: u16) -> u8;
    fn read_register(&self, addr: u16) -> u8;
    fn write_vram(&mut self, addr: u16, val: u8);
    fn write_register(&mut self, addr: u16, val: u8);

    fn read_oam(&mut self, addr: u16) -> u8; // mut read for bug on read
    fn write_oam(&mut self, addr: u16, value: u8);

    fn pending_vblank(&self) -> bool;
    fn set_pending_vblank(&mut self, value: bool);
    fn pending_stat(&self) -> bool;
    fn set_pending_stat(&mut self, value: bool);

    fn step_oam_fetcher(&mut self);
    fn tick(&mut self, ct: &mut Box<dyn GameCT>, boot_enable: bool);
    fn handle_window_switch(&mut self, use_window: bool);
    fn push_pixel_to_screen(&mut self, ct: &mut Box<dyn GameCT>);
    fn mode_pixel_transfer(&mut self, ct: &mut Box<dyn GameCT>);
    fn mode_oam_search(&mut self);
    fn lcd_status(&self) -> &LcdStatus;
    fn hdma1(&self) -> u8;
    fn hdma2(&self) -> u8;
    fn hdma3(&self) -> u8;
    fn hdma4(&self) -> u8;
    fn write_hdma_value(&mut self, addr: u16, value: u8);

    fn write_wram_value(&mut self, addr: u16, value: u8);
    fn read_wram_value(&mut self, addr: u16) -> u8;
}

pub trait ObjectManager {
    fn new() -> Self
    where
        Self: Sized;

    fn read(&mut self, addr: u16) -> u8; // mut read for bug
    fn write(&mut self, addr: u16, value: u8);

    fn set_accessed_oam_row(&mut self, value: u8);
    fn update_accessed_oam_row(&mut self, value: u8);
    fn accessed_oam_row(&mut self) -> u8;
    fn sprite(&mut self, index: u8) -> &mut Sprite;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cram {
    data: Vec<u8>,
}

impl Cram {
    pub fn new() -> Cram {
        Self {
            data: vec![0x00; 64],
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum HdmaMode {
    Gdma,
    Hblank,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ppu<V: Vram, P: PFetcher<V, C>, O: ObjectManager, C: ColorType + Copy> {
    pub dots: u32,
    lcd_status: LcdStatus,
    wly: u8,
    ly: u8,
    internal_ly: u8,
    x: usize,
    pixel_fetcher: P,
    oam_fetcher: OamFetcher<V, C>,
    bg_fifo: PixelFifo<C>,
    obj_piso: ObjPiso<C>,
    visible_sprites: [Option<Sprite>; 10],
    pixels_to_discard: u8,
    use_window: bool,
    wx_at_window_start: u8,
    is_wx_glitch_happened: bool,
    fetching_sprite: bool,
    current_sprite_to_fetch: Option<usize>,
    wy_equal_ly_condition_met: bool,
    oam_scan_index: u8,
    visible_sprites_count: u8,
    current_obj_height: u8,
    lcd_was_enabled: bool,
    is_first_scanline_after_lcd_on: bool,
    stat_interrupt_line: bool,
    stall_dots: u8,
    frame_blanked: bool,
    // Memory-mapped registers owned by PPU
    lcdc_byte: u8, // 0xFF40
    scy: u8,       // 0xFF42
    scx: u8,       // 0xFF43
    lyc: u8,       // 0xFF45
    bgp: u8,       // 0xFF47
    obp0: u8,      // 0xFF48
    obp1: u8,      // 0xFF49
    wy: u8,        // 0xFF4A
    wx: u8,        // 0xFF4B
    // Pending interrupts to be drained by MMU after tick
    pub pending_vblank: bool,
    pub pending_stat: bool,
    bgpi: u8,
    bgpd: u8,
    obpi: u8,
    obpd: u8,
    vram: V,
    wram: Wram,
    oam: O,
    bg_cram: Cram,
    obj_cram: Cram,
    key0_sys: u8, //0xFF4C
    key1_spd: u8, //0xFF4D
    opri: u8,     //0xFF6C
    hdma1: u8,    //0xFF51
    hdma2: u8,    //0xFF52
    hdma3: u8,    //0xFF53
    hdma4: u8,    //0xFF54
    hdma5: u8,    //0xFF55
    compatibility: bool,
    boot_enable: bool,
}
impl<V: Vram, P: PFetcher<V, C>, O: ObjectManager, C: ColorType + Copy + Default> Ppu<V, P, O, C> {
    fn pending_vblank(&self) -> bool {
        self.pending_vblank
    }
    fn set_pending_vblank(&mut self, value: bool) {
        self.pending_vblank = value
    }
    fn pending_stat(&self) -> bool {
        self.pending_stat
    }
    fn set_pending_stat(&mut self, value: bool) {
        self.pending_stat = value
    }

    fn new(compatibility: bool) -> Self {
        Self {
            compatibility,
            boot_enable: true,
            dots: 0,
            lcd_status: LcdStatus::new(),
            wly: 0x00,
            ly: 0x00,
            internal_ly: 0x00,
            x: 0,
            pixel_fetcher: P::new(),
            oam_fetcher: OamFetcher::default(),
            bg_fifo: PixelFifo::<C>::default(),
            obj_piso: ObjPiso::<C>::default(),
            visible_sprites: [None; 10],
            pixels_to_discard: 0,
            use_window: false,
            wx_at_window_start: 0x00,
            is_wx_glitch_happened: false,
            fetching_sprite: false,
            current_sprite_to_fetch: None,
            wy_equal_ly_condition_met: false,
            oam_scan_index: 0,
            visible_sprites_count: 0,
            current_obj_height: 0,
            lcd_was_enabled: false,
            is_first_scanline_after_lcd_on: false,
            stat_interrupt_line: false,
            stall_dots: 0,
            frame_blanked: false,
            lcdc_byte: 0x00,
            scy: 0x00,
            scx: 0x00,
            lyc: 0x00,
            bgp: 0x00,
            obp0: 0x00,
            obp1: 0x00,
            wy: 0x00,
            wx: 0x00,
            pending_vblank: false,
            pending_stat: false,
            vram: V::new(),
            wram: Wram::new(),
            oam: O::new(),
            bgpd: 0x00,
            bgpi: 0x00,
            obpd: 0x00,
            obpi: 0x00,
            bg_cram: Cram::new(),
            obj_cram: Cram::new(),
            key0_sys: 0x00,
            key1_spd: 0x00,
            opri: 0x00,
            hdma1: 0x00,
            hdma2: 0x00,
            hdma3: 0x00,
            hdma4: 0x00,
            hdma5: 0x00,
        }
    }

    fn read_oam(&mut self, addr: u16) -> u8 {
        self.oam.read(addr)
    }

    fn write_oam(&mut self, addr: u16, value: u8) {
        self.oam.write(addr, value);
    }

    fn write_vram(&mut self, addr: u16, val: u8) {
        self.vram.write(addr, val);
    }

    fn read_vram(&self, addr: u16) -> u8 {
        self.vram.read(addr)
    }

    fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.lcdc_byte,
            0xFF41 => self.lcd_status.struct_to_byte(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4C => self.key0_sys,
            0xFF4D => self.key1_spd,
            0xFF4F => self.vram.vbk(),
            0xFF51 => 0xFF,
            0xFF52 => 0xFF,
            0xFF53 => 0xFF,
            0xFF54 => 0xFF,
            0xFF68 => self.bgpi,
            0xFF69 => {
                if !self.lcd_status.get_ppu_mode().eq(&PixelTransfer) {
                    self.bgpd
                } else {
                    0xFF
                }
            }
            0xFF6A => self.obpi,
            0xFF6B => {
                if !self.lcd_status.get_ppu_mode().eq(&PixelTransfer) {
                    self.obpd
                } else {
                    0xFF
                }
            }
            0xFF6C => self.opri,
            0xFF70 => self.wram.svbk_wbk(),
            _ => 0xFF,
        }
    }

    fn write_register(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF40 => self.lcdc_byte = val,
            0xFF41 => {
                // CPU can only write bits 3-6; bits 0-2 are PPU-controlled; bit 7 always 1
                let ppu_bits = self.lcd_status.struct_to_byte() & 0b0000_0111;
                self.lcd_status
                    .update_from_byte((val & 0b0111_1000) | ppu_bits | 0x80);
            }
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => {} // LY is read-only
            0xFF45 => self.lyc = val,
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            0xFF4C => {
                self.key0_sys = val;
                self.assign_value_to_opri(val)
            }
            0xFF4D => self.key1_spd = val,
            0xFF4F => self.vram.set_vbk(val),
            0xFF51 => self.hdma1 = val,
            0xFF52 => self.hdma2 = val,
            0xFF53 => self.hdma3 = val,
            0xFF54 => self.hdma4 = val,
            0xFF68 => {
                if !self.lcd_status.get_ppu_mode().eq(&PixelTransfer) {
                    self.bgpi = val
                }
            }
            0xFF69 => {
                if !self.lcd_status.get_ppu_mode().eq(&PixelTransfer) {
                    let addr = self.bgpi & 0b00111111;
                    let auto_increment = self.bgpi >> 7;
                    self.bg_cram.data[addr as usize] = val;
                    if auto_increment == 1 {
                        let data_high = self.bgpi & 0b11000000;
                        let data_low = ((self.bgpi & 0b00111111) + 1) & 0b00111111;
                        self.bgpi = data_high | data_low;
                    }
                }
            }
            0xFF6A => {
                if !self.lcd_status.get_ppu_mode().eq(&PixelTransfer) {
                    self.obpi = val
                }
            }
            0xFF6B => {
                if !self.lcd_status.get_ppu_mode().eq(&PixelTransfer) {
                    let addr = self.obpi & 0b00111111;
                    let auto_increment = self.obpi >> 7;
                    self.obj_cram.data[addr as usize] = val;
                    if auto_increment == 1 {
                        let data_high = self.obpi & 0b11000000;
                        let data_low: u8 = ((self.obpi & 0b00111111) + 1) & 0b00111111;
                        self.obpi = data_high | data_low;
                    }
                }
            }
            0xFF6C => self.opri = val,
            0xFF70 => self.wram.set_svbk_wbk(val),
            _ => {}
        }
    }

    fn assign_value_to_opri(&mut self, value: u8) {
        if value & 0b00000100 == 0 {
            self.opri = 0x00;
        } else {
            self.opri = 0x01;
        }
    }
}

impl<V: Vram, P: PFetcher<V, C>, O: ObjectManager, C: ColorType + Copy> Ppu<V, P, O, C> {
    fn read_lcdc(&self) -> LcdControl {
        LcdControl::from_byte(self.lcdc_byte)
    }
    fn sort_sprites_by_x(&mut self) -> Vec<Sprite> {
        let mut sprites: Vec<(usize, Sprite)> = self
            .visible_sprites
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.map(|sprite| (i, sprite)))
            .collect();

        sprites.sort_by(|(index_a, sprite_a), (index_b, sprite_b)| {
            if sprite_a.x != sprite_b.x {
                sprite_a.x.cmp(&sprite_b.x)
            } else {
                index_a.cmp(index_b)
            }
        });

        sprites.into_iter().map(|(_, s)| s).collect()
    }

    fn step_pixel_fetcher(&mut self, use_window: bool) {
        let tile_pixels = self.pixel_fetcher.tick(
            &self.bg_fifo,
            &mut self.vram,
            self.ly,
            self.scx,
            self.scy,
            self.wly,
            &LcdControl::from_byte(self.lcdc_byte),
            use_window,
            self.bgp,
            &self.bg_cram,
            self.compatibility,
            self.boot_enable,
        );

        if let Some(pixels) = tile_pixels {
            for pixel in pixels {
                self.bg_fifo.push(pixel);
            }
        }
    }

    fn reset_for_new_scanline(&mut self) {
        self.x = 0;
        self.bg_fifo.clear();
        self.obj_piso.reset();
        self.pixel_fetcher.reset_for_scanline();
        self.pixels_to_discard = self.scx % 8;
        self.use_window = false;
        self.is_wx_glitch_happened = false;
        self.is_first_scanline_after_lcd_on = false;
        self.stall_dots = 0;
    }

    fn advance_to_next_scanline(&mut self) {
        if self.read_lcdc().is_window_enabled()
            && self.wy_equal_ly_condition_met
            && self.wx <= 166
            && self.wly < WIN_SIZE_Y as u8
        {
            self.wly += 1;
        }

        self.ly += 1;
        self.internal_ly += 1;

        self.check_lyc_equals_ly();
        self.reset_for_new_scanline();
    }

    fn mode_hblank(&mut self) {
        let scanline_dots = if self.is_first_scanline_after_lcd_on {
            SCANLINE_DOTS - 16
        } else {
            SCANLINE_DOTS
        };

        if self.dots >= scanline_dots {
            self.dots -= scanline_dots;

            self.advance_to_next_scanline();

            if self.ly >= WIN_SIZE_Y as u8 {
                self.update_ppu_mode(PpuMode::VBlank);
                self.pending_vblank = true;
                self.frame_blanked = false;
            } else {
                self.update_ppu_mode(PpuMode::OamSearch);
            }
        }
    }

    fn handle_ly153_quirk(&mut self) {
        self.ly = 0;
        self.check_lyc_equals_ly();
    }

    fn end_frame(&mut self) {
        self.internal_ly = 0;
        self.ly = 0;

        self.wly = 0;
        self.reset_for_new_scanline();
        self.wy_equal_ly_condition_met = false;

        self.update_ppu_mode(PpuMode::OamSearch);
    }

    fn advance_vblank_scanline(&mut self) {
        self.internal_ly += 1;
        self.ly = self.internal_ly;
        self.check_lyc_equals_ly();
    }

    fn mode_vblank(&mut self) {
        if self.internal_ly == 153 && self.dots == 4 {
            self.handle_ly153_quirk();
        }

        if self.dots >= SCANLINE_DOTS {
            self.dots -= SCANLINE_DOTS;

            if self.internal_ly == 153 {
                self.end_frame();
            } else {
                self.advance_vblank_scanline();
            }
        }
    }

    fn reset_when_ppu_disabled(&mut self) {
        self.ly = 0;
        self.internal_ly = 0;

        self.dots = 0;
        self.update_ppu_mode(PpuMode::HBlank);

        self.lcd_was_enabled = false;
        self.stat_interrupt_line = false;

        self.wly = 0;
        self.wy_equal_ly_condition_met = false;
    }

    fn check_lyc_equals_ly(&mut self) {
        let lyc_match = self.ly == self.lyc;
        self.lcd_status.set_lyc_equals_ly(lyc_match);
    }

    fn update_ppu_mode(&mut self, mode: PpuMode) {
        self.lcd_status.update_ppu_mode(mode);
    }

    fn evaluate_stat_interrupt(&mut self) {
        let current_line = self.lcd_status.stat_interrupt_line();

        if !self.stat_interrupt_line && current_line {
            self.pending_stat = true;
        }

        self.stat_interrupt_line = current_line;
    }
}

impl<P: PFetcher<DmgVram, DmgColor>, O: ObjectManager> PixelProcessor
    for Ppu<DmgVram, P, O, DmgColor>
{
    fn read_wram_value(&mut self, addr: u16) -> u8 {
        self.wram.read(addr)
    }

    fn write_wram_value(&mut self, addr: u16, value: u8) {
        self.wram.write(addr, value)
    }

    fn lcd_status(&self) -> &LcdStatus {
        &self.lcd_status
    }
    fn tick(&mut self, ct: &mut Box<dyn GameCT>, boot_enable: bool) {
        self.boot_enable = boot_enable;
        self.check_lyc_equals_ly();

        if !self.read_lcdc().is_ppu_enabled() {
            self.reset_when_ppu_disabled();
            return;
        }

        if !self.lcd_was_enabled {
            self.is_first_scanline_after_lcd_on = true;
            self.lcd_was_enabled = true;
            self.frame_blanked = true;
        }

        self.dots += 1;

        if self.wy == self.ly {
            self.wy_equal_ly_condition_met = true;
        }

        match self.lcd_status.get_ppu_mode() {
            PpuMode::OamSearch => self.mode_oam_search(),
            PpuMode::PixelTransfer => self.mode_pixel_transfer(ct),
            PpuMode::HBlank => self.mode_hblank(),
            PpuMode::VBlank => self.mode_vblank(),
        };

        self.evaluate_stat_interrupt();
    }

    fn mode_oam_search(&mut self) {
        if self.dots == 1 {
            self.oam.set_accessed_oam_row(0);
            self.oam_scan_index = 0;
            self.visible_sprites_count = 0;
            self.visible_sprites = [None; 10];
            self.current_obj_height = if self.read_lcdc().is_obj_size_8x16() {
                16
            } else {
                8
            };
        }

        if self.dots.is_multiple_of(2) && self.oam_scan_index < 40 {
            let sprite = self.oam.sprite(self.oam_scan_index);

            if sprite.is_visible(self.ly, self.current_obj_height)
                && self.visible_sprites_count < 10
            {
                sprite.oam_index = self.oam_scan_index;
                let visible_sprites_count = self.visible_sprites_count;
                self.visible_sprites[visible_sprites_count as usize] = Some(*sprite);
                self.visible_sprites_count += 1;
            }
            self.oam_scan_index += 1;
        }

        if self.dots.is_multiple_of(4) {
            self.oam.update_accessed_oam_row(8);
        }

        if self.dots >= OAM_DOTS {
            let sorted = self.sort_sprites_by_x();
            self.visible_sprites = [None; 10];

            for (i, sprite) in sorted.into_iter().enumerate() {
                self.visible_sprites[i] = Some(sprite);
            }

            self.update_ppu_mode(PpuMode::PixelTransfer);
            self.oam.set_accessed_oam_row(0xFF);
        }
    }

    fn step_oam_fetcher(&mut self) {
        let height: u8 = if LcdControl::from_byte(self.lcdc_byte).is_obj_size_8x16() {
            16
        } else {
            8
        };

        if self.fetching_sprite {
            if let Some(index) = self.current_sprite_to_fetch
                && let Some(sprite) = self.visible_sprites[index]
            {
                self.fetching_sprite = !self.oam_fetcher.tick(
                    &self.vram,
                    &sprite,
                    &mut self.obj_piso,
                    self.ly,
                    height,
                    self.x,
                    self.obp0,
                    self.obp1,
                    false,
                );

                if !self.fetching_sprite {
                    self.visible_sprites[index] = None;

                    let remaining_pixels = self.bg_fifo.len() as u8;
                    if remaining_pixels < 6 {
                        self.stall_dots = 6 - remaining_pixels;
                    }
                }
            };
        } else {
            if !LcdControl::from_byte(self.lcdc_byte).is_obj_enabled() {
                return;
            }

            for (index, sprite_opt) in self.visible_sprites.iter_mut().enumerate() {
                if let Some(sprite) = sprite_opt
                    && sprite.x as usize <= self.x + 8
                {
                    self.current_sprite_to_fetch = Some(index);
                    self.pixel_fetcher.reset_to_state_1();

                    self.fetching_sprite = !self.oam_fetcher.tick(
                        &self.vram,
                        sprite,
                        &mut self.obj_piso,
                        self.ly,
                        height,
                        self.x,
                        self.obp0,
                        self.obp1,
                        false,
                    );

                    if !self.fetching_sprite {
                        *sprite_opt = None;
                    }

                    break;
                }
            }
        }
    }
    fn handle_window_switch(&mut self, use_window: bool) {
        if !self.use_window && use_window {
            self.pixel_fetcher.reset_for_window();
            self.bg_fifo.clear();
            let wx = self.wx;
            self.wx_at_window_start = wx;
            self.pixels_to_discard = 0;
        }

        self.use_window = use_window;

        if self.use_window
            && self.wx != self.wx_at_window_start
            && self.x + 7 >= self.wx as usize
            && !self.is_wx_glitch_happened
        {
            let glitched_pixel = Pixel::new_bg(
                DmgColor::apply_background_palette_bgp(0, self.bgp),
                false,
                0,
            );
            self.bg_fifo.push(glitched_pixel);
            self.is_wx_glitch_happened = true;
        }
    }

    fn push_pixel_to_screen(&mut self, ct: &mut Box<dyn GameCT>) {
        if let Some(bg_pixel) = self.bg_fifo.pop() {
            if self.pixels_to_discard > 0 {
                self.pixels_to_discard -= 1;
            } else {
                let obj_pixel = self.obj_piso.shift_out();

                let bg_color_index: u8;
                let bg_color: DmgColor;

                if !self.read_lcdc().is_bg_window_enabled() {
                    bg_color_index = 0;
                    bg_color = DmgColor::apply_background_palette_bgp(0, self.bgp);
                } else {
                    bg_color_index = bg_pixel.get_color_base_index();
                    bg_color = *bg_pixel.get_color();
                }

                let obj_color_index = obj_pixel.get_color_base_index();

                let mut final_color = if obj_color_index == 0 {
                    bg_color
                } else {
                    let priority = obj_pixel.get_priority();

                    if priority && bg_color_index != 0 {
                        bg_color
                    } else {
                        *obj_pixel.get_color()
                    }
                };

                let ly = self.ly as usize;
                let offset = ly * WIN_SIZE_X + self.x;

                final_color = if self.frame_blanked {
                    LCD_OFF_COLOR_DMG
                } else {
                    final_color
                };
                ct.put_pixel_to_frame(offset, final_color.rgb());
                let x = self.x;
                self.x = x + 1;
            }
        }
    }

    fn mode_pixel_transfer(&mut self, ct: &mut Box<dyn GameCT>) {
        if self.ly < WIN_SIZE_Y as u8 {
            let wx = self.wx;

            let use_window = self.read_lcdc().is_window_enabled()
                && self.wy_equal_ly_condition_met
                && (self.x + 7 >= wx as usize);

            self.step_oam_fetcher();

            if !self.fetching_sprite {
                self.step_pixel_fetcher(use_window);
                if self.stall_dots > 0 {
                    self.stall_dots -= 1;
                } else {
                    self.handle_window_switch(use_window);
                    self.push_pixel_to_screen(ct);
                }
            }
        }

        if self.x == 160 {
            self.update_ppu_mode(PpuMode::HBlank);
        }
    }

    fn new(compatibility: bool) -> Self
    where
        Self: Sized,
    {
        Self::new(compatibility)
    }

    fn read_vram(&self, addr: u16) -> u8 {
        self.read_vram(addr)
    }

    fn read_register(&self, addr: u16) -> u8 {
        self.read_register(addr)
    }

    fn write_vram(&mut self, addr: u16, val: u8) {
        self.write_vram(addr, val)
    }

    fn write_register(&mut self, addr: u16, val: u8) {
        self.write_register(addr, val);
    }

    fn read_oam(&mut self, addr: u16) -> u8 {
        self.read_oam(addr)
    }

    fn write_oam(&mut self, addr: u16, value: u8) {
        self.write_oam(addr, value);
    }

    fn pending_vblank(&self) -> bool {
        self.pending_vblank()
    }

    fn set_pending_vblank(&mut self, value: bool) {
        self.set_pending_vblank(value)
    }

    fn pending_stat(&self) -> bool {
        self.pending_stat()
    }

    fn set_pending_stat(&mut self, value: bool) {
        self.set_pending_stat(value)
    }
    fn hdma1(&self) -> u8 {
        0
    }
    fn hdma2(&self) -> u8 {
        0
    }
    fn hdma3(&self) -> u8 {
        0
    }
    fn hdma4(&self) -> u8 {
        0
    }
    fn write_hdma_value(&mut self, _addr: u16, _value: u8) {
        todo!()
    }
}

impl<P: PFetcher<CgbVram, CgbColor>, O: ObjectManager> PixelProcessor
    for Ppu<CgbVram, P, O, CgbColor>
{
    fn read_wram_value(&mut self, addr: u16) -> u8 {
        self.wram.read(addr)
    }

    fn write_wram_value(&mut self, addr: u16, value: u8) {
        self.wram.write(addr, value)
    }

    fn lcd_status(&self) -> &LcdStatus {
        &self.lcd_status
    }
    fn tick(&mut self, ct: &mut Box<dyn GameCT>, boot_enable: bool) {
        self.boot_enable = boot_enable;
        self.check_lyc_equals_ly();

        if !self.read_lcdc().is_ppu_enabled() {
            self.reset_when_ppu_disabled();
            return;
        }

        if !self.lcd_was_enabled {
            self.is_first_scanline_after_lcd_on = true;
            self.lcd_was_enabled = true;
            self.frame_blanked = true;
        }

        self.dots += 1;

        if self.wy == self.ly {
            self.wy_equal_ly_condition_met = true;
        }
        match self.lcd_status.get_ppu_mode() {
            PpuMode::OamSearch => self.mode_oam_search(),
            PpuMode::PixelTransfer => self.mode_pixel_transfer(ct),
            PpuMode::HBlank => self.mode_hblank(),
            PpuMode::VBlank => self.mode_vblank(),
        };

        self.evaluate_stat_interrupt();
    }
    fn step_oam_fetcher(&mut self) {
        let height: u8 = if LcdControl::from_byte(self.lcdc_byte).is_obj_size_8x16() {
            16
        } else {
            8
        };

        let is_dmg_mode = !self.boot_enable && self.compatibility;

        if self.fetching_sprite {
            if let Some(index) = self.current_sprite_to_fetch
                && let Some(sprite) = self.visible_sprites[index]
            {
                self.fetching_sprite = !self.oam_fetcher.tick(
                    &mut self.vram,
                    &sprite,
                    &mut self.obj_piso,
                    self.ly,
                    height,
                    self.x,
                    &self.obj_cram,
                    self.opri,
                    self.obp0,
                    self.obp1,
                    is_dmg_mode,
                );

                if !self.fetching_sprite {
                    self.visible_sprites[index] = None;

                    let remaining_pixels = self.bg_fifo.len() as u8;
                    if remaining_pixels < 6 {
                        self.stall_dots = 6 - remaining_pixels;
                    }
                }
            };
        } else {
            if !LcdControl::from_byte(self.lcdc_byte).is_obj_enabled() {
                return;
            }

            for (index, sprite_opt) in self.visible_sprites.iter_mut().enumerate() {
                if let Some(sprite) = sprite_opt
                    && sprite.x as usize <= self.x + 8
                {
                    self.current_sprite_to_fetch = Some(index);
                    self.pixel_fetcher.reset_to_state_1();

                    self.fetching_sprite = !self.oam_fetcher.tick(
                        &mut self.vram,
                        sprite,
                        &mut self.obj_piso,
                        self.ly,
                        height,
                        self.x,
                        &self.obj_cram,
                        self.opri,
                        self.obp0,
                        self.obp1,
                        is_dmg_mode,
                    );

                    if !self.fetching_sprite {
                        *sprite_opt = None;
                    }

                    break;
                }
            }
        }
    }
    fn handle_window_switch(&mut self, use_window: bool) {
        if !self.use_window && use_window {
            self.pixel_fetcher.reset_for_window();
            self.bg_fifo.clear();
            let wx = self.wx;
            self.wx_at_window_start = wx;
            self.pixels_to_discard = 0;
        }

        self.use_window = use_window;

        if self.use_window
            && self.wx != self.wx_at_window_start
            && self.x + 7 >= self.wx as usize
            && !self.is_wx_glitch_happened
        {
            let glitched_pixel = Pixel::new_bg(
                CgbColor::apply_background_palette_bgp(0, self.bgp),
                false,
                0,
            );
            self.bg_fifo.push(glitched_pixel);
            self.is_wx_glitch_happened = true;
        }
    }

    fn mode_oam_search(&mut self) {
        if self.dots == 1 {
            self.oam.set_accessed_oam_row(0);
            self.oam_scan_index = 0;
            self.visible_sprites_count = 0;
            self.visible_sprites = [None; 10];
            self.current_obj_height = if self.read_lcdc().is_obj_size_8x16() {
                16
            } else {
                8
            };
        }

        if self.dots.is_multiple_of(2) && self.oam_scan_index < 40 {
            let sprite = self.oam.sprite(self.oam_scan_index);

            if sprite.is_visible(self.ly, self.current_obj_height)
                && self.visible_sprites_count < 10
            {
                sprite.oam_index = self.oam_scan_index;
                let visible_sprites_count = self.visible_sprites_count;
                self.visible_sprites[visible_sprites_count as usize] = Some(*sprite);
                self.visible_sprites_count += 1;
            }
            self.oam_scan_index += 1;
        }

        if self.dots.is_multiple_of(4) {
            self.oam.update_accessed_oam_row(8);
        }

        let sorted: Vec<Sprite>;

        if self.dots >= OAM_DOTS {
            sorted = self.sort_sprites_by_x();
            self.visible_sprites = [None; 10];

            for (i, sprite) in sorted.into_iter().enumerate() {
                self.visible_sprites[i] = Some(sprite);
            }

            self.update_ppu_mode(PpuMode::PixelTransfer);
            self.oam.set_accessed_oam_row(0xFF);
        }
    }

    fn push_pixel_to_screen(&mut self, ct: &mut Box<dyn GameCT>) {
        if let Some(bg_pixel) = self.bg_fifo.pop() {
            if self.pixels_to_discard > 0 {
                self.pixels_to_discard -= 1;
            } else {
                let obj_pixel = self.obj_piso.shift_out();

                let bg_color_index = bg_pixel.get_color_base_index();
                let bg_color = *bg_pixel.get_color();

                let obj_color_index = obj_pixel.get_color_base_index();

                let obj_priority = obj_pixel.get_priority();
                let bg_priority = bg_pixel.get_priority();
                let mut final_color = if obj_color_index != 0
                    && (bg_color_index == 0
                        || self.lcdc_byte & 0b00000001 == 0x00
                        || (!obj_priority && !bg_priority))
                {
                    *obj_pixel.get_color()
                } else {
                    bg_color
                };

                let ly = self.ly as usize;
                let offset = ly * WIN_SIZE_X + self.x;

                final_color = if self.frame_blanked {
                    LCD_OFF_COLOR_CGB
                } else {
                    final_color
                };
                ct.put_pixel_to_frame(offset, final_color.rgb());
                let x = self.x;
                self.x = x + 1;
            }
        }
    }

    fn mode_pixel_transfer(&mut self, ct: &mut Box<dyn GameCT>) {
        if self.ly < WIN_SIZE_Y as u8 {
            let wx = self.wx;

            let use_window = self.read_lcdc().is_window_enabled()
                && self.wy_equal_ly_condition_met
                && (self.x + 7 >= wx as usize);

            self.step_oam_fetcher();

            if !self.fetching_sprite {
                self.step_pixel_fetcher(use_window);
                if self.stall_dots > 0 {
                    self.stall_dots -= 1;
                } else {
                    self.handle_window_switch(use_window);
                    self.push_pixel_to_screen(ct);
                }
            }
        }

        if self.x == 160 {
            self.update_ppu_mode(PpuMode::HBlank);
        }
    }

    fn new(compatibility: bool) -> Self
    where
        Self: Sized,
    {
        Self::new(compatibility)
    }

    fn read_vram(&self, addr: u16) -> u8 {
        self.read_vram(addr)
    }

    fn read_register(&self, addr: u16) -> u8 {
        self.read_register(addr)
    }

    fn write_vram(&mut self, addr: u16, val: u8) {
        self.write_vram(addr, val)
    }

    fn write_register(&mut self, addr: u16, val: u8) {
        self.write_register(addr, val);
    }

    fn read_oam(&mut self, addr: u16) -> u8 {
        self.read_oam(addr)
    }

    fn write_oam(&mut self, addr: u16, value: u8) {
        self.write_oam(addr, value);
    }

    fn pending_vblank(&self) -> bool {
        self.pending_vblank()
    }

    fn set_pending_vblank(&mut self, value: bool) {
        self.set_pending_vblank(value)
    }

    fn pending_stat(&self) -> bool {
        self.pending_stat()
    }

    fn set_pending_stat(&mut self, value: bool) {
        self.set_pending_stat(value)
    }
    fn hdma1(&self) -> u8 {
        self.hdma1
    }
    fn hdma2(&self) -> u8 {
        self.hdma2
    }
    fn hdma3(&self) -> u8 {
        self.hdma3
    }
    fn hdma4(&self) -> u8 {
        self.hdma4
    }
    fn write_hdma_value(&mut self, addr: u16, value: u8) {
        let vbk = self.vram.vbk();
        self.vram.write_with_custom_vbk(addr, value, vbk);
    }
}
