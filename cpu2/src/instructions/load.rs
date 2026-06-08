use crate::Cpu;
use crate::defines::Flag;
use crate::flags::FlagsOps;
use crate::implemenation::{H, L, P, PC, Reg8, Reg16, S, SP, WZ, Z};

pub fn load_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(cpu.get_r8::<Src>());
}

pub fn read_memory<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(cpu.bus[cpu.get_r16::<Addr>() as usize]);
}

pub fn read_memory_0xff<Lsb: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let addr: u16 = 0xFF << 8 | cpu.get_r8::<Lsb>() as u16;
    cpu.set_r8::<Dest>(cpu.bus[addr as usize]);
}

pub fn write_memory_0xff<Lsb: Reg8, Value: Reg8>(cpu: &mut Cpu) {
    let addr: u16 = 0xFF << 8 | cpu.get_r8::<Lsb>() as u16;
    cpu.bus[addr as usize] = cpu.get_r8::<Value>();
}

pub fn write_memory<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    cpu.bus[cpu.get_r16::<Addr>() as usize] = cpu.get_r8::<Value>();
}

pub fn load_r16_r16<Dest: Reg16, Src: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Src>());
}

pub fn load_r16_r16_and_ime<Dest: Reg16, Src: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Src>());
    todo!("IME & interrupt not done");
}

pub fn read_memory_decr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    read_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_sub(1));
}

pub fn write_memory_decr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_sub(1));
}

pub fn read_memory_incr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    read_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_add(1));
}

pub fn write_memory_incr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_add(1));
}

pub fn ld_hl_sp_e_low(cpu: &mut Cpu) {
    let sp_low = cpu.get_r8::<P>();
    let e = cpu.get_r8::<Z>();
    let result = sp_low.wrapping_add(e);
    cpu.set_r8::<L>(result);

    let h = (sp_low & 0x0F) + (e & 0x0F) > 0x0F;
    let c = (sp_low as u16) + (e as u16) > 0xFF;

    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, h);
    cpu.flags.set_flag(Flag::Carry, c);
}

pub fn ld_hl_sp_e_high(cpu: &mut Cpu) {
    let sp_high = cpu.get_r8::<S>();
    let e = cpu.get_r8::<Z>();
    let adj: u8 = if e & 0x80 != 0 { 0xFF } else { 0x00 };
    let carry: u8 = cpu.flags.get_flag(Flag::Carry) as u8;

    cpu.set_r8::<H>(sp_high.wrapping_add(adj).wrapping_add(carry));
}

pub fn write_memory_reassign_pc<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Value>(cpu);
    cpu.set_r16::<PC>(cpu.get_r16::<WZ>());
}

pub fn write_memory_rst_0<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x0);
}
pub fn write_memory_rst_1<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x8);
}
pub fn write_memory_rst_2<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x10);
}
pub fn write_memory_rst_3<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x18);
}
pub fn write_memory_rst_4<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x20);
}
pub fn write_memory_rst_5<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x28);
}
pub fn write_memory_rst_6<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x30);
}
pub fn write_memory_rst_7<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(0x38);
}
