use crate::Cpu;
use crate::defines::R16::SP;
use crate::defines::{R8, R16};
use crate::implemenation::GetReg;
use crate::instructions::other::sp_incr;


pub fn load_sp_in_accu(cpu: &mut Cpu) {
    cpu.accumulator.accumulate_u8(cpu.bus[cpu.registers.get(R16::SP) as usize]);
    cpu.registers.incr_pc();
}

pub fn write_n_in_accu(cpu: &mut Cpu) {
    cpu.accumulator.accumulate_u8(cpu.bus[cpu.registers.get(R16::HL) as usize]);
}

pub fn load_pc_in_accu(cpu: &mut Cpu) {
    cpu.accumulator.accumulate_u8(cpu.bus[cpu.registers.get(R16::PC) as usize]);
    cpu.registers.incr_pc();
}

pub fn load_mem_in_accu(cpu: &mut Cpu) {
    let value = cpu.bus[cpu.accumulator.get_u16_at(0) as usize];
    cpu.accumulator.reset();
    cpu.accumulator.accumulate_u8(value);
}

pub fn write_a_in_c0x_ff(cpu: &mut Cpu) {
    cpu.bus[0xFF00 + cpu.registers.get(R8::C) as usize] = cpu.registers.get(R8::A);
}

pub fn load_mem0x_ff_in_accu(cpu: &mut Cpu) {
    let value = cpu.bus[0xFF00 + cpu.accumulator.get_u8_at(0) as usize];
    cpu.accumulator.reset();
    cpu.accumulator.accumulate_u8(value);
}

pub fn read_memory_from_sp(cpu: &mut Cpu) {
    let value = cpu.bus[cpu.registers.get(R16::SP) as usize];
    cpu.accumulator.accumulate_u8(value);
    sp_incr(cpu);
}