#[cfg(test)]
mod tests {
    use crate::cpu_def::*;
    use crate::cpu::defines::Flag;
    use crate::mmu::timers::DmgTimers;
    use crate::ppu::DmgPpu;
    use crate::mmu::MemoryMapper;
    use crate::cpu::flags::FlagsOps;
    use crate::mmu::DmgMmu;
    use crate::gameboy::GameBoy;
    use crate::mmu::mbc::*;

    // Creates a GameBoy with the given first opcode and pads with NOPs so post-instruction
    // fetch never goes out of bounds. Tests operate on the GameBoy and its inner CPU (`gb.cpu`).
    fn gb<M>(opcode: u8) -> GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>> {
        let mut gb: GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>> =
            GameBoy::new(None, Vec::new(), None).expect("Failed to create gb");
        gb.bus.write_byte(0x8000, opcode);
        gb.cpu.r8 = Default::default(); //mental retardation
        gb.cpu.set_r16::<PC>(0x8000);
        gb
    }

    fn ticks(gb: &mut GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>>, n: usize) {
        for _ in 0..n {
            gb.cpu.tick(&mut gb.bus);
        }
    }

    #[test]
    fn op_00_noop() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x00);
        c.cpu.set_r16::<PC>(0x8000); // set correct pc lo
        c.cpu.first_read(&mut c.bus);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0);
        assert_eq!(c.cpu.get_r8::<B>(), 0);
        assert_eq!(c.cpu.get_r8::<C>(), 0);
        assert_eq!(c.cpu.get_r8::<D>(), 0);
        assert_eq!(c.cpu.get_r8::<E>(), 0);
        assert_eq!(c.cpu.get_r8::<H>(), 0);
        assert_eq!(c.cpu.get_r8::<L>(), 0);
        assert_eq!(c.cpu.get_r16::<SP>(), 0);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8002);
        assert_eq!(c.cpu.get_r16::<AF>(), 0x0000);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x0000);
        assert_eq!(c.cpu.get_r16::<DE>(), 0x0000);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_01_ld_bc_d16() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x01);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x34);
        c.bus.write_byte(0x8002, 0x12);
        
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8004);
    }

    #[test]
    fn op_02_ld_a_bc() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x02);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0xAB);
        c.cpu.set_r16::<BC>(0x8003);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8003), c.cpu.get_r8::<A>());
    }

    #[test]
    fn op_03_inc_bc() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x03);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<BC>(0x1234);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x1235);
    }

    #[test]
    fn op_04_inc_b_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x04);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_04_inc_b_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x04);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }


    #[test]
    fn op_0c_inc_c_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_0c_inc_c_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_14_inc_d_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x14);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_14_inc_d_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x14);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1c_inc_e_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1c_inc_e_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_24_inc_h_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x24);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_24_inc_h_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x24);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2c_inc_l_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2c_inc_l_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3c_inc_a_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3c_inc_a_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_34_inc_hl_mem_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x34);
        c.cpu.first_read(&mut c.bus);
        
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0x0F); 
        
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_34_inc_hl_mem_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x34);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0xFE);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_05_dec_b_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x05);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_05_dec_b_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x05);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_0d_dec_c_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_0d_dec_c_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_15_dec_d_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x15);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_15_dec_d_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x15);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1d_dec_e_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1d_dec_e_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_25_dec_h_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x25);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_25_dec_h_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x25);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2d_dec_l_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2d_dec_l_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3d_dec_a_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3d_dec_a_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_35_dec_hl_mem_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x35);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0xFE);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0xFD);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }


    #[test]
    fn op_35_dec_hl_mem_no_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x35);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0x0F);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0x0E);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_80_add_a_b_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x80);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x0F);
        c.cpu.set_r8::<B>(0x01);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_80_add_a_b_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x80);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0xFF);
        c.cpu.set_r8::<B>(0x01);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x00);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_80_add_a_b_half_carry_carry_and_zero() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x80);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0xFF);
        c.cpu.set_r8::<B>(0x01);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x00);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_06_ld_b_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x06);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0xAB);
    }

    #[test]
    fn op_07_rlca() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x07);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b1000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0011);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_07_rlca_no_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x07);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b0000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0010);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_08_ld_a16_sp() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x08);
        c.bus.write_byte(0x8003, 0x08);

        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0x1234);
        c.bus.write_byte(0x8001, 0x50);
        c.bus.write_byte(0x8002, 0x80);
        ticks(&mut c, 5);
        assert_eq!(c.bus.read_byte(0x8050), 0x34);
        assert_eq!(c.bus.read_byte(0x8051), 0x12);

        c.bus.write_byte(0x8004, 0x34);
        c.bus.write_byte(0x8005, 0x80);
        ticks(&mut c, 5);
        assert_eq!(c.bus.read_byte(0x8034), 0x34);
        assert_eq!(c.bus.read_byte(0x8035), 0x12);
    }


    #[test]
    fn op_09_add_hl_bc() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x09);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x1234);
        c.cpu.set_r16::<BC>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x2468);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_09_add_hl_bc_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x09);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x0FFF);
        c.cpu.set_r16::<BC>(0x0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x1000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_09_add_hl_bc_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x09);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xFFFF);
        c.cpu.set_r16::<BC>(0x0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_09_add_hl_bc_carry_other_inverted() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x09);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x0001);
        c.cpu.set_r16::<BC>(0xFFFF);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_0a_ld_a_bc() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0A);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<BC>(0xC000);
        c.bus.write_byte(0xC000, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_0b_dec_bc() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0B);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<BC>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x1233);
    }

    
    #[test]
    fn op_1b_dec_de() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1B);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<DE>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<DE>(), 0x1233);
    }
    
    #[test]
    fn op_2b_dec_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2B);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x1233);
    }

    #[test]
    fn op_3b_dec_sp() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3B);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<SP>(), 0x1233);
    }

    #[test]
    fn op_0e_ld_c_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0E);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<C>(), 0xAB);
    }

    #[test]
    fn op_1e_ld_e_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1E);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<E>(), 0xAB);
    }

    #[test]
    fn op_2e_ld_l_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2E);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0xAB);
    }

    #[test]
    fn op_3e_ld_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3E);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_4e_ld_c_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x4E);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<C>(), 0xAB);
    }

    #[test]
    fn op_66_ld_h_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x66);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<H>(), 0xAB);
    }

    #[test]
    fn op_46_ld_b_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x46);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0xAB);
    }

    #[test]
    fn op_56_ld_d_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x56);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<D>(), 0xAB);
    }

    #[test]
    fn op_6e_ld_l_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x6E);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0xAB);
    }

    #[test]
    fn op_7e_ld_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x7E);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_0f_rrca() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0F);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b0000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1000_0000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_0f_rrca_no_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x0F);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b0000_0010);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0001);
        assert!(!c.cpu.flags.get_flag(Flag::Zero)); 
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }


    #[test]
    fn op_11_ld_de_d16() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x11);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x34);
        c.bus.write_byte(0x8002, 0x12);
        
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<DE>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8004);
    }

    #[test]
    fn op_21_ld_hl_d16() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x21);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x34);
        c.bus.write_byte(0x8002, 0x12);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8004);
    }

    #[test]
    fn op_31_ld_sp_d16() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x31);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x34);
        c.bus.write_byte(0x8002, 0x12);
        ticks(&mut c, 3);   
        assert_eq!(c.cpu.get_r16::<SP>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8004);
    }

    #[test]
    fn op_12_ld_a_de() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x12);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<DE>(0xC000);
        c.cpu.set_r8::<A>(0x69);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0xC000), 0x69);
    }

    #[test]
    fn op_22_ld_a_hl_plus() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x22);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x69);
        c.cpu.set_r16::<HL>(0xC000);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0xC000), 0x69);
        assert_eq!(c.cpu.get_r16::<HL>(), 0xC001);
    }

    #[test]
    fn op_32_ld_a_hl_minus() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x32);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x69);
        c.cpu.set_r16::<HL>(0xC001);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0xC001), 0x69);
        assert_eq!(c.cpu.get_r16::<HL>(), 0xC000);
    }

    #[test]
    fn op_13_inc_de() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x13);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<DE>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<DE>(), 0x1235);
    }


    #[test]
    fn op_16_ld_d_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x16);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<D>(), 0xAB);
    }

    #[test]
    fn op_26_ld_h_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x26);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<H>(), 0xAB);
    }

    #[test]
    fn op_17_rla() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x17);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b1000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0010);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_17_rla_no_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x17);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b0000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0010);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_18_jr_r8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x18);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8008);
    }

    #[test]
    fn op_18_jr_r8_negative() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x18);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xFB);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x7FFE);
    }

    #[test]
    fn op_28_jr_z_r8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x28);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        c.cpu.flags.set_flag(Flag::Zero, true);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8008);
    }

    #[test]
    fn op_28_jr_z_r8_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x28);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        c.cpu.flags.set_flag(Flag::Zero, false);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8003);
    }

    #[test]
    fn op_30_jr_nc_r8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x30);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8008);
    }

    #[test]
    fn op_30_jr_nc_r8_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x30);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c,3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8003);
    }

    #[test]
    fn op_38_jr_c_r8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x38);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8008);
    }

    #[test]
    fn op_38_jr_c_r8_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x38);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8003);
    }

    #[test]
    fn op_19_add_hl_de() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x19);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x1234);
        c.cpu.set_r16::<DE>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x2468);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_29_add_hl_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x29);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x1234);
        c.cpu.set_r16::<HL>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x2468);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_39_add_hl_sp() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x39);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x1234);
        c.cpu.set_r16::<SP>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x2468);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_1a_ld_a_de() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1A);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<DE>(0xC000);
        c.bus.write_byte(0xC000, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_2a_ld_a_hl_plus() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2A);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
        assert_eq!(c.cpu.get_r16::<HL>(), 0xC001);
    }

    #[test]
    fn op_3a_ld_a_hl_minus() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3A);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC001);
        c.bus.write_byte(0xC001, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
        assert_eq!(c.cpu.get_r16::<HL>(), 0xC000);
    }

    #[test]
    fn op_1f_rra() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1f);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b1000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0100_0000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_1f_rra_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x1f);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0b1000_0001);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1100_0000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_23_inc_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x23);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x1234);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x1235);
    }

    #[test]
    fn op_27_daa_add_adjust_low() {
   
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x27);
        c.cpu.first_read(&mut c.bus);
        
        c.cpu.set_r8::<A>(0x4B);
        c.cpu.flags.set_flag(Flag::Subtract, false);
        c.cpu.flags.set_flag(Flag::HalfCarry, false);
        c.cpu.flags.set_flag(Flag::Carry, false);
        
        ticks(&mut c, 1);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0x51);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry)); 
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_27_daa_add_zero_and_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x27);
        c.cpu.first_read(&mut c.bus);
        
        c.cpu.set_r8::<A>(0x9A);
        c.cpu.flags.set_flag(Flag::Subtract, false);
        c.cpu.flags.set_flag(Flag::HalfCarry, false);
        c.cpu.flags.set_flag(Flag::Carry, false);
        
        ticks(&mut c, 1);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0x00);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_27_daa_sub_adjust_with_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x27);
        c.cpu.first_read(&mut c.bus);
        
        c.cpu.set_r8::<A>(0x1B);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, true);
        c.cpu.flags.set_flag(Flag::Carry, false);
        
        ticks(&mut c, 1);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0x15);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));    
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));     
    }

    #[test]
    fn op_27_daa_sub_with_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x27);
        c.cpu.first_read(&mut c.bus);
         
        c.cpu.set_r8::<A>(0xE3);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, false);
        c.cpu.flags.set_flag(Flag::Carry, true);
        
        ticks(&mut c, 1);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0x83);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    pub fn op_2f_cpl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2f);
        c.cpu.first_read(&mut c.bus);
         
        c.cpu.set_r8::<A>(0b1010_0101);
        
        ticks(&mut c, 1);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0b0101_1010);
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    pub fn op_2f_cpl_2() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x2f);
        c.cpu.first_read(&mut c.bus);
         
        c.cpu.set_r8::<A>(0b0000_0000);
        
        ticks(&mut c, 1);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0b1111_1111);
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_33_inc_sp() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x33);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0x1234);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<SP>(), 0x1235);
    }

    #[test]
    fn op_36_ld_hl_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x36);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8050);
        c.bus.write_byte(0x8001, 0xAB);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0x8050), 0xAB);
    }

    #[test]
    fn op_37_scf() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x37);
        c.cpu.first_read(&mut c.bus);
        ticks(&mut c, 1);
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_3f_scf() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x3F);
        c.cpu.first_read(&mut c.bus);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    macro_rules! test_ld_r8_r8 {
    ($name:ident, $opcode:expr, $dest:ident, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<$src>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, false);
            c.cpu.flags.set_flag(Flag::HalfCarry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<$dest>(), 0x5A);
            
            assert!(c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
        }
    };
    ($name:ident, $opcode:expr, $dest:ident, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<$dest>(0x00);
            c.cpu.set_r8::<$src>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, false);
            c.cpu.flags.set_flag(Flag::HalfCarry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<$dest>(), 0x5A);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x5A);
            
            assert!(c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
        }
    };
}

    test_ld_r8_r8!(op_40_ld_b_b, 0x40, B, B, same);
    test_ld_r8_r8!(op_41_ld_b_c, 0x41, B, C);
    test_ld_r8_r8!(op_42_ld_b_d, 0x42, B, D);
    test_ld_r8_r8!(op_43_ld_b_e, 0x43, B, E);
    test_ld_r8_r8!(op_44_ld_b_h, 0x44, B, H);
    test_ld_r8_r8!(op_45_ld_b_l, 0x45, B, L);
    test_ld_r8_r8!(op_47_ld_b_a, 0x47, B, A);

    // Destination C (0x48 - 0x4F)
    test_ld_r8_r8!(op_48_ld_c_b, 0x48, C, B);
    test_ld_r8_r8!(op_49_ld_c_c, 0x49, C, C, same);
    test_ld_r8_r8!(op_4a_ld_c_d, 0x4A, C, D);
    test_ld_r8_r8!(op_4b_ld_c_e, 0x4B, C, E);
    test_ld_r8_r8!(op_4c_ld_c_h, 0x4C, C, H);
    test_ld_r8_r8!(op_4d_ld_c_l, 0x4D, C, L);
    test_ld_r8_r8!(op_4f_ld_c_a, 0x4F, C, A);

    test_ld_r8_r8!(op_50_ld_d_b, 0x50, D, B);
    test_ld_r8_r8!(op_51_ld_d_c, 0x51, D, C);
    test_ld_r8_r8!(op_52_ld_d_d, 0x52, D, D, same);
    test_ld_r8_r8!(op_53_ld_d_e, 0x53, D, E);
    test_ld_r8_r8!(op_54_ld_d_h, 0x54, D, H);
    test_ld_r8_r8!(op_55_ld_d_l, 0x55, D, L);
    test_ld_r8_r8!(op_57_ld_d_a, 0x57, D, A);

    test_ld_r8_r8!(op_58_ld_e_b, 0x58, E, B);
    test_ld_r8_r8!(op_59_ld_e_c, 0x59, E, C);
    test_ld_r8_r8!(op_5a_ld_e_d, 0x5A, E, D);
    test_ld_r8_r8!(op_5b_ld_e_e, 0x5B, E, E, same);
    test_ld_r8_r8!(op_5c_ld_e_h, 0x5C, E, H);
    test_ld_r8_r8!(op_5d_ld_e_l, 0x5D, E, L);
    test_ld_r8_r8!(op_5f_ld_e_a, 0x5F, E, A);

    test_ld_r8_r8!(op_60_ld_h_b, 0x60, H, B);
    test_ld_r8_r8!(op_61_ld_h_c, 0x61, H, C);
    test_ld_r8_r8!(op_62_ld_h_d, 0x62, H, D);
    test_ld_r8_r8!(op_63_ld_h_e, 0x63, H, E);
    test_ld_r8_r8!(op_64_ld_h_h, 0x64, H, H, same);
    test_ld_r8_r8!(op_65_ld_h_l, 0x65, H, L);
    test_ld_r8_r8!(op_67_ld_h_a, 0x67, H, A);

    test_ld_r8_r8!(op_68_ld_l_b, 0x68, L, B);
    test_ld_r8_r8!(op_69_ld_l_c, 0x69, L, C);
    test_ld_r8_r8!(op_6a_ld_l_d, 0x6A, L, D);
    test_ld_r8_r8!(op_6b_ld_l_e, 0x6B, L, E);
    test_ld_r8_r8!(op_6c_ld_l_h, 0x6C, L, H);
    test_ld_r8_r8!(op_6d_ld_l_l, 0x6D, L, L, same);
    test_ld_r8_r8!(op_6f_ld_l_a, 0x6F, L, A);

    test_ld_r8_r8!(op_78_ld_a_b, 0x78, A, B);
    test_ld_r8_r8!(op_79_ld_a_c, 0x79, A, C);
    test_ld_r8_r8!(op_7a_ld_a_d, 0x7A, A, D);
    test_ld_r8_r8!(op_7b_ld_a_e, 0x7B, A, E);
    test_ld_r8_r8!(op_7c_ld_a_h, 0x7C, A, H);
    test_ld_r8_r8!(op_7d_ld_a_l, 0x7D, A, L);
    test_ld_r8_r8!(op_7f_ld_a_a, 0x7F, A, A, same);


    macro_rules! test_add_a_r8 {
        ($name:ident, $opcode:expr, $src:ident, same) => {
            #[test]
            fn $name() {
                let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
                c.cpu.first_read(&mut c.bus);
                
                c.cpu.set_r8::<A>(0x84);
                
                c.cpu.flags.set_flag(Flag::Zero, true);
                c.cpu.flags.set_flag(Flag::Subtract, true);
                c.cpu.flags.set_flag(Flag::HalfCarry, true);
                c.cpu.flags.set_flag(Flag::Carry, false);
                
                ticks(&mut c, 1);
                
                assert_eq!(c.cpu.get_r8::<A>(), 0x08);
                
                assert!(!c.cpu.flags.get_flag(Flag::Zero));
                assert!(!c.cpu.flags.get_flag(Flag::Subtract));
                assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
                assert!(c.cpu.flags.get_flag(Flag::Carry));
            }
        };
        ($name:ident, $opcode:expr, $src:ident) => {
            #[test]
            fn $name() {
                let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
                c.cpu.first_read(&mut c.bus);
                
                c.cpu.set_r8::<A>(0x78);
                c.cpu.set_r8::<$src>(0x8A);
                
                c.cpu.flags.set_flag(Flag::Zero, true);
                c.cpu.flags.set_flag(Flag::Subtract, true);
                c.cpu.flags.set_flag(Flag::HalfCarry, false);
                c.cpu.flags.set_flag(Flag::Carry, false);
                
                ticks(&mut c, 1);
                
                assert_eq!(c.cpu.get_r8::<A>(), 0x02);
                assert_eq!(c.cpu.get_r8::<$src>(), 0x8A);
                
                assert!(!c.cpu.flags.get_flag(Flag::Zero));
                assert!(!c.cpu.flags.get_flag(Flag::Subtract));
                assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
                assert!(c.cpu.flags.get_flag(Flag::Carry));
            }
        };
    }

    test_add_a_r8!(op_80_add_a_b, 0x80, B);
    test_add_a_r8!(op_81_add_a_c, 0x81, C);
    test_add_a_r8!(op_82_add_a_d, 0x82, D);
    test_add_a_r8!(op_83_add_a_e, 0x83, E);
    test_add_a_r8!(op_84_add_a_h, 0x84, H);
    test_add_a_r8!(op_85_add_a_l, 0x85, L);
    test_add_a_r8!(op_87_add_a_a, 0x87, A, same);

    macro_rules! test_and_a_r8 {
    ($name:ident, $opcode:expr, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, true);  
            c.cpu.flags.set_flag(Flag::HalfCarry, false);
            c.cpu.flags.set_flag(Flag::Carry, true);     
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x5A);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    ($name:ident, $opcode:expr, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0xF0);
            c.cpu.set_r8::<$src>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, true);
            c.cpu.flags.set_flag(Flag::HalfCarry, false);
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x50);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x5A);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
            }
        };
    }

    test_and_a_r8!(op_a0_and_a_b, 0xA0, B);
    test_and_a_r8!(op_a1_and_a_c, 0xA1, C);
    test_and_a_r8!(op_a2_and_a_d, 0xA2, D);
    test_and_a_r8!(op_a3_and_a_e, 0xA3, E);
    test_and_a_r8!(op_a4_and_a_h, 0xA4, H);
    test_and_a_r8!(op_a5_and_a_l, 0xA5, L);
    test_and_a_r8!(op_a7_and_a_a, 0xA7, A, same);


    macro_rules! test_xor_a_r8 {
    ($name:ident, $opcode:expr, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, false);
            c.cpu.flags.set_flag(Flag::Subtract, true);
            c.cpu.flags.set_flag(Flag::HalfCarry, true);
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x00);
            
            assert!(c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    ($name:ident, $opcode:expr, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0xF0);
            c.cpu.set_r8::<$src>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, true);
            c.cpu.flags.set_flag(Flag::HalfCarry, true);
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0xAA);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x5A);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
}

    test_xor_a_r8!(op_a8_xor_a_b, 0xA8, B);
    test_xor_a_r8!(op_a9_xor_a_c, 0xA9, C);
    test_xor_a_r8!(op_aa_xor_a_d, 0xAA, D);
    test_xor_a_r8!(op_ab_xor_a_e, 0xAB, E);
    test_xor_a_r8!(op_ac_xor_a_h, 0xAC, H);
    test_xor_a_r8!(op_ad_xor_a_l, 0xAD, L);
    test_xor_a_r8!(op_af_xor_a_a, 0xAF, A, same);

    macro_rules! test_or_a_r8 {
    ($name:ident, $opcode:expr, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, true);
            c.cpu.flags.set_flag(Flag::HalfCarry, true);
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x5A);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    ($name:ident, $opcode:expr, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x50);
            c.cpu.set_r8::<$src>(0x0A);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, true);
            c.cpu.flags.set_flag(Flag::HalfCarry, true);
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x5A);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x0A); 
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
}
    test_or_a_r8!(op_b0_or_a_b, 0xB0, B);
    test_or_a_r8!(op_b1_or_a_c, 0xB1, C);
    test_or_a_r8!(op_b2_or_a_d, 0xB2, D);
    test_or_a_r8!(op_b3_or_a_e, 0xB3, E);
    test_or_a_r8!(op_b4_or_a_h, 0xB4, H);
    test_or_a_r8!(op_b5_or_a_l, 0xB5, L);
    test_or_a_r8!(op_b7_or_a_a, 0xB7, A, same);


    macro_rules! test_cp_a_r8 {
    ($name:ident, $opcode:expr, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Zero, false);
            c.cpu.flags.set_flag(Flag::Subtract, false);
            c.cpu.flags.set_flag(Flag::HalfCarry, true);
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x5A);
            
            assert!(c.cpu.flags.get_flag(Flag::Zero));
            assert!(c.cpu.flags.get_flag(Flag::Subtract));
            assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    ($name:ident, $opcode:expr, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x3A);
            c.cpu.set_r8::<$src>(0x5F);
            
            c.cpu.flags.set_flag(Flag::Zero, true);       
            c.cpu.flags.set_flag(Flag::Subtract, false);
            c.cpu.flags.set_flag(Flag::HalfCarry, false);
            c.cpu.flags.set_flag(Flag::Carry, false);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x3A);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x5F);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(c.cpu.flags.get_flag(Flag::Carry));
        }
    };
}
    test_cp_a_r8!(op_b8_cp_a_b, 0xB8, B);
    test_cp_a_r8!(op_b9_cp_a_c, 0xB9, C);
    test_cp_a_r8!(op_ba_cp_a_d, 0xBA, D);
    test_cp_a_r8!(op_bb_cp_a_e, 0xBB, E);
    test_cp_a_r8!(op_bc_cp_a_h, 0xBC, H);
    test_cp_a_r8!(op_bd_cp_a_l, 0xBD, L);
    test_cp_a_r8!(op_bf_cp_a_a, 0xBF, A, same);


    macro_rules! test_adc_a_r8 {
    ($name:ident, $opcode:expr, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x7F);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, true);
            c.cpu.flags.set_flag(Flag::HalfCarry, false); 
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            ticks(&mut c, 1);
            assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    ($name:ident, $opcode:expr, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x78);
            c.cpu.set_r8::<$src>(0x87);
            
            c.cpu.flags.set_flag(Flag::Zero, false);
            c.cpu.flags.set_flag(Flag::Subtract, true);
            c.cpu.flags.set_flag(Flag::HalfCarry, false);
            c.cpu.flags.set_flag(Flag::Carry, true);      
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x00);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x87); 
            
            assert!(c.cpu.flags.get_flag(Flag::Zero));
            assert!(!c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(c.cpu.flags.get_flag(Flag::Carry));
        }
    };
}

    test_adc_a_r8!(op_88_adc_a_b, 0x88, B);
    test_adc_a_r8!(op_89_adc_a_c, 0x89, C);
    test_adc_a_r8!(op_8a_adc_a_d, 0x8A, D);
    test_adc_a_r8!(op_8b_adc_a_e, 0x8B, E);
    test_adc_a_r8!(op_8c_adc_a_h, 0x8C, H);
    test_adc_a_r8!(op_8d_adc_a_l, 0x8D, L);
    test_adc_a_r8!(op_8f_adc_a_a, 0x8F, A, same);


    macro_rules! test_sub_a_r8 {
    ($name:ident, $opcode:expr, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            // Pour SUB A, A -> Soustraction d'une valeur par elle-même (0x5A - 0x5A = 0x00)
            c.cpu.set_r8::<A>(0x5A);
            
            // On initialise les drapeaux à l'inverse du résultat attendu
            c.cpu.flags.set_flag(Flag::Zero, false);     // Attendu: true
            c.cpu.flags.set_flag(Flag::Subtract, false); // Attendu: true
            c.cpu.flags.set_flag(Flag::HalfCarry, true); // Attendu: false
            c.cpu.flags.set_flag(Flag::Carry, true);     // Attendu: false
            
            ticks(&mut c, 1);
            
            // L'accumulateur A doit maintenant valoir 0
            assert_eq!(c.cpu.get_r8::<A>(), 0x00);
            
            // Vérification des drapeaux
            assert!(c.cpu.flags.get_flag(Flag::Zero));
            assert!(c.cpu.flags.get_flag(Flag::Subtract));
            assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    ($name:ident, $opcode:expr, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x3A);
            c.cpu.set_r8::<$src>(0x5F);
            
            c.cpu.flags.set_flag(Flag::Zero, true); 
            c.cpu.flags.set_flag(Flag::Subtract, false);
            c.cpu.flags.set_flag(Flag::HalfCarry, false);
            c.cpu.flags.set_flag(Flag::Carry, false);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0xDB);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x5F);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(c.cpu.flags.get_flag(Flag::Carry));
        }
    };
}

    test_sub_a_r8!(op_90_sub_a_b, 0x90, B);
    test_sub_a_r8!(op_91_sub_a_c, 0x91, C);
    test_sub_a_r8!(op_92_sub_a_d, 0x92, D);
    test_sub_a_r8!(op_93_sub_a_e, 0x93, E);
    test_sub_a_r8!(op_94_sub_a_h, 0x94, H);
    test_sub_a_r8!(op_95_sub_a_l, 0x95, L);
    test_sub_a_r8!(op_97_sub_a_a, 0x97, A, same);

    macro_rules! test_sbc_a_r8 {
    ($name:ident, $opcode:expr, $src:ident, same) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x5A);
            
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            c.cpu.flags.set_flag(Flag::Zero, true);
            c.cpu.flags.set_flag(Flag::Subtract, false);
            c.cpu.flags.set_flag(Flag::HalfCarry, false);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
            
            assert!(!c.cpu.flags.get_flag(Flag::Zero));
            assert!(c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    ($name:ident, $opcode:expr, $src:ident) => {
        #[test]
        fn $name() {
            let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>($opcode);
            c.cpu.first_read(&mut c.bus);
            
            c.cpu.set_r8::<A>(0x80);
            c.cpu.set_r8::<$src>(0x7F);
            
            c.cpu.flags.set_flag(Flag::Carry, true);
            
            c.cpu.flags.set_flag(Flag::Zero, false);      
            c.cpu.flags.set_flag(Flag::Subtract, false);  
            c.cpu.flags.set_flag(Flag::HalfCarry, false);
            
            ticks(&mut c, 1);
            
            assert_eq!(c.cpu.get_r8::<A>(), 0x00);
            assert_eq!(c.cpu.get_r8::<$src>(), 0x7F);
            
            assert!(c.cpu.flags.get_flag(Flag::Zero));
            assert!(c.cpu.flags.get_flag(Flag::Subtract));
            assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
            assert!(!c.cpu.flags.get_flag(Flag::Carry));
        }
    };
    }
    test_sbc_a_r8!(op_98_sbc_a_b, 0x98, B);
    test_sbc_a_r8!(op_99_sbc_a_c, 0x99, C);
    test_sbc_a_r8!(op_9a_sbc_a_d, 0x9A, D);
    test_sbc_a_r8!(op_9b_sbc_a_e, 0x9B, E);
    test_sbc_a_r8!(op_9c_sbc_a_h, 0x9C, H);
    test_sbc_a_r8!(op_9d_sbc_a_l, 0x9D, L);
    test_sbc_a_r8!(op_9f_sbc_a_a, 0x9F, A, same);

    #[test]
    fn op_70_ld_hl_b() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x70);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.cpu.set_r8::<B>(0x69);

        ticks(&mut c, 1);
        assert_eq!(c.bus.read_byte(0x8005), 0x69);
    }

        #[test]
    fn op_71_ld_hl_c() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x71);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.cpu.set_r8::<C>(0x69);

        ticks(&mut c, 1);
        assert_eq!(c.bus.read_byte(0x8005), 0x69);
    }


    #[test]
    fn op_72_ld_hl_d() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x72);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.cpu.set_r8::<D>(0x69);

        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8005), 0x69);
    }


    #[test]
    fn op_73_ld_hl_e() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x73);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.cpu.set_r8::<E>(0x69);

        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8005), 0x69);
    }

    #[test]
    fn op_74_ld_hl_h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x74);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x05);
        c.cpu.set_r8::<H>(0x80);

        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8005), 0x80);
    }

    #[test]
    fn op_75_ld_hl_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x75);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x05);
        c.cpu.set_r8::<H>(0x80);

        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8005), 0x05);
    }

    #[test]
    fn op_77_ld_hl_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x77);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.cpu.set_r8::<A>(0x69);


        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8005), 0x69);
    }
    
    #[test]
    fn op_86_add_a_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x86);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.cpu.set_r8::<A>(0x12);
        c.bus.write_byte(0x8005, 0x12);

        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x24);
    }

    #[test]
    fn op_8e_adc_a_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x8E);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.bus.write_byte(0x8005, 0x00);
        c.cpu.set_r8::<A>(0x0F);

        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, false); 
        c.cpu.flags.set_flag(Flag::Carry, true); 

        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x10);
        
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry)); 
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_96_sub_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x96);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.bus.write_byte(0x8005, 0x10);
        c.cpu.set_r8::<A>(0x10);

        c.cpu.flags.set_flag(Flag::Zero, false);
        c.cpu.flags.set_flag(Flag::Subtract, false);
        c.cpu.flags.set_flag(Flag::HalfCarry, true); 
        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 2);
        
        // 0x10 - 0x10 = 0x00
        assert_eq!(c.cpu.get_r8::<A>(), 0x00);
        
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract)); 
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_9e_sbc_a_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x9E);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.bus.write_byte(0x8005, 0x00);
        c.cpu.set_r8::<A>(0x00);

        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, false);
        c.cpu.flags.set_flag(Flag::HalfCarry, false); 
        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 2);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
        
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));    
    }

    #[test]
    fn op_ae_xor_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xAE);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.bus.write_byte(0x8005, 0xAA);
        c.cpu.set_r8::<A>(0xAA);

        c.cpu.flags.set_flag(Flag::Zero, false);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, true); 
        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 2);
        
        // 0xAA XOR 0xAA = 0x00
        assert_eq!(c.cpu.get_r8::<A>(), 0x00);
        
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry)); 
        assert!(!c.cpu.flags.get_flag(Flag::Carry)); 
    }

    #[test]
    fn op_b6_or_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xB6);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.bus.write_byte(0x8005, 0x00);
        c.cpu.set_r8::<A>(0x00);

        c.cpu.flags.set_flag(Flag::Zero, false);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, true); 
        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 2);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0x00);
        
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_be_cp_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xBE);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0x8005);
        c.bus.write_byte(0x8005, 0x43);
        c.cpu.set_r8::<A>(0x42);

        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, false);
        c.cpu.flags.set_flag(Flag::HalfCarry, false); 
        c.cpu.flags.set_flag(Flag::Carry, false);

        ticks(&mut c, 2);
        
        assert_eq!(c.cpu.get_r8::<A>(), 0x42); 
        
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry)); 
        assert!(c.cpu.flags.get_flag(Flag::Carry));   
    }


    // =========================================================================
    // RET (Inconditionnel) - Opcode 0xC9
    // =========================================================================
    #[test]
    fn op_c9_ret() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC9);
        c.cpu.first_read(&mut c.bus);

        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34); // LSB
        c.bus.write_byte(0xFFFE, 0x12); // MSB

        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, false);
        c.cpu.flags.set_flag(Flag::HalfCarry, true); 
        c.cpu.flags.set_flag(Flag::Carry, false);

        ticks(&mut c, 3); // 4 cycles au total

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);

        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_c0_ret_nz_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC0);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, false);

        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_c0_ret_nz_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC0);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, true);

        let pc_before = c.cpu.get_r16::<PC>();
        ticks(&mut c, 2);
        
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 1);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
    }

    #[test]
    fn op_c8_ret_z_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC8);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, true);

        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_c8_ret_z_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC8);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, false);

        let pc_before = c.cpu.get_r16::<PC>();
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 1);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
    }

    #[test]
    fn op_d0_ret_nc_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD0);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, false);

        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_d0_ret_nc_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD0);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, true);

        let pc_before = c.cpu.get_r16::<PC>();
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
    }

    #[test]
    fn op_d8_ret_c_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD8);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_d8_ret_c_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD8);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, false);

        let pc_before = c.cpu.get_r16::<PC>();
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 1);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
    }

    #[test]
    fn op_d9_reti() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD9);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        c.cpu.ime = false; 

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);

        assert_eq!(c.cpu.ime, true);
        
        assert!(c.cpu.ime); 
    }


    #[test]
    fn op_c3_jp() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC3);
        c.cpu.first_read(&mut c.bus);
        
        // On écrit l'adresse cible 0x1234 juste après l'opcode
        let pc = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc, 0x34);     // LSB
        c.bus.write_byte(pc + 1, 0x12); // MSB

        ticks(&mut c, 3); // 1 (first_read) + 3 = 4 cycles

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
    }


    #[test]
    fn op_c2_jp_nz_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC2);
        c.cpu.first_read(&mut c.bus);
        
        let pc = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc, 0x34);
        c.bus.write_byte(pc + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, false);

        ticks(&mut c, 4); 
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
    }

    #[test]
    fn op_c2_jp_nz_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC2);
        c.cpu.first_read(&mut c.bus);
        
        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, true); 

        ticks(&mut c, 3);
        
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
    }

    #[test]
    fn op_ca_jp_z_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCA);
        c.cpu.first_read(&mut c.bus);
        
        let pc = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc, 0x34);
        c.bus.write_byte(pc + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, true);

        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
    }

    #[test]
    fn op_ca_jp_z_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCA);
        c.cpu.first_read(&mut c.bus);
        
        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Zero, false);

        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
    }

    #[test]
    fn op_d2_jp_nc_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD2);
        c.cpu.first_read(&mut c.bus);
        
        let pc = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc, 0x34);
        c.bus.write_byte(pc + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, false);

        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
    }

    #[test]
    fn op_d2_jp_nc_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD2);
        c.cpu.first_read(&mut c.bus);
        
        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
    }

    #[test]
    fn op_da_jp_c_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xDA);
        c.cpu.first_read(&mut c.bus);
        
        let pc = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc, 0x34);
        c.bus.write_byte(pc + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x1234);
    }

    #[test]
    fn op_da_jp_c_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xDA);
        c.cpu.first_read(&mut c.bus);
        
        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
    }

    #[test]
    fn op_c1_pop_bc() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC1);
        c.cpu.first_read(&mut c.bus);   

        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x34);
        c.bus.write_byte(0xFFFE, 0x12);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<BC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_d1_pop_de() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD1);
        c.cpu.first_read(&mut c.bus);   

        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x56);
        c.bus.write_byte(0xFFFE, 0x34);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<DE>(), 0x3456);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_e1_pop_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE1);
        c.cpu.first_read(&mut c.bus);   

        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x78);
        c.bus.write_byte(0xFFFE, 0x56);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<HL>(), 0x5678);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_f1_pop_af_flags_true() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF1);
        c.cpu.first_read(&mut c.bus);   

        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0xFF);
        c.bus.write_byte(0xFFFE, 0x12);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r8::<A>(), 0x12);
        assert_eq!(c.cpu.get_r16::<AF>(), 0x12F0); 
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
        
        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), true);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), true);
    }

    #[test]
    fn op_f1_pop_af_flags_false() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF1);
        c.cpu.first_read(&mut c.bus);   

        c.cpu.set_r16::<SP>(0xFFFD);
        c.bus.write_byte(0xFFFD, 0x00);
        c.bus.write_byte(0xFFFE, 0x12);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r8::<A>(), 0x12);
        assert_eq!(c.cpu.get_r16::<AF>(), 0x1200); 
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
        
        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), false);
    }

    #[test]
    fn op_c5_push_bc() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC5);
        c.cpu.first_read(&mut c.bus);

        c.cpu.set_r16::<BC>(0x1234);
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), 0x12);
        assert_eq!(c.bus.read_byte(0xFFFD), 0x34);
    }

    #[test]
    fn op_d5_push_de() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD5);
        c.cpu.first_read(&mut c.bus);

        c.cpu.set_r16::<DE>(0x5678);
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), 0x56);
        assert_eq!(c.bus.read_byte(0xFFFD), 0x78);
    }

    #[test]
    fn op_e5_push_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE5);
        c.cpu.first_read(&mut c.bus);

        c.cpu.set_r16::<HL>(0x9ABC);
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), 0x9A);
        assert_eq!(c.bus.read_byte(0xFFFD), 0xBC);
    }

    #[test]
    fn op_f5_push_af() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF5);
        c.cpu.first_read(&mut c.bus);

        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, true);
        c.cpu.flags.set_flag(Flag::Carry, true);
        c.cpu.set_r8::<A>(0x12);
        
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), 0x12);
        assert_eq!(c.bus.read_byte(0xFFFD), 0xF0);
    }
 
    #[test]
    fn op_cd_call() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCD);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);  
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 6);

        let ret_addr = pc_before + 2;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1235);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_c4_call_nz_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC4);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Zero, false);

        ticks(&mut c, 6);

        let ret_addr = pc_before + 2;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1235);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_c4_call_nz_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC4);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Zero, true);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 3);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_cc_call_z_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCC);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Zero, true);

        ticks(&mut c, 6);

        let ret_addr = pc_before + 2;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1235);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_cc_call_z_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCC);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Zero, false);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 3);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_cc_call_z_taken_with_overlap() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCC);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x00);
        c.bus.write_byte(pc_before + 1, 0xC0);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Zero, true);

        c.bus.write_byte(0xC000, 0x3C);
        c.cpu.set_r8::<A>(0x00);

        ticks(&mut c, 6);

        let ret_addr = pc_before + 2;

        assert_eq!(c.cpu.get_r16::<PC>(), 0xC001);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);

        ticks(&mut c, 1);

        assert_eq!(c.cpu.get_r8::<A>(), 0x01);
    }

    #[test]
    fn op_d4_call_nc_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD4);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Carry, false);

        ticks(&mut c, 6);

        let ret_addr = pc_before + 2;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1235);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_dc_call_c_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xDC);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 6);

        let ret_addr = pc_before + 2;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x1235);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_d4_call_nc_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD4);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Carry, true);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 3);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_dc_call_c_not_taken() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xDC);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x34);
        c.bus.write_byte(pc_before + 1, 0x12);

        c.cpu.set_r16::<SP>(0xFFFF);
        c.cpu.flags.set_flag(Flag::Carry, false);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 3);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFF);
    }

    #[test]
    fn op_c7_rst_00h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC7);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;
        assert_eq!(c.cpu.get_r16::<PC>(), 0x0001);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_cf_rst_08h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCF);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x0009);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_d7_rst_10h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD7);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x0011);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_df_rst_18h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xDF);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x0019);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_e7_rst_20h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE7);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x0021);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_ef_rst_28h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xEF);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x0029);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_f7_rst_30h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF7);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x0031);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_ff_rst_38h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xFF);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.cpu.set_r16::<SP>(0xFFFF);

        ticks(&mut c, 4);

        let ret_addr = pc_before;

        assert_eq!(c.cpu.get_r16::<PC>(), 0x0039);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFD);
        assert_eq!(c.bus.read_byte(0xFFFE), (ret_addr >> 8) as u8);
        assert_eq!(c.bus.read_byte(0xFFFD), (ret_addr & 0xFF) as u8);
    }

    #[test]
    fn op_c6_add_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xC6);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x20); // d8 = 0x20

        c.cpu.set_r8::<A>(0x10); // A = 0x10

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2); // Overlap inclus
        assert_eq!(c.cpu.get_r8::<A>(), 0x30);            // 0x10 + 0x20 = 0x30
    }

    #[test]
    fn op_d6_sub_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xD6);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x10); // d8 = 0x10

        c.cpu.set_r8::<A>(0x50); // A = 0x50

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x40);            // 0x50 - 0x10 = 0x40
    }

    #[test]
    fn op_ce_adc_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCE);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x20); // d8 = 0x20

        c.cpu.set_r8::<A>(0x10);
        c.cpu.flags.set_flag(Flag::Carry, true); // On active la retenue

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x31);            // 0x10 + 0x20 + 1 = 0x31
    }

    #[test]
    fn op_de_sbc_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xDE);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x10); // d8 = 0x10

        c.cpu.set_r8::<A>(0x50);
        c.cpu.flags.set_flag(Flag::Carry, true); // On active la retenue (borrow)

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x3F);            // 0x50 - 0x10 - 1 = 0x3F
    }

    #[test]
    fn op_e6_and_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE6);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0xAA); // d8 = 0b10101010

        c.cpu.set_r8::<A>(0xFF); // A = 0b11111111

        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAA);            // 0xFF & 0xAA = 0xAA
    }

    #[test]
    fn op_ee_xor_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xEE);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0xAA); // d8 = 0b10101010

        c.cpu.set_r8::<A>(0xFF); // A = 0b11111111

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x55);
    }

    #[test]
    fn op_f6_or_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF6);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x05); // d8 = 0x05

        c.cpu.set_r8::<A>(0x50); // A = 0x50

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x55);            // 0x50 | 0x05 = 0x55
    }

    #[test]
    fn op_fe_cp_a_d8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xFE);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x50); // d8 = 0x50

        c.cpu.set_r8::<A>(0x50); // A = 0x50 (Soustraction virtuelle: 0x50 - 0x50)

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x50); // CP ne modifie PAS le registre A !
        
        assert!(c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_e0_ldh_a8_a() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE0);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x80);

        c.cpu.set_r8::<A>(0x5A); 

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.bus.read_byte(0xFF80), 0x5A);
    }

    #[test]
    fn op_f0_ldh_a_a8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF0);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x80);

        c.bus.write_byte(0xFF80, 0xA5);
        c.cpu.set_r8::<A>(0x00);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xA5);
    }

    #[test]
    fn op_e2_ld_ff00_c_a() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE2);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        
        c.cpu.set_r8::<C>(0x80); // C = 0x80 -> Adresse cible = 0xFF00 + 0x80 = 0xFF80
        c.cpu.set_r8::<A>(0x7E); // Valeur à stocker

        ticks(&mut c, 2); // 2 ticks pour LD ($FF00+C), A

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 1);
        assert_eq!(c.bus.read_byte(0xFF80), 0x7E);
    }

    #[test]
    fn op_f2_ld_a_ff00_c() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF2);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();

        c.cpu.set_r8::<C>(0x80);
        c.bus.write_byte(0xFF80, 0xBD);
        c.cpu.set_r8::<A>(0x00);

        ticks(&mut c, 2);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0xBD);
    }

    #[test]
    fn op_ea_ld_a16_a() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xEA);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        // On écrit l'adresse immédiate 0xC000 (WRAM) après l'opcode
        c.bus.write_byte(pc_before, 0x00);     // LSB
        c.bus.write_byte(pc_before + 1, 0xC0); // MSB

        c.cpu.set_r8::<A>(0xABCDEF_u32 as u8); // Disons 0x42 pour faire simple
        c.cpu.set_r8::<A>(0x42);

        ticks(&mut c, 4); // LD (a16), A prend 4 ticks

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 3); // 3 octets consommés + overlap
        assert_eq!(c.bus.read_byte(0xC000), 0x42);
    }

    #[test]
    fn op_fa_ld_a_a16() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xFA);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        // On va lire depuis l'adresse 0xC005
        c.bus.write_byte(pc_before, 0x05);     // LSB
        c.bus.write_byte(pc_before + 1, 0xC0); // MSB

        c.bus.write_byte(0xC005, 0x77);
        c.cpu.set_r8::<A>(0x00);

        ticks(&mut c, 4); // LD A, (a16) prend 4 ticks

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 3);
        assert_eq!(c.cpu.get_r8::<A>(), 0x77);
    }

    #[test]
    fn op_f9_ld_sp_hl() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF9);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        
        c.cpu.set_r16::<HL>(0xABCD);
        c.cpu.set_r16::<SP>(0x1111);

        ticks(&mut c, 2); // 1 octet, prend 2 ticks

        // L'instruction fait 1 octet, le fetch overlap passe donc au suivant (+1)
        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 1);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xABCD);
    }

    #[test]
    fn op_f8_ld_hl_sp_r8() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF8);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x05);

        c.cpu.set_r16::<SP>(0x2000);
        c.cpu.set_r16::<HL>(0x0000);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x2005);
        

    }

    #[test]
    fn op_f8_ld_hl_sp_r8_no_flags() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF8);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x02); // r8 = +2

        c.cpu.set_r16::<SP>(0x2000);
        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, true);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x2002);

        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Subtract), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::HalfCarry), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), false);
    }

    #[test]
    fn op_f8_ld_hl_sp_r8_with_carry_and_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF8);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x01); // r8 = +1

        c.cpu.set_r16::<SP>(0x20FF); // Les 8 bits du bas sont au max

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<HL>(), 0x2100);

        // Verification des flags
        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Subtract), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::HalfCarry), true);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), true);
    }

    #[test]
    fn op_f8_ld_hl_sp_r8_negative_offset() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF8);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0xFF); 

        c.cpu.set_r16::<SP>(0x2001);

        ticks(&mut c, 3);

        assert_eq!(c.cpu.get_r16::<HL>(), 0x2000); // 0x2001 - 1 = 0x2000

        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Subtract), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::HalfCarry), true);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), true);
    }

    #[test]
    fn op_e8_add_sp_r8_no_flags() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE8);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x02);

        c.cpu.set_r16::<SP>(0x2000);
        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, true);

        ticks(&mut c, 4);

        assert_eq!(c.cpu.get_r16::<PC>(), pc_before + 2);
        assert_eq!(c.cpu.get_r16::<SP>(), 0x2002);

        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Subtract), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::HalfCarry), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), false);
    }

    #[test]
    fn op_e8_add_sp_r8_with_carry_and_half_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE8);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0x01);

        c.cpu.set_r16::<SP>(0x20FF);

        ticks(&mut c, 4);

        assert_eq!(c.cpu.get_r16::<SP>(), 0x2100);

        // Vérification des flags
        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Subtract), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), true);
    }

    #[test]
    fn op_e8_add_sp_r8_negative_offset() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xE8);
        c.cpu.first_read(&mut c.bus);

        let pc_before = c.cpu.get_r16::<PC>();
        c.bus.write_byte(pc_before, 0xFF);

        c.cpu.set_r16::<SP>(0x2001);

        ticks(&mut c, 4);

        assert_eq!(c.cpu.get_r16::<SP>(), 0x2000); 

        assert_eq!(c.cpu.flags.get_flag(Flag::Zero), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::Subtract), false);
        assert_eq!(c.cpu.flags.get_flag(Flag::HalfCarry), true);
        assert_eq!(c.cpu.flags.get_flag(Flag::Carry), true);
    }

    #[test]
    fn op_f3_di_execution() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xF3);
        c.cpu.first_read(&mut c.bus);

        c.cpu.ime = true;

        c.cpu.tick(&mut c.bus);

        assert_eq!(c.cpu.ime, false);
    }

    #[test]
    fn op_fb_ei_and_delay_execution() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xFB); // EI
        c.cpu.first_read(&mut c.bus);

        c.cpu.ime = false;
        c.cpu.ime_delay = false;

        let next_pc = c.cpu.get_r16::<PC>();
        c.bus.write_byte(next_pc, 0x00);

        c.cpu.tick(&mut c.bus);

        assert_eq!(c.cpu.ime, true);
        assert_eq!(c.cpu.ime_delay, false);
    }

    #[test]
    fn op_76_halt_execution() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0x76); 
        c.cpu.first_read(&mut c.bus);

        c.cpu.halted = false;
        
        let next_pc = c.cpu.get_r16::<PC>();
        c.bus.write_byte(next_pc, 0x00);

        c.cpu.tick(&mut c.bus);

        assert_eq!(c.cpu.halted, true);
    }
    // ============================================================
    // Instructions CB préfixées
    // ============================================================

    // ---------- RLC (0x00–0x07) ----------
    #[test]
    fn cb_00_rlc_b() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x00);
        c.cpu.set_r8::<B>(0b1000_0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0b0000_0011);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_05_rlc_l() {
        // 0x05 = RLC L. Doit modifier L, pas la mémoire pointée par HL.
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x05);
        c.cpu.set_r8::<H>(0xC0);
        c.cpu.set_r8::<L>(0b1000_0001);
        c.bus.write_byte(0xC081, 0xAA); // sentinelle : ne doit pas changer
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0b0000_0011);
        assert_eq!(c.bus.read_byte(0xC081), 0xAA);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_06_rlc_hl_mem() {
        // 0x06 = RLC (HL). Doit modifier la mémoire, pas B.
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x06);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b1000_0001);
        c.cpu.set_r8::<B>(0x77); // sentinelle : ne doit pas changer
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b0000_0011);
        assert_eq!(c.cpu.get_r8::<B>(), 0x77);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_07_rlc_a() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x07);
        c.cpu.set_r8::<A>(0b0000_0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0010);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    // ---------- RRC (0x08–0x0F) ----------
    #[test]
    fn cb_08_rrc_b() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x08);
        c.cpu.set_r8::<B>(0b0000_0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0b1000_0000);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_0d_rrc_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x0D);
        c.cpu.set_r8::<H>(0xC0);
        c.cpu.set_r8::<L>(0b0000_0001);
        c.bus.write_byte(0xC001, 0xAA);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0b1000_0000);
        assert_eq!(c.bus.read_byte(0xC001), 0xAA);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_0e_rrc_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x0E);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b0000_0001);
        c.cpu.set_r8::<B>(0x77);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b1000_0000);
        assert_eq!(c.cpu.get_r8::<B>(), 0x77);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_0f_rrc_a() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x0F);
        c.cpu.set_r8::<A>(0b0000_0010);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0001);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    // ---------- RL (0x10–0x17) ----------
    #[test]
    fn cb_10_rl_b_with_carry_in() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x10);
        c.cpu.set_r8::<B>(0b0000_0001);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0b0000_0011);
        assert!(!c.cpu.flags.get_flag(Flag::Carry)); // bit7 d'entrée = 0
    }

    #[test]
    fn cb_15_rl_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x15);
        c.cpu.set_r8::<H>(0xC0);
        c.cpu.set_r8::<L>(0b1000_0001);
        c.cpu.flags.set_flag(Flag::Carry, true);
        c.bus.write_byte(0xC081, 0xAA);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0b0000_0011);
        assert_eq!(c.bus.read_byte(0xC081), 0xAA);
        assert!(c.cpu.flags.get_flag(Flag::Carry)); // bit7 d'entrée = 1
    }

    #[test]
    fn cb_16_rl_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x16);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b1000_0001);
        c.cpu.flags.set_flag(Flag::Carry, false);
        c.cpu.set_r8::<B>(0x77);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b0000_0010);
        assert_eq!(c.cpu.get_r8::<B>(), 0x77);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    // ---------- RR (0x18–0x1F) ----------
    #[test]
    fn cb_18_rr_b_with_carry_in() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x18);
        c.cpu.set_r8::<B>(0b0000_0001);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0b1000_0000);
        assert!(c.cpu.flags.get_flag(Flag::Carry)); // bit0 d'entrée = 1
    }

    #[test]
    fn cb_1d_rr_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x1D);
        c.cpu.set_r8::<H>(0xC0);
        c.cpu.set_r8::<L>(0b0000_0001);
        c.cpu.flags.set_flag(Flag::Carry, false);
        c.bus.write_byte(0xC001, 0xAA);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0b0000_0000);
        assert_eq!(c.bus.read_byte(0xC001), 0xAA);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_1e_rr_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x1E);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b0000_0000);
        c.cpu.flags.set_flag(Flag::Carry, true);
        c.cpu.set_r8::<B>(0x77);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b1000_0000);
        assert_eq!(c.cpu.get_r8::<B>(), 0x77);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    // ---------- SLA (0x20–0x27) ----------
    #[test]
    fn cb_20_sla_b() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x20);
        c.cpu.set_r8::<B>(0b1000_0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0b0000_0010);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_25_sla_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x25);
        c.cpu.set_r8::<H>(0xC0);
        c.cpu.set_r8::<L>(0b0100_0000);
        c.bus.write_byte(0xC040, 0xAA);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0b1000_0000);
        assert_eq!(c.bus.read_byte(0xC040), 0xAA);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_26_sla_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x26);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b1000_0000);
        c.cpu.set_r8::<B>(0x77);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0x00);
        assert_eq!(c.cpu.get_r8::<B>(), 0x77);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    // ---------- SRA (0x28–0x2F) ----------
    #[test]
    fn cb_28_sra_b_preserves_sign_bit() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x28);
        c.cpu.set_r8::<B>(0b1000_0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0b1100_0000); // bit7 conservé
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_2d_sra_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x2D);
        c.cpu.set_r8::<H>(0xC0);
        c.cpu.set_r8::<L>(0b0000_0010);
        c.bus.write_byte(0xC002, 0xAA);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0b0000_0001);
        assert_eq!(c.bus.read_byte(0xC002), 0xAA);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_2e_sra_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x2E);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b1000_0001);
        c.cpu.set_r8::<B>(0x77);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b1100_0000);
        assert_eq!(c.cpu.get_r8::<B>(), 0x77);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    // ---------- SWAP (0x30–0x37) — déjà correct, on verrouille le comportement ----------
    #[test]
    fn cb_30_swap_b() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x30);
        c.cpu.set_r8::<B>(0x12);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0x21);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_35_swap_l_zero_flag() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x35);
        c.cpu.set_r8::<L>(0x00);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0x00);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn cb_36_swap_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x36);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0x12);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0x21);
    }

    // ---------- SRL (0x38–0x3F) ----------
    #[test]
    fn cb_38_srl_b() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x38);
        c.cpu.set_r8::<B>(0b1000_0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 0b0100_0000);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_3d_srl_l() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x3D);
        c.cpu.set_r8::<H>(0xC0);
        c.cpu.set_r8::<L>(0b0000_0001);
        c.bus.write_byte(0xC001, 0xAA);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<L>(), 0x00);
        assert_eq!(c.bus.read_byte(0xC001), 0xAA);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_3e_srl_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x3E);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b0000_0010);
        c.cpu.set_r8::<B>(0x77);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b0000_0001);
        assert_eq!(c.cpu.get_r8::<B>(), 0x77);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    // ---------- BIT (0x40–0x7F) ----------
    #[test]
    fn cb_40_bit0_b_clear() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x40);
        c.cpu.set_r8::<B>(0b0000_0000);
        ticks(&mut c, 2);
        assert!(c.cpu.flags.get_flag(Flag::Zero));      // bit absent
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn cb_47_bit0_a_set_preserves_carry() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x47);
        c.cpu.set_r8::<A>(0b0000_0001);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 2);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));     // bit présent
        assert!(c.cpu.flags.get_flag(Flag::Carry));     // BIT ne touche pas Carry
    }

    #[test]
    fn cb_46_bit0_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x46);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b0000_0001);
        ticks(&mut c, 3);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn cb_7c_bit7_h() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x7C);
        c.cpu.set_r8::<H>(0b1000_0000);
        ticks(&mut c, 2);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
    }

    // ---------- RES (0x80–0xBF) ----------
    #[test]
    fn cb_87_res0_a_does_not_touch_flags() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x87);
        c.cpu.set_r8::<A>(0b1111_1111);
        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, true);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1111_1110);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_86_res0_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x86);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b1111_1111);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b1111_1110);
    }

    // ---------- SET (0xC0–0xFF) ----------
    #[test]
    fn cb_c7_set0_a_does_not_touch_flags() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xC7);
        c.cpu.set_r8::<A>(0b0000_0000);
        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0001);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn cb_c6_set0_hl_mem() {
        let mut c = gb::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>(0xCB);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0xC6);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0b0000_0000);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0xC000), 0b0000_0001);
    }


}

