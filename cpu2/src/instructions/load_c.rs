use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;


pub fn load_c_a(cpu: &mut Cpu) {
    let a_val = cpu.registers.get(R8::A);
    cpu.registers.set(R8::C, a_val);
}

pub fn load_c_b(cpu: &mut Cpu) {
    let b_val = cpu.registers.get(R8::B);
    cpu.registers.set(R8::C, b_val);
}

pub fn load_c_c(cpu: &mut Cpu) {
    //what the fuck
    cpu.registers.set(R8::C, cpu.registers.get(R8::C));
}

pub fn load_c_d(cpu: &mut Cpu) {
    let d_val = cpu.registers.get(R8::D);
    cpu.registers.set(R8::C, d_val);
}

pub fn load_c_e(cpu: &mut Cpu) {
    let e_val = cpu.registers.get(R8::E);
    cpu.registers.set(R8::C, e_val);
}

pub fn load_c_h(cpu: &mut Cpu) {
    let h_val = cpu.registers.get(R8::H);
    cpu.registers.set(R8::C, h_val);
}

pub fn load_c_l(cpu: &mut Cpu) {
    let l_val = cpu.registers.get(R8::L);
    cpu.registers.set(R8::C, l_val);
}

pub fn load_c_tmp(cpu: &mut Cpu) {
    cpu.registers.set(R8::C, cpu.accumulator.get_u8_at(0));
}

pub fn load_hl_c(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::C);
}