use crate::defines::Flag;
use crate::flags::FlagsOps;
use crate::{defines::Cpu, implemenation::Reg8};

pub fn sub_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest.wrapping_sub(src);

    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, true);
    cpu.flags.set_flag(Flag::HalfCarry, (dest & 0x0F) < (src & 0x0F));
    cpu.flags.set_flag(Flag::Carry, dest < src);
}

pub fn sub_r8_r8_with_carry<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let borrow = cpu.flags.get_flag(Flag::Carry) as u8;
    let result = dest.wrapping_sub(src).wrapping_sub(borrow);
    cpu.set_r8::<Dest>(result);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, true);
    cpu.flags.set_flag(Flag::HalfCarry, (dest & 0x0F) < (src & 0x0F) + borrow);
    cpu.flags.set_flag(Flag::Carry, (dest as u16) < (src as u16) + borrow as u16);
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
    fn sub_r8_r8_basic() {
        // Bug: uses wrapping_add instead of wrapping_sub
        let mut c = cpu();
        c.set_r8::<A>(10);
        c.set_r8::<B>(3);
        sub_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 7);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn sub_r8_r8_zero_flag() {
        let mut c = cpu();
        c.set_r8::<A>(5);
        c.set_r8::<B>(5);
        sub_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn sub_r8_r8_carry_borrow() {
        let mut c = cpu();
        c.set_r8::<A>(0x00);
        c.set_r8::<B>(0x01);
        sub_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0xFF);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn sub_r8_r8_half_carry_borrow() {
        let mut c = cpu();
        c.set_r8::<A>(0x10);
        c.set_r8::<B>(0x01);
        sub_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x0F);
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn sub_r8_r8_with_carry_uses_carry_flag() {
        // Bug: uses Flag::Zero instead of Flag::Carry for carry-in
        // Bug: computes src - dest instead of dest - src
        let mut c = cpu();
        c.set_r8::<A>(10);
        c.set_r8::<B>(3);
        c.flags.set_flag(Flag::Carry, true);
        c.flags.set_flag(Flag::Zero, false);
        sub_r8_r8_with_carry::<B, A>(&mut c);
        // Expected: A = A - B - carry = 10 - 3 - 1 = 6
        assert_eq!(c.get_r8::<A>(), 6);
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn sub_r8_r8_with_carry_no_carry() {
        let mut c = cpu();
        c.set_r8::<A>(10);
        c.set_r8::<B>(3);
        c.flags.set_flag(Flag::Carry, false);
        c.flags.set_flag(Flag::Zero, false);
        sub_r8_r8_with_carry::<B, A>(&mut c);
        // Expected: A = A - B - 0 = 7
        assert_eq!(c.get_r8::<A>(), 7);
    }
}
