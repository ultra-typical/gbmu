use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;


pub fn load_f_a(cpu: &mut Cpu) {
    let a_val = cpu.registers.get(R8::A);
    cpu.registers.set(R8::F, a_val);
}

pub fn load_f_b(cpu: &mut Cpu) {
    let b_val = cpu.registers.get(R8::B);
    cpu.registers.set(R8::F, b_val);
}

pub fn load_f_c(cpu: &mut Cpu) {
    let c_val = cpu.registers.get(R8::C);
    cpu.registers.set(R8::F, c_val);
}

pub fn load_f_d(cpu: &mut Cpu) {
    let d_val = cpu.registers.get(R8::D)    ;
    cpu.registers.set(R8::F, d_val);
}

pub fn load_f_e(cpu: &mut Cpu) {
    let e_val = cpu.registers.get(R8::E);
    cpu.registers.set(R8::F, e_val);
}

pub fn load_f_h(cpu: &mut Cpu) {
    let h_val = cpu.registers.get(R8::H);
    cpu.registers.set(R8::F, h_val);
}

pub fn load_f_l(cpu: &mut Cpu) {
    //are we for real
    cpu.registers.set(R8::F, cpu.registers.get(R8::L));
}

pub fn load_f_tmp(cpu: &mut Cpu) {
    cpu.registers.set(R8::F, cpu.accumulator.get_u8_at(0));
}


pub fn load_hl_f(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::F);
}