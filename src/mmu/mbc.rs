#![allow(unused_variables)]
use std::cmp::min;

use chrono::{Local, DateTime};
const ONLY_ROM_SIZE: usize = 0xC000;
const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x2000;

pub trait Mbc {
    fn new(rom_image: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Result<Self, String> where Self: Sized;
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
    fn dump(&self) -> Option<Vec<u8>>;
}

#[derive(Clone)]
pub  struct Mbc1 {
    banks: Vec<[u8; ROM_BANK_SIZE]>,
    ram_gate_register: bool, // If ramg is set to 0b1010 -> 
    bank_register_1: u8,
    bank_register_2: u8,
    mode_register: bool,
    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,
}

fn get_rom_bank_size(rom: &[u8]) -> Result<usize, String>{
    let code = rom[0x148];
    match code {
        0 => Ok(2),
        1 => Ok(4),
        2 => Ok(8),
        3 => Ok(16),
        4 => Ok(32),
        5 => Ok(64),
        6 => Ok(128),
        7 => Ok(256),
        8 => Ok(512),
        _ => Err(format!("Rom size code can't be {}", code))
    }
}

fn get_ram_bank_size(rom: &[u8]) -> Result<usize, String>{
    let code = rom[0x149];
    match code {
        0 => Ok(0),
        1 => Ok(0),
        2 => Ok(1),
        3 => Ok(4),
        4 => Ok(16),
        5 => Ok(8),
        _ => Err(format!("Ram size code can't be {}", code)),
    }
}

fn map_rom_into_bank(rom_image: &[u8]) -> Result<Vec<[u8; ROM_BANK_SIZE]>, String> {
    let banks: Vec<[u8; ROM_BANK_SIZE]> = rom_image .chunks_exact(ROM_BANK_SIZE)
        .map(|slice|{
            let mut data = [0; ROM_BANK_SIZE];
            data.copy_from_slice(slice);
            data
        }).collect();
    let supposed_rom_bank_size = get_rom_bank_size(rom_image)?;
    println!("ram banks count {}", banks.len());
    if banks.len() != supposed_rom_bank_size {
        return Err(
            format!("Inconsistent Rom Header : size must be : {}", supposed_rom_bank_size)
        );
    }
    Ok(banks)
}

fn map_ram_banks(rom_image: &[u8], saved_ram: Option<Vec<u8>>) -> Result<Vec<[u8; RAM_BANK_SIZE]>, String> {
    let supposed_ram_bank_size = get_ram_bank_size(rom_image)?;
    println!("rom banks count {}", supposed_ram_bank_size);
    let Some(saved_ram) = saved_ram else {
        return Ok(vec![[0u8; RAM_BANK_SIZE]; supposed_ram_bank_size]);
    };
    let ram_banks: Vec<[u8; RAM_BANK_SIZE]> = saved_ram.chunks(RAM_BANK_SIZE).map(
        | chunk | chunk.try_into().expect("Invalid size of saved ram file")
    ).collect();

    if ram_banks.len() == supposed_ram_bank_size {
        Ok(ram_banks)
    } else {
        Err(
            format!(
                "Invalid number of ram in banks supposed != detected {} != {}",
                supposed_ram_bank_size,
                ram_banks.len()
            )
        )
    }
}

impl Mbc for Mbc1 {
    fn dump(&self) -> Option<Vec<u8>> {
        self.ram_banks.concat().into()
    }

