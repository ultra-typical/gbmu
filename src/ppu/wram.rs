use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wram {
    bank0: Vec<u8>,
    bank1: Vec<u8>,
    bank2: Vec<u8>,
    bank3: Vec<u8>,
    bank4: Vec<u8>,
    bank5: Vec<u8>,
    bank6: Vec<u8>,
    bank7: Vec<u8>,
    svbk_wbk: u8, //0xFF70
}

impl Wram {
    pub fn new() -> Self {
        Self {
            bank0: vec![0x00; 0x1000],
            bank1: vec![0x00; 0x1000],
            bank2: vec![0x00; 0x1000],
            bank3: vec![0x00; 0x1000],
            bank4: vec![0x00; 0x1000],
            bank5: vec![0x00; 0x1000],
            bank6: vec![0x00; 0x1000],
            bank7: vec![0x00; 0x1000],
            svbk_wbk: 0x00,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0xC000..=0xCFFF => self.bank0[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => match self.svbk_wbk() {
                0x00 => self.bank1[(addr - 0xD000) as usize],
                0x01 => self.bank1[(addr - 0xD000) as usize],
                0x02 => self.bank2[(addr - 0xD000) as usize],
                0x03 => self.bank3[(addr - 0xD000) as usize],
                0x04 => self.bank4[(addr - 0xD000) as usize],
                0x05 => self.bank5[(addr - 0xD000) as usize],
                0x06 => self.bank6[(addr - 0xD000) as usize],
                0x07 => self.bank7[(addr - 0xD000) as usize],
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0xC000..=0xCFFF => self.bank0[(addr - 0xC000) as usize] = data,
            0xD000..=0xDFFF => match self.svbk_wbk() {
                0x00 => self.bank1[(addr - 0xD000) as usize] = data,
                0x01 => self.bank1[(addr - 0xD000) as usize] = data,
                0x02 => self.bank2[(addr - 0xD000) as usize] = data,
                0x03 => self.bank3[(addr - 0xD000) as usize] = data,
                0x04 => self.bank4[(addr - 0xD000) as usize] = data,
                0x05 => self.bank5[(addr - 0xD000) as usize] = data,
                0x06 => self.bank6[(addr - 0xD000) as usize] = data,
                0x07 => self.bank7[(addr - 0xD000) as usize] = data,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn svbk_wbk(&self) -> u8 {
        self.svbk_wbk & 0b00000111
    }

    pub fn set_svbk_wbk(&mut self, wbk: u8) {
        self.svbk_wbk = wbk;
    }
}
