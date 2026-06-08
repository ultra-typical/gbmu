use std::result;

use crate::{
    Cpu,
    defines::Flag,
    flags::FlagsOps,
    implemenation::{PC, W, WZ, Z},
    instructions::load::load_r16_r16,
};

pub trait Cond {
    fn is_met(cpu: &Cpu) -> bool;
}

pub struct CondNZ; // Not Zero
impl Cond for CondNZ {
    fn is_met(cpu: &Cpu) -> bool {
        !cpu.flags.get_flag(Flag::Zero)
    }
}

pub struct CondZ; // Zero
impl Cond for CondZ {
    fn is_met(cpu: &Cpu) -> bool {
        cpu.flags.get_flag(Flag::Zero)
    }
}

pub struct CondNC; // Not Carry
impl Cond for CondNC {
    fn is_met(cpu: &Cpu) -> bool {
        !cpu.flags.get_flag(Flag::Carry)
    }
}

pub struct CondC; // Carry
impl Cond for CondC {
    fn is_met(cpu: &Cpu) -> bool {
        cpu.flags.get_flag(Flag::Carry)
    }
}

pub fn check_cond<Cc: Cond>(cpu: &mut Cpu) {
    if !Cc::is_met(cpu) {
        cpu.op_index = cpu.queue.len();
    }
}

pub fn relative_jump(cpu: &mut Cpu) {
    let z = cpu.get_r8::<Z>();
    let pc = cpu.get_r16::<PC>();

    let pc_low = (pc & 0xFF) as u8;
    let pc_high = (pc >> 8) as u8;

    let z_sign = (z & 0x80) != 0;

    let sum = z as u16 + pc_low as u16;
    let result = sum as u8;
    let carry_7 = (sum & 0x0100) != 0;

    let adj = if carry_7 && !z_sign {
        1i8
    } else if !carry_7 && z_sign {
        -1i8
    } else {
        0i8
    };

    let w = (pc_high as i32 + adj as i32) as u8;

    cpu.set_r8::<Z>(result);
    cpu.set_r8::<W>(w);

    let wz = ((w as u16) << 8) | (result as u16);
    cpu.set_r16::<PC>(wz);
}
