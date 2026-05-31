use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;

pub fn load_l_a(cpu: &mut Cpu) {
    let a_val = cpu.registers.get(R8::A);
    cpu.registers.set(R8::L, a_val);
}

pub fn load_l_b(cpu: &mut Cpu) {
    let b_val = cpu.registers.get(R8::B);
    cpu.registers.set(R8::L, b_val);
}

pub fn load_l_c(cpu: &mut Cpu) {
    let c_val = cpu.registers.get(R8::C);
    cpu.registers.set(R8::L, c_val);
}

pub fn load_l_d(cpu: &mut Cpu) {
    let d_val = cpu.registers.get(R8::D)    ;
    cpu.registers.set(R8::L, d_val);
}

pub fn load_l_e(cpu: &mut Cpu) {
    let e_val = cpu.registers.get(R8::E);
    cpu.registers.set(R8::L, e_val);
}

pub fn load_l_h(cpu: &mut Cpu) {
    let h_val = cpu.registers.get(R8::H);
    cpu.registers.set(R8::L, h_val);
}

pub fn load_l_l(cpu: &mut Cpu) {
    //we defo are deadass
    cpu.registers.set(R8::L, cpu.registers.get(R8::L));
}

pub fn load_l_tmp(cpu: &mut Cpu) {
    cpu.registers.set(R8::L, cpu.accumulator.get_u8_at(0));
}

pub fn load_hl_l(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::L);
}