use std::sync::RwLock;

pub mod apu;
pub mod interrupt;
pub mod mbc;
pub mod oam;
pub mod timers;

use crate::communications::GameCT;
use crate::mmu::apu::Apu;
use crate::mmu::interrupt::Interrupt;
use crate::mmu::interrupt::InterruptController;
use crate::mmu::mbc::Mbc;
use crate::mmu::oam::Oam;
use crate::mmu::timers::TimingComponent;
use crate::ppu::PixelProcessor;

#[derive(PartialEq, Eq, Debug)]
pub enum MemoryRegion {
    Mbc,             // 0x000-0x7FFF: read-only
    Vram,            // 0x8000-0x9FFF
    ERam,            // 0xA000-0xBFFF
    Wram,            // 0xC000-0xDFFF
    Mram,            // 0xE000-0xFDFF: mirror of C000-DDFF
    Oam,             // 0xFE00-0xFE9F: Sprite Attribute Table
    Unusable,        // 0xFEA0-0xFEFF
    InterruptFlag,   // 0xFF0F: Interruption Flag: Inside IO
    Timers,          // 0xFF04-0xFF07
    Audio,           // 0xFF10-0xFF26
    WaveRam,         // 0xFF30-0xFF3F
    Io,              // 0xFF00-0xFF7F
    HRam,            // 0xFF80-0xFFFE
    InterruptEnable, // 0xFFFF: Interruption Enable
}

impl MemoryRegion {
    pub fn from(addr: u16) -> Self {
        match addr {
            0x0000..=0x7FFF => MemoryRegion::Mbc,
            0x8000..=0x9FFF => MemoryRegion::Vram,
            0xA000..=0xBFFF => MemoryRegion::ERam,
            0xC000..=0xDFFF => MemoryRegion::Wram,
            0xE000..=0xFDFF => MemoryRegion::Mram,
            0xFE00..=0xFE9F => MemoryRegion::Oam,
            0xFEA0..=0xFEFF => MemoryRegion::Unusable,
            0xFF04..=0xFF07 => MemoryRegion::Timers,
            0xFF0F => MemoryRegion::InterruptFlag,
            0xFF10..=0xFF26 => MemoryRegion::Audio,
            0xFF30..=0xFF3F => MemoryRegion::WaveRam,
            0xFF00..=0xFF7F => MemoryRegion::Io,
            0xFF80..=0xFFFE => MemoryRegion::HRam,
            0xFFFF => MemoryRegion::InterruptEnable,
        }
    }

    pub fn to_address(&self) -> u16 {
        match self {
            MemoryRegion::Mbc => 0x0000,
            MemoryRegion::Vram => 0x8000,
            MemoryRegion::ERam => 0xA000,
            MemoryRegion::Wram => 0xC000,
            MemoryRegion::Mram => 0xE000,
            MemoryRegion::Oam => 0xFE00,
            MemoryRegion::Unusable => 0xFEA0,
            MemoryRegion::Timers => 0xFF04,
            MemoryRegion::InterruptFlag => 0xFF0F,
            MemoryRegion::Io => 0xFF00,
            MemoryRegion::HRam => 0xFF80,
            MemoryRegion::InterruptEnable => 0xFFFF,
            MemoryRegion::Audio => 0xFF10,
            MemoryRegion::WaveRam => 0xFF30,
        }
    }
}

pub trait MemoryMapper {
    fn write_timers(&mut self, addr: u16, value: u8);
    fn new(
        wrapped_boot_rom: Option<[u8; 0x100]>,
        rom_data: Vec<u8>,
        ram_data: Option<Vec<u8>>,
    ) -> Result<Self, String>
    where
        Self: Sized;
    fn get_boot_enable(&self) -> bool;
    fn get_boot_rom(&self) -> &[u8; 0x0100];
    fn get_button_state(&self) -> &u8;
    fn get_cart(&mut self) -> &mut dyn Mbc;
    fn get_timer(&mut self) -> &mut dyn TimingComponent;
    fn get_data(&self) -> &[u8; 0x10000];
    fn get_dma_source(&self) -> u16;
    fn get_dpad_state(&self) -> &u8;
    fn get_interrupts(&mut self) -> &mut InterruptController;
    fn get_ppu(&mut self) -> &mut dyn PixelProcessor;
    fn get_apu(&mut self) -> &mut Apu;
    fn set_boot_enable(&mut self, enabled: bool);
    fn set_button_state(&mut self, buttons: u8);
    fn set_dma_source(&mut self, val: u16);
    fn set_dpad_state(&mut self, dpad: u8);
    fn update_data(&mut self, index: usize, val: u8);
    fn get_dma_index(&mut self) -> u8;
    fn set_dma_index(&mut self, val: u8);

