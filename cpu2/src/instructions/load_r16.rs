use std::ptr::addr_eq;

use crate::{Cpu, defines::{R8, R16}, implemenation::GetReg, instructions::other::sp_decr};
use crate::defines::Flag;
use crate::flags::FlagsOps;

pub fn load_tmp_bc(cpu: &mut Cpu) {
    cpu.accumulator.value = cpu.bus[cpu.registers.get(R16::BC) as usize] as u32;
}

pub fn load_tmp_de(cpu: &mut Cpu) {
    cpu.accumulator.value = cpu.bus[cpu.registers.get(R16::DE) as usize] as u32;
}

pub fn load_tmp_hl(cpu: &mut Cpu) {
    cpu.accumulator.value = cpu.bus[cpu.registers.get(R16::HL) as usize] as u32;
}

pub fn load_tmp_hl_decr(cpu: &mut Cpu) {
    cpu.accumulator.value = cpu.bus[cpu.registers.get(R16::HL) as usize] as u32;
    cpu.registers.set(R16::HL, cpu.registers.get(R16::HL).wrapping_sub(1));
}

pub fn load_tmp_hl_incr(cpu: &mut Cpu) {
    cpu.accumulator.value = cpu.bus[cpu.registers.get(R16::HL) as usize] as u32;
    cpu.registers.set(R16::HL, cpu.registers.get(R16::HL).wrapping_add(1));
}

pub fn write_a_in_mem_decr(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::A);
    cpu.registers.set(R16::HL, cpu.registers.get(R16::HL).wrapping_sub(1));
}

pub fn write_a_in_mem_incr(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::A);
    cpu.registers.set(R16::HL, cpu.registers.get(R16::HL).wrapping_add(1));
}

pub fn write_a_in_bc(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::BC) as usize] = cpu.registers.get(R8::A);
}

pub fn write_a_in_de(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::DE) as usize] = cpu.registers.get(R8::A);
}

pub fn write_a_in_hl(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::HL) as usize] = cpu.registers.get(R8::A);
}

pub fn write_tmp_in_bc(cpu: &mut Cpu) {
    cpu.registers.set(R16::BC, cpu.accumulator.get_u16_at(0));
}

pub fn write_tmp_in_de(cpu: &mut Cpu) {
    cpu.registers.set(R16::DE, cpu.accumulator.get_u16_at(0));
}

pub fn write_tmp_in_af(cpu: &mut Cpu) {
    cpu.registers.set(R16::AF, cpu.accumulator.get_u16_at(0));
}

pub fn write_tmp_in_hl(cpu: &mut Cpu) {
    cpu.registers.set(R16::HL, cpu.accumulator.get_u16_at(0));
}

pub fn write_tmp_in_sp(cpu: &mut Cpu) {
    cpu.registers.set(R16::SP, cpu.accumulator.get_u16_at(0));
}

pub fn load_hl_in_sp(cpu: &mut Cpu) {
    cpu.registers.set(R16::SP, cpu.registers.get(R16::HL));
}

pub fn write_msb_bc_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::B);
    sp_decr(cpu);
}

pub fn write_lsb_bc_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::C);
}

pub fn write_msb_de_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::D);
    sp_decr(cpu);
}

pub fn write_lsb_de_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::E);
}

pub fn write_msb_hl_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::H);
    sp_decr(cpu);
}

pub fn write_lsb_hl_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::L);
}

pub fn write_msb_af_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::H);
    sp_decr(cpu);
}

pub fn write_lsb_af_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.registers.get(R16::SP) as usize] = cpu.registers.get(R8::L);
}

pub fn write_msb_sp_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.accumulator.get_u16_at(0) as usize] = (cpu.registers.get(R16::SP) >> 8) as u8;
}

pub fn write_lsb_sp_in_mem(cpu: &mut Cpu) {
    cpu.bus[cpu.accumulator.get_u16_at(0) as usize] = (cpu.registers.get(R16::SP) & 0xFF) as u8;
    let tmp = cpu.accumulator.get_u16_at(0);
    cpu.accumulator.reset();
    cpu.accumulator.accumulate_u16(tmp.wrapping_add(1));
}

pub fn add_accu_to_lsb_sp(cpu: &mut Cpu) {

    let sp_lsb = (cpu.registers.get(R16::SP) & 0xFF) as u8;
    let accu = cpu.registers.get(R8::A);

    let result = sp_lsb.wrapping_add(accu);
    cpu.registers.set(R8::L, result);

    cpu.registers.r8[R8::A as usize] = result;

    cpu.registers.flags.set_flag(Flag::Zero, result == 0);
    cpu.registers.flags.set_flag(Flag::Subtract, false); 
    cpu.registers.flags.set_flag(Flag::HalfCarry,(sp_lsb & 0x0F) + (accu & 0x0F) > 0x0F);
    cpu.registers.flags.set_flag(Flag::Carry, (sp_lsb as u16) + (accu as u16) > 0xFF);

    cpu.accumulator.accumulate_u8(result & 0b01000000);
}

pub fn put_spe_in_h(cpu: &mut Cpu) {

    let adj;
    if (cpu.accumulator.get_u8_at(0) >> 7) & 0x01 == 0 {
        adj = 0x00;
    } else {
        adj = 0xFF;
    }
    let sp_msb = (cpu.registers.get(R16::SP) >> 8) as u8;
    let res = sp_msb + adj + cpu.registers.flags.get_flag(Flag::Carry) as u8;
    cpu.registers.set(R8::H, res);
}