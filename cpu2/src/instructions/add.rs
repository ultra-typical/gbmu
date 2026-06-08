use crate::defines::{Cpu, Flag};
use crate::flags::FlagsOps;
use crate::implemenation::{P, Reg8, S, W, Z};

pub fn add_r8_r8<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest.wrapping_add(src);

    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}

pub fn add_r8_r8_with_carry<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = src + dest + cpu.flags.get_flag(Flag::Zero) as u8;
    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}

pub fn add_r8_r8_no_zero_flag<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest.wrapping_add(src);

    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}

pub fn add_r8_r8_with_carry_and_no_zero_flag<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = src + dest + cpu.flags.get_flag(Flag::Zero) as u8;
    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}

pub fn add_hl_sp_e_low(cpu: &mut Cpu) {
    let sp_low = cpu.get_r8::<P>();
    let e = cpu.get_r8::<Z>();
    let result = sp_low.wrapping_add(e);
    cpu.set_r8::<Z>(result);

    let h = (sp_low & 0x0F) + (e & 0x0F) > 0x0F;
    let c = (sp_low as u16) + (e as u16) > 0xFF;

    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, h);
    cpu.flags.set_flag(Flag::Carry, c);
}

pub fn add_hl_sp_e_high(cpu: &mut Cpu) {
    let sp_high = cpu.get_r8::<S>();
    let e = cpu.get_r8::<Z>();
    let adj: u8 = if e & 0x80 != 0 { 0xFF } else { 0x00 };
    let carry: u8 = cpu.flags.get_flag(Flag::Carry) as u8;

    cpu.set_r8::<W>(sp_high.wrapping_add(adj).wrapping_add(carry));
}
