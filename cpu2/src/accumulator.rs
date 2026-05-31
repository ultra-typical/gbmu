use crate::defines::Accumulator;

impl Accumulator {

    pub fn new() -> Self {
        Accumulator { value: 0, pos: 0 }
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.pos = 0;
    }

    pub fn accumulate_u8(&mut self, value: u8) {
        if self.pos > 3 {
            panic!("Accumulator overflow")
        }
        self.value |= (value as u32) << (self.pos * 8);
        self.pos += 1;
    }

    pub fn accumulate_u16(&mut self, value: u16) {
        if self.pos > 2 {
            panic!("Accumulator overflow")
        }
        self.value |= (value as u32) << (self.pos * 8);
        self.pos += 2;
    }

    pub fn get_u16_at(&self, index: usize) -> u16 {
        if index >= 2 {
            panic!("Accumulator index out of bounds")
        }
        if self.pos < 2 {
            panic!("Accumulator not enough data")
        }
        (self.value >> (index * 16)) as u16
    }

    pub fn get_u8_at(&self, index: usize) -> u8 {
        if index >= 4 {
            panic!("Accumulator index out of bounds")
        }
        (self.value >> (index * 8)) as u8
    }

}




#[test]
fn test_accumulator_accumulate_u16() {
    let mut acc = Accumulator::new();
    acc.accumulate_u16(0x1234);
    acc.accumulate_u16(0x5678);
    assert_eq!(acc.get_u16_at(0), 0x1234);
    assert_eq!(acc.get_u16_at(1), 0x5678);
}

#[test]
fn test_accumulator_get_u16_at() {
    let mut acc = Accumulator::new();
    acc.accumulate_u8(0x12);
    acc.accumulate_u8(0x34);
    acc.accumulate_u8(0x56);
    acc.accumulate_u8(0x78);
    println!("Accumulated value: {:b}", acc.value);
    assert_eq!(acc.get_u16_at(0), 0x3412);
    assert_eq!(acc.get_u16_at(1), 0x7856);
}

#[test]
fn test_accumulator_get_u8_at() {
    let mut acc = Accumulator::new();
    acc.accumulate_u8(0x12);
    acc.accumulate_u8(0x34);
    acc.accumulate_u8(0x56);
    acc.accumulate_u8(0x78);
    println!("Accumulated value: {:b}", acc.value);
    assert_eq!(acc.get_u8_at(0), 0x12);
    assert_eq!(acc.get_u8_at(1), 0x34);
    assert_eq!(acc.get_u8_at(2), 0x56);
    assert_eq!(acc.get_u8_at(3), 0x78);
}

#[test]
#[should_panic]
fn test_accumulator_overflow() {
    let mut acc = Accumulator::new();
    acc.accumulate_u8(0b10010);
    acc.accumulate_u8(0b110100);
    acc.accumulate_u8(0b1011010);
    acc.accumulate_u8(0b1111000);
    acc.accumulate_u8(0b1111000);
    assert_eq!(acc.get_u16_at(0), 0b11110001011010);
}