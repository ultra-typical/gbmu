use crate::Cpu;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;


pub fn load_a_a(cpu: &mut Cpu) {
    //what the fuck
    cpu.registers.set(R8::A, cpu.registers.get(R8::A));
}

pub fn load_a_b(cpu: &mut Cpu) {
    let b_val = cpu.registers.get(R8::B);
    cpu.registers.set(R8::A, b_val);
}

pub fn load_a_c(cpu: &mut Cpu) {
    let c_val = cpu.registers.get(R8::C);
    cpu.registers.set(R8::A, c_val);
}

pub fn load_a_d(cpu: &mut Cpu) {
    let d_val = cpu.registers.get(R8::D);
    cpu.registers.set(R8::A, d_val);
}

pub fn load_a_e(cpu: &mut Cpu) {
    let e_val = cpu.registers.get(R8::E);
    cpu.registers.set(R8::A, e_val);
}

pub fn load_a_h(cpu: &mut Cpu) {
    let h_val = cpu.registers.get(R8::H);
    cpu.registers.set(R8::A, h_val);
}

pub fn load_a_l(cpu: &mut Cpu) {
    let l_val = cpu.registers.get(R8::L);
    cpu.registers.set(R8::A, l_val);
}

pub fn load_a_accu(cpu: &mut Cpu) {
    cpu.registers.set(R8::A, cpu.accumulator.get_u8_at(0));
}

pub fn load_hl_a(cpu: &mut Cpu) { 
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::A);
}

pub fn load_a_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.accumulator.get_u16_at(0) as usize] = cpu.registers.get(R8::A);
}