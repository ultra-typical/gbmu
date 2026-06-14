use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::cpu_def::*;
use crate::mmu::MemoryMapper;
use crate::{cpu::defines::Cpu, cpu_def::Reg8};

impl<M: MemoryMapper> Cpu<M> {
    pub fn add_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_add(src);

        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
    }

    pub fn add_r8_r8_with_carry<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);
        let carry = self.flags.get_flag(Flag::Carry) as u8;

        let result = dest.wrapping_add(src).wrapping_add(carry);
        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);

        let half_carry_check = (dest & 0x0F) + (src & 0x0F) + carry;
        self.flags.set_flag(Flag::HalfCarry, half_carry_check > 0x0F);

        let carry_check = (dest as u16) + (src as u16) + (carry as u16);
        self.flags.set_flag(Flag::Carry, carry_check > 0xFF);
    }

    pub fn add_r8_r8_no_zero_flag<Src: Reg8, Dest: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_add(src);

        self.set_r8::<Dest>(result);

        self.flags.set_flag(Flag::Subtract, false);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
        println!(
            "ADD R8 R8 No Zero Flag: src: {:02X}, dest: {:02X}, result: {:02X}, flags: {:08b}",
            src,
            dest,
            result,
            self.flags
        );
    }

    pub fn add_r8_r8_with_carry_and_no_zero_flag<Src: Reg8, Dest: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_add(src).wrapping_add(self.flags.get_flag(Flag::Carry) as u8);
        self.set_r8::<Dest>(result);

        let carry = (src as u16) + (dest as u16) + (self.flags.get_flag(Flag::Carry) as u16) > 0xFF;
        let half_carry = (src & 0x0F) + (dest & 0x0F) + (self.flags.get_flag(Flag::Carry) as u8) > 0x0F;
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, half_carry);
        self.flags.set_flag(Flag::Carry, carry);
    }

    pub fn add_hl_sp_e_low(&mut self, _bus: &mut M) {
        let sp_low = Self::get_r8::<P>(self);
        let e = Self::get_r8::<Z>(self);
        let result = sp_low.wrapping_add(e);
        Self::set_r8::<Z>(self, result);

        let h = (sp_low & 0x0F) + (e & 0x0F) > 0x0F;
        let c = (sp_low as u16) + (e as u16) > 0xFF;

        self.flags.set_flag(Flag::Zero, false);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, h);
        self.flags.set_flag(Flag::Carry, c);
    }

    pub fn add_hl_sp_e_high(&mut self, _bus: &mut M) {
        let sp_high = Self::get_r8::<S>(self);
        let e = Self::get_r8::<Z>(self);
        let adj: u8 = if e & 0x80 != 0 { 0xFF } else { 0x00 };
        let carry: u8 = self.flags.get_flag(Flag::Carry) as u8;

        self.set_r8::<W>(sp_high.wrapping_add(adj).wrapping_add(carry));
    }
}
