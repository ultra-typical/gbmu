use crate::cpu::defines::Cpu;
use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::cpu::*;
use crate::mmu::MemoryMapper;

//Some ops effectively use 2 cycles but work on one (i.e. LD (HL), r) so that we put a nothing op so it stills takes two cycles and fetch accordingly
//

impl<M: MemoryMapper> Cpu<M> {
    pub fn noop(&mut self, _bus: &mut M) {}

    pub fn halt(&mut self, _bus: &mut M) {
        self.halted = true;
    }

    pub fn decrement_r16<Reg: Reg16>(&mut self, _bus: &mut M) {
        self.set_r16::<Reg>(self.get_r16::<Reg>().wrapping_sub(1));
    }

    pub fn set_ime_0(&mut self, _bus: &mut M) {
        self.ime = false;
    }

    pub fn set_ime_delay_1(&mut self, _bus: &mut M ) {
        self.ime_delay = true;
    }

    pub fn set_ime_1(&mut self, _bus: &mut M) {
        self.ime = true;
    }

    pub fn cpl(&mut self, _bus: &mut M) {
        let a = self.get_r8::<A>();
        self.set_r8::<A>(!a);

        self.flags.set_flag(Flag::Subtract, true);
        self.flags.set_flag(Flag::HalfCarry, true);
    }

