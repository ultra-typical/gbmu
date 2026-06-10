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

    let result = src.wrapping_add(dest).wrapping_add(cpu.flags.get_flag(Flag::Carry) as u8);
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

    let result = src.wrapping_add(dest).wrapping_add(cpu.flags.get_flag(Flag::Carry) as u8);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::{Cpu, Flag};
    use crate::flags::FlagsOps;
    use crate::implemenation::{A, B, P, S, W, Z};

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
    fn add_r8_r8_basic() {
        let mut c = cpu();
        c.set_r8::<A>(5);
        c.set_r8::<B>(3);
        add_r8_r8::<B, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 8);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn add_r8_r8_zero_flag() {
        let mut c = cpu();
        add_r8_r8::<B, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn add_r8_r8_half_carry() {
        let mut c = cpu();
        c.set_r8::<A>(0x08);
        c.set_r8::<B>(0x08);
        add_r8_r8::<B, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x10);
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn add_r8_r8_carry() {
        let mut c = cpu();
        c.set_r8::<A>(0x80);
        c.set_r8::<B>(0x80);
        add_r8_r8::<B, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x00);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn add_r8_r8_with_carry_uses_carry_not_zero() {
        // Bug: uses Flag::Zero instead of Flag::Carry for the carry-in
        let mut c = cpu();
        c.set_r8::<A>(5);
        c.set_r8::<B>(1);
        c.flags.set_flag(Flag::Carry, true);
        c.flags.set_flag(Flag::Zero, false);
        add_r8_r8_with_carry::<B, A>(&mut c);
        // Expected: 5 + 1 + 1(carry) = 7
        assert_eq!(c.get_r8::<A>(), 7);
    }

    #[test]
    fn add_r8_r8_with_carry_no_carry() {
        let mut c = cpu();
        c.set_r8::<A>(5);
        c.set_r8::<B>(2);
        c.flags.set_flag(Flag::Carry, false);
        c.flags.set_flag(Flag::Zero, false);
        add_r8_r8_with_carry::<B, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 7);
    }

    #[test]
    fn add_r8_r8_no_zero_flag_wraps_without_setting_zero() {
        let mut c = cpu();
        c.set_r8::<A>(0xFF);
        c.set_r8::<B>(0x01);
        add_r8_r8_no_zero_flag::<B, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x00);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn add_r8_r8_no_zero_flag_clears_subtract() {
        let mut c = cpu();
        c.set_r8::<A>(1);
        c.set_r8::<B>(2);
        c.flags.set_flag(Flag::Subtract, true);
        add_r8_r8_no_zero_flag::<B, A>(&mut c);
        assert!(!c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn add_hl_sp_e_low_basic() {
        let mut c = cpu();
        c.set_r8::<P>(0x10);
        c.set_r8::<Z>(0x05);
        add_hl_sp_e_low(&mut c);
        assert_eq!(c.get_r8::<Z>(), 0x15);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn add_hl_sp_e_low_half_carry() {
        let mut c = cpu();
        c.set_r8::<P>(0x08);
        c.set_r8::<Z>(0x08);
        add_hl_sp_e_low(&mut c);
        assert_eq!(c.get_r8::<Z>(), 0x10);
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn add_hl_sp_e_high_positive_no_carry() {
        let mut c = cpu();
        c.set_r8::<S>(0x12);
        c.set_r8::<Z>(0x00);
        c.flags.set_flag(Flag::Carry, false);
        add_hl_sp_e_high(&mut c);
        assert_eq!(c.get_r8::<W>(), 0x12);
    }

    #[test]
    fn add_hl_sp_e_high_with_carry() {
        let mut c = cpu();
        c.set_r8::<S>(0x12);
        c.set_r8::<Z>(0x00);
        c.flags.set_flag(Flag::Carry, true);
        add_hl_sp_e_high(&mut c);
        assert_eq!(c.get_r8::<W>(), 0x13);
    }

    #[test]
    fn add_hl_sp_e_high_negative_offset() {
        let mut c = cpu();
        c.set_r8::<S>(0x12);
        c.set_r8::<Z>(0x80); // sign bit set → adj = 0xFF
        c.flags.set_flag(Flag::Carry, false);
        add_hl_sp_e_high(&mut c);
        // 0x12 + 0xFF = 0x11 (wrapping)
        assert_eq!(c.get_r8::<W>(), 0x11);
    }
}
