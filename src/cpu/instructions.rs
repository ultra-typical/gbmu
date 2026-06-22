use crate::cpu::defines::{Cpu, Instruction};
use crate::cpu::ops::cond::*;
use crate::cpu::*;
use crate::mmu::MemoryMapper;

pub fn build_instructions<M: MemoryMapper>() -> Vec<Instruction<M>> {
    vec![
        Instruction {
            opcode: 0x00,
            micro_ops: vec![Cpu::noop],
        },
        Instruction {
            opcode: 0x01,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<BC, WZ>,
            ],
        },
        Instruction {
            opcode: 0x02,
            micro_ops: vec![Cpu::write_memory::<BC, A>, Cpu::noop],
        },
        Instruction {
            opcode: 0x03,
            micro_ops: vec![Cpu::inc_r16::<BC>, Cpu::noop],
        },
        Instruction {
            opcode: 0x04,
            micro_ops: vec![Cpu::inc_r8::<B>],
        },
        Instruction {
            opcode: 0x05,
            micro_ops: vec![Cpu::dec_r8::<B>],
        },
        Instruction {
            opcode: 0x06,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::load_r8_r8::<B, Z>],
        },
        Instruction {
            opcode: 0x07,
            micro_ops: vec![Cpu::rlca],
        },
        Instruction {
            opcode: 0x08,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::write_memory_incr::<WZ, P>,
                Cpu::write_memory::<WZ, S>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x09,
            micro_ops: vec![
                Cpu::add_r8_r8_no_zero_flag::<C, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<B, H>,
            ],
        },
        Instruction {
            opcode: 0x0A,
            micro_ops: vec![Cpu::read_memory::<BC, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x0B,
            micro_ops: vec![Cpu::dec_r16::<BC>, Cpu::noop],
        },
        Instruction {
            opcode: 0x0C,
            micro_ops: vec![Cpu::inc_r8::<C>],
        },
        Instruction {
            opcode: 0x0D,
            micro_ops: vec![Cpu::dec_r8::<C>],
        },
        Instruction {
            opcode: 0x0E,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::load_r8_r8::<C, Z>],
        },
        Instruction {
            opcode: 0x0F,
            micro_ops: vec![Cpu::rrca],
        },
        Instruction {
            opcode: 0x10,
            micro_ops: vec![Cpu::stop],
        },
        Instruction {
            opcode: 0x11,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<DE, WZ>,
            ],
        },
        Instruction {
            opcode: 0x12,
            micro_ops: vec![Cpu::write_memory::<DE, A>, Cpu::noop],
        },
        Instruction {
            opcode: 0x13,
            micro_ops: vec![Cpu::inc_r16::<DE>, Cpu::noop],
        },
        Instruction {
            opcode: 0x14,
            micro_ops: vec![Cpu::inc_r8::<D>],
        },
        Instruction {
            opcode: 0x15,
            micro_ops: vec![Cpu::dec_r8::<D>],
        },
        Instruction {
            opcode: 0x16,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::load_r8_r8::<D, Z>],
        },
        Instruction {
            opcode: 0x17,
            micro_ops: vec![Cpu::rla],
        },
        Instruction {
            opcode: 0x18,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::relative_jump,
                Cpu::load_r16_r16::<PC, WZ>
                ],
        },
        Instruction {
            opcode: 0x19,
            micro_ops: vec![
                Cpu::add_r8_r8_no_zero_flag::<E, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<D, H>,
            ],
        },
        Instruction {
            opcode: 0x1A,
            micro_ops: vec![Cpu::read_memory::<DE, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x1B,
            micro_ops: vec![Cpu::dec_r16::<DE>, Cpu::noop],
        },
        Instruction {
            opcode: 0x1C,
            micro_ops: vec![Cpu::inc_r8::<E>],
        },
        Instruction {
            opcode: 0x1D,
            micro_ops: vec![Cpu::dec_r8::<E>],
        },
        Instruction {
            opcode: 0x1E,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::load_r8_r8::<E, Z>],
        },
        Instruction {
            opcode: 0x1F,
            micro_ops: vec![Cpu::rra],
        },
        Instruction {
            opcode: 0x20,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::check_cond::<CondNZ>,
                Cpu::relative_jump,
            ],
        },
        Instruction {
            opcode: 0x21,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<HL, WZ>,
            ],
        },
        Instruction {
            opcode: 0x22,
            micro_ops: vec![Cpu::write_memory_incr::<HL, A>, Cpu::noop],
        },
        Instruction {
            opcode: 0x23,
            micro_ops: vec![Cpu::inc_r16::<HL>, Cpu::noop],
        },
        Instruction {
            opcode: 0x24,
            micro_ops: vec![Cpu::inc_r8::<H>],
        },
        Instruction {
            opcode: 0x25,
            micro_ops: vec![Cpu::dec_r8::<H>],
        },
        Instruction {
            opcode: 0x26,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::load_r8_r8::<H, Z>],
        },
        Instruction {
            opcode: 0x27,
            micro_ops: vec![Cpu::daa],
        },
        Instruction {
            opcode: 0x28,
            micro_ops: vec![
                Cpu::read_memory_incr_check::<PC, Z, CondZ>,
                Cpu::relative_jump,
                Cpu::load_r16_r16::<PC, WZ>
            ],
        },
        Instruction {
            opcode: 0x29,
            micro_ops: vec![
                Cpu::add_r8_r8_no_zero_flag::<L, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<H, H>,
            ],
        },
        Instruction {
            opcode: 0x2A,
            micro_ops: vec![Cpu::read_memory_incr::<HL, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x2B,
            micro_ops: vec![Cpu::dec_r16::<HL>, Cpu::noop],
        },
        Instruction {
            opcode: 0x2C,
            micro_ops: vec![Cpu::inc_r8::<L>],
        },
        Instruction {
            opcode: 0x2D,
            micro_ops: vec![Cpu::dec_r8::<L>],
        },
        Instruction {
            opcode: 0x2E,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::load_r8_r8::<L, Z>],
        },
        Instruction {
            opcode: 0x2F,
            micro_ops: vec![Cpu::cpl],
        },
        Instruction {
            opcode: 0x30,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::check_cond::<CondNC>,
                Cpu::relative_jump,
            ],
        },
        Instruction {
            opcode: 0x31,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<SP, WZ>,
            ],
        },
        Instruction {
            opcode: 0x32,
            micro_ops: vec![Cpu::write_memory_decr::<HL, A>, Cpu::noop],
        },
        Instruction {
            opcode: 0x33,
            micro_ops: vec![Cpu::inc_r16::<SP>, Cpu::noop],
        },
        Instruction {
            opcode: 0x34,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::inc_addr::<HL, Z>, Cpu::noop],
        },
        Instruction {
            opcode: 0x35,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::dec_addr::<HL, Z>, Cpu::noop],
        },
        Instruction {
            opcode: 0x36,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::write_memory::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x37,
            micro_ops: vec![Cpu::scf],
        },
        Instruction {
            opcode: 0x38,
            micro_ops: vec![
                Cpu::read_memory_incr_check::<PC, Z, CondC>,
                Cpu::relative_jump,
                Cpu::load_r16_r16::<PC, WZ>
            ],
        },
        Instruction {
            opcode: 0x39,
            micro_ops: vec![
                Cpu::add_r8_r8_no_zero_flag::<P, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<S, H>,
            ],
        },
        Instruction {
            opcode: 0x3A,
            micro_ops: vec![Cpu::read_memory_decr::<HL, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x3B,
            micro_ops: vec![Cpu::dec_r16::<SP>, Cpu::noop],
        },
        Instruction {
            opcode: 0x3C,
            micro_ops: vec![Cpu::inc_r8::<A>],
        },
        Instruction {
            opcode: 0x3D,
            micro_ops: vec![Cpu::dec_r8::<A>],
        },
        Instruction {
            opcode: 0x3E,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x3F,
            micro_ops: vec![Cpu::ccf],
        },
        Instruction {
            opcode: 0x40,
            micro_ops: vec![Cpu::load_r8_r8::<B, B>],
        },
        Instruction {
            opcode: 0x41,
            micro_ops: vec![Cpu::load_r8_r8::<B, C>],
        },
        Instruction {
            opcode: 0x42,
            micro_ops: vec![Cpu::load_r8_r8::<B, D>],
        },
        Instruction {
            opcode: 0x43,
            micro_ops: vec![Cpu::load_r8_r8::<B, E>],
        },
        Instruction {
            opcode: 0x44,
            micro_ops: vec![Cpu::load_r8_r8::<B, H>],
        },
        Instruction {
            opcode: 0x45,
            micro_ops: vec![Cpu::load_r8_r8::<B, L>],
        },
        Instruction {
            opcode: 0x46,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<B, Z>],
        },
        Instruction {
            opcode: 0x47,
            micro_ops: vec![Cpu::load_r8_r8::<B, A>],
        },
        Instruction {
            opcode: 0x48,
            micro_ops: vec![Cpu::load_r8_r8::<C, B>],
        },
        Instruction {
            opcode: 0x49,
            micro_ops: vec![Cpu::load_r8_r8::<C, C>],
        },
        Instruction {
            opcode: 0x4a,
            micro_ops: vec![Cpu::load_r8_r8::<C, D>],
        },
        Instruction {
            opcode: 0x4b,
            micro_ops: vec![Cpu::load_r8_r8::<C, E>],
        },
        Instruction {
            opcode: 0x4c,
            micro_ops: vec![Cpu::load_r8_r8::<C, H>],
        },
        Instruction {
            opcode: 0x4d,
            micro_ops: vec![Cpu::load_r8_r8::<C, L>],
        },
        Instruction {
            opcode: 0x4e,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<C, Z>],
        },
        Instruction {
            opcode: 0x4f,
            micro_ops: vec![Cpu::load_r8_r8::<C, A>],
        },
        Instruction {
            opcode: 0x50,
            micro_ops: vec![Cpu::load_r8_r8::<D, B>],
        },
        Instruction {
            opcode: 0x51,
            micro_ops: vec![Cpu::load_r8_r8::<D, C>],
        },
        Instruction {
            opcode: 0x52,
            micro_ops: vec![Cpu::load_r8_r8::<D, D>],
        },
        Instruction {
            opcode: 0x53,
            micro_ops: vec![Cpu::load_r8_r8::<D, E>],
        },
        Instruction {
            opcode: 0x54,
            micro_ops: vec![Cpu::load_r8_r8::<D, H>],
        },
        Instruction {
            opcode: 0x55,
            micro_ops: vec![Cpu::load_r8_r8::<D, L>],
        },
        Instruction {
            opcode: 0x56,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<D, Z>],
        },
        Instruction {
            opcode: 0x57,
            micro_ops: vec![Cpu::load_r8_r8::<D, A>],
        },
        Instruction {
            opcode: 0x58,
            micro_ops: vec![Cpu::load_r8_r8::<E, B>],
        },
        Instruction {
            opcode: 0x59,
            micro_ops: vec![Cpu::load_r8_r8::<E, C>],
        },
        Instruction {
            opcode: 0x5a,
            micro_ops: vec![Cpu::load_r8_r8::<E, D>],
        },
        Instruction {
            opcode: 0x5b,
            micro_ops: vec![Cpu::load_r8_r8::<E, E>],
        },
        Instruction {
            opcode: 0x5c,
            micro_ops: vec![Cpu::load_r8_r8::<E, H>],
        },
        Instruction {
            opcode: 0x5d,
            micro_ops: vec![Cpu::load_r8_r8::<E, L>],
        },
        Instruction {
            opcode: 0x5e,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<E, Z>],
        },
        Instruction {
            opcode: 0x5f,
            micro_ops: vec![Cpu::load_r8_r8::<E, A>],
        },
        Instruction {
            opcode: 0x60,
            micro_ops: vec![Cpu::load_r8_r8::<H, B>],
        },
        Instruction {
            opcode: 0x61,
            micro_ops: vec![Cpu::load_r8_r8::<H, C>],
        },
        Instruction {
            opcode: 0x62,
            micro_ops: vec![Cpu::load_r8_r8::<H, D>],
        },
        Instruction {
            opcode: 0x63,
            micro_ops: vec![Cpu::load_r8_r8::<H, E>],
        },
        Instruction {
            opcode: 0x64,
            micro_ops: vec![Cpu::load_r8_r8::<H, H>],
        },
        Instruction {
            opcode: 0x65,
            micro_ops: vec![Cpu::load_r8_r8::<H, L>],
        },
        Instruction {
            opcode: 0x66,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<H, Z>],
        },
        Instruction {
            opcode: 0x67,
            micro_ops: vec![Cpu::load_r8_r8::<H, A>],
        },
        Instruction {
            opcode: 0x68,
            micro_ops: vec![Cpu::load_r8_r8::<L, B>],
        },
        Instruction {
            opcode: 0x69,
            micro_ops: vec![Cpu::load_r8_r8::<L, C>],
        },
        Instruction {
            opcode: 0x6a,
            micro_ops: vec![Cpu::load_r8_r8::<L, D>],
        },
        Instruction {
            opcode: 0x6b,
            micro_ops: vec![Cpu::load_r8_r8::<L, E>],
        },
        Instruction {
            opcode: 0x6c,
            micro_ops: vec![Cpu::load_r8_r8::<L, H>],
        },
        Instruction {
            opcode: 0x6d,
            micro_ops: vec![Cpu::load_r8_r8::<L, L>],
        },
        Instruction {
            opcode: 0x6e,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<L, Z>],
        },
        Instruction {
            opcode: 0x6f,
            micro_ops: vec![Cpu::load_r8_r8::<L, A>],
        },
        Instruction {
            opcode: 0x70,
            micro_ops: vec![Cpu::write_memory::<HL, B>, Cpu::noop],
        },
        Instruction {
            opcode: 0x71,
            micro_ops: vec![Cpu::write_memory::<HL, C>, Cpu::noop],
        },
        Instruction {
            opcode: 0x72,
            micro_ops: vec![Cpu::write_memory::<HL, D>, Cpu::noop],
        },
        Instruction {
            opcode: 0x73,
            micro_ops: vec![Cpu::write_memory::<HL, E>, Cpu::noop],
        },
        Instruction {
            opcode: 0x74,
            micro_ops: vec![Cpu::write_memory::<HL, H>, Cpu::noop],
        },
        Instruction {
            opcode: 0x75,
            micro_ops: vec![Cpu::write_memory::<HL, L>, Cpu::noop],
        },
        Instruction {
            opcode: 0x76,
            micro_ops: vec![Cpu::halt],
        },
        Instruction {
            opcode: 0x77,
            micro_ops: vec![Cpu::write_memory::<HL, A>, Cpu::noop],
        },
        Instruction {
            opcode: 0x78,
            micro_ops: vec![Cpu::load_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0x79,
            micro_ops: vec![Cpu::load_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0x7a,
            micro_ops: vec![Cpu::load_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0x7b,
            micro_ops: vec![Cpu::load_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0x7c,
            micro_ops: vec![Cpu::load_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0x7d,
            micro_ops: vec![Cpu::load_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0x7e,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x7f,
            micro_ops: vec![Cpu::load_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0x80,
            micro_ops: vec![Cpu::add_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0x81,
            micro_ops: vec![Cpu::add_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0x82,
            micro_ops: vec![Cpu::add_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0x83,
            micro_ops: vec![Cpu::add_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0x84,
            micro_ops: vec![Cpu::add_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0x85,
            micro_ops: vec![Cpu::add_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0x86,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::add_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x87,
            micro_ops: vec![Cpu::add_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0x88,
            micro_ops: vec![Cpu::add_r8_r8_with_carry::<A, B>],
        },
        Instruction {
            opcode: 0x89,
            micro_ops: vec![Cpu::add_r8_r8_with_carry::<A, C>],
        },
        Instruction {
            opcode: 0x8A,
            micro_ops: vec![Cpu::add_r8_r8_with_carry::<A, D>],
        },
        Instruction {
            opcode: 0x8B,
            micro_ops: vec![Cpu::add_r8_r8_with_carry::<A, E>],
        },
        Instruction {
            opcode: 0x8C,
            micro_ops: vec![Cpu::add_r8_r8_with_carry::<A, H>],
        },
        Instruction {
            opcode: 0x8D,
            micro_ops: vec![Cpu::add_r8_r8_with_carry::<A, L>],
        },
        Instruction {
            opcode: 0x8E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::add_r8_r8_with_carry::<A, Z>],
        },
        Instruction {
            opcode: 0x8F,
            micro_ops: vec![Cpu::add_r8_r8_with_carry::<A, A>],
        },
        Instruction {
            opcode: 0x90,
            micro_ops: vec![Cpu::sub_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0x91,
            micro_ops: vec![Cpu::sub_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0x92,
            micro_ops: vec![Cpu::sub_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0x93,
            micro_ops: vec![Cpu::sub_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0x94,
            micro_ops: vec![Cpu::sub_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0x95,
            micro_ops: vec![Cpu::sub_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0x96,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::sub_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x97,
            micro_ops: vec![Cpu::sub_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0x98,
            micro_ops: vec![Cpu::sub_r8_r8_with_carry::<A, B>],
        },
        Instruction {
            opcode: 0x99,
            micro_ops: vec![Cpu::sub_r8_r8_with_carry::<A, C>],
        },
        Instruction {
            opcode: 0x9A,
            micro_ops: vec![Cpu::sub_r8_r8_with_carry::<A, D>],
        },
        Instruction {
            opcode: 0x9B,
            micro_ops: vec![Cpu::sub_r8_r8_with_carry::<A, E>],
        },
        Instruction {
            opcode: 0x9C,
            micro_ops: vec![Cpu::sub_r8_r8_with_carry::<A, H>],
        },
        Instruction {
            opcode: 0x9D,
            micro_ops: vec![Cpu::sub_r8_r8_with_carry::<A, L>],
        },
        Instruction {
            opcode: 0x9E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::sub_r8_r8_with_carry::<A, Z>],
        },
        Instruction {
            opcode: 0x9F,
            micro_ops: vec![Cpu::sub_r8_r8_with_carry::<A, A>],
        },
        Instruction {
            opcode: 0xA0,
            micro_ops: vec![Cpu::and_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0xA1,
            micro_ops: vec![Cpu::and_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0xA2,
            micro_ops: vec![Cpu::and_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0xA3,
            micro_ops: vec![Cpu::and_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0xA4,
            micro_ops: vec![Cpu::and_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0xA5,
            micro_ops: vec![Cpu::and_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0xA6,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::and_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xA7,
            micro_ops: vec![Cpu::and_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0xA8,
            micro_ops: vec![Cpu::xor_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0xA9,
            micro_ops: vec![Cpu::xor_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0xAA,
            micro_ops: vec![Cpu::xor_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0xAB,
            micro_ops: vec![Cpu::xor_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0xAC,
            micro_ops: vec![Cpu::xor_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0xAD,
            micro_ops: vec![Cpu::xor_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0xAE,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::xor_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xAF,
            micro_ops: vec![Cpu::xor_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0xB0,
            micro_ops: vec![Cpu::or_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0xB1,
            micro_ops: vec![Cpu::or_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0xB2,
            micro_ops: vec![Cpu::or_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0xB3,
            micro_ops: vec![Cpu::or_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0xB4,
            micro_ops: vec![Cpu::or_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0xB5,
            micro_ops: vec![Cpu::or_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0xB6,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::or_r8_r8::<A,Z>],
        },
        Instruction {
            opcode: 0xB7,
            micro_ops: vec![Cpu::or_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0xB8,
            micro_ops: vec![Cpu::cp_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0xB9,
            micro_ops: vec![Cpu::cp_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0xBA,
            micro_ops: vec![Cpu::cp_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0xBB,
            micro_ops: vec![Cpu::cp_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0xBC,
            micro_ops: vec![Cpu::cp_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0xBD,
            micro_ops: vec![Cpu::cp_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0xBE,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::cp_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xBF,
            micro_ops: vec![Cpu::cp_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0xC0,
            micro_ops: vec![
                Cpu::check_cond::<CondNZ>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC1,
            micro_ops: vec![
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<BC, WZ>,
            ],
        },
        Instruction {
            opcode: 0xC2,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondNZ>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC3,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC4,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr_check::<PC, W, CondNZ>,
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC5,
            micro_ops: vec![
                Cpu::decrement_r16::<SP>,
                Cpu::write_memory_decr::<SP, B>,
                Cpu::write_memory::<SP, C>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC6,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::add_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xC7,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x00, SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC8,
            micro_ops: vec![
                Cpu::check_cond::<CondZ>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC9,
            micro_ops: vec![
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCA,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondZ>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCB,
            micro_ops: vec![Cpu::decode_cb],
        },
        Instruction {
            opcode: 0xCC,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr_check::<PC, W, CondZ>,
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCD,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCE,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::add_r8_r8_with_carry::<A, Z>,
            ],
        },
        Instruction {
            opcode: 0xCF,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x08, SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD0,
            micro_ops: vec![
                Cpu::check_cond::<CondNC>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD1,
            micro_ops: vec![
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<DE, WZ>,
            ],
        },
        Instruction {
            opcode: 0xD2,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondNC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD3,
            micro_ops: Vec::new(),
        },
        Instruction {
            opcode: 0xD4,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr_check::<PC, W, CondNC>,
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD5,
            micro_ops: vec![
                Cpu::decrement_r16::<SP>,
                Cpu::write_memory_decr::<SP, D>,
                Cpu::write_memory::<SP, E>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD6,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::sub_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xD7,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x0010, SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD8,
            micro_ops: vec![
                Cpu::check_cond::<CondC>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD9,
            micro_ops: vec![
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16_and_ime::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xDA,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xDB,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xDC,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr_check::<PC, W, CondC>,
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xDD,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xDE,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::sub_r8_r8_with_carry::<A, Z>,
            ],
        },
        Instruction {
            opcode: 0xDF,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x0018, SP, PcC>,
                Cpu::noop                
            ],
        },
        Instruction {
            opcode: 0xE0,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::write_memory_0xff::<Z, A>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xE1,
            micro_ops: vec![
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<HL, WZ>,
            ],
        },
        Instruction {
            opcode: 0xE2,
            micro_ops: vec![Cpu::write_memory_0xff::<C, A>, Cpu::noop],
        },
        Instruction {
            opcode: 0xE3,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xE4,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xE5,
            micro_ops: vec![
                Cpu::decrement_r16::<SP>,
                Cpu::write_memory_decr::<SP, H>,
                Cpu::write_memory::<SP, L>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xE6,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::and_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xE7,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x20, SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xE8,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::add_hl_sp_e_low,
                Cpu::add_hl_sp_e_high,
                Cpu::load_r16_r16::<SP, WZ>,
            ],
        },
        Instruction {
            opcode: 0xE9,
            micro_ops: vec![Cpu::load_r16_r16::<PC, HL>],
        },
        Instruction {
            opcode: 0xEA,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::write_memory::<WZ, A>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xEB,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xEC,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xED,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xEE,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::xor_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xEF,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x28, SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xF0,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>,  Cpu::read_memory_0xff::<Z, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xF1,
            micro_ops: vec![
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16_af_flags::<AF, WZ>,
            ],
        },  
        Instruction {
            opcode: 0xF2,
            micro_ops: vec![Cpu::read_memory_0xff::<C, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xF3,
            micro_ops: vec![Cpu::set_ime_0],
        },
        Instruction {
            opcode: 0xF4,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xF5,
            micro_ops: vec![
                Cpu::decrement_r16::<SP>,
                Cpu::write_memory_decr::<SP, A>,
                Cpu::write_memory::<SP, F>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xF6,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::or_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xF7,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x30, SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xF8,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::ld_hl_sp_e_low,
                Cpu::ld_hl_sp_e_high,
            ],
        },
        Instruction {
            opcode: 0xF9,
            micro_ops: vec![Cpu::load_r16_r16::<SP, HL>, Cpu::noop],
        },
        Instruction {
            opcode: 0xFA,
            micro_ops: vec![
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::read_memory::<WZ, Z>,
                Cpu::load_r8_r8::<A, Z>,
            ],
        },
        Instruction {
            opcode: 0xFB,
            micro_ops: vec![Cpu::set_ime_delay_1],
        },
        Instruction {
            opcode: 0xFC,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xFD,
            micro_ops: vec![],
        },
        Instruction {
            opcode: 0xFE,
            micro_ops: vec![Cpu::read_memory_incr::<PC, Z>, Cpu::cp_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xFF,
            micro_ops: vec![
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x38, SP, PcC>,
                Cpu::noop,
            ],
        },
    ]
}
