use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;


pub fn load_h_a(cpu: &mut Cpu) {
    let a_val = cpu.registers.get(R8::A);
    cpu.registers.set(R8::H, a_val);
}

pub fn load_h_b(cpu: &mut Cpu) {
    let b_val = cpu.registers.get(R8::B);
    cpu.registers.set(R8::H, b_val);
}

pub fn load_h_c(cpu: &mut Cpu) {
    let c_val = cpu.registers.get(R8::C);
    cpu.registers.set(R8::H, c_val);
}

pub fn load_h_d(cpu: &mut Cpu) {
    let d_val = cpu.registers.get(R8::D);
    cpu.registers.set(R8::H, d_val);
}

pub fn load_h_e(cpu: &mut Cpu) {
    let e_val = cpu.registers.get(R8::E);
    cpu.registers.set(R8::H, e_val);
}

pub fn load_h_h(cpu: &mut Cpu) {
    //could u stop being retarded
    cpu.registers.set(R8::H, cpu.registers.get(R8::H));
}

pub fn load_h_l(cpu: &mut Cpu) {
    let l_val = cpu.registers.get(R8::L);
    cpu.registers.set(R8::H, l_val);
}

pub fn load_h_tmp(cpu: &mut Cpu) {
    cpu.registers.set(R8::H, cpu.accumulator.get_u8_at(0));
}

pub fn load_hl_h(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::H);
}