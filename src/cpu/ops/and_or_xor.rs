use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::mmu::MemoryMapper;
use crate::{cpu::defines::Cpu, cpu::Reg8};

impl<M: MemoryMapper> Cpu<M> {
    pub fn and_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest & src;

        self.set_r8::<Dest>(result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, true);

        self.flags.set_flag(Flag::Carry, false);
    }

    pub fn or_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest | src;

        self.set_r8::<Dest>(result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);

        self.flags.set_flag(Flag::Carry, false);
    }

    pub fn xor_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest ^ src;

        self.set_r8::<Dest>(result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);

        self.flags.set_flag(Flag::Carry, false);
    }
}
