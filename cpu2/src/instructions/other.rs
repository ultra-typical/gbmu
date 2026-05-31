use crate::{Cpu, defines::R16, implemenation::GetReg};


//Some ops effectively use 2 cycles but work on one (i.e. LD (HL), r) so that we put a nothing op so it stills takes two cycles and fetch accordingly
pub fn noop(_cpu: &mut Cpu) {}

pub fn halt(_cpu: &mut Cpu) {
    todo!();
}

pub fn sp_decr(cpu: &mut Cpu) {
    cpu.registers.set(R16::SP, cpu.registers.get(R16::SP) - 1);
}

pub fn sp_incr(cpu: &mut Cpu) {
    cpu.registers.set(R16::SP, cpu.registers.get(R16::SP) + 1);
}

