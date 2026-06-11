#[cfg(test)]
mod tests {
    use crate::cpu::defines::Flag;
    use crate::cpu::flags::FlagsOps;
    use crate::cpu_def::*;
    use crate::gameboy::GameBoy;
    use crate::mmu::GbaMmu;
    use crate::mmu::MemoryMapper;
    use crate::mmu::mbc;
    use crate::mmu::mbc::*;

    // Creates a GameBoy with the given first opcode and pads with NOPs so post-instruction
    // fetch never goes out of bounds. Tests operate on the GameBoy and its inner CPU (`gb.cpu`).
    fn gb<M>(opcode: u8) -> GameBoy<GbaMmu<RomOnly>> {
        let list = vec![opcode];
        let mut gb: GameBoy<GbaMmu<RomOnly>> =
            GameBoy::new(None, list.clone(), None).expect("Failed to create gb");
        let bus = GbaMmu::new(None, list, None).expect("Failed to create bus");
        gb.bus = bus;
        // Pre-fetch the first instruction so tests can call `tick()` immediately.
        gb.cpu.set_r16::<PC>(0x0);
        gb.cpu.first_read(&mut gb.bus);
        gb
    }

    // Sets the immediate byte at PC=1 (after the opcode is fetched at PC=0).
    fn cpu_n<M>(opcode: u8, n: u8) -> GameBoy<GbaMmu<mbc::RomOnly>> {
        let mut c = gb::<M>(opcode);
        c.bus.write_byte(1, n);
        c
    }

    // Sets the 16-bit immediate at PC=1 (lo) and PC=2 (hi).
    fn cpu_nn<M>(opcode: u8, lo: u8, hi: u8) -> GameBoy<GbaMmu<mbc::RomOnly>> {
        let mut c = gb::<M>(opcode);
        c.bus.write_byte(1, lo);
        c.bus.write_byte(2, hi);
        c
    }

    fn ticks(gb: &mut GameBoy<GbaMmu<mbc::RomOnly>>, n: usize) {
        for _ in 0..n {
            gb.cpu.tick(&mut gb.bus);
        }
    }

    // --- NOP ---

