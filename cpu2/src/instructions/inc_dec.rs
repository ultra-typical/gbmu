use crate::instructions::load::write_memory;
use crate::{defines::Cpu, implemenation::Reg8, implemenation::Reg16};

pub fn inc_r8<Dest: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(cpu.get_r8::<Dest>().wrapping_add(1));
}

//Increment the value pointed BY the r16 and update it
pub fn inc_addr<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    inc_r8::<Value>(cpu);
    write_memory::<Addr, Value>(cpu);
}

pub fn dec_r8<Reg: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Reg>(cpu.get_r8::<Reg>().wrapping_sub(1));
}

//Decrement the value pointed BY the r16 and update it
pub fn dec_addr<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    dec_r8::<Value>(cpu);
    write_memory::<Addr, Value>(cpu);
}

pub fn inc_r16<Dest: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Dest>().wrapping_add(1));
}

pub fn dec_r16<Dest: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Dest>().wrapping_sub(1));
}
