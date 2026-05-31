use crate::Cpu;
use crate::R8;
use crate::flags::FlagsOps;
use crate::defines::Flag;

pub fn add_r8(cpu: &mut Cpu, a: u8, any_r8:u8)
{
    let result = a.wrapping_add(any_r8);

    cpu.registers.r8[R8::A as usize] = result;

    cpu.registers.flags.set_flag(Flag::Zero, result == 0);
    cpu.registers.flags.set_flag(Flag::Subtract, false); 
    cpu.registers.flags.set_flag(Flag::HalfCarry,(a & 0x0F) + (any_r8 & 0x0F) > 0x0F);
    cpu.registers.flags.set_flag(Flag::Carry, (a as u16) + (any_r8 as u16) > 0xFF);
}


pub fn add_a_b(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    let b = cpu.registers.r8[R8::B as usize];
    add_r8(cpu, a, b);

}

pub fn add_a_c(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    let c = cpu.registers.r8[R8::C as usize];
    add_r8(cpu, a, c);
    
}

pub fn add_a_d(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    let d = cpu.registers.r8[R8::D as usize];
    add_r8(cpu, a, d);

}

pub fn add_a_e(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    let e = cpu.registers.r8[R8::E as usize];
    add_r8(cpu, a, e);

}

pub fn add_a_h(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    let h = cpu.registers.r8[R8::H as usize];
    add_r8(cpu, a, h);
}


pub fn add_a_l(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    let l = cpu.registers.r8[R8::L as usize];
    add_r8(cpu, a, l);

}

pub fn add_a_hl(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    add_r8(cpu, a, cpu.accumulator.get_u8_at(0));
}
