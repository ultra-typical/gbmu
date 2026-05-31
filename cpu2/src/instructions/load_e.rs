use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;


pub fn load_e_a(cpu: &mut Cpu) {
    let a_val = cpu.registers.get(R8::A);
    cpu.registers.set(R8::E, a_val);
}

pub fn load_e_b(cpu: &mut Cpu) {
    let b_val = cpu.registers.get(R8::B);
    cpu.registers.set(R8::E, b_val);
}

pub fn load_e_c(cpu: &mut Cpu) {
    let c_val = cpu.registers.get(R8::C);
    cpu.registers.set(R8::E, c_val);
}

pub fn load_e_d(cpu: &mut Cpu) {
    let d_val = cpu.registers.get(R8::D);
    cpu.registers.set(R8::E, d_val);
}

pub fn load_e_e(cpu: &mut Cpu) {
    //h u h 
    cpu.registers.set(R8::E, cpu.registers.get(R8::E));
}

pub fn load_e_h(cpu: &mut Cpu) {
    let h_val = cpu.registers.get(R8::H);
    cpu.registers.set(R8::E, h_val);
}

pub fn load_e_l(cpu: &mut Cpu) {
    let l_val = cpu.registers.get(R8::L);
    cpu.registers.set(R8::E, l_val);
}

pub fn load_e_tmp(cpu: &mut Cpu) {
    cpu.registers.set(R8::E, cpu.accumulator.get_u8_at(0));
}


pub fn load_hl_e(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::E);
}