use crate::Cpu;
use crate::defines::Flag;
use crate::flags::FlagsOps;
use crate::implemenation::{H, L, P, PC, Reg8, Reg16, S, SP, WZ, Z};

pub fn load_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(cpu.get_r8::<Src>());
}

pub fn read_memory<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(cpu.bus[cpu.get_r16::<Addr>() as usize]);
}

pub fn read_memory_0xff<Lsb: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let addr: u16 = 0xFF << 8 | cpu.get_r8::<Lsb>() as u16;
    cpu.set_r8::<Dest>(cpu.bus[addr as usize]);
}

pub fn write_memory_0xff<Lsb: Reg8, Value: Reg8>(cpu: &mut Cpu) {
    let addr: u16 = 0xFF << 8 | cpu.get_r8::<Lsb>() as u16;
    cpu.bus[addr as usize] = cpu.get_r8::<Value>();
}

pub fn write_memory<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    cpu.bus[cpu.get_r16::<Addr>() as usize] = cpu.get_r8::<Value>();
}

pub fn load_r16_r16<Dest: Reg16, Src: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Src>());
}

pub fn load_r16_r16_and_ime<Dest: Reg16, Src: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Src>());
    todo!("IME & interrupt not done");
}

pub fn read_memory_decr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    read_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_sub(1));
}

pub fn write_memory_decr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_sub(1));
}

pub fn read_memory_incr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    read_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_add(1));
}

pub fn write_memory_incr<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<Addr>(cpu.get_r16::<Addr>().wrapping_add(1));
}

pub fn ld_hl_sp_e_low(cpu: &mut Cpu) {
    let sp_low = cpu.get_r8::<P>();
    let e = cpu.get_r8::<Z>();
    let result = sp_low.wrapping_add(e);
    cpu.set_r8::<L>(result);

    let h = (sp_low & 0x0F) + (e & 0x0F) > 0x0F;
    let c = (sp_low as u16) + (e as u16) > 0xFF;

    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, h);
    cpu.flags.set_flag(Flag::Carry, c);
}

pub fn ld_hl_sp_e_high(cpu: &mut Cpu) {
    let sp_high = cpu.get_r8::<S>();
    let e = cpu.get_r8::<Z>();
    let adj: u8 = if e & 0x80 != 0 { 0xFF } else { 0x00 };
    let carry: u8 = cpu.flags.get_flag(Flag::Carry) as u8;

    cpu.set_r8::<H>(sp_high.wrapping_add(adj).wrapping_add(carry));
}

pub fn write_memory_reassign_pc<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Value>(cpu);
    cpu.set_r16::<PC>(cpu.get_r16::<WZ>());
}