    fn read_timers(&mut self, addr: u16) -> u8;

    fn ram_dump(&mut self) -> Option<Vec<u8>> {
        self.get_cart().dump()
    }

    fn set_dma_cycle(&mut self, nb: u8);
    fn oam_write_direct(&mut self, index: u16, val: u8);

    fn read_byte_raw(&mut self, addr: u16) -> u8
    where
        Self: Sized,
    {
        if self.get_boot_enable() && addr <= 0x00FF {
            return self.get_boot_rom()[addr as usize];
        }

        match MemoryRegion::from(addr) {
            MemoryRegion::Mbc | MemoryRegion::ERam => self.get_cart().read(addr),
            MemoryRegion::Vram => self.get_ppu().read_vram(addr),
            MemoryRegion::Mram => {
                let mirror = addr - 0x2000;
                self.get_data()[mirror as usize]
            }
            MemoryRegion::Timers => self.read_timers(addr),
            MemoryRegion::Audio => self.get_apu().read(addr),
            MemoryRegion::WaveRam => self.get_apu().read(addr),
            MemoryRegion::Io => {
                if addr == 0xFF00 {
                    let selection = self.get_data()[0xFF00] & 0b0011_0000;
                    let mut result = 0x0F;

                    if selection & 0b0001_0000 == 0 {
                        result &= self.get_dpad_state();
                    }
                    if selection & 0b0010_0000 == 0 {
                        result &= self.get_button_state();
                    }
                    0b1100_0000 | selection | result
                } else if matches!(addr, 0xFF40..=0xFF4B) && addr != 0xFF46 {
                    self.get_ppu().read_register(addr)
                } else {
                    self.get_data()[addr as usize]
                }
            }
            MemoryRegion::Oam => self.get_ppu().read_oam(addr),
            MemoryRegion::Unusable => 0xFF,
            MemoryRegion::InterruptFlag => self.get_interrupts().read_interrupt_flag(),
            MemoryRegion::InterruptEnable => self.get_interrupts().read_interrupt_enable(),
            _ => self.get_data()[addr as usize],
        }
    }

    fn read_byte(&mut self, addr: u16) -> u8 where Self: Sized {
        if self.get_dma_index() != 0xFF && (MemoryRegion::from(addr) != MemoryRegion::HRam || MemoryRegion::from(addr) != MemoryRegion::Mbc) {
            return 0xFF;
        }
        self.read_byte_raw(addr)
    }

    fn write_byte(&mut self, addr: u16, val: u8) where Self: Sized {
        if self.get_dma_index() != 0xFF && (MemoryRegion::from(addr) != MemoryRegion::HRam || MemoryRegion::from(addr) != MemoryRegion::Mbc) {
            return;
        }
        self.write_byte_raw(addr, val);
    }

