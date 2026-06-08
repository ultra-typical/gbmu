use crate::defines::Flag;
use crate::flags::FlagsOps;
use crate::{defines::Cpu, implemenation::Reg8};

pub fn and_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest & src;

    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, true);

    cpu.flags.set_flag(Flag::Carry, false);
}

pub fn or_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest | src;

    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);

    cpu.flags.set_flag(Flag::Carry, false);
}

pub fn xor_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest ^ src;

    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);

    cpu.flags.set_flag(Flag::Carry, false);
}