    fn new(rom_image: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Result<Self, String> {
        println!("new Mbc1");
        let banks = map_rom_into_bank(&rom_image)?;
        let ram_banks = map_ram_banks(&rom_image, saved_ram)?;
        if ram_banks.len() > 4 {
            Err(
                "Supposed ram bank size can't be more than 4 in mbc1 cartridge.".to_string()
            )
        } else {
            Ok(Mbc1 {
                banks,
                ram_gate_register: false,
                bank_register_1: 0b1,
                bank_register_2: 0b0,
                mode_register: false,
                ram_banks,
            })
        }

    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..0x4000 => {
                if self.mode_register {
                    self.banks[0][addr as usize]
                } else {
                    self.banks[(self.bank_register_2 << 5) as usize][addr as usize]
                }
            },
            0x4000..0x8000 => {
                self.banks[
                    ((self.bank_register_2 << 5) + self.bank_register_1) as usize
                ][
                    addr as usize - ROM_BANK_SIZE
                ]
            },
            0xA000..0xC000 => {
                if self.ram_gate_register {
                    self.ram_banks[
                        self.mode_register as usize * self.bank_register_2 as usize
                    ][addr as usize - 0xA000]
                } else {
                    0
                }
            },
            _ => unreachable!()
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..0x2000 => self.ram_gate_register = (val & 0b1111) == 0b1010,
            0x2000..0x4000 => self.bank_register_1 = val & 0b11111,
            0x4000..0x6000 => self.bank_register_2 = val & 0b11,
            0x6000..0x8000 => self.mode_register = (val & 0b1) == 0b1,
            0xA000..0xC000 => {
                if self.ram_gate_register {
                    self.ram_banks[
                        self.mode_register as usize * self.bank_register_2 as usize
                    ][addr as usize - 0xA000] = val
                }
            },
            _ => unreachable!()
        }
    }
}

pub struct Mbc2 {
    rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    ram_gate_register: bool,
    rom_bank_register: u8,
    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,
}

impl Mbc for Mbc2 {
    fn dump(&self) -> Option<Vec<u8>> {
        self.ram_banks.concat().into()
    }
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..0x4000 => {
                self.rom_banks[0][addr as usize]
            },
            0x4000..0x8000 => {
                self.rom_banks[self.rom_bank_register as usize][(addr - 0x4000) as usize]
            },
            0xA000..0xC000 => {
                if self.ram_gate_register {
                    self.ram_banks[0][(addr & 0b1111_1111) as usize]
                } else {
                    0
                }
            }
            _ => unreachable!()
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..0x4000 => {
                if addr & 0b1_0000_0000 == 0b1_0000_0000 {
                    self.ram_gate_register = (val & 0b1111) == 0b1010
                } else {
                    let new_value = val & 0b1111;
                    self.rom_bank_register = (new_value == 0) as u8 + new_value;
                }
            },
            0x4000..0x8000 => { }, // do nothing 
            0xA000..0xC000 => { 
                if self.ram_gate_register {
                    self.ram_banks[0][(addr & 0b1111_1111) as usize] = val;
                }

            },
            _ => unreachable!()
        }
    }

    fn new(rom_image: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Result<Self, String> where Self: Sized {
        println!("New Mbc2");
        let rom_banks = map_rom_into_bank(&rom_image)?;
        let ram_banks = map_ram_banks(&rom_image, saved_ram);
        Ok(Mbc2{
            rom_banks,
            ram_gate_register: false,
            rom_bank_register: 0b0001,
            ram_banks: vec![[0; RAM_BANK_SIZE]; 1],
        })
    }
}

pub struct RomOnly {
    bank: Box<[u8; ONLY_ROM_SIZE]>,
}

