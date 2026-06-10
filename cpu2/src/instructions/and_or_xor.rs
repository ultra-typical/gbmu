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
    fn and_basic() {
        let mut c = cpu();
        c.set_r8::<A>(0b1111_0000);
        c.set_r8::<B>(0b1010_1010);
        and_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b1010_0000);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn and_zero_flag() {
        let mut c = cpu();
        c.set_r8::<A>(0b1111_0000);
        c.set_r8::<B>(0b0000_1111);
        and_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn and_always_sets_half_carry() {
        let mut c = cpu();
        c.set_r8::<A>(0xFF);
        c.set_r8::<B>(0xFF);
        and_r8_r8::<A, B>(&mut c);
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn or_basic() {
        let mut c = cpu();
        c.set_r8::<A>(0b1111_0000);
        c.set_r8::<B>(0b0000_1111);
        or_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0xFF);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn or_zero_flag() {
        let mut c = cpu();
        or_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn xor_basic() {
        let mut c = cpu();
        c.set_r8::<A>(0b1111_0000);
        c.set_r8::<B>(0b1010_1010);
        xor_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b0101_1010);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn xor_self_zero() {
        let mut c = cpu();
        c.set_r8::<A>(0xFF);
        xor_r8_r8::<A, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0);
        assert!(c.flags.get_flag(Flag::Zero));
    }
}
