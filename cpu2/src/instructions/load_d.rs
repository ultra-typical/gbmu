use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;


pub fn load_d_a(cpu: &mut Cpu) {
    let a_val = cpu.registers.get(R8::A);
    cpu.registers.set(R8::D, a_val);
}

pub fn load_d_b(cpu: &mut Cpu) {
    let b_val = cpu.registers.get(R8::B);
    cpu.registers.set(R8::D, b_val);
}

pub fn load_d_c(cpu: &mut Cpu) {
    let c_val = cpu.registers.get(R8::C);
    cpu.registers.set(R8::D, c_val);
}

pub fn load_d_d(cpu: &mut Cpu) {
    //what the fuck
    cpu.registers.set(R8::D, cpu.registers.get(R8::D));
}

pub fn load_d_e(cpu: &mut Cpu) {
    let e_val = cpu.registers.get(R8::E);
    cpu.registers.set(R8::D, e_val);
}

pub fn load_d_h(cpu: &mut Cpu) {
    let h_val = cpu.registers.get(R8::H);
    cpu.registers.set(R8::D, h_val);
}

pub fn load_d_l(cpu: &mut Cpu) {
    let l_val = cpu.registers.get(R8::L);
    cpu.registers.set(R8::D, l_val);
}

pub fn load_d_tmp(cpu: &mut Cpu) {
    cpu.registers.set(R8::D, cpu.accumulator.get_u8_at(0));
}


pub fn load_hl_d(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::D);
}