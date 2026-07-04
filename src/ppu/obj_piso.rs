// PISO stands for "Parallel In Serial Out"
// Since it's not a real FIFO in its behavior

use crate::ppu::{
    Cram,
    colors_palette::{CgbColor, ColorType, DmgColor},
    pixel::Pixel,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct ObjPiso<C: ColorType>
where
    C: std::marker::Copy,
{
    pixels: [Pixel<C>; 8],
}

impl<C: ColorType + Copy> Default for ObjPiso<C> {
    fn default() -> Self {
        Self {
            pixels: [Pixel::default(); 8],
        }
    }
}

impl<C: ColorType + Copy> ObjPiso<C> {
    pub fn shift_out(&mut self) -> Pixel<C> {
        let out = self.pixels[0];

        for i in 0..7 {
            self.pixels[i] = self.pixels[i + 1];
        }

        self.pixels[7] = Pixel::default();

        out
    }

    pub fn reset(&mut self) {
        self.pixels = [Pixel::default(); 8];
    }
}

impl ObjPiso<DmgColor> {
    #[allow(clippy::too_many_arguments)]
    pub fn merge(
        &mut self,
        tile_data_low: u8,
        tile_data_high: u8,
        sprite_x: u8,
        x_flip: bool,
        palette: u8,
        priority: bool,
        scanline_x: usize,
        oam_index: u8,
    ) {
        for i in 0..8 {
            let pos = (sprite_x as i16 + i as i16 - 8) - scanline_x as i16;

            // Check if the pixel is outside the fifo
            if !(0..8).contains(&pos) {
                continue;
            }

            // Create the corresponding mask to extract the appropiate bit of tile data
            // given the current fifo index and whether the sprite is horizontally mirrored.
            let bit = if x_flip { i } else { 7 - i };
            let low = (tile_data_low >> bit) & 1;
            let high = (tile_data_high >> bit) & 1;

            // Merge the two bits to create the pixels's color number
            let color_index = low | (high << 1);
            // If the color number of the new pixel is transparent,
            // the pixel at the current fifo index is not modified
            if color_index == 0 {
                continue;
            }

            // Extract the current pixel at the current fifo index to compare it
            // against the new pixel
            let current_pixel = &self.pixels[pos as usize];
            let current_pixel_color_value = current_pixel.get_color_base_index();
            // The current fifo index is set to the new pixel if the current pixel
            // is transparent
            if current_pixel_color_value == 0 {
                let color = DmgColor::apply_background_palette_bgp(color_index, palette);
                self.pixels[pos as usize] = Pixel::new_obj(color, priority, oam_index);
            }
        }
    }
}

impl ObjPiso<CgbColor> {
    #[allow(clippy::too_many_arguments)]
    pub fn merge(
        &mut self,
        tile_data_low: u8,
        tile_data_high: u8,
        sprite_x: u8,
        x_flip: bool,
        palette_index: u8,
        priority: bool,
        scanline_x: usize,
        cram: &Cram,
        oam_index: u8,
        opri: u8,
        obp: u8,
        is_dmg_mode: bool,
    ) {
        for i in 0..8 {
            let pos = (sprite_x as i16 + i as i16 - 8) - scanline_x as i16;

            // Check if the pixel is outside the fifo
            if !(0..8).contains(&pos) {
                continue;
            }

            // Create the corresponding mask to extract the appropiate bit of tile data
            // given the current fifo index and whether the sprite is horizontally mirrored.
            let bit = if x_flip { i } else { 7 - i };
            let low = (tile_data_low >> bit) & 1;
            let high = (tile_data_high >> bit) & 1;

            // Merge the two bits to create the pixels's color number
            let color_index = low | (high << 1);
            // If the color number of the new pixel is transparent,
            // the pixel at the current fifo index is not modified
            if color_index == 0 {
                continue;
            }

            // Extract the current pixel at the current fifo index to compare it
            // against the new pixel
            let current_pixel = &self.pixels[pos as usize];
            let current_pixel_color_value = current_pixel.get_color_base_index();
            let current_pixel_oam_index = current_pixel.get_oam_index();
            // The current fifo index is set to the new pixel if the current pixel
            // is transparent
            if current_pixel_color_value == 0
                || opri & 0b00000001 == 0 && current_pixel_oam_index > oam_index
            {
                // In DMG compatibility mode the color index goes through
                // OBP0/OBP1 before indexing CRAM, but transparency keeps
                // using the raw index.
                let cram_index = if is_dmg_mode {
                    (obp >> (color_index * 2)) & 0b11
                } else {
                    color_index
                };
                let mut color =
                    CgbColor::apply_background_palette_cram(cram, palette_index, cram_index);
                color.base_index = color_index;
                self.pixels[pos as usize] = Pixel::new_obj(color, priority, oam_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ppu::colors_palette::DmgColor;
    use crate::ppu::pixel::Pixel;

    // helper
    fn make_pixel<C: ColorType>(color_index: u8) -> Pixel<C> {
        Pixel::new_obj(ColorType::new(0, color_index), false, 0)
    }

    #[test]
    fn merge_replaces_transparent_pixel() {
        let mut piso = ObjPiso::<DmgColor>::default();

        // sprite x = 8 -> aligned on FIFO[0]
        piso.merge(
            0b1000_0000, // bit 7 = 1 -> pixel visible for i=0
            0,
            8,
            false,
            0b1110_0100,
            false,
            0,
            0,
        );

        assert_ne!(
            piso.pixels[0].get_color_base_index(),
            0,
            "The pixel should not be transparent anymore"
        );
    }

    #[test]
    fn merge_does_not_replace_existing_pixel() {
        let mut piso = ObjPiso::<DmgColor>::default();

        piso.pixels[0] = make_pixel(2);

        // Try to put a pixel at the same place
        piso.merge(0b1000_0000, 0, 8, false, 0b1110_0100, false, 0, 0);

        assert_eq!(
            piso.pixels[0].get_color_base_index(),
            2,
            "The pixel should not change."
        );
    }

    #[test]
    fn merge_ignores_pixels_left_overscan() {
        let mut piso = ObjPiso::<DmgColor>::default();

        // sprite_x = 5 -> some pixels will have pos < 0
        piso.merge(
            0xFF, // all actives pixels
            0,
            5,
            false,
            0b1110_0100,
            false,
            0,
            0,
        );
        assert_ne!(
            piso.pixels[0].get_color_base_index(),
            0,
            "The last slots should not be transparents"
        );
        assert_ne!(
            piso.pixels[1].get_color_base_index(),
            0,
            "The last slots should not be transparents"
        );
        assert_ne!(
            piso.pixels[2].get_color_base_index(),
            0,
            "The last slots should not be transparents"
        );
        assert_ne!(
            piso.pixels[3].get_color_base_index(),
            0,
            "The last slots should not be transparents"
        );
        assert_ne!(
            piso.pixels[4].get_color_base_index(),
            0,
            "The last slots should not be transparents"
        );

        assert_eq!(
            piso.pixels[5].get_color_base_index(),
            0,
            "The first slots have to stay transparents"
        );
        assert_eq!(
            piso.pixels[6].get_color_base_index(),
            0,
            "The first slots have to stay transparents"
        );
        assert_eq!(
            piso.pixels[7].get_color_base_index(),
            0,
            "The first slots have to stay transparents"
        );
    }

    #[test]
    fn merge_ignores_transparent_pixels() {
        let mut piso = ObjPiso::<DmgColor>::default();

        // tile data = 0 -> all pixels transparents
        piso.merge(0, 0, 8, false, 0b1110_0100, false, 0, 0);

        for i in 0..8 {
            assert_eq!(
                piso.pixels[i].get_color_base_index(),
                0,
                "The pixel is not transparent."
            );
        }
    }

    #[test]
    fn shift_out_shifts_correctly() {
        let mut piso = ObjPiso::<DmgColor>::default();

        for i in 0..8 {
            piso.pixels[i] = make_pixel((i % 4) as u8);
        }

        let out = piso.shift_out();

        assert_eq!(
            out.get_color_base_index(),
            0,
            "The first pixel should shift out of the piso"
        );

        // shift
        for i in 0..7 {
            assert_eq!(
                piso.pixels[i].get_color_base_index(),
                ((i + 1) % 4) as u8,
                "A pixel didn't shift."
            );
        }

        assert_eq!(
            piso.pixels[7].get_color_base_index(),
            0,
            "The last pixel should be transparent"
        );
    }

    #[test]
    fn reset_clears_fifo() {
        let mut piso = ObjPiso::<DmgColor>::default();

        piso.pixels[3] = make_pixel(2);

        piso.reset();

        for i in 0..8 {
            assert_eq!(
                piso.pixels[i].get_color_base_index(),
                0,
                "The reset function hasn't erased all pixels."
            );
        }
    }
}