impl Mbc for RomOnly{
    fn dump(&self) -> Option<Vec<u8>> { None }
    fn new(rom_image: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Result<Self, String> {
        println!("New Romonly");
        let mut bank = Box::new([0; ONLY_ROM_SIZE]);
        let end = min(ONLY_ROM_SIZE, rom_image.len());
        bank[..end].copy_from_slice(&rom_image[..end]);
        Ok(RomOnly {
            bank
        })
    }

    fn read(&self, addr: u16) -> u8 {
        self.bank[addr as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        if (0xA000..0xC000).contains(&addr) {
            self.bank[addr as usize] = val
        }

    }
}

#[derive(Clone, Default, Debug)]
struct RtcRegisters {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day_counter: u16, // 9 used bits
    halted: bool,
    day_carry: bool,
}

#[derive(Clone, Debug)]
struct Rtc {
    base: RtcRegisters,
    base_timestamp: DateTime<Local>,
    latched: Option<RtcRegisters>,
}

impl Rtc {
    fn new() -> Self {
        println!("New Rtc");
        Rtc {
            base: RtcRegisters::default(),
            base_timestamp: Local::now(),
            latched: None,
        }
    }

    pub fn read_registers(&self, selector: u8) -> u8 {
        let registers = self.latched.clone().unwrap_or_else(|| self.current());

        match selector {
            0x08 => registers.seconds,
            0x09 => registers.minutes,
            0x0A => registers.hours,
            0x0B => (registers.day_counter & 0xFF) as u8,
            0x0C => {
                let mut dh = 0u8;
                if registers.day_counter & 0b1_0000_0000 != 0 { dh |= 1; }
                if registers.halted { dh |= 0b0100_0000; }
                if registers.day_carry { dh |= 0b1000_0000; }

                dh
            },
            _ => 0xFF
        }
    }

    pub fn write_registers(&mut self, selector: u8, val: u8) {
        let mut registers = self.current(); 

        match selector {
            0x08 => registers.seconds = val & 0b0011_1111, // Max 59
            0x09 => registers.minutes = val & 0b0011_1111, // Max 59
            0x0A => registers.hours = val & 0b0001_1111,   // Max 23
            0x0B => registers.day_counter = (registers.day_counter & 0b1_0000_0000) | (val as u16),
            0x0C => {
                registers.day_counter = (registers.day_counter & 0xFF) | (((val & 1) as u16) << 8);
                registers.halted = (val & 0b0100_0000) != 0;
                registers.day_carry = (val & 0b1000_0000) != 0;
            },
            _ => return,
        }
        
        self.base = registers;
        self.base_timestamp = Local::now();
    }

    pub fn current(&self) -> RtcRegisters {
        if self.base.halted {
            return self.base.clone();
        }

        let elapsed = Local::now() - self.base_timestamp;
        let elapsed_secs = elapsed.num_seconds().max(0);

        let total_seconds = self.base.seconds as u64 + elapsed_secs as u64;
        let new_seconds = total_seconds % 60;
        let carry_minutes = total_seconds / 60;

        let total_minutes = self.base.minutes as u64 + carry_minutes;
        let new_minutes = total_minutes % 60;
        let carry_hours = total_minutes / 60;

        let total_hours = self.base.hours as u64 + carry_hours;
        let new_hours = total_hours % 24;
        let carry_days = total_hours / 24;

        let total_days = self.base.day_counter as u64 + carry_days;
        let new_days = total_days % 512;

        let new_day_carry = self.base.day_carry || total_days >= 512;

        RtcRegisters {
            seconds: new_seconds as u8,
            minutes: new_minutes as u8,
            hours: new_hours as u8,
            day_counter: new_days as u16,
            halted: false,
            day_carry: new_day_carry,
        }
    }
}

pub struct Mbc3 {
    rtc: Rtc,
    ram_timer_enable: bool,
    latch_clock_data: bool,
    rom_bank_nb: u8,
    ram_rtc_select: u8,
    rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,
}

impl Mbc for Mbc3 {
    fn dump(&self) -> Option<Vec<u8>> {
        self.ram_banks.concat().into()
    }
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..0x4000 => self.rom_banks[0][addr as usize],
            0x4000..0x8000 => self.rom_banks[self.rom_bank_nb as usize][(addr - 0x4000) as usize],
            0xA000..0xC000 => {
                if !self.ram_timer_enable {
                    return 0xFF;
                }

                let selector = self.ram_rtc_select as usize;
                match selector {
                    0x00..0x08
                        if selector < self.ram_banks.len() => {
                            self.ram_banks[selector][(addr - 0xA000) as usize]
                    },
                    0x08..0x0D => {
                        self.rtc.read_registers(selector as u8)
                    },
                    _ => 0xFF,
                }
            },
            _ => unreachable!(),
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..0x2000 => self.ram_timer_enable = (val & 0b1111) == 0b1010,
            0x2000..0x4000 => {
                let masked_val = val & 0b0111_1111;

                self.rom_bank_nb = (masked_val != 0) as u8 * masked_val + (masked_val == 0) as u8
            },
            0x4000..0x6000 => self.ram_rtc_select = val,
            0x6000..0x8000 => {
                let new_bit = (val & 0b0000_0001) == 1;

                if !self.latch_clock_data && new_bit {
                    self.rtc.latched = Some(self.rtc.current());
                }

                self.latch_clock_data = new_bit;
            },
            0xA000..0xC000 => {
                if !self.ram_timer_enable {
                    return;
                }

                match self.ram_rtc_select {
                   0x00..0x08 => {
                        let bank = self.ram_rtc_select as usize;
                        if bank < self.ram_banks.len() {
                            self.ram_banks[bank][(addr - 0xA000) as usize] = val;
                        }
                    },
                    0x08..0x0D => {
                        self.rtc.write_registers(self.ram_rtc_select, val);
                    },
                    _ => {}
                }
            }
            _ => unreachable!(),
        }
        
    }
    fn new(rom_image: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Result<Self, String> where Self: Sized {
        let rom_banks = map_rom_into_bank(&rom_image)?;
        let ram_banks = map_ram_banks(&rom_image, saved_ram)?;

        println!("New Mbc3");

        Ok(
            Mbc3 {
                rtc: Rtc::new(),
                rom_banks,
                ram_banks,
                latch_clock_data: false,
                ram_timer_enable: false,
                rom_bank_nb: 0,
                ram_rtc_select: 0,
            }
        )
    }
}

#[allow(unused)]
pub struct Mbc5 {
    ram_gate_enable: bool,
    rom_bank_register: u16,
    ram_bank_register: u8,
    rumble: bool,
    rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,
    
}

impl Mbc for Mbc5 {
    fn dump(&self) -> Option<Vec<u8>> {
        self.ram_banks.concat().into()
    }

    fn new(rom_image: Vec<u8>, saved_ram: Option<Vec<u8>>) -> Result<Self, String> where Self: Sized {
        println!("New Mbc5");
        let rom_banks = map_rom_into_bank(&rom_image)?;
        let ram_banks = map_ram_banks(&rom_image, saved_ram)?;

        Ok(
            Mbc5 {
                rom_banks,
                ram_banks,
                ram_gate_enable: false,
                rom_bank_register: 0,
                ram_bank_register: 0,
                rumble: false,
            }
        )
    }
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..0x4000 => self.rom_banks[0][addr as usize],
            0x4000..0x8000 => {
                self.rom_banks[
                    (self.rom_bank_register as usize) % self.rom_banks.len()
                ][
                    (addr  - 0x4000) as usize
                ]
            },
            0xA000..0xC000 => {
                self.ram_banks[
                    self.ram_bank_register as usize
                ][
                    (addr - 0xA000) as usize
                ]
            },
            _ => unreachable!(),
        }
    }
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..0x2000 => self.ram_gate_enable = val == 0b0000_1010,
            0x2000..0x3000 => self.rom_bank_register &= 0x100 + val as u16,
            0x3000..0x4000 => self.rom_bank_register = self.rom_bank_register & (0x0FF + val as u16) & 0x100,
            0x4000..0x6000 => {
                self.ram_bank_register = val & 0x0F;
                self.rumble = (val & 0x10) != 0;
            }
            _ => unreachable!(),
        }
    }
}

