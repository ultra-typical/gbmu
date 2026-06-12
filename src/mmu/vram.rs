pub struct CgbVram {
    bank0: Box<[u8; 0x2000]>,
    bank1: Box<[u8; 0x2000]>,
    vbk: u8,        // 0xFF4F
}

pub struct DmgVram {
    bank0: Box<[u8; 0x2000]>,
}

pub trait Vram {
    fn new() -> Self;
    fn write(&mut self, addr: u16, byte: u8);
    fn read(&self, addr: u16) -> u8;
}

impl Vram for DmgVram {
    fn new() -> Self {
        Self {
            bank0: Box::new([0; 0x2000]),
        }
    }
    fn write(&mut self, addr: u16, byte: u8) {
        let addr_in_bank = addr - 0x8000;
        self.bank0[addr_in_bank as usize] = byte;
    }
    fn read(&self, addr: u16) -> u8 {
        let addr_in_bank = addr - 0x8000;
        self.bank0[addr_in_bank as usize]
    }
}

impl Vram for CgbVram {
    fn new() -> Self {
        Self {
            bank0: Box::new([0x00; 0x2000]),
            bank1: Box::new([0x00; 0x2000]),
            vbk: 0x00
        }
    }
    fn write(&mut self, addr: u16, byte: u8) {
        let addr_in_bank = addr - 0x8000;
        match self.vbk {
            0x00 => self.bank0[addr_in_bank as usize] = byte,
            0x01 => self.bank1[addr_in_bank as usize] = byte,
            _ => {}
        }
    }
    fn read(&self, addr: u16) -> u8 {
        let addr_in_bank = addr - 0x8000;
        match self.vbk {
            0x00 => self.bank0[addr_in_bank as usize],
            0x01 => self.bank1[addr_in_bank as usize],
            _ => unreachable!()
        }
    }
}