    fn write_byte_raw(&mut self, addr: u16, val: u8)
    where
        Self: Sized,
    {
        if val != 0 && addr == 0xFF50 {
            self.update_data(addr as usize, val);
            self.set_boot_enable(false);
            return;
        }

        match MemoryRegion::from(addr) {
            MemoryRegion::Mbc | MemoryRegion::ERam => self.get_cart().write(addr, val),
            MemoryRegion::Vram => {
                self.get_ppu().write_vram(addr, val);
            }
            MemoryRegion::Mram => {
                let mirror = addr - 0x2000;
                self.update_data(mirror as usize, val);
            }
            MemoryRegion::Timers => self.get_timer().write(addr, val),
            MemoryRegion::Audio => self.get_apu().write(addr, val),
            MemoryRegion::WaveRam => self.get_apu().write(addr, val),
            MemoryRegion::Io => {
                if addr == 0xFF00 {
                    let selection_bits = val & 0b0011_0000;
                    let current_inputs = self.get_data()[0xFF00] & 0x0F;
                    let val = 0b1100_0000 | selection_bits | current_inputs;
                    self.update_data(0xFF00, val);
                    self.update_joypad_register();
                } else if matches!(addr, 0xFF40..=0xFF4F) && addr != 0xFF46 {
                    self.get_ppu().write_register(addr, val);
                } else if addr == 0xFF46 {
                    let source_base = (val as u16) << 8;

                    for i in 0..160 {
                        let src_addr = source_base + i;
                        let byte = self.read_byte_raw(src_addr);
                        self.oam_write_direct(i, byte);
                    }

                    self.set_dma_cycle(160);
                } else {
                    self.update_data(addr as usize, val);
                }
            }
            MemoryRegion::Oam => {
                self.get_ppu().write_oam(addr, val);
            }
            MemoryRegion::Unusable => {}
            MemoryRegion::InterruptFlag => self.get_interrupts().write_interrupt_flag(val),
            MemoryRegion::InterruptEnable => self.get_interrupts().write_interrupt_enable(val),
            _ => self.update_data(addr as usize, val),
        }
    }

    fn read_interrupt_enable(&mut self) -> u8 {
        self.get_interrupts().read_interrupt_enable()
    }
    fn read_interrupt_flag(&mut self) -> u8 {
        self.get_interrupts().read_interrupt_flag()
    }
    fn interrupts_next_request(&mut self) -> Option<Interrupt> {
        self.get_interrupts().next_request()
    }
    fn interrupts_clear_request(&mut self, interrupt: Interrupt) {
        self.get_interrupts().clear_request(interrupt);
    }
    fn interrupts_request(&mut self, interrupt: Interrupt) {
        self.get_interrupts().request(interrupt);
    }

    fn update_joypad_register(&mut self) {
        let mut new_inputs = 0x0F;
        let selection = self.get_data()[0xFF00] & 0b0011_0000;

        if selection & 0b0001_0000 == 0 {
            new_inputs &= self.get_dpad_state();
        }
        if selection & 0b0010_0000 == 0 {
            new_inputs &= self.get_button_state();
        }

        let old_inputs = self.get_data()[0xFF00] & 0x0F;
        if old_inputs & !new_inputs & 0x0F != 0 {
            self.interrupts_request(Interrupt::Joypad);
        }

        let val = 0xC0 | selection | new_inputs;
        self.update_data(0xFF00, val);
    }

    fn tick_timers(&mut self)
    where
        Self: Sized,
    {
        if self.get_timer().tick() {
            let interrupt_flags_addr = MemoryRegion::InterruptFlag.to_address();
            let mut interrupt_flags = self.read_byte_raw(interrupt_flags_addr);
            interrupt_flags |= 0b100;
            self.write_byte_raw(interrupt_flags_addr, interrupt_flags);
        }
    }

    fn update_keys(&mut self, dpad: u8, buttons: u8) {
        self.set_dpad_state(dpad);
        self.set_button_state(buttons);
        self.update_joypad_register();
    }

    fn tick_apu(&mut self) {
        self.get_apu().step();
    }

    fn tick_ppu(&mut self, ct: &mut Box<dyn GameCT>) where Self: Sized;

    fn tick_dma(&mut self);
}

impl<C: Mbc, T: TimingComponent, P: PixelProcessor> MemoryMapper for DmgMmu<C, T, P> {
    fn get_timer(&mut self) -> &mut dyn TimingComponent { &mut self.timers }
    fn get_dma_index(&mut self) -> u8 { self.dma_index }
    fn set_dma_index(&mut self, val: u8) { self.dma_index = val }
    fn write_timers(&mut self, addr: u16, value: u8) { self.timers.write(addr, value) }
    fn read_timers(&mut self, addr: u16) -> u8 { self.timers.read(addr) }

    fn tick_dma(&mut self) {
        let byte = self.read_byte_raw(self.dma_source + self.dma_index as u16);

        let dma_index = self.dma_index;

        self.get_ppu().write_oam(0xFE00 + dma_index as u16, byte);

        self.dma_index+= 1;

        if self.dma_index == 160 { self.dma_index=0xFF; }
    }