    #[test]
    fn op_00_nop_preserves_state() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x00);
        c.cpu.set_r8::<A>(42);
        c.cpu.flags.set_flag(Flag::Carry, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, true);
        c.cpu.flags.set_flag(Flag::Zero, true);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 42);
        assert!(
            c.cpu.flags.get_flag(Flag::Carry)
                && c.cpu.flags.get_flag(Flag::HalfCarry)
                && c.cpu.flags.get_flag(Flag::Zero)
                && c.cpu.flags.get_flag(Flag::Subtract)
        );
    }

    // --- ADD A, r ---

    #[test]
    fn op_80_add_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x80);
        c.cpu.set_r8::<A>(5);
        c.cpu.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 8);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_83_add_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x83);
        c.cpu.set_r8::<A>(2);
        c.cpu.set_r8::<E>(10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 12);
    }

    #[test]
    fn op_87_add_a_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x87);
        c.cpu.set_r8::<A>(7);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 14);
    }

    #[test]
    fn op_86_add_a_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x86);
        c.cpu.set_r8::<A>(5);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 7);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 12);
    }

    #[test]
    fn op_c6_add_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xC6, 10);
        c.cpu.set_r8::<A>(5);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 15);
    }

    #[test]
    fn op_c6_add_a_n_zero_flag() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xC6, 0);
        c.cpu.set_r8::<A>(0);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
    }

    // --- ADC A, r ---

    #[test]
    fn op_88_adc_a_b_uses_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x88);
        c.cpu.set_r8::<A>(5);
        c.cpu.set_r8::<B>(3);
        c.cpu.flags.set_flag(Flag::Carry, true);
        c.cpu.flags.set_flag(Flag::Zero, false);
        ticks(&mut c, 1);
        // Expected: 5 + 3 + 1(carry) = 9
        assert_eq!(c.cpu.get_r8::<A>(), 9);
    }

    #[test]
    fn op_ce_adc_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xCE, 4);
        c.cpu.set_r8::<A>(10);
        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 14);
    }

    // --- SUB A, r ---

    #[test]
    fn op_90_sub_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x90);
        c.cpu.set_r8::<A>(10);
        c.cpu.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 7);
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_97_sub_a_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x97);
        c.cpu.set_r8::<A>(5);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn op_93_sub_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x93);
        c.cpu.set_r8::<A>(20);
        c.cpu.set_r8::<D>(7);
        c.cpu.set_r8::<E>(3);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 17);
    }

    #[test]
    fn op_96_sub_a_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x96);
        c.cpu.set_r8::<A>(15);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 5);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 10);
    }

    #[test]
    fn op_d6_sub_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xD6, 4);
        c.cpu.set_r8::<A>(10);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 6);
    }

    // --- SBC A, r ---

    #[test]
    fn op_9b_sbc_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x9B);
        c.cpu.set_r8::<A>(20);
        c.cpu.set_r8::<D>(7);
        c.cpu.set_r8::<E>(3);
        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 17);
    }

    // --- AND A, r ---

    #[test]
    fn op_a0_and_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xA0);
        c.cpu.set_r8::<A>(0b1111_0000);
        c.cpu.set_r8::<B>(0b1010_1010);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1010_0000);
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_a7_and_a_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xA7);
        c.cpu.set_r8::<A>(0b1010_1010);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1010_1010);
    }

    #[test]
    fn op_a3_and_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xA3);
        c.cpu.set_r8::<A>(0b1010_1010);
        c.cpu.set_r8::<D>(0b1010_0000);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1010_0000);
    }

    #[test]
    fn op_a6_and_a_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xA6);
        c.cpu.set_r8::<A>(0xFF);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 0xF0);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xF0);
    }

    #[test]
    fn op_e6_and_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xE6, 0x0F);
        c.cpu.set_r8::<A>(0xFF);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x0F);
    }

    // --- OR A, r ---

    #[test]
    fn op_b0_or_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xB0);
        c.cpu.set_r8::<A>(0b1111_0000);
        c.cpu.set_r8::<B>(0b0000_1111);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
    }

    #[test]
    fn op_b3_or_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xB3);
        c.cpu.set_r8::<A>(0x00);
        c.cpu.set_r8::<D>(0xF0);
        c.cpu.set_r8::<E>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x0F);
    }

    #[test]
    fn op_b6_or_a_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xB6);
        c.cpu.set_r8::<A>(0xF0);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 0x0F);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
    }

    #[test]
    fn op_f6_or_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xF6, 0b1010_1010);
        c.cpu.set_r8::<A>(0b1111_0000);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1111_1010);
    }

    // --- XOR A, r ---

    #[test]
    fn op_a8_xor_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xA8);
        c.cpu.set_r8::<A>(0xFF);
        c.cpu.set_r8::<B>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0xF0);
    }

    #[test]
    fn op_af_xor_a_a_clears() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xAF);
        c.cpu.set_r8::<A>(0xFF);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x00);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_ab_xor_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xAB);
        c.cpu.set_r8::<A>(0xFF);
        c.cpu.set_r8::<D>(0xF0);
        c.cpu.set_r8::<E>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0xF0);
    }

    #[test]
    fn op_ee_xor_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xEE, 0x0F);
        c.cpu.set_r8::<A>(0xFF);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xF0);
    }

    // --- CP A, r ---

    #[test]
    fn op_b8_cp_a_b_equal() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xB8);
        c.cpu.set_r8::<A>(5);
        c.cpu.set_r8::<B>(5);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 5); // A unchanged
        assert!(c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn op_b8_cp_a_b_not_equal() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xB8);
        c.cpu.set_r8::<A>(10);
        c.cpu.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_bb_cp_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xBB);
        c.cpu.set_r8::<A>(5);
        c.cpu.set_r8::<D>(10);
        c.cpu.set_r8::<E>(5);
        ticks(&mut c, 1);
        assert!(c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_fe_cp_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xFE, 5);
        c.cpu.set_r8::<A>(10);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
    }

    // --- INC r8 ---

    #[test]
    fn op_04_inc_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x04);
        c.cpu.set_r8::<B>(41);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 42);
    }

    #[test]
    fn op_3c_inc_a_wraps() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3C);
        c.cpu.set_r8::<A>(0xFF);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0);
    }

    #[test]
    fn op_1c_inc_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x1C);
        c.cpu.set_r8::<E>(9);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 10);
    }

    #[test]
    fn op_34_inc_hl_addr() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x34);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 41);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0x8000), 42);
    }

    // --- DEC r8 ---

    #[test]
    fn op_05_dec_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x05);
        c.cpu.set_r8::<B>(10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 9);
    }

    #[test]
    fn op_3d_dec_a_wraps() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3D);
        c.cpu.set_r8::<A>(0x00);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
    }

    #[test]
    fn op_35_dec_hl_addr() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x35);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 10);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0x8000), 9);
    }

    // --- INC / DEC r16 ---

    #[test]
    fn op_23_inc_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x23);
        c.cpu.set_r16::<HL>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x1235);
    }

    #[test]
    fn op_03_inc_bc() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x03);
        c.cpu.set_r16::<BC>(0x00FF);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x0100);
    }

    #[test]
    fn op_33_inc_sp() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x33);
        c.cpu.set_r16::<SP>(0x00FF);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<SP>(), 0x0100);
    }

    #[test]
    fn op_2b_dec_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x2B);
        c.cpu.set_r16::<HL>(0x1235);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x1234);
    }

    #[test]
    fn op_0b_dec_bc() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x0B);
        c.cpu.set_r16::<BC>(0x0100);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x00FF);
    }

    // --- LD r, r (0x40–0x7F block) ---

    #[test]
    fn op_41_ld_b_c() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x41);
        c.cpu.set_r8::<C>(99);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 99);
    }

    #[test]
    fn op_78_ld_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x78);
        c.cpu.set_r8::<B>(77);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 77);
    }

    #[test]
    fn op_7b_ld_a_e() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x7B);
        c.cpu.set_r8::<E>(55);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 55);
    }

    #[test]
    fn op_57_ld_d_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x57);
        c.cpu.set_r8::<A>(33);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 33);
    }

    #[test]
    fn op_4f_ld_c_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x4F);
        c.cpu.set_r8::<A>(0xAB);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0xAB);
    }

    // --- LD A, (HL) / LD (HL), r ---

    #[test]
    fn op_7e_ld_a_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x7E);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 55);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 55);
    }

    #[test]
    fn op_77_ld_hl_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x77);
        c.cpu.set_r8::<A>(33);
        c.cpu.set_r16::<HL>(0x9000);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x9000), 33);
    }

    #[test]
    fn op_70_ld_hl_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x70);
        c.cpu.set_r8::<B>(0x55);
        c.cpu.set_r16::<HL>(0x8000);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8000), 0x55);
    }

    // --- LD (HL), n ---

    #[test]
    fn op_36_ld_hl_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0x36, 0xAB);
        c.cpu.set_r16::<HL>(0x8000);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0x8000), 0xAB);
    }

    // --- LD r, n ---

    #[test]
    fn op_06_ld_b_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0x06, 42);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<B>(), 42);
    }

    #[test]
    fn op_3e_ld_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0x3E, 77);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 77);
    }

    // --- LD r16, nn ---

    #[test]
    fn op_21_ld_hl_nn() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0x21, 0x34, 0x12);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x1234);
    }

    #[test]
    fn op_01_ld_bc_nn() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0x01, 0x78, 0x56);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x5678);
    }

    #[test]
    fn op_31_ld_sp_nn() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0x31, 0xFE, 0xFF);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFE);
    }

    // --- LD A, (rr) / LD (rr), A ---

    #[test]
    fn op_0a_ld_a_bc() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x0A);
        c.cpu.set_r16::<BC>(0x8000);
        c.bus.write_byte(0x8000, 0x42);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x42);
    }

    #[test]
    fn op_1a_ld_a_de() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x1A);
        c.cpu.set_r16::<DE>(0x8001);
        c.bus.write_byte(0x8001, 0x99);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x99);
    }

    #[test]
    fn op_02_ld_bc_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x02);
        c.cpu.set_r8::<A>(0x77);
        c.cpu.set_r16::<BC>(0x8000);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8000), 0x77);
    }

    #[test]
    fn op_12_ld_de_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x12);
        c.cpu.set_r8::<A>(0x55);
        c.cpu.set_r16::<DE>(0x8001);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8001), 0x55);
    }

    // --- LD A, (nn) / LD (nn), A ---

    #[test]
    fn op_fa_ld_a_nn() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xFA);
        c.bus.write_byte(1, 0x50); // lo (PC=1)
        c.bus.write_byte(2, 0x80); // hi → 0x8050 (PC=2)
        c.bus.write_byte(0x8050, 0xAB);
        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_ea_ld_nn_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xEA);
        c.bus.write_byte(1, 0x50); // lo (PC=1)
        c.bus.write_byte(2, 0x80); // hi → 0x8050 (PC=2)
        c.cpu.set_r8::<A>(0xCC);
        ticks(&mut c, 4);
        assert_eq!(c.bus.read_byte(0x8050), 0xCC);
    }

    // --- LD A, (HL±) / LD (HL±), A ---

    #[test]
    fn op_3a_ld_a_hl_minus() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3A);
        c.cpu.set_r16::<HL>(0x8001);
        c.bus.write_byte(0x8001, 0x55);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x55);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x8000);
    }

    #[test]
    fn op_2a_ld_a_hl_plus() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x2A);
        c.cpu.set_r16::<HL>(0x8000);
        c.bus.write_byte(0x8000, 0x44);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x44);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x8001);
    }

    #[test]
    fn op_32_ld_hl_minus_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x32);
        c.cpu.set_r8::<A>(0x99);
        c.cpu.set_r16::<HL>(0x8001);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8001), 0x99);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x8000);
    }

    #[test]
    fn op_22_ld_hl_plus_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x22);
        c.cpu.set_r8::<A>(0x66);
        c.cpu.set_r16::<HL>(0x8000);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8000), 0x66);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x8001);
    }

    // --- LDH ---

    #[test]
    fn op_f2_ldh_a_c() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xF2);
        c.cpu.set_r8::<C>(0x40);
        c.bus.write_byte(0xFF40, 0xAB);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_e2_ldh_c_a() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xE2);
        c.cpu.set_r8::<A>(0xCD);
        c.cpu.set_r8::<C>(0x40);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0xFF40), 0xCD);
    }

    #[test]
    fn op_f0_ldh_a_n() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xF0, 0x40); // n=0x40 → address 0xFF40
        c.bus.write_byte(0xFF40, 0x77);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r8::<A>(), 0x77);
    }

    // --- LD (nn), SP ---

    #[test]
    fn op_08_ld_nn_sp() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x08);
        c.bus.write_byte(1, 0x00); // lo (PC=1)
        c.bus.write_byte(2, 0x80); // hi → 0x8000 (PC=2)
        c.cpu.set_r16::<SP>(0x1234); // P=0x34 (low), S=0x12 (high)
        ticks(&mut c, 5);
        assert_eq!(c.bus.read_byte(0x8000), 0x34); // SP low byte first (little-endian)
        assert_eq!(c.bus.read_byte(0x8001), 0x12); // SP high byte second
    }

    // --- LD SP, HL ---

    #[test]
    fn op_f9_ld_sp_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xF9);
        c.cpu.set_r16::<HL>(0xFFFE);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFE);
    }

    // --- LD HL, SP+e ---

    #[test]
    fn op_f8_ld_hl_sp_e() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xF8, 0x10);
        c.cpu.set_r16::<SP>(0x0100);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0110);
    }

    #[test]
    fn op_f8_ld_hl_sp_e_negative() {
        let mut c = cpu_n::<GbaMmu<mbc::RomOnly>>(0xF8, (-1i8) as u8); // e = -1
        c.cpu.set_r16::<SP>(0x0100);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x00FF);
    }

    // --- Misc: SCF, CCF, CPL, DAA ---

    #[test]
    fn op_37_scf() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x37);
        c.cpu.flags.set_flag(Flag::Carry, false);
        c.cpu.flags.set_flag(Flag::Subtract, true);
        c.cpu.flags.set_flag(Flag::HalfCarry, true);
        ticks(&mut c, 1);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3f_ccf_toggles_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3F);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_3f_ccf_sets_carry_when_clear() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3F);
        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 1);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_2f_cpl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x2F);
        c.cpu.set_r8::<A>(0b1010_0101);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0101_1010);
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_27_daa_after_add() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x27);
        // After ADD A=0x09 + B=0x09 = 0x12 (binary), DAA should give 0x18 (BCD)
        c.cpu.set_r8::<A>(0x12);
        c.cpu.flags.set_flag(Flag::Subtract, false);
        c.cpu.flags.set_flag(Flag::HalfCarry, false);
        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x12);
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
    }

    // --- Rotate instructions ---

    #[test]
    fn op_07_rlca_msb_to_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x07);
        c.cpu.set_r8::<A>(0b1000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0011);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_0f_rrca_lsb_to_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x0F);
        c.cpu.set_r8::<A>(0b0000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1000_0000);
        assert!(c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_17_rla_through_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x17);
        c.cpu.set_r8::<A>(0b1000_0000);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b0000_0001); // old carry shifted in
        assert!(c.cpu.flags.get_flag(Flag::Carry)); // old msb shifted out
    }

    #[test]
    fn op_1f_rra_through_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x1F);
        c.cpu.set_r8::<A>(0b0000_0001);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0b1000_0000); // old carry shifted in
        assert!(c.cpu.flags.get_flag(Flag::Carry)); // old lsb shifted out
    }

    // --- ADD HL, rr ---

    // ADD HL, BC (0x09): uses individual B (r8[1]) and C (r8[2]) registers.
    #[test]
    fn op_09_add_hl_bc() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x09);
        c.cpu.set_r8::<B>(0x01); // high byte contribution
        c.cpu.set_r8::<C>(0x05); // low byte contribution
        c.cpu.set_r16::<HL>(0x0010);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0115);
    }

    #[test]
    fn op_19_add_hl_de() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x19);
        c.cpu.set_r8::<D>(0x00); // high byte
        c.cpu.set_r8::<E>(0x10); // low byte
        c.cpu.set_r16::<HL>(0x0001);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0011);
    }

    #[test]
    fn op_29_add_hl_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x29);
        c.cpu.set_r16::<HL>(0x0010);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0020);
    }

    #[test]
    fn op_39_add_hl_sp() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x39);
        c.cpu.set_r8::<S>(0x00); // SP high
        c.cpu.set_r8::<P>(0x05); // SP low
        c.cpu.set_r16::<HL>(0x0010);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0015);
    }

    // --- JR ---

    #[test]
    fn op_18_jr_forward() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x18);
        c.bus.write_byte(1, 0x05); // e = +5 (PC=1)
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 7);
    }

    // --- JR cc ---

    #[test]
    fn op_20_jr_nz_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x20);
        c.bus.write_byte(1, 0x02); // e = +2 (PC=1)
        c.cpu.flags.set_flag(Flag::Zero, false); // NZ condition met
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 4);
    }

    #[test]
    fn op_20_jr_nz_not_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x20);
        c.bus.write_byte(1, 0x02);
        c.cpu.flags.set_flag(Flag::Zero, true); // NZ not met
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), 2);
    }

    #[test]
    fn op_28_jr_z_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x28);
        c.bus.write_byte(1, 0x02);
        c.cpu.flags.set_flag(Flag::Zero, true);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 4);
    }

    #[test]
    fn op_28_jr_z_not_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x28);
        c.bus.write_byte(1, 0x02);
        c.cpu.flags.set_flag(Flag::Zero, false);
        ticks(&mut c, 2);
        assert_eq!(c.cpu.get_r16::<PC>(), 2);
    }

    #[test]
    fn op_30_jr_nc_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x30);
        c.bus.write_byte(1, 0x02);
        c.cpu.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 4);
    }

    #[test]
    fn op_38_jr_c_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x38);
        c.bus.write_byte(1, 0x02);
        c.cpu.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 4);
    }

    // --- JP nn ---

    #[test]
    fn op_c3_jp_nn() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0xC3, 0x05, 0x00); // target = 0x0005
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x0005);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<PC>(), 6);
    }

    #[test]
    fn op_e9_jp_hl() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xE9);
        c.cpu.set_r16::<HL>(0x0005);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<PC>(), 6);
    }

    // --- JP cc, nn ---

    #[test]
    fn op_c2_jp_nz_taken() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0xC2, 0x08, 0x00); // target = 0x0008
        c.cpu.flags.set_flag(Flag::Zero, false);
        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x0008);
    }

    #[test]
    fn op_c2_jp_nz_not_taken() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0xC2, 0x08, 0x00);
        c.cpu.flags.set_flag(Flag::Zero, true);
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 3);
    }

    #[test]
    fn op_ca_jp_z_taken() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0xCA, 0x08, 0x00);
        c.cpu.flags.set_flag(Flag::Zero, true);
        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x0008);
    }

    // --- CALL nn ---

    #[test]
    fn op_cd_call_nn_sets_pc() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0xCD, 0x08, 0x00); // target = 0x0008
        c.cpu.set_r16::<SP>(0xFFFE);
        ticks(&mut c, 5);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x0008);
    }

    #[test]
    fn op_cd_call_nn_saves_return_address() {
        let mut c = cpu_nn::<GbaMmu<mbc::RomOnly>>(0xCD, 0x08, 0x00);
        c.cpu.set_r16::<SP>(0xFFFE);
        ticks(&mut c, 5);
        assert_eq!(c.cpu.get_r16::<SP>(), 0xFFFC);
        assert_eq!(c.bus.read_byte(0xFFFD), 0x02); // PC low
        assert_eq!(c.bus.read_byte(0xFFFC), 0x00); // PC high
    }

    // --- RET ---

    #[test]
    fn op_c9_ret() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xC9);
        c.cpu.set_r16::<SP>(0xFFFC);
        c.bus.write_byte(0xFFFC, 0x00); // PcC (high byte)
        c.bus.write_byte(0xFFFD, 0x08); // PcP (low byte)
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x0800); // documents endianness bug
    }

    #[test]
    fn op_c0_ret_nz_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xC0);
        c.cpu.flags.set_flag(Flag::Zero, false);
        c.cpu.set_r16::<SP>(0xFFFC);
        c.bus.write_byte(0xFFFC, 0x00);
        c.bus.write_byte(0xFFFD, 0x08);
        ticks(&mut c, 4);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x0800);
    }

    #[test]
    fn op_c0_ret_nz_not_taken() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0xC0);
        c.cpu.flags.set_flag(Flag::Zero, true); // NZ not met
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<PC>(), 1); // PC advanced by fetch only
    }
}
