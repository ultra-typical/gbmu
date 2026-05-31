use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;


pub fn load_b_a(cpu: &mut Cpu) {
    let a_val = cpu.registers.get(R8::A);
    cpu.registers.set(R8::B, a_val);
}

pub fn load_b_b(cpu: &mut Cpu) {
    //what the fuck
    cpu.registers.set(R8::B, cpu.registers.get(R8::B));
}

pub fn load_b_c(cpu: &mut Cpu) {
    let c_val = cpu.registers.get(R8::C);
    cpu.registers.set(R8::B, c_val);
}

pub fn load_b_d(cpu: &mut Cpu) {
    let d_val = cpu.registers.get(R8::D);
    cpu.registers.set(R8::B, d_val);
}

pub fn load_b_e(cpu: &mut Cpu) {
    let e_val = cpu.registers.get(R8::E);
    cpu.registers.set(R8::B, e_val);
}

pub fn load_b_h(cpu: &mut Cpu) {
    let h_val = cpu.registers.get(R8::H);
    cpu.registers.set(R8::B, h_val);
}

pub fn load_b_l(cpu: &mut Cpu) {
    let l_val = cpu.registers.get(R8::L);
    cpu.registers.set(R8::B, l_val);
}

pub fn load_b_tmp(cpu: &mut Cpu) {
    cpu.registers.set(R8::B, cpu.accumulator.get_u8_at(0));
}


pub fn load_hl_b(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::B);
}