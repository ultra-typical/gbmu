pub mod colors_palette;
mod lcd_control;
mod lcd_status;
mod pixel;
mod pixel_fifo;
mod obj_piso;
mod pixel_fetcher;
mod oam_fetcher;


use std::cell::RefCell;
use std::rc::Rc;

use crate::communications::GameCT;
use crate::mmu::mbc::Mbc;
use crate::mmu::Mmu;
use crate::mmu::oam::Sprite;
use crate::mmu::interrupt::Interrupt;
use crate::ppu::colors_palette::Color;
use crate::ppu::lcd_control::LcdControl;
use crate::ppu::lcd_status::LcdStatus;
use crate::ppu::lcd_status::PpuMode;
use crate::ppu::pixel::Pixel;
use crate::ppu::pixel_fifo::PixelFifo;
use crate::ppu::obj_piso::ObjPiso;
use crate::ppu::pixel_fetcher::PixelFetcher;
use crate::ppu::oam_fetcher::OamFetcher;

pub const WIN_SIZE_X: usize = 160; // Window size in X direction
pub const WIN_SIZE_Y: usize = 144; // Window size in Y direction
const LYC_ADDR: u16 = 0xFF45; // LY Compare
const STAT_ADDR: u16 = 0xFF41; // LCDC Status
const SCX_ADDR: u16 = 0xFF43; // Scroll X
const SCY_ADDR: u16 = 0xFF42; // Scroll Y
const BGP_ADDR: u16 = 0xFF47; // Background Palette
const WY_ADDR: u16 = 0xFF4A; // Window Y Position
const WX_ADDR: u16 = 0xFF4B; // Window X Position
const LCD_CONTROL_ADDR: u16 = 0xFF40; // LCDC Control

const OAM_DOTS: u32 = 80; // always 80
const SCANLINE_DOTS: u32 = 456; // always 456

pub struct Ppu<T: Mbc> {
    pub bus: Rc<RefCell<Mmu<T>>>,
    pub dots: u32,
    lcd_status: LcdStatus, // LCD Status register
    wly: u8,               // Window internal line counter
    ly: u8,
    internal_ly: u8, // needed for the ly=153 quirk
    x: usize,
    pixel_fetcher: PixelFetcher,
    oam_fetcher: OamFetcher,
    bg_fifo: PixelFifo, // Background pixel FIFO
    obj_piso: ObjPiso, // Objects (sprites) PISO
    visible_sprites: [Option<Sprite>; 10],
    pixels_to_discard: u8, // Required in order to prevent the SCX misalignment bug
    use_window: bool, // Required for BG FIFO in order to know if the window is activated midline
    wx_at_window_start: u8, // Required to handle the WX hardware glitch
    is_wx_glitch_happened: bool, // Required to handle the WX hardware glitch
    fetching_sprite: bool, // pixel fetcher and pixel shifter need to be paused while oam fetcher is called
    current_sprite_to_fetch: Option<usize>,
    wy_equal_ly_condition_met: bool,
    oam_scan_index: u8, // index scanned sprites in OAM Search
    visible_sprites_count: u8,
    current_obj_height: u8,
    lcd_was_enabled: bool, // for the LCD on/off quirk. We need to detect if the ppu is on for the first time since it was off.
    is_first_scanline_after_lcd_on: bool, // for the LCD on/off quirk. If first scanline since the ppu is on, the cycle is shorter.
    stat_interrupt_line: bool,
    stall_dots: u8, // to handle the sprite penalty in mode pixel transfer
}

impl<T: Mbc> Ppu<T> {
    pub fn new(bus: Rc<RefCell<Mmu<T>>>) -> Self {
        Ppu {
            bus,
            dots: 0,
            lcd_status: LcdStatus::new(),
            wly: 0x00,
            ly: 0x00,
            internal_ly: 0x00,
            x: 0,
            pixel_fetcher: PixelFetcher::default(),
            oam_fetcher: OamFetcher::default(),
            bg_fifo: PixelFifo::default(),
            obj_piso: ObjPiso::default(),
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
        }
    }



    fn apply_background_palette(&self, color_index: u8) -> Color {
        let palette = self.bus.borrow_mut().read_byte(BGP_ADDR);

        let index = (palette >> (color_index * 2)) & 0b11;

        Color::from_index(index)
    }


    fn sort_sprites_by_x(&self) -> Vec<Sprite> {
        let mut sprites: Vec<(usize, Sprite)> = self.visible_sprites
            .iter()
            .enumerate()
            .filter_map(| (i, s) | s.map(| sprite | (i, sprite)))
            .collect();

        sprites.sort_by(| (index_a, sprite_a), (index_b, sprite_b) | {
            if sprite_a.x != sprite_b.x {
                sprite_a.x.cmp(&sprite_b.x)
            } else {
                index_a.cmp(index_b)
            }
        });

        sprites.into_iter().map(| (_, s) | s).collect()
    }


    fn mode_oam_search(&mut self) -> bool {
        if self.dots == 1 {
            self.bus.borrow_mut().set_accessed_oam_row(0);
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
            let mmu = self.bus.borrow_mut();
            let oam = mmu.get_oam();

            let mut sprite = oam.sprites[self.oam_scan_index as usize];

            if sprite.is_visible(self.ly, self.current_obj_height)
                && self.visible_sprites_count < 10 {
                sprite.oam_index = self.oam_scan_index;

                self.visible_sprites[self.visible_sprites_count as usize] = Some(sprite);

                self.visible_sprites_count += 1;
            }
            self.oam_scan_index += 1;
        }

        // accessed_oam_row count in M-cycles
        if self.dots.is_multiple_of(4){
            self.bus.borrow_mut().update_accessed_oam_row(8);
        }

        if self.dots >= OAM_DOTS {
            let sorted = self.sort_sprites_by_x();
            self.visible_sprites = [None; 10];

            for (i, sprite) in sorted.into_iter().enumerate() {
                self.visible_sprites[i] = Some(sprite);
            }

            self.update_ppu_mode(PpuMode::PixelTransfer);
            self.bus.borrow_mut().set_accessed_oam_row(0xFF);
        }

        false
    }

    fn handle_window_switch(&mut self, use_window: bool) {
        // check if window is activated in the middle of scanline
        if !self.use_window && use_window {
            self.pixel_fetcher.reset_for_window();
            self.bg_fifo.clear();

            self.wx_at_window_start = self.read_wx();
            self.pixels_to_discard = 0;
        }

        self.use_window = use_window;

        // check wx glitch
        let wx = self.read_wx();
        if self.use_window && wx != self.wx_at_window_start
            && self.x + 7 >= wx as usize
            && !self.is_wx_glitch_happened {
                let glitched_pixel = Pixel::new_bg(self.apply_background_palette(0),  0);

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
                let bg_color: Color;

                // If BG is disabled, color 0 everywhere
                if !self.read_lcdc().is_bg_window_enabled() {
                    bg_color_index = 0;
                    bg_color = self.apply_background_palette(0);
                }
                else {
                    bg_color_index = bg_pixel.get_color_index();
                    bg_color = *bg_pixel.get_color();
                }

                let obj_color_index = obj_pixel.get_color_index();

                let final_color = if obj_color_index == 0 {
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
                ct.put_pixel_to_frame(offset, final_color);
                self.x += 1;
            }
        }

    }


    fn step_pixel_fetcher(&mut self, use_window: bool) {
        let scy = self.read_scy();
        let scx = self.read_scx();

        let tile_pixels = self.pixel_fetcher.tick(&self.bus,
            &self.bg_fifo,
            self.ly,
            scx,
            scy,
            self.wly,
            &self.read_lcdc(),
            use_window
        );

        if let Some(pixels) = tile_pixels {
            for pixel in pixels {
                self.bg_fifo.push(pixel);
            }
        }
    }


    fn step_oam_fetcher(&mut self) {
        let height:u8 = if self.read_lcdc().is_obj_size_8x16() {
            16
        } else {
            8
        };

        if self.fetching_sprite {
            if let Some(index) = self.current_sprite_to_fetch 
                && let Some(sprite) = self.visible_sprites[index] {

                self.fetching_sprite = !self.oam_fetcher.tick(
                    &self.bus,
                    &sprite,
                    &mut self.obj_piso,
                    self.ly,
                    height,
                    self.x,
                );

                if !self.fetching_sprite {
                    self.visible_sprites[index] = None;

                    let remaining_pixels = self.bg_fifo.len() as u8;
                    if remaining_pixels < 6 {
                        self.stall_dots = 6 - remaining_pixels;
                    }
                }
            };
        }
        else {
            if !self.read_lcdc().is_obj_enabled() { return; }

            for (index, sprite_opt) in self.visible_sprites.iter_mut().enumerate() {
                if let Some(sprite) = sprite_opt 
                    && sprite.x as usize <= self.x + 8 {
                    self.current_sprite_to_fetch = Some(index);
                    self.pixel_fetcher.reset_to_state_1();

                    self.fetching_sprite = !self.oam_fetcher.tick(
                        &self.bus,
                        sprite,
                        &mut self.obj_piso,
                        self.ly,
                        height,
                        self.x,
                    );

                    if !self.fetching_sprite {
                        *sprite_opt = None;
                    }

                    break;
                }
            }
        }
    }


