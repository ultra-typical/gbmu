use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::mmu::MemoryMapper;
use crate::{cpu::defines::Cpu, cpu_def::Reg8};

impl<M: MemoryMapper> Cpu<M> {
    pub fn cp_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_sub(src);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, true);
        self.flags
            .set_flag(Flag::HalfCarry, (dest & 0x0F) < (src & 0x0F));
        self.flags
            .set_flag(Flag::Carry, dest < src);
    }
}
