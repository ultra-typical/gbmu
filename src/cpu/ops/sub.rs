use crate::cpu::defines::Cpu;
use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::cpu::*;
use crate::mmu::MemoryMapper;

impl<M: MemoryMapper> Cpu<M> {

    pub fn sub_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_sub(src);
        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, true);
        self.flags
            .set_flag(Flag::HalfCarry, (dest & 0x0F) < (src & 0x0F));
        self.flags
            .set_flag(Flag::Carry, dest < src);
    }
    pub fn sub_r8_r8_with_carry<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);
        let carry = self.flags.get_flag(Flag::Carry) as u8;

        let result = dest.wrapping_sub(src).wrapping_sub(carry);
        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, true);
        
        self.flags
            .set_flag(Flag::HalfCarry, (dest & 0x0F) < (src & 0x0F) + carry);

        self.flags
            .set_flag(Flag::Carry, (dest as u16) < (src as u16) + (carry as u16));
    }
}
