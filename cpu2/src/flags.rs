use crate::defines::Flag;
pub trait FlagsOps {
    fn get_flag(&self, f: Flag) -> bool;
    fn set_flag(&mut self, f: Flag, value: bool);
}

impl FlagsOps for u8 {
    fn get_flag(&self, f: Flag) -> bool {
        match f {
            Flag::Zero => *self >> 7 & 0b00000001 > 0,
            Flag::Subtract => *self >> 6 & 0b00000001 > 0,
            Flag::HalfCarry => *self >> 5 & 0b00000001 > 0,
            Flag::Carry => *self >> 4 & 0b00000001 > 0,
        }
    }

    fn set_flag(&mut self, f: Flag, value: bool) {
        match f {
            Flag::Zero => {
                if value {
                    *self |= (value as u8) << 7
                } else {
                    *self &= !(1u8 << 7)
                }
            }
            Flag::Subtract => {
                if value {
                    *self |= (value as u8) << 6
                } else {
                    *self &= !(1u8 << 6)
                }
            }
            Flag::HalfCarry => {
                if value {
                    *self |= (value as u8) << 5
                } else {
                    *self &= !(1u8 << 5)
                }
            }
            Flag::Carry => {
                if value {
                    *self |= (value as u8) << 4
                } else {
                    *self &= !(1u8 << 4)
                }
            }
        }
    }
}