    fn oam_write_direct(&mut self, addr: u16, val: u8) {
        self.ppu.write_oam(addr, val);
    }

    fn set_dma_cycle(&mut self, nb: u8) {
        self.dma_clocks_left = nb;
    }

    fn new(
        wrapped_boot_rom: Option<[u8; 0x100]>,
        rom_data: Vec<u8>,
        ram_data: Option<Vec<u8>>,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        let boot_enable = wrapped_boot_rom.is_some();
        let boot_rom = wrapped_boot_rom.unwrap_or([0xFF; 0x0100]);
        Ok(Self {
            apu: Apu::new(),
            data: Box::new([0xFF; 0x10000]),
            cart: C::new(rom_data, ram_data)?,
            interrupts: InterruptController::new(),
            timers: T::new(),
            oam: RwLock::new(Oam::default()),
            ppu: P::new(),
            boot_enable,
            boot_rom,
            dpad_state: 0x0F,
            button_state: 0x0F,
            dma_source: 0x0,
            dma_index: 0xFF,
            dma_clocks_left: 0,
        })
    }

    fn tick_ppu(&mut self, ct: &mut Box<dyn GameCT>)
    where
        Self: Sized,
    {
        self.ppu.tick(ct);
        if self.ppu.pending_vblank() {
            self.interrupts_request(Interrupt::VBlank);
            self.ppu.set_pending_vblank(false);
        }
        if self.ppu.pending_stat() {
            self.interrupts_request(Interrupt::LcdStat);
            self.ppu.set_pending_stat(false);
        }
    }

    fn get_cart(&mut self) -> &mut dyn Mbc {
        &mut self.cart
    }
    fn get_data(&self) -> &[u8; 0x10000] {
        &self.data
    }

    fn get_interrupts(&mut self) -> &mut InterruptController {
        &mut self.interrupts
    }

    fn get_boot_rom(&self) -> &[u8; 0x0100] {
        &self.boot_rom
    }

    fn get_button_state(&self) -> &u8 {
        &self.button_state
    }

    fn get_dpad_state(&self) -> &u8 {
        &self.dpad_state
    }

    fn get_boot_enable(&self) -> bool {
        self.boot_enable
    }

    fn get_dma_source(&self) -> u16 {
        self.dma_source
    }

    fn set_boot_enable(&mut self, enabled: bool) {
        self.boot_enable = enabled;
    }

    fn set_dma_source(&mut self, val: u16) {
        self.dma_source = val;
    }

    fn set_dpad_state(&mut self, dpad: u8) {
        self.dpad_state = dpad;
    }

    fn set_button_state(&mut self, buttons: u8) {
        self.button_state = buttons;
    }

    fn update_data(&mut self, index: usize, val: u8) {
        self.data[index] = val;
    }

    fn get_ppu(&mut self) -> &mut dyn PixelProcessor {
        &mut self.ppu
    }

    fn get_apu(&mut self) -> &mut Apu {
        &mut self.apu
    }
}

pub struct DmgMmu<C: Mbc, T: TimingComponent, P: PixelProcessor> {
    data: Box<[u8; 0x10000]>, // 0xFFFF (65535) + 1 = 0x10000 (65536)
    cart: C,
    interrupts: InterruptController,
    timers: T,
    oam: RwLock<Oam>,
    apu: Apu,
    pub ppu: P,
    boot_enable: bool,
    boot_rom: [u8; 0x0100],
    dpad_state: u8,   // for joypad
    button_state: u8, // for joypad
    dma_source: u16,
    pub dma_index: u8,
    dma_clocks_left: u8,
}

impl<C: Mbc, T: TimingComponent, P: PixelProcessor> Default for DmgMmu<C, T, P> {
    fn default() -> Self {
        DmgMmu::<C, T, P>::new(None, vec![], None).expect("This is not suppose to happen")
    }
}

