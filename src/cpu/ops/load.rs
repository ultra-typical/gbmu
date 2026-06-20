use crate::cpu::defines::Cpu;
use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::cpu_def::*;
use crate::mmu::MemoryMapper;

impl<M: MemoryMapper> Cpu<M> {
    pub fn load_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        Self::set_r8::<Dest>(self, Self::get_r8::<Src>(self));
    }

    pub fn read_memory<Addr: Reg16, Dest: Reg8>(&mut self, bus: &mut M) {
        Self::set_r8::<Dest>(self, bus.read_byte(self.get_r16::<Addr>()));
    }

    pub fn read_memory_0xff<Lsb: Reg8, Dest: Reg8>(&mut self, bus: &mut M) {
        let addr: u16 = 0xFF << 8 | Self::get_r8::<Lsb>(self) as u16;
        Self::set_r8::<Dest>(self, bus.read_byte(addr));
    }

    pub fn write_memory_0xff<Lsb: Reg8, Value: Reg8>(&mut self, bus: &mut M) {
        let addr: u16 = 0xFF << 8 | Self::get_r8::<Lsb>(self) as u16;
        bus.write_byte(addr, Self::get_r8::<Value>(self));
    }

    pub fn write_memory<Addr: Reg16, Value: Reg8>(&mut self, bus: &mut M) {
        bus.write_byte(Self::get_r16::<Addr>(self), Self::get_r8::<Value>(self));
    }

    pub fn load_r16_r16<Dest: Reg16, Src: Reg16>(&mut self, _bus: &mut M) {
        Self::set_r16::<Dest>(self, Self::get_r16::<Src>(self));
    }

    pub fn load_r16_r16_and_ime<Dest: Reg16, Src: Reg16>(&mut self, _bus: &mut M) {
        Self::set_r16::<Dest>(self, Self::get_r16::<Src>(self));
        self.ime = true;
    }

pub fn load_r16_r16_af_flags<Dest: Reg16, Src: Reg16>(&mut self, _bus: &mut M) {
    let mut val = Self::get_r16::<Src>(self);
    
    val &= 0xFFF0;

    Self::set_r16::<Dest>(self, val);

    let f_byte = (val & 0xFF) as u8;

    self.flags.set_flag(Flag::Zero,      (f_byte & 0x80) != 0);
    self.flags.set_flag(Flag::Subtract,       (f_byte & 0x40) != 0);
    self.flags.set_flag(Flag::HalfCarry, (f_byte & 0x20) != 0);
    self.flags.set_flag(Flag::Carry,     (f_byte & 0x10) != 0);
}

    pub fn read_memory_decr<Addr: Reg16, Dest: Reg8>(&mut self, bus: &mut M) {
        Self::read_memory::<Addr, Dest>(self, bus);
        Self::set_r16::<Addr>(self, Self::get_r16::<Addr>(self).wrapping_sub(1));
    }

    pub fn write_memory_decr<Addr: Reg16, Dest: Reg8>(&mut self, bus: &mut M) {
        Self::write_memory::<Addr, Dest>(self, bus);
        Self::set_r16::<Addr>(self, Self::get_r16::<Addr>(self).wrapping_sub(1));
    }

    pub fn read_memory_incr<Addr: Reg16, Dest: Reg8>(&mut self, bus: &mut M) {
        Self::read_memory::<Addr, Dest>(self, bus);
        Self::set_r16::<Addr>(self, Self::get_r16::<Addr>(self).wrapping_add(1));
    }

    pub fn write_memory_incr<Addr: Reg16, Dest: Reg8>(&mut self, bus: &mut M) {
        Self::write_memory::<Addr, Dest>(self, bus);
        Self::set_r16::<Addr>(self, Self::get_r16::<Addr>(self).wrapping_add(1));
    }

    pub fn ld_hl_sp_e_low(&mut self, _bus: &mut M) {
        let sp_low = Self::get_r8::<P>(self);
        let e = Self::get_r8::<Z>(self);
        let result = sp_low.wrapping_add(e);
        Self::set_r8::<L>(self, result);

        let h = (sp_low & 0x0F) + (e & 0x0F) > 0x0F;
        let c = (sp_low as u16) + (e as u16) > 0xFF;

        self.flags.set_flag(Flag::Zero, false);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, h);
        self.flags.set_flag(Flag::Carry, c);
    }

    pub fn ld_hl_sp_e_high(&mut self, _bus: &mut M) {
        let sp_high = Self::get_r8::<S>(self);
        let e = Self::get_r8::<Z>(self);
        let adj: u8 = if e & 0x80 != 0 { 0xFF } else { 0x00 };
        let carry: u8 = self.flags.get_flag(Flag::Carry) as u8;

        Self::set_r8::<H>(self, sp_high.wrapping_add(adj).wrapping_add(carry));
    }

    pub fn write_memory_reassign_pc<Addr: Reg16, Value: Reg8>(&mut self, bus: &mut M) {
        Self::write_memory::<Addr, Value>(self, bus);
        Self::set_r16::<PC>(self, Self::get_r16::<WZ>(self));
    }

    pub fn write_memory_rst<const B: u16, Addr: Reg16, Dest: Reg8>(&mut self, bus: &mut M) {
        Self::write_memory::<Addr, Dest>(self, bus);
        Self::set_r16::<PC>(self, B);
    }
}
