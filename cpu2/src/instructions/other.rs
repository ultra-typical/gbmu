use crate::implemenation::{A, HL, Reg8};
use crate::instructions::load::write_memory;
use crate::{Cpu, defines::Flag, flags::FlagsOps, implemenation::Reg16};

//Some ops effectively use 2 cycles but work on one (i.e. LD (HL), r) so that we put a nothing op so it stills takes two cycles and fetch accordingly
pub fn noop(_cpu: &mut Cpu) {}

pub fn halt(_cpu: &mut Cpu) {
    todo!();
}

pub fn decrement_r16<Reg: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_sub(1));
}

pub fn set_ime_0(_cpu: &mut Cpu) {
    todo!()
}

pub fn set_ime_1(_cpu: &mut Cpu) {
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

pub fn rlc<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit7 = (val & 0x80) >> 7;
    let result = (val << 1) | bit7;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn write_rlc_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rlc::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_rrc_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rrc::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_rl_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rl::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_rr_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rr::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_sla_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    sla::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_sra_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    sra::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn rl<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit7 = (val & 0x80) >> 7;
    let result = (val << 1) | old_carry;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn rrc<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit0 = val & 0x01;
    let result = (val >> 1) | (bit0 << 7);

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn rr<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit0 = val & 0x01;
    let result = (val >> 1) | (old_carry << 7);

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn sla<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit7 = (val & 0x80) >> 7;
    let result = val << 1;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn sra<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit0 = val & 0x01;
    let bit7 = val & 0x80;
    let result = (val >> 1) | bit7;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn swap<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let result = val.rotate_left(4);

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, false);
}

pub fn srl<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit0 = val & 0x01;
    let result = val >> 1;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}


pub fn bit<const B: u8, Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let is_bit_zero = (val & (1 << B)) == 0;

    cpu.flags.set_flag(Flag::Zero, is_bit_zero);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, true);
}

pub fn res<const B: u8, Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let result = val & !(1 << B);
    cpu.set_r8::<Reg>(result);
}

pub fn set<const B: u8, Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let result = val | (1 << B);
    cpu.set_r8::<Reg>(result);
}


pub fn write_swap_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    swap::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_srl_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    srl::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_res_mem<const B: u8, Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    res::<B, Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_set_mem<const B: u8, Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    set::<B, Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

