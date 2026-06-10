use crate::{
    Cpu,
    defines::Flag,
    flags::FlagsOps,
    implemenation::{PC, W, Z},
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::{Cpu, Flag, MicroOp};
    use crate::flags::FlagsOps;
    use crate::implemenation::{PC, Z};

    fn noop_op(_: &mut Cpu) {}
    static QUEUE: &[MicroOp] = &[noop_op, noop_op, noop_op];

    fn cpu_with_queue() -> Cpu {
        Cpu {
            queue: QUEUE,
            r8: [0; 14],
            flags: 0,
            instructions_list: vec![],
            op_index: 1,
            bus: [0; 0x10000],
        }
    }

    fn cpu() -> Cpu {
        Cpu {
            queue: &[],
            r8: [0; 14],
            flags: 0,
            instructions_list: vec![],
            op_index: 0,
            bus: [0; 0x10000],
        }
    }

    #[test]
    fn cond_nz_met_does_not_skip() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Zero, false);
        check_cond::<CondNZ>(&mut c);
        assert_ne!(c.op_index, QUEUE.len());
    }

    #[test]
    fn cond_nz_not_met_skips() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Zero, true);
        check_cond::<CondNZ>(&mut c);
        assert_eq!(c.op_index, QUEUE.len());
    }

    #[test]
    fn cond_z_met_does_not_skip() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Zero, true);
        check_cond::<CondZ>(&mut c);
        assert_ne!(c.op_index, QUEUE.len());
    }

    #[test]
    fn cond_z_not_met_skips() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Zero, false);
        check_cond::<CondZ>(&mut c);
        assert_eq!(c.op_index, QUEUE.len());
    }

    #[test]
    fn cond_nc_met_does_not_skip() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Carry, false);
        check_cond::<CondNC>(&mut c);
        assert_ne!(c.op_index, QUEUE.len());
    }

    #[test]
    fn cond_nc_not_met_skips() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Carry, true);
        check_cond::<CondNC>(&mut c);
        assert_eq!(c.op_index, QUEUE.len());
    }

    #[test]
    fn cond_c_met_does_not_skip() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Carry, true);
        check_cond::<CondC>(&mut c);
        assert_ne!(c.op_index, QUEUE.len());
    }

    #[test]
    fn cond_c_not_met_skips() {
        let mut c = cpu_with_queue();
        c.flags.set_flag(Flag::Carry, false);
        check_cond::<CondC>(&mut c);
        assert_eq!(c.op_index, QUEUE.len());
    }

    #[test]
    fn relative_jump_positive_offset() {
        let mut c = cpu();
        c.set_r16::<PC>(0x0100);
        c.set_r8::<Z>(0x10); // +16
        relative_jump(&mut c);
        assert_eq!(c.get_r16::<PC>(), 0x0110);
    }

    #[test]
    fn relative_jump_negative_offset() {
        let mut c = cpu();
        c.set_r16::<PC>(0x0110);
        c.set_r8::<Z>((-16i8) as u8); // -16
        relative_jump(&mut c);
        assert_eq!(c.get_r16::<PC>(), 0x0100);
    }

    #[test]
    fn relative_jump_zero_offset() {
        let mut c = cpu();
        c.set_r16::<PC>(0x0200);
        c.set_r8::<Z>(0x00);
        relative_jump(&mut c);
        assert_eq!(c.get_r16::<PC>(), 0x0200);
    }

    #[test]
    fn relative_jump_page_boundary_positive() {
        let mut c = cpu();
        c.set_r16::<PC>(0x00F0);
        c.set_r8::<Z>(0x20); // +32, crosses 0x0100
        relative_jump(&mut c);
        assert_eq!(c.get_r16::<PC>(), 0x0110);
    }

    #[test]
    fn relative_jump_page_boundary_negative() {
        let mut c = cpu();
        c.set_r16::<PC>(0x0100);
        c.set_r8::<Z>((-32i8) as u8); // -32, crosses page boundary
        relative_jump(&mut c);
        assert_eq!(c.get_r16::<PC>(), 0x00E0);
    }
}
