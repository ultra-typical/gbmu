use crate::implemenation::A;
use crate::{Cpu, defines::Flag, flags::FlagsOps, implemenation::Reg16};

//Some ops effectively use 2 cycles but work on one (i.e. LD (HL), r) so that we put a nothing op so it stills takes two cycles and fetch accordingly
pub fn noop(_cpu: &mut Cpu) {}

pub fn halt(_cpu: &mut Cpu) {
    todo!();
}

pub fn decrement_r16<Reg: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_sub(1));
}

pub fn set_ime_0(cpu: &mut Cpu) {
    todo!()
}

pub fn set_ime_1(cpu: &mut Cpu) {
    todo!()
}

pub fn cpl(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    cpu.set_r8::<A>(!a);

    cpu.flags.set_flag(Flag::Subtract, true);
    cpu.flags.set_flag(Flag::HalfCarry, true);
}

pub fn scf(cpu: &mut Cpu) {
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, true);
}

pub fn ccf(cpu: &mut Cpu) {
    let c = cpu.flags.get_flag(Flag::Carry);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, !c);
}

pub fn daa(cpu: &mut Cpu) {
    let mut a = cpu.get_r8::<A>();
    let mut adjust = 0;
    let mut carry = false;

    let n = cpu.flags.get_flag(Flag::Subtract);
    let h = cpu.flags.get_flag(Flag::HalfCarry);
    let c = cpu.flags.get_flag(Flag::Carry);

    if h || (!n && (a & 0x0F) > 0x09) {
        adjust |= 0x06;
    }

    if c || (!n && a > 0x99) {
        adjust |= 0x60;
        carry = true;
    }

    if n {
        a = a.wrapping_sub(adjust);
    } else {
        a = a.wrapping_add(adjust);
    }

    cpu.set_r8::<A>(a);
    cpu.flags.set_flag(Flag::Zero, a == 0);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, carry);
}

pub fn rlca(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let bit7 = (a & 0x80) >> 7;
    let result = (a << 1) | bit7;

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn rrca(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let bit0 = a & 0x01;
    let result = (a >> 1) | (bit0 << 7);

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn rla(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit7 = (a & 0x80) >> 7;
    let result = (a << 1) | old_carry;

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn rra(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit0 = a & 0x01;
    let result = (a >> 1) | (old_carry << 7);

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

// pub fn increment_r16<Reg: Reg16>(cpu: &mut Cpu) {
//     cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_add(1));
// }