// pub fn increment_r16<Reg: Reg16>(cpu: &mut Cpu) {
//     cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_add(1));
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::{Cpu, Flag};
    use crate::flags::FlagsOps;
    use crate::implemenation::{A, B, C};

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

    // --- noop ---

    #[test]
    fn noop_does_nothing() {
        let mut c = cpu();
        c.set_r8::<A>(42);
        noop(&mut c);
        assert_eq!(c.get_r8::<A>(), 42);
        assert_eq!(c.flags, 0);
    }

    // --- cpl ---

    #[test]
    fn cpl_flips_all_bits() {
        let mut c = cpu();
        c.set_r8::<A>(0b1010_0101);
        cpl(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b0101_1010);
        assert!(c.flags.get_flag(Flag::Subtract));
        assert!(c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn cpl_zero_becomes_ff() {
        let mut c = cpu();
        c.set_r8::<A>(0x00);
        cpl(&mut c);
        assert_eq!(c.get_r8::<A>(), 0xFF);
    }

    // --- scf ---

    #[test]
    fn scf_sets_carry_clears_n_h() {
        let mut c = cpu();
        c.flags.set_flag(Flag::Subtract, true);
        c.flags.set_flag(Flag::HalfCarry, true);
        scf(&mut c);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
    }

    // --- ccf ---

    #[test]
    fn ccf_toggles_carry() {
        let mut c = cpu();
        c.flags.set_flag(Flag::Carry, true);
        ccf(&mut c);
        assert!(!c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));

        ccf(&mut c);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    // --- daa ---

    #[test]
    fn daa_after_bcd_add() {
        let mut c = cpu();
        // 9 + 9 = 18 → 0x12 in BCD → DAA should fix A=0x18 to 0x18 (already valid BCD)
        // Actually 9+9=18 decimal = 0x12 hex but BCD needs it as 0x18
        // Let's use: A=0x15 (9+6 without correction), N=false, H=false, C=false
        // → lower nibble 5 ≤ 9 and upper 1 ≤ 9, no adjust needed
        c.set_r8::<A>(0x15);
        c.flags.set_flag(Flag::Subtract, false);
        daa(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x15);
        assert!(!c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn daa_after_bcd_add_with_lower_overflow() {
        let mut c = cpu();
        // 8 + 5 = 13 → 0x0D, DAA should add 6 to get 0x13 (BCD for 13)
        c.set_r8::<A>(0x0D);
        c.flags.set_flag(Flag::Subtract, false);
        daa(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x13);
    }

    // --- rlca ---

    #[test]
    fn rlca_rotates_left_circular() {
        let mut c = cpu();
        c.set_r8::<A>(0b1000_0001);
        rlca(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b0000_0011);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn rlca_no_carry() {
        let mut c = cpu();
        c.set_r8::<A>(0b0000_0001);
        rlca(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b0000_0010);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // --- rrca ---

    #[test]
    fn rrca_rotates_right_circular() {
        let mut c = cpu();
        c.set_r8::<A>(0b0000_0001);
        rrca(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b1000_0000);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rrca_no_carry() {
        let mut c = cpu();
        c.set_r8::<A>(0b1000_0000);
        rrca(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b0100_0000);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // --- rla ---

    #[test]
    fn rla_rotates_left_through_carry() {
        let mut c = cpu();
        c.set_r8::<A>(0b1000_0000);
        c.flags.set_flag(Flag::Carry, true);
        rla(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b0000_0001);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rla_no_carry_in() {
        let mut c = cpu();
        c.set_r8::<A>(0b0100_0000);
        c.flags.set_flag(Flag::Carry, false);
        rla(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b1000_0000);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // --- rra ---

    #[test]
    fn rra_rotates_right_through_carry() {
        let mut c = cpu();
        c.set_r8::<A>(0b0000_0001);
        c.flags.set_flag(Flag::Carry, true);
        rra(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b1000_0000);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rra_no_carry_in() {
        let mut c = cpu();
        c.set_r8::<A>(0b0000_0010);
        c.flags.set_flag(Flag::Carry, false);
        rra(&mut c);
        assert_eq!(c.get_r8::<A>(), 0b0000_0001);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // --- CB: rlc ---

    #[test]
    fn rlc_rotates_left_circular() {
        let mut c = cpu();
        c.set_r8::<B>(0b1000_0001);
        rlc::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0000_0011);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rlc_zero_flag() {
        let mut c = cpu();
        c.set_r8::<B>(0x00);
        rlc::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // --- CB: rrc ---

    #[test]
    fn rrc_rotates_right_circular() {
        let mut c = cpu();
        c.set_r8::<B>(0b0000_0001);
        rrc::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b1000_0000);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    // --- CB: rl ---

    #[test]
    fn rl_rotates_left_through_carry() {
        let mut c = cpu();
        c.set_r8::<B>(0b1000_0000);
        c.flags.set_flag(Flag::Carry, true);
        rl::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0000_0001);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rl_zero_flag_when_result_zero() {
        let mut c = cpu();
        c.set_r8::<B>(0x00);
        c.flags.set_flag(Flag::Carry, false);
        rl::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(c.flags.get_flag(Flag::Zero));
    }

    // --- CB: rr ---

    #[test]
    fn rr_rotates_right_through_carry() {
        let mut c = cpu();
        c.set_r8::<B>(0b0000_0001);
        c.flags.set_flag(Flag::Carry, true);
        rr::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b1000_0000);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    // --- CB: sla ---

    #[test]
    fn sla_shifts_left() {
        let mut c = cpu();
        c.set_r8::<B>(0b1000_0001);
        sla::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0000_0010);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn sla_zero_flag() {
        let mut c = cpu();
        c.set_r8::<B>(0b1000_0000);
        sla::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Carry));
    }

    // --- CB: sra ---

    #[test]
    fn sra_preserves_sign_bit() {
        let mut c = cpu();
        c.set_r8::<B>(0b1000_0001);
        sra::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b1100_0000);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn sra_positive_value() {
        let mut c = cpu();
        c.set_r8::<B>(0b0100_0010);
        sra::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0010_0001);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // --- CB: swap ---

    #[test]
    fn swap_nibbles() {
        let mut c = cpu();
        c.set_r8::<B>(0xAB);
        swap::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0xBA);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn swap_zero_flag() {
        let mut c = cpu();
        c.set_r8::<B>(0x00);
        swap::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(c.flags.get_flag(Flag::Zero));
    }

    // --- CB: srl ---

    #[test]
    fn srl_shifts_right_logical() {
        let mut c = cpu();
        c.set_r8::<B>(0b1000_0001);
        srl::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0100_0000);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn srl_clears_msb() {
        let mut c = cpu();
        c.set_r8::<B>(0b1000_0000);
        srl::<B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0100_0000);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // --- CB: bit ---

    #[test]
    fn bit_set_clears_zero_flag() {
        let mut c = cpu();
        c.set_r8::<B>(0b0000_1000); // bit 3 is set
        bit::<3, B>(&mut c);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn bit_clear_sets_zero_flag() {
        let mut c = cpu();
        c.set_r8::<B>(0b0000_0000); // bit 3 is clear
        bit::<3, B>(&mut c);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn bit_does_not_modify_register() {
        let mut c = cpu();
        c.set_r8::<B>(0b1111_1111);
        bit::<7, B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b1111_1111);
    }

    // --- CB: res ---

    #[test]
    fn res_clears_bit() {
        let mut c = cpu();
        c.set_r8::<B>(0b1111_1111);
        res::<3, B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b1111_0111);
    }

    #[test]
    fn res_already_clear() {
        let mut c = cpu();
        c.set_r8::<B>(0b0000_0000);
        res::<0, B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0000_0000);
    }

    // --- CB: set ---

    #[test]
    fn set_sets_bit() {
        let mut c = cpu();
        c.set_r8::<B>(0b0000_0000);
        set::<3, B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b0000_1000);
    }

    #[test]
    fn set_already_set() {
        let mut c = cpu();
        c.set_r8::<B>(0b1111_1111);
        set::<7, B>(&mut c);
        assert_eq!(c.get_r8::<B>(), 0b1111_1111);
    }

    #[test]
    fn set_all_bits() {
        let mut c = cpu();
        c.set_r8::<C>(0x00);
        for b in 0u8..8 {
            match b {
                0 => set::<0, C>(&mut c),
                1 => set::<1, C>(&mut c),
                2 => set::<2, C>(&mut c),
                3 => set::<3, C>(&mut c),
                4 => set::<4, C>(&mut c),
                5 => set::<5, C>(&mut c),
                6 => set::<6, C>(&mut c),
                7 => set::<7, C>(&mut c),
                _ => {}
            }
        }
        assert_eq!(c.get_r8::<C>(), 0xFF);
    }
}