pub fn write_memory_rst<const B: u16, Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    write_memory::<Addr, Dest>(cpu);
    cpu.set_r16::<SP>(B);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::{Cpu, Flag};
    use crate::flags::FlagsOps;
    use crate::implemenation::{A, B, C, H, L, HL, PC, SP, WZ, Z};

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
    fn load_r8_r8_copies_value() {
        let mut c = cpu();
        c.set_r8::<B>(42);
        load_r8_r8::<A, B>(&mut c);
        assert_eq!(c.get_r8::<A>(), 42);
        assert_eq!(c.get_r8::<B>(), 42);
    }

    #[test]
    fn read_memory_loads_from_bus() {
        let mut c = cpu();
        c.bus[0x1234] = 99;
        c.set_r16::<HL>(0x1234);
        read_memory::<HL, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 99);
    }

    #[test]
    fn write_memory_stores_to_bus() {
        let mut c = cpu();
        c.set_r8::<A>(77);
        c.set_r16::<HL>(0x2000);
        write_memory::<HL, A>(&mut c);
        assert_eq!(c.bus[0x2000], 77);
    }

    #[test]
    fn read_memory_0xff_reads_from_high_page() {
        let mut c = cpu();
        c.set_r8::<C>(0x40);
        c.bus[0xFF40] = 0xAB;
        read_memory_0xff::<C, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn write_memory_0xff_writes_to_high_page() {
        let mut c = cpu();
        c.set_r8::<A>(0xCD);
        c.set_r8::<C>(0x40);
        write_memory_0xff::<C, A>(&mut c);
        assert_eq!(c.bus[0xFF40], 0xCD);
    }

    #[test]
    fn read_memory_incr_increments_addr() {
        let mut c = cpu();
        c.bus[0x8000] = 0x42;
        c.set_r16::<HL>(0x8000);
        read_memory_incr::<HL, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x42);
        assert_eq!(c.get_r16::<HL>(), 0x8001);
    }

    #[test]
    fn write_memory_incr_increments_addr() {
        let mut c = cpu();
        c.set_r8::<A>(0x55);
        c.set_r16::<HL>(0xC000);
        write_memory_incr::<HL, A>(&mut c);
        assert_eq!(c.bus[0xC000], 0x55);
        assert_eq!(c.get_r16::<HL>(), 0xC001);
    }

    #[test]
    fn read_memory_decr_decrements_addr() {
        let mut c = cpu();
        c.bus[0x8001] = 0x33;
        c.set_r16::<HL>(0x8001);
        read_memory_decr::<HL, A>(&mut c);
        assert_eq!(c.get_r8::<A>(), 0x33);
        assert_eq!(c.get_r16::<HL>(), 0x8000);
    }

    #[test]
    fn write_memory_decr_decrements_addr() {
        let mut c = cpu();
        c.set_r8::<A>(0x11);
        c.set_r16::<HL>(0xC001);
        write_memory_decr::<HL, A>(&mut c);
        assert_eq!(c.bus[0xC001], 0x11);
        assert_eq!(c.get_r16::<HL>(), 0xC000);
    }

    #[test]
    fn load_r16_r16_copies_pair() {
        let mut c = cpu();
        c.set_r16::<SP>(0x1234);
        load_r16_r16::<HL, SP>(&mut c);
        assert_eq!(c.get_r16::<HL>(), 0x1234);
    }

    #[test]
    fn ld_hl_sp_e_low_basic() {
        use crate::implemenation::P;
        let mut c = cpu();
        c.set_r8::<P>(0x50);
        c.set_r8::<Z>(0x10);
        ld_hl_sp_e_low(&mut c);
        assert_eq!(c.get_r8::<L>(), 0x60);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn ld_hl_sp_e_low_carry() {
        use crate::implemenation::P;
        let mut c = cpu();
        c.set_r8::<P>(0xFF);
        c.set_r8::<Z>(0x01);
        ld_hl_sp_e_low(&mut c);
        assert_eq!(c.get_r8::<L>(), 0x00);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn ld_hl_sp_e_high_no_carry() {
        use crate::implemenation::S;
        let mut c = cpu();
        c.set_r8::<S>(0x30);
        c.set_r8::<Z>(0x10);
        c.flags.set_flag(Flag::Carry, false);
        ld_hl_sp_e_high(&mut c);
        assert_eq!(c.get_r8::<H>(), 0x30);
    }

    #[test]
    fn ld_hl_sp_e_high_with_carry() {
        use crate::implemenation::S;
        let mut c = cpu();
        c.set_r8::<S>(0x30);
        c.set_r8::<Z>(0x10);
        c.flags.set_flag(Flag::Carry, true);
        ld_hl_sp_e_high(&mut c);
        assert_eq!(c.get_r8::<H>(), 0x31);
    }

    #[test]
    fn write_memory_reassign_pc_updates_pc() {
        let mut c = cpu();
        c.set_r16::<WZ>(0x0150);
        c.set_r8::<A>(0x42);
        c.set_r16::<SP>(0xFFFE);
        write_memory_reassign_pc::<SP, A>(&mut c);
        assert_eq!(c.bus[0xFFFE], 0x42);
        assert_eq!(c.get_r16::<PC>(), 0x0150);
    }
}