impl<C: Mbc, T: TimingComponent, P: PixelProcessor> MemoryMapper for CgbMmu<C, T, P> {
    fn get_timer(&mut self) -> &mut dyn TimingComponent {
        &mut self.timers
    }
    fn get_dma_index(&mut self) -> u8 {
        self.dma_index
    }
    fn set_dma_index(&mut self, val: u8) {
        self.dma_index = val
    }
    fn write_timers(&mut self, addr: u16, value: u8) {
        self.timers.write(addr, value)
    }
    fn read_timers(&mut self, addr: u16) -> u8 {
        self.timers.read(addr)
    }

    fn tick_dma(&mut self) {
        let byte = self.read_byte_raw(self.dma_source + self.dma_index as u16);

        let dma_index = self.dma_index;

        self.get_ppu().write_oam(0xFE00 + dma_index as u16, byte);

        self.dma_index+= 1;

        if self.dma_index == 160 { self.dma_index=0xFF; }
    }

    fn oam_write_direct(&mut self, addr: u16, val: u8) {
        self.ppu.write_oam(addr, val);
    }

    fn set_dma_cycle(&mut self, nb: u8) {
        self.dma_clocks_left = nb;
    }

    fn new(
        wrapped_boot_rom: Option<[u8; 0x100]>,
        rom_data: Vec<u8>,
        ram_data: Option<Vec<u8>>,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        let boot_enable = wrapped_boot_rom.is_some();
        let boot_rom = wrapped_boot_rom.unwrap_or([0xFF; 0x0100]);
        Ok(CgbMmu {
            apu: Apu::new(),
            data: Box::new([0xFF; 0x10000]),
            cart: C::new(rom_data, ram_data)?,
            interrupts: InterruptController::new(),
            timers: T::new(),
            ppu: P::new(),
            boot_enable,
            boot_rom,
            dpad_state: 0x0F,
            button_state: 0x0F,
            dma_source: 0x0,
            dma_index: 0xFF,
            dma_clocks_left: 0,
        })
    }

    fn tick_ppu(&mut self, ct: &mut Box<dyn GameCT>)
    where
        Self: Sized,
    {
        self.ppu.tick(ct);
        if self.ppu.pending_vblank() {
            self.interrupts_request(Interrupt::VBlank);
            self.ppu.set_pending_vblank(false);
        }
        if self.ppu.pending_stat() {
            self.interrupts_request(Interrupt::LcdStat);
            self.ppu.set_pending_stat(false);
        }
    }

    fn get_cart(&mut self) -> &mut dyn Mbc {
        &mut self.cart
    }
    fn get_data(&self) -> &[u8; 0x10000] {
        &self.data
    }

    fn get_interrupts(&mut self) -> &mut InterruptController {
        &mut self.interrupts
    }

    fn get_boot_rom(&self) -> &[u8; 0x0100] {
        &self.boot_rom
    }

    fn get_button_state(&self) -> &u8 {
        &self.button_state
    }

    fn get_dpad_state(&self) -> &u8 {
        &self.dpad_state
    }

    fn get_boot_enable(&self) -> bool {
        self.boot_enable
    }

    fn get_dma_source(&self) -> u16 {
        self.dma_source
    }

    fn set_boot_enable(&mut self, enabled: bool) {
        self.boot_enable = enabled;
    }
    fn set_dma_source(&mut self, val: u16) {
        self.dma_source = val;
    }

    fn set_dpad_state(&mut self, dpad: u8) {
        self.dpad_state = dpad;
    }

    fn set_button_state(&mut self, buttons: u8) {
        self.button_state = buttons;
    }

    fn update_data(&mut self, index: usize, val: u8) {
        self.data[index] = val;
    }

    fn tick_timers(&mut self) {
        if self.timers.tick() {
            let interrupt_flags_addr = MemoryRegion::InterruptFlag.to_address();
            let mut interrupt_flags = self.read_byte_raw(interrupt_flags_addr);
            interrupt_flags |= 0b100;
            self.write_byte_raw(interrupt_flags_addr, interrupt_flags);
        }
    }

    fn get_ppu(&mut self) -> &mut dyn PixelProcessor {
        &mut self.ppu
    }

    fn get_apu(&mut self) -> &mut Apu {
        &mut self.apu
    }
}

