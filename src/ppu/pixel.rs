use crate::ppu::colors_palette::Color;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    color: Color,
    color_index: u8,
    priority: bool,
}

impl Pixel {
    pub fn new_bg(color: Color, color_index: u8) -> Self {
        let priority = false;

        Pixel {
            color,
            color_index,
            priority,
        }
    }

    pub fn new_obj(color: Color, color_index: u8, priority: bool) -> Self {
        Pixel {
            color,
            color_index,
            priority,
        }
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }


    pub fn get_color_index(&self) -> u8 {
        self.color_index
    }

    pub fn get_priority(&self) -> bool {
        self.priority
    }

}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            color: Color::White,
            color_index: 0,
            priority: false,
        }
    }
}
