#![allow(unused_variables)]

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

const WHITE: &[u8; 3] = &[255, 255, 255];
const LIGHTGRAY: &[u8; 3] = &[192, 192, 192];
const DARKGRAY: &[u8; 3] = &[96, 96, 96];
const BLACK:   &[u8; 3] = &[0, 0, 0];

impl Color {
    pub fn to_rgb(self) -> &'static [u8; 3] {
        match self {
            Color::White => WHITE,
            Color::LightGray => LIGHTGRAY,
            Color::DarkGray => DARKGRAY ,
            Color::Black => BLACK,
        }
    }

    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => unreachable!(),
        }
    }
}