pub struct CgbMmu<C: Mbc, T: TimingComponent, P: PixelProcessor> {
    data: Box<[u8; 0x10000]>, // 0xFFFF (65535) + 1 = 0x10000 (65536)
    cart: C,
    interrupts: InterruptController,
    timers: T,
    apu: Apu,
    pub ppu: P,
    boot_enable: bool,
    boot_rom: [u8; 0x0100],
    dpad_state: u8,   // for joypad
    button_state: u8, // for joypad
    dma_source: u16,
    pub dma_index: u8,
    dma_clocks_left: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mmu::mbc::RomOnly;
    use crate::mmu::timers::DmgTimers;
    use crate::ppu::DmgPpu;

    use super::{DmgMmu, MemoryRegion};

    fn default_dmg_mmu_from(rom: Vec<u8>) -> DmgMmu<RomOnly, DmgTimers, DmgPpu> {
        DmgMmu::<RomOnly, DmgTimers, DmgPpu>::new(None, rom, None).unwrap()
    }

    fn default_dmg_mmu() -> DmgMmu<RomOnly, DmgTimers, DmgPpu> {
        DmgMmu::<RomOnly, DmgTimers, DmgPpu>::default()
    }

    #[test]
    fn mmu_routes_reads_and_writes() {
        let rom = vec![0x12, 0x34, 0x56, 0x78];
        let mut mmu = default_dmg_mmu_from(rom);
        // Reading from ROM region gives you the first bank data
        assert_eq!(mmu.read_byte(0x0000), 0x12);
        assert_eq!(mmu.read_byte(0x0001), 0x34);

        // Write to WRAM region and read back
        mmu.write_byte(0xC000, 0xAB);
        assert_eq!(mmu.read_byte(0xC000), 0xAB);
    }

    #[test]
    fn memory_region_from_addr() {
        assert_eq!(MemoryRegion::from(0x0000), MemoryRegion::Mbc);
        assert_eq!(MemoryRegion::from(0x8000), MemoryRegion::Vram);
        assert_eq!(MemoryRegion::from(0xA123), MemoryRegion::ERam);
        assert_eq!(MemoryRegion::from(0xC123), MemoryRegion::Wram);
        assert_eq!(MemoryRegion::from(0xE123), MemoryRegion::Mram);
        assert_eq!(MemoryRegion::from(0xFE50), MemoryRegion::Oam);
        assert_eq!(MemoryRegion::from(0xFEA0), MemoryRegion::Unusable);
        assert_eq!(MemoryRegion::from(0xFF0F), MemoryRegion::InterruptFlag);
        assert_eq!(MemoryRegion::from(0xFF10), MemoryRegion::Audio);
        assert_eq!(MemoryRegion::from(0xFF30), MemoryRegion::WaveRam);
        assert_eq!(MemoryRegion::from(0xFF00), MemoryRegion::Io);
        assert_eq!(MemoryRegion::from(0xFF80), MemoryRegion::HRam);
        assert_eq!(MemoryRegion::from(0xFFFF), MemoryRegion::InterruptEnable);
    }

    // MRAM ECHO RAM
    #[test]
    fn echo_ram_mirror() {
        let mut mmu = default_dmg_mmu();

        // Write to Work RAM (0xC000) and read from Echo RAM (0xE000)
        mmu.write_byte(0xC000, 0xAA);
        assert_eq!(mmu.read_byte(0xE000), 0xAA);

        // Write to Echo RAM and read from Work RAM
        mmu.write_byte(0xE010, 0xBB);
        assert_eq!(mmu.read_byte(0xC010), 0xBB);
    }

    // UNUSABLE REGION
    #[test]
    fn unusable_region_behavior() {
        let mut mmu = default_dmg_mmu();

        // Unusable region reads back as 0xFF
        let base = 0xFEA0;
        assert_eq!(mmu.read_byte(base), 0xFF);
        assert_eq!(mmu.read_byte(base + 0x1F), 0xFF);

        // Writes to unusable region are ignored (reads still 0xFF)
        mmu.write_byte(base, 0x00);
        mmu.write_byte(base + 0x1F, 0x12);
        assert_eq!(mmu.read_byte(base), 0xFF);
        assert_eq!(mmu.read_byte(base + 0x1F), 0xFF);
    }
}
