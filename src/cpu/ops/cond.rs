
use crate::cpu_def::*;
use crate::{
    cpu::{
        defines::{Cpu, Flag},
        flags::FlagsOps,
    },
    cpu_def::{PC, Reg16, W, Z},
    mmu::MemoryMapper,
};

pub trait Cond<M: MemoryMapper> {
    fn is_met(cpu: &Cpu<M>) -> bool;
}

pub struct CondNZ; // Not Zero
impl<M: MemoryMapper> Cond<M> for CondNZ {
    fn is_met(cpu: &Cpu<M>) -> bool {
        !cpu.flags.get_flag(Flag::Zero)
    }
}

pub struct CondZ; // Zero
impl<M: MemoryMapper> Cond<M> for CondZ {
    fn is_met(cpu: &Cpu<M>) -> bool {
        cpu.flags.get_flag(Flag::Zero)
    }
}

pub struct CondNC; // Not Carry
impl<M: MemoryMapper> Cond<M> for CondNC {
    fn is_met(cpu: &Cpu<M>) -> bool {
        !cpu.flags.get_flag(Flag::Carry)
    }
}

pub struct CondC; // Carry
impl<M: MemoryMapper> Cond<M> for CondC {
    fn is_met(cpu: &Cpu<M>) -> bool {
        cpu.flags.get_flag(Flag::Carry)
    }
}

impl<M: MemoryMapper> Cpu<M> {

    pub fn read_memory_incr_check<Src: Reg16, Dest: Reg8, Cc: Cond<M>>(&mut self, bus: &mut M) {
        Self::read_memory_incr::<Src, Dest>(self, bus);
        
        if !Cc::is_met(self) {
            self.load_queue(&[Cpu::noop]);
        }
    }

    pub fn check_cond<Cc: Cond<M>>(&mut self, _bus: &mut M) {
        if !Cc::is_met(self) {
            self.load_queue(&[Cpu::noop]);
        }
    }

    pub fn check_cond_and_execute<Cc: Cond<M>>(&mut self, _bus: &mut M) {
        if !Cc::is_met(self) {
            self.load_queue(&[Cpu::noop]);
        }
    }

    pub fn relative_jump(&mut self, _bus: &mut M) {
        let z = Self::get_r8::<Z>(self);
        let pc = Self::get_r16::<PC>(self);

        let pc_low = (pc & 0xFF) as u8;
        let pc_high = (pc >> 8) as u8;

        let z_sign = (z & 0x80) != 0;

        let sum = z as u16 + pc_low as u16;
        let result = sum as u8;
        let carry_7 = (sum & 0x0100) != 0;

        let adj = if carry_7 && !z_sign {
            1i8
        } else if !carry_7 && z_sign {
            -1i8
        } else {
            0i8
        };

        let w = (pc_high as i32 + adj as i32) as u8;

        Self::set_r8::<Z>(self, result);
        Self::set_r8::<W>(self, w);

        let wz = ((w as u16) << 8) | (result as u16);
        Self::set_r16::<PC>(self, wz);
    }

}
