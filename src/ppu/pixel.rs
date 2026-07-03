use crate::ppu::colors_palette::ColorType;

#[derive(Debug, Copy, Clone)]
pub struct Pixel<C: ColorType> {
    color: C,
    color_index: u8,
    priority: bool,
}

impl<C: ColorType> Pixel<C> {
    pub fn new_bg(color: C, color_index: u8) -> Self {
        let priority = false;
        Self::new_obj(color, color_index, priority)
    }

    pub fn new_obj(color: C, color_index: u8, priority: bool) -> Self {
        Pixel {
            color,
            color_index,
            priority,
        }
    }

    pub fn get_color(&self) -> &C {
        &self.color
    }

    pub fn get_color_index(&self) -> u8 {
        self.color_index
    }

    pub fn get_priority(&self) -> bool {
        self.priority
    }
}

impl<C: ColorType + Copy> Default for Pixel<C> {
    fn default() -> Self {
        Pixel {
            color: ColorType::new(0),
            color_index: 0,
            priority: false,
        }
    }
}
