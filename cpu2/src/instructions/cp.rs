use crate::defines::Flag;
use crate::flags::FlagsOps;
use crate::{defines::Cpu, implemenation::Reg8};

pub fn cp_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest.wrapping_sub(src);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, true);
    cpu.flags.set_flag(Flag::HalfCarry, (dest & 0x0F) < (src & 0x0F));
    cpu.flags.set_flag(Flag::Carry, dest < src);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::{Cpu, Flag};
    use crate::flags::FlagsOps;
    use crate::implemenation::{A, B};

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
    fn cp_equal_sets_zero_and_subtract() {
        let mut c = cpu();
        c.set_r8::<A>(5);
        c.set_r8::<B>(5);
        cp_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 5); // dest unchanged
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn cp_dest_larger_no_carry() {
        let mut c = cpu();
        c.set_r8::<A>(10);
        c.set_r8::<B>(3);
        cp_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 10); // dest unchanged
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn cp_dest_smaller_sets_carry() {
        // Bug: carry flag uses addition check instead of subtraction borrow
        let mut c = cpu();
        c.set_r8::<A>(3);
        c.set_r8::<B>(5);
        cp_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 3); // dest unchanged
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cp_half_carry_borrow() {
        // Bug: half-carry flag uses addition check instead of subtraction borrow
        let mut c = cpu();
        c.set_r8::<A>(0x10);
        c.set_r8::<B>(0x01);
        cp_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x10); // dest unchanged
        assert!(c.flags.get_flag(Flag::HalfCarry));
    }
}