    fn mode_pixel_transfer(&mut self, ct: &mut Box<dyn GameCT>) -> bool {
        if self.ly < WIN_SIZE_Y as u8 {
            let wx = self.read_wx();

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

        false
    }

    fn reset_for_new_scanline(&mut self) {
        self.x = 0;
        self.bg_fifo.clear();
        self.obj_piso.reset();
        self.pixel_fetcher.reset_for_scanline();       
        self.pixels_to_discard = self.read_scx() % 8;
        self.use_window = false;
        self.is_wx_glitch_happened = false;
        self.is_first_scanline_after_lcd_on = false;
        self.stall_dots = 0;
    }

    fn advance_to_next_scanline(&mut self) {
        let wy = self.read_wy();
        let wx = self.read_wx();

        if self.read_lcdc().is_window_enabled()
            && self.ly >= wy
            && wx <= 166 {
            self.wly += 1;
        }

        self.ly += 1;
        self.internal_ly += 1;
        self.write_ly_to_mmu();

        self.check_lyc_equals_ly();

        self.reset_for_new_scanline();
    }

    // End of scanline
    fn mode_hblank(&mut self) -> bool {
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

                self.bus.borrow_mut().interrupts_request(Interrupt::VBlank);

                return true;
            } else {
                self.update_ppu_mode(PpuMode::OamSearch);
            }
        }
        false
    }

    fn handle_ly153_quirk(&mut self) {
        self.ly = 0;
        self.write_ly_to_mmu();
        self.check_lyc_equals_ly();
    }

    fn end_frame(&mut self) {
        self.internal_ly = 0;
        self.ly = 0;
        self.write_ly_to_mmu();
        
        self.wly = 0;
        self.reset_for_new_scanline();
        self.wy_equal_ly_condition_met = false;

        self.update_ppu_mode(PpuMode::OamSearch);
    }

    fn advance_vblank_scanline(&mut self) {
        self.internal_ly += 1;
        self.ly = self.internal_ly;
        self.write_ly_to_mmu();
        self.check_lyc_equals_ly();
    }

    // End of frame
    fn mode_vblank(&mut self) -> bool {
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
            
        false
    }

    fn reset_when_ppu_disabled(&mut self) {
        self.ly = 0;
        self.internal_ly = 0;
        self.write_ly_to_mmu();

        self.dots = 0;
        self.update_ppu_mode(PpuMode::HBlank);

        self.lcd_was_enabled = false;
        self.stat_interrupt_line = false;
    }

    pub fn tick(&mut self, ct: &mut Box<dyn GameCT>)-> bool {
        self.read_lcd_status();
        self.check_lyc_equals_ly();

        if !self.read_lcdc().is_ppu_enabled() {
            self.reset_when_ppu_disabled();
            return false;
        }

        if !self.lcd_was_enabled {
            self.is_first_scanline_after_lcd_on = true;
            self.lcd_was_enabled = true;
        }

        self.dots += 1;

        let wy = self.read_wy();

        if wy == self.ly { self.wy_equal_ly_condition_met = true; }

        let was_updated = match self.lcd_status.get_ppu_mode() {
            PpuMode::OamSearch => self.mode_oam_search(),
            PpuMode::PixelTransfer => self.mode_pixel_transfer(ct),
            PpuMode::HBlank => self.mode_hblank(),
            PpuMode::VBlank => self.mode_vblank(),
        };

        self.evaluate_stat_interrupt();

        was_updated
    }

    fn check_lyc_equals_ly(&mut self) {
        /*
            LYC == LY is an hardware condition:
                - update a flag in STAT
                - can trigger a LCD STAT interrupt
            It's used by many games to synchronize with scanline
        */
        let lyc_match = self.ly == self.read_lyc();
        self.lcd_status.set_lyc_equals_ly(lyc_match);
        self.write_stat_to_mmu();
        
        // if self.lcd_status.get_lyc_equals_ly() {
        //     self.bus.borrow_mut().interrupts_request(Interrupt::LcdStat);
        // }
    }

    fn update_ppu_mode(&mut self, mode: PpuMode) {
        self.lcd_status.update_ppu_mode(mode);
        self.write_stat_to_mmu();
    }

    fn evaluate_stat_interrupt(&mut self) {
        let current_line = self.lcd_status.stat_interrupt_line();

        if !self.stat_interrupt_line && current_line {
            self.bus.borrow_mut().interrupts_request(Interrupt::LcdStat);
        }

        self.stat_interrupt_line = current_line;
    }


    fn read_lcd_status(&mut self) {
        let stat_byte = {
            let bus = self.bus.borrow_mut();
            bus.read_byte(STAT_ADDR)
        };

        self.lcd_status.update_from_byte(stat_byte);
    }

    fn read_scy(&self) -> u8 {
        let bus = self.bus.borrow_mut();
        bus.read_byte(SCY_ADDR)
    }

    fn read_scx(&self) -> u8 {
        let bus = self.bus.borrow_mut();
        bus.read_byte(SCX_ADDR)
    }

    fn read_wy(&self) -> u8 {
        let bus = self.bus.borrow_mut();
        bus.read_byte(WY_ADDR)
    }

    fn read_wx(&self) -> u8 {
        let bus = self.bus.borrow_mut();
        bus.read_byte(WX_ADDR)
    }

    fn read_lyc(&self) -> u8 {
        let bus = self.bus.borrow_mut();
        bus.read_byte(LYC_ADDR)
    }

    fn read_lcdc(&self) -> LcdControl {
        let bus = self.bus.borrow_mut();
        let byte = bus.read_byte(LCD_CONTROL_ADDR);

        LcdControl::from_byte(byte)
    }

    fn write_ly_to_mmu(&mut self) {
        let mut bus = self.bus.borrow_mut();
        bus.set_ly_from_ppu(self.ly);
    }

    fn write_stat_to_mmu(&mut self) {
        let mut bus = self.bus.borrow_mut();
        bus.set_stat_byte_from_ppu(self.lcd_status.struct_to_byte());
    }
}