    pub fn scf(&mut self, _bus: &mut M) {
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, true);
    }

    pub fn ccf(&mut self, _bus: &mut M) {
        let c = self.flags.get_flag(Flag::Carry);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, !c);
    }

    pub fn daa(&mut self, _bus: &mut M) {
        let mut a = self.get_r8::<A>();
        let mut adjust = 0;
        let mut carry = false;

        let n = self.flags.get_flag(Flag::Subtract);
        let h = self.flags.get_flag(Flag::HalfCarry);
        let c = self.flags.get_flag(Flag::Carry);

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

        self.set_r8::<A>(a);
        self.flags.set_flag(Flag::Zero, a == 0);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, carry);
    }

    pub fn rlca(&mut self, _bus: &mut M) {
        let a = self.get_r8::<A>();
        let bit7 = (a & 0x80) >> 7;
        let result = (a << 1) | bit7;

        self.set_r8::<A>(result);
        self.flags.set_flag(Flag::Zero, false);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit7 == 1);
    }

    pub fn rrca(&mut self, _bus: &mut M) {
        let a = self.get_r8::<A>();
        let bit0 = a & 0x01;
        let result = (a >> 1) | (bit0 << 7);

        self.set_r8::<A>(result);
        self.flags.set_flag(Flag::Zero, false);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit0 == 1);
    }

    pub fn rla(&mut self, _bus: &mut M) {
        let a = self.get_r8::<A>();
        let old_carry = if self.flags.get_flag(Flag::Carry) {
            1
        } else {
            0
        };
        let bit7 = (a & 0x80) >> 7;
        let result = (a << 1) | old_carry;

        self.set_r8::<A>(result);
        self.flags.set_flag(Flag::Zero, false);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit7 == 1);
    }

    pub fn rra(&mut self, _bus: &mut M) {
        let a = self.get_r8::<A>();
        let old_carry = if self.flags.get_flag(Flag::Carry) {
            1
        } else {
            0
        };
        let bit0 = a & 0x01;
        let result = (a >> 1) | (old_carry << 7);

        self.set_r8::<A>(result);
        self.flags.set_flag(Flag::Zero, false);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit0 == 1);
    }

    pub fn rlc<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let bit7 = (val & 0x80) >> 7;
        let result = (val << 1) | bit7;

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit7 == 1);
    }

    pub fn write_rlc_mem<Addr: Reg16, Reg: Reg8>(&mut self, _bus: &mut M) {
        Self::rlc::<Reg>(self, _bus);
        Self::write_memory::<Addr, Reg>(self, _bus);
    }

    pub fn write_rrc_mem<Addr: Reg16, Reg: Reg8>(&mut self, _bus: &mut M) {
        Self::rrc::<Reg>(self, _bus);
        Self::write_memory::<HL, Reg>(self, _bus);
    }

    pub fn write_rl_mem<Addr: Reg16, Reg: Reg8>(&mut self, _bus: &mut M) {
        Self::rl::<Reg>(self, _bus);
        Self::write_memory::<HL, Reg>(self, _bus);
    }

    pub fn write_rr_mem<Addr: Reg16, Reg: Reg8>(&mut self, _bus: &mut M) {
        Self::rr::<Reg>(self, _bus);
        Self::write_memory::<HL, Reg>(self, _bus);
    }

    pub fn write_sla_mem<Addr: Reg16, Reg: Reg8>(&mut self, _bus: &mut M) {
        Self::sla::<Reg>(self, _bus);
        Self::write_memory::<HL, Reg>(self, _bus);
    }

    pub fn write_sra_mem<Addr: Reg16, Reg: Reg8>(&mut self, _bus: &mut M) {
        Self::sra::<Reg>(self, _bus);
        Self::write_memory::<HL, Reg>(self, _bus);
    }

    pub fn rl<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let old_carry = if self.flags.get_flag(Flag::Carry) {
            1
        } else {
            0
        };
        let bit7 = (val & 0x80) >> 7;
        let result = (val << 1) | old_carry;

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit7 == 1);
    }

    pub fn rrc<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let bit0 = val & 0x01;
        let result = (val >> 1) | (bit0 << 7);

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit0 == 1);
    }

    pub fn rr<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let old_carry = if self.flags.get_flag(Flag::Carry) {
            1
        } else {
            0
        };
        let bit0 = val & 0x01;
        let result = (val >> 1) | (old_carry << 7);

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit0 == 1);
    }

    pub fn sla<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let bit7 = (val & 0x80) >> 7;
        let result = val << 1;

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit7 == 1);
    }

    pub fn sra<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let bit0 = val & 0x01;
        let bit7 = val & 0x80;
        let result = (val >> 1) | bit7;

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit0 == 1);
    }

    pub fn swap<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let result = val.rotate_left(4);

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, false);
    }

    pub fn srl<Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let bit0 = val & 0x01;
        let result = val >> 1;

        self.set_r8::<Reg>(result);
        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, false);
        self.flags.set_flag(Flag::Carry, bit0 == 1);
    }

    pub fn bit<const B: u8, Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let is_bit_zero = (val & (1 << B)) == 0;

        self.flags.set_flag(Flag::Zero, is_bit_zero);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, true);
    }

    pub fn res<const B: u8, Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let result = val & !(1 << B);
        self.set_r8::<Reg>(result);
    }

    pub fn set<const B: u8, Reg: Reg8>(&mut self, _bus: &mut M) {
        let val = self.get_r8::<Reg>();
        let result = val | (1 << B);
        self.set_r8::<Reg>(result);
    }

    pub fn write_swap_mem<Addr: Reg16, Reg: Reg8>(&mut self, bus: &mut M) {
        Self::swap::<Reg>(self, bus);
        Self::write_memory::<Addr, Reg>(self, bus);
    }

    pub fn write_srl_mem<Addr: Reg16, Reg: Reg8>(&mut self, bus: &mut M) {
        Self::srl::<Reg>(self, bus);
        Self::write_memory::<Addr, Reg>(self, bus);
    }

    pub fn write_res_mem<const B: u8, Addr: Reg16, Reg: Reg8>(&mut self, bus: &mut M) {
        Self::res::<{ B }, Reg>(self, bus);
        Self::write_memory::<Addr, Reg>(self, bus);
    }

    pub fn write_set_mem<const B: u8, Addr: Reg16, Reg: Reg8>(&mut self, bus: &mut M) {
        Self::set::<{ B }, Reg>(self, bus);
        Self::write_memory::<Addr, Reg>(self, bus);
    }

    pub fn stop(&mut self, _bus: &mut M) {
        todo!("stop todo");
    }
}

// pub fn increment_r16<Reg: Reg16>(&mut self, _bus: &mut M) {
//     self.set_r16::<Reg>(self.get_r16::<Reg>().wrapping_add(1));
// }
