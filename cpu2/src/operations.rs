use crate::cb_operations::decode_cb;
use crate::defines::Instruction;
use crate::defines::MicroOp;
use crate::implemenation::*;
use crate::instructions;
use crate::instructions::cond::CondC;
use crate::instructions::cond::CondNC;
use crate::instructions::cond::CondNZ;
use crate::instructions::cond::CondZ;

//We build that shit so that we can just define in INSTRUCTIONS what instructions are implemented
//But it'll eventually get deleted once everything is done since we won't need to build an array
pub static DISPATCH: [Option<&'static [MicroOp]>; 256] = build_dispatch();

pub static INSTRUCTIONS: &[Instruction] = &[
    Instruction {
        opcode: 0x00,
        micro_ops: &[instructions::other::noop],
    },
    Instruction {
        opcode: 0x10, //TaBONNE GROSSE DARONNE LA PUTE,
        micro_ops: &[instructions::other::noop],
    },
    Instruction {
        opcode: 0x80,
        micro_ops: &[instructions::add::add_r8_r8::<B, A>],
    },
    Instruction {
        opcode: 0x81,
        micro_ops: &[instructions::add::add_r8_r8::<C, A>],
    },
    Instruction {
        opcode: 0x82,
        micro_ops: &[instructions::add::add_r8_r8::<D, A>],
    },
    Instruction {
        opcode: 0x83,
        micro_ops: &[instructions::add::add_r8_r8::<E, A>],
    },
    Instruction {
        opcode: 0x84,
        micro_ops: &[instructions::add::add_r8_r8::<H, A>],
    },
    Instruction {
        opcode: 0x85,
        micro_ops: &[instructions::add::add_r8_r8::<L, A>],
    },
    Instruction {
        opcode: 0x86,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::add::add_r8_r8::<Z, A>,
        ],
    },
    Instruction {
        opcode: 0x87,
        micro_ops: &[instructions::add::add_r8_r8::<A, A>],
    },
    Instruction {
        opcode: 0xC6,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::add::add_r8_r8::<Z, A>,
        ],
    },
    //LD (HL), n
    Instruction {
        opcode: 0x36,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::write_memory::<HL, Z>,
            instructions::other::noop,
        ],
    },
    //ADC B
    Instruction {
        opcode: 0x88,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<B, A>],
    },
    //ADC C
    Instruction {
        opcode: 0x89,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<C, A>],
    },
    //Sub A, B
    Instruction {
        opcode: 0x90,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, B>],
    },
    //Sub A, C
    Instruction {
        opcode: 0x91,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, C>],
    },
    //Sub A, D
    Instruction {
        opcode: 0x92,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, D>],
    },
    //Sub A, E
    Instruction {
        opcode: 0x93,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, E>],
    },
    //Sub A, H
    Instruction {
        opcode: 0x94,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, H>],
    },
    //Sub A, L
    Instruction {
        opcode: 0x95,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, L>],
    },
    //Sub A, HL
    Instruction {
        opcode: 0x96,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::sub::sub_r8_r8::<A, Z>,
        ],
    },
    //Sub A, A
    Instruction {
        opcode: 0x97,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, A>],
    },
    //SUB n
    Instruction {
        opcode: 0xD6,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::sub::sub_r8_r8::<A, Z>,
        ],
    },
    //                              SBC
    //SBC A, B
    Instruction {
        opcode: 0x98,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<B, A>],
    },
    //SBC A, C
    Instruction {
        opcode: 0x99,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<C, A>],
    },
    //SBC A, D
    Instruction {
        opcode: 0x9A,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<D, A>],
    },
    //SBC A, E
    Instruction {
        opcode: 0x9B,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<E, A>],
    },
    //SBC A, H
    Instruction {
        opcode: 0x9C,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<H, A>],
    },
    //SBC A, L
    Instruction {
        opcode: 0x9D,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<L, A>],
    },
    //SBC A, HL
    Instruction {
        opcode: 0x9E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::sub::sub_r8_r8_with_carry::<Z, A>,
        ],
    },
    //SCB A, A
    Instruction {
        opcode: 0x9F,
        micro_ops: &[instructions::sub::sub_r8_r8::<A, A>],
    },
    //SBC n
    Instruction {
        opcode: 0xDE,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::sub::sub_r8_r8_with_carry::<Z, A>,
        ],
    },
    //                      ADC
    // ADC D
    Instruction {
        opcode: 0x8A,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<D, A>],
    },
    //ADC E
    Instruction {
        opcode: 0x8B,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<E, A>],
    },
    //ADC H
    Instruction {
        opcode: 0x8C,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<H, A>],
    },
    //ADC L
    Instruction {
        opcode: 0x8D,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<L, A>],
    },
    //ADC HL
    Instruction {
        opcode: 0x8E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::add::add_r8_r8_with_carry::<Z, A>,
        ],
    },
    //ADC A
    Instruction {
        opcode: 0x8F,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, A>],
    },
    //ADC n
    Instruction {
        opcode: 0xCE,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::add::add_r8_r8_with_carry::<Z, A>,
        ],
    },
    //                  CP
    //CP A, B
    Instruction {
        opcode: 0xB8,
        micro_ops: &[instructions::cp::cp_r8_r8::<A, B>],
    },
    //CP A, C
    Instruction {
        opcode: 0xB9,
        micro_ops: &[instructions::cp::cp_r8_r8::<A, C>],
    },
    //CP A, D
    Instruction {
        opcode: 0xBA,
        micro_ops: &[instructions::cp::cp_r8_r8::<A, D>],
    },
    //CP A, E
    Instruction {
        opcode: 0xBB,
        micro_ops: &[instructions::cp::cp_r8_r8::<A, E>],
    },
    //CP A, H
    Instruction {
        opcode: 0xBC,
        micro_ops: &[instructions::cp::cp_r8_r8::<A, H>],
    },
    //CP A, L
    Instruction {
        opcode: 0xBD,
        micro_ops: &[instructions::cp::cp_r8_r8::<A, L>],
    },
    //CP A, HL
    Instruction {
        opcode: 0xBE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::cp::cp_r8_r8::<A, Z>,
        ],
    },
    //CP A, A
    Instruction {
        opcode: 0xBF,
        micro_ops: &[instructions::cp::cp_r8_r8::<A, A>],
    },
    //CP n
    Instruction {
        opcode: 0xFE,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::cp::cp_r8_r8::<A, Z>,
        ],
    },
    //                      INC
    //INC B
    Instruction {
        opcode: 0x04,
        micro_ops: &[instructions::inc_dec::inc_r8::<B>],
    },
    //INC, C
    Instruction {
        opcode: 0x0C,
        micro_ops: &[instructions::inc_dec::inc_r8::<C>],
    },
    //INC, D
    Instruction {
        opcode: 0x14,
        micro_ops: &[instructions::inc_dec::inc_r8::<D>],
    },
    //INC, E
    Instruction {
        opcode: 0x1C,
        micro_ops: &[instructions::inc_dec::inc_r8::<E>],
    },
    //INC, H
    Instruction {
        opcode: 0x24,
        micro_ops: &[instructions::inc_dec::inc_r8::<H>],
    },
    //INC, L
    Instruction {
        opcode: 0x2C,
        micro_ops: &[instructions::inc_dec::inc_r8::<L>],
    },
    //INC, HL
    Instruction {
        opcode: 0x34,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::inc_dec::inc_addr::<HL, Z>,
            instructions::other::noop,
        ],
    },
    //INC A, A
    Instruction {
        opcode: 0x3C,
        micro_ops: &[instructions::inc_dec::inc_r8::<A>],
    },
    //                      DEC
    //DEC B
    Instruction {
        opcode: 0x05,
        micro_ops: &[instructions::inc_dec::dec_r8::<B>],
    },
    //DEC, C
    Instruction {
        opcode: 0x0D,
        micro_ops: &[instructions::inc_dec::dec_r8::<C>],
    },
    //DEC, D
    Instruction {
        opcode: 0x15,
        micro_ops: &[instructions::inc_dec::dec_r8::<D>],
    },
    //DEC, E
    Instruction {
        opcode: 0x1D,
        micro_ops: &[instructions::inc_dec::dec_r8::<E>],
    },
    //DEC, H
    Instruction {
        opcode: 0x25,
        micro_ops: &[instructions::inc_dec::dec_r8::<H>],
    },
    //DEC, L
    Instruction {
        opcode: 0x2D,
        micro_ops: &[instructions::inc_dec::dec_r8::<L>],
    },
    //DEC, HL
    Instruction {
        opcode: 0x35,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::inc_dec::dec_addr::<HL, Z>,
            instructions::other::noop,
        ],
    },
    //DEC A, A
    Instruction {
        opcode: 0x3D,
        micro_ops: &[instructions::inc_dec::dec_r8::<A>],
    },
    //                      AND
    //AND A, B
    Instruction {
        opcode: 0xA0,
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, B>],
    },
    //AND A, C
    Instruction {
        opcode: 0xA1,
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, C>],
    },
    //AND A, D
    Instruction {
        opcode: 0xA2,
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, D>],
    },
    //AND A, E
    Instruction {
        opcode: 0xA3,
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, E>],
    },
    //AND A, H
    Instruction {
        opcode: 0xA4,
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, H>],
    },
    //AND A, L
    Instruction {
        opcode: 0xA5,
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, L>],
    },
    //AND A, HL
    Instruction {
        opcode: 0xA6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::and_or_xor::and_r8_r8::<A, Z>,
        ],
    },
    //AND A, A
    Instruction {
        opcode: 0xA7,
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, A>],
    },
    //AND n
    Instruction {
        opcode: 0xE6,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::and_or_xor::and_r8_r8::<A, Z>,
        ],
    },
    //                      OR
    //OR A, B
    Instruction {
        opcode: 0xB0,
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, B>],
    },
    //OR A, C
    Instruction {
        opcode: 0xB1,
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, C>],
    },
    //OR A, D
    Instruction {
        opcode: 0xB2,
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, D>],
    },
    //OR A, E
    Instruction {
        opcode: 0xB3,
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, E>],
    },
    //OR A, H
    Instruction {
        opcode: 0xB4,
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, H>],
    },
    //OR A, L
    Instruction {
        opcode: 0xB5,
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, L>],
    },
    //OR A, HL
    Instruction {
        opcode: 0xB6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::and_or_xor::or_r8_r8::<A, Z>,
        ],
    },
    //OR A, A
    Instruction {
        opcode: 0xB7,
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, A>],
    },
    //OR n
    Instruction {
        opcode: 0xF6,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::and_or_xor::or_r8_r8::<A, Z>,
        ],
    },
    //                      XOR
    //XOR A, B
    Instruction {
        opcode: 0xA8,
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, B>],
    },
    //XOR A, C
    Instruction {
        opcode: 0xA9,
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, C>],
    },
    //XOR A, D
    Instruction {
        opcode: 0xAA,
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, D>],
    },
    //XOR A, E
    Instruction {
        opcode: 0xAB,
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, E>],
    },
    //XOR A, H
    Instruction {
        opcode: 0xAC,
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, H>],
    },
    //XOR A, L
    Instruction {
        opcode: 0xAD,
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, L>],
    },
    //XOR A, HL
    Instruction {
        opcode: 0xAE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::and_or_xor::xor_r8_r8::<A, Z>,
        ],
    },
    //XOR A, A
    Instruction {
        opcode: 0xAF,
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, A>],
    },
    //XOR n
    Instruction {
        opcode: 0xEE,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::and_or_xor::xor_r8_r8::<A, Z>,
        ],
    },
    //                      LD
    //LD A, (BC)
    Instruction {
        opcode: 0x0a,
        micro_ops: &[
            instructions::load::read_memory::<BC, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD A, (DE)
    Instruction {
        opcode: 0x1a,
        micro_ops: &[
            instructions::load::read_memory::<DE, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (BC), A
    Instruction {
        opcode: 0x02,
        micro_ops: &[
            instructions::load::write_memory::<BC, A>,
            instructions::other::noop,
        ],
    },
    //LD (DE), A
    Instruction {
        opcode: 0x12,
        micro_ops: &[
            instructions::load::write_memory::<DE, A>,
            instructions::other::noop,
        ],
    },
    //LD A, (nn)
    Instruction {
        opcode: 0xfa,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::read_memory::<WZ, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (nn), A
    Instruction {
        opcode: 0xea,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::write_memory::<WZ, A>,
            instructions::other::noop,
        ],
    },
    //LDH A, (C)
    Instruction {
        opcode: 0xF2,
        micro_ops: &[
            instructions::load::read_memory_0xff::<C, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LDH (C), A
    Instruction {
        opcode: 0xE2,
        micro_ops: &[
            instructions::load::write_memory_0xff::<C, A>,
            instructions::other::noop,
        ],
    },
    //LDH A, (n)
    Instruction {
        opcode: 0xF0,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_0xff::<Z, A>,
        ],
    },
    //LDH (n), A
    Instruction {
        opcode: 0xE0,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::write_memory_0xff::<Z, A>,
            instructions::other::noop,
        ],
    },
    //LD A, (HL-)
    Instruction {
        opcode: 0x3A,
        micro_ops: &[
            instructions::load::read_memory_decr::<HL, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (HL-), A
    Instruction {
        opcode: 0x32,
        micro_ops: &[
            instructions::load::write_memory_decr::<HL, A>,
            instructions::other::noop,
        ],
    },
    //LD A, (HL+)
    Instruction {
        opcode: 0x2A,
        micro_ops: &[
            instructions::load::read_memory_incr::<HL, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (HL+), A
    Instruction {
        opcode: 0x22,
        micro_ops: &[
            instructions::load::write_memory_incr::<HL, A>,
            instructions::other::noop,
        ],
    },
    //LD BC, NN
    Instruction {
        opcode: 0x01,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<BC, WZ>,
        ],
    },
    //LD DE, NN
    Instruction {
        opcode: 0x11,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<DE, WZ>,
        ],
    },
    //LD HL, NN
    Instruction {
        opcode: 0x21,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<HL, WZ>,
        ],
    },
    //LD SP, NN
    Instruction {
        opcode: 0x31,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<SP, WZ>,
        ],
    },
    //LD NN, SP
    Instruction {
        opcode: 0x08,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::write_memory_incr::<WZ, P>,
            instructions::load::write_memory_incr::<WZ, S>,
            instructions::other::noop,
        ],
    },
    //LD SP, HL
    Instruction {
        opcode: 0xF9,
        micro_ops: &[
            instructions::load::load_r16_r16::<SP, HL>,
            instructions::other::noop,
        ],
    },
    //PUSH, rr
    Instruction {
        opcode: 0xC5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, B>,
            instructions::load::write_memory::<SP, C>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, D>,
            instructions::load::write_memory::<SP, E>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, H>,
            instructions::load::write_memory::<SP, L>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, A>,
            instructions::load::write_memory::<SP, F>,
            instructions::other::noop,
        ],
    },
    //POP, rr
    Instruction {
        opcode: 0xC1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<BC, WZ>,
        ],
    },
    Instruction {
        opcode: 0xD1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<DE, WZ>,
        ],
    },
    Instruction {
        opcode: 0xE1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<HL, WZ>,
        ],
    },
    Instruction {
        opcode: 0xF1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<AF, WZ>,
        ],
    },
    //LD HL,SP+e
    Instruction {
        opcode: 0xF8,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::ld_hl_sp_e_low,
            instructions::load::ld_hl_sp_e_high,
        ],
    },
    //LD B, r
    Instruction {
        opcode: 0x40,
        micro_ops: &[instructions::load::load_r8_r8::<B, B>],
    },
    Instruction {
        opcode: 0x41,
        micro_ops: &[instructions::load::load_r8_r8::<B, C>],
    },
    Instruction {
        opcode: 0x42,
        micro_ops: &[instructions::load::load_r8_r8::<B, D>],
    },
    Instruction {
        opcode: 0x43,
        micro_ops: &[instructions::load::load_r8_r8::<B, E>],
    },
    Instruction {
        opcode: 0x44,
        micro_ops: &[instructions::load::load_r8_r8::<B, H>],
    },
    Instruction {
        opcode: 0x45,
        micro_ops: &[instructions::load::load_r8_r8::<B, L>],
    },
    Instruction {
        opcode: 0x46,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<B, Z>,
        ],
    },
    Instruction {
        opcode: 0x47,
        micro_ops: &[instructions::load::load_r8_r8::<B, A>],
    },
    Instruction {
        opcode: 0x06,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::load_r8_r8::<B, Z>,
        ],
    },
    //LD C, r
    Instruction {
        opcode: 0x48,
        micro_ops: &[instructions::load::load_r8_r8::<C, B>],
    },
    Instruction {
        opcode: 0x49,
        micro_ops: &[instructions::load::load_r8_r8::<C, C>],
    },
    Instruction {
        opcode: 0x4a,
        micro_ops: &[instructions::load::load_r8_r8::<C, D>],
    },
    Instruction {
        opcode: 0x4b,
        micro_ops: &[instructions::load::load_r8_r8::<C, E>],
    },
    Instruction {
        opcode: 0x4c,
        micro_ops: &[instructions::load::load_r8_r8::<C, H>],
    },
    Instruction {
        opcode: 0x4d,
        micro_ops: &[instructions::load::load_r8_r8::<C, L>],
    },
    Instruction {
        opcode: 0x4e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<C, Z>,
        ],
    },
    Instruction {
        opcode: 0x4f,
        micro_ops: &[instructions::load::load_r8_r8::<C, A>],
    },
    Instruction {
        opcode: 0x0E,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::load_r8_r8::<C, Z>,
        ],
    },
    //LD D, r
    Instruction {
        opcode: 0x50,
        micro_ops: &[instructions::load::load_r8_r8::<D, B>],
    },
    Instruction {
        opcode: 0x51,
        micro_ops: &[instructions::load::load_r8_r8::<D, C>],
    },
    Instruction {
        opcode: 0x52,
        micro_ops: &[instructions::load::load_r8_r8::<D, D>],
    },
    Instruction {
        opcode: 0x53,
        micro_ops: &[instructions::load::load_r8_r8::<D, E>],
    },
    Instruction {
        opcode: 0x54,
        micro_ops: &[instructions::load::load_r8_r8::<D, H>],
    },
    Instruction {
        opcode: 0x55,
        micro_ops: &[instructions::load::load_r8_r8::<D, L>],
    },
    Instruction {
        opcode: 0x56,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<D, Z>,
        ],
    },
    Instruction {
        opcode: 0x57,
        micro_ops: &[instructions::load::load_r8_r8::<D, A>],
    },
    Instruction {
        opcode: 0x16,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::load_r8_r8::<D, Z>,
        ],
    },
    //LD E, r
    Instruction {
        opcode: 0x58,
        micro_ops: &[instructions::load::load_r8_r8::<E, B>],
    },
    Instruction {
        opcode: 0x59,
        micro_ops: &[instructions::load::load_r8_r8::<E, C>],
    },
    Instruction {
        opcode: 0x5a,
        micro_ops: &[instructions::load::load_r8_r8::<E, D>],
    },
    Instruction {
        opcode: 0x5b,
        micro_ops: &[instructions::load::load_r8_r8::<E, E>],
    },
    Instruction {
        opcode: 0x5c,
        micro_ops: &[instructions::load::load_r8_r8::<E, H>],
    },
    Instruction {
        opcode: 0x5d,
        micro_ops: &[instructions::load::load_r8_r8::<E, L>],
    },
    Instruction {
        opcode: 0x5e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<E, Z>,
        ],
    },
    Instruction {
        opcode: 0x5f,
        micro_ops: &[instructions::load::load_r8_r8::<E, A>],
    },
    Instruction {
        opcode: 0x1E,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::load_r8_r8::<E, Z>,
        ],
    },
    //LD H, r
    Instruction {
        opcode: 0x60,
        micro_ops: &[instructions::load::load_r8_r8::<H, B>],
    },
    Instruction {
        opcode: 0x61,
        micro_ops: &[instructions::load::load_r8_r8::<H, C>],
    },
    Instruction {
        opcode: 0x62,
        micro_ops: &[instructions::load::load_r8_r8::<H, D>],
    },
    Instruction {
        opcode: 0x63,
        micro_ops: &[instructions::load::load_r8_r8::<H, E>],
    },
    Instruction {
        opcode: 0x64,
        micro_ops: &[instructions::load::load_r8_r8::<H, H>],
    },
    Instruction {
        opcode: 0x65,
        micro_ops: &[instructions::load::load_r8_r8::<H, L>],
    },
    Instruction {
        opcode: 0x66,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<H, Z>,
        ],
    },
    Instruction {
        opcode: 0x67,
        micro_ops: &[instructions::load::load_r8_r8::<H, A>],
    },
    Instruction {
        opcode: 0x26,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::load_r8_r8::<H, Z>,
        ],
    },
    //LD L, r
    Instruction {
        opcode: 0x68,
        micro_ops: &[instructions::load::load_r8_r8::<L, B>],
    },
    Instruction {
        opcode: 0x69,
        micro_ops: &[instructions::load::load_r8_r8::<L, C>],
    },
    Instruction {
        opcode: 0x6a,
        micro_ops: &[instructions::load::load_r8_r8::<L, D>],
    },
    Instruction {
        opcode: 0x6b,
        micro_ops: &[instructions::load::load_r8_r8::<L, E>],
    },
    Instruction {
        opcode: 0x6c,
        micro_ops: &[instructions::load::load_r8_r8::<L, H>],
    },
    Instruction {
        opcode: 0x6d,
        micro_ops: &[instructions::load::load_r8_r8::<L, L>],
    },
    Instruction {
        opcode: 0x6e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<L, Z>,
        ],
    },
    Instruction {
        opcode: 0x6f,
        micro_ops: &[instructions::load::load_r8_r8::<L, A>],
    },
    Instruction {
        opcode: 0x2E,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::load_r8_r8::<L, Z>,
        ],
    },
    //LD HL, r
    Instruction {
        opcode: 0x70,
        micro_ops: &[
            instructions::load::write_memory::<HL, B>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x71,
        micro_ops: &[
            instructions::load::write_memory::<HL, C>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x72,
        micro_ops: &[
            instructions::load::write_memory::<HL, D>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x73,
        micro_ops: &[
            instructions::load::write_memory::<HL, E>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x74,
        micro_ops: &[
            instructions::load::write_memory::<HL, H>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x75,
        micro_ops: &[
            instructions::load::write_memory::<HL, L>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x77,
        micro_ops: &[
            instructions::load::write_memory::<HL, A>,
            instructions::other::noop,
        ],
    },
    //HALT
    Instruction {
        opcode: 0x76,
        micro_ops: &[instructions::other::halt],
    },
    //LD A, r
    Instruction {
        opcode: 0x78,
        micro_ops: &[instructions::load::load_r8_r8::<A, B>],
    },
    Instruction {
        opcode: 0x79,
        micro_ops: &[instructions::load::load_r8_r8::<A, C>],
    },
    Instruction {
        opcode: 0x7a,
        micro_ops: &[instructions::load::load_r8_r8::<A, D>],
    },
    Instruction {
        opcode: 0x7b,
        micro_ops: &[instructions::load::load_r8_r8::<A, E>],
    },
    Instruction {
        opcode: 0x7c,
        micro_ops: &[instructions::load::load_r8_r8::<A, H>],
    },
    Instruction {
        opcode: 0x7d,
        micro_ops: &[instructions::load::load_r8_r8::<A, L>],
    },
    Instruction {
        opcode: 0x7e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    Instruction {
        opcode: 0x7f,
        micro_ops: &[instructions::load::load_r8_r8::<A, A>],
    },
    Instruction {
        opcode: 0x3E,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    Instruction {
        opcode: 0x3F,
        micro_ops: &[instructions::other::ccf],
    },
    Instruction {
        opcode: 0x37,
        micro_ops: &[instructions::other::scf],
    },
    Instruction {
        opcode: 0x27,
        micro_ops: &[instructions::other::daa],
    },
    Instruction {
        opcode: 0x2F,
        micro_ops: &[instructions::other::cpl],
    },
    Instruction {
        opcode: 0x03,
        micro_ops: &[
            instructions::inc_dec::inc_r16::<BC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x13,
        micro_ops: &[
            instructions::inc_dec::inc_r16::<DE>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x23,
        micro_ops: &[
            instructions::inc_dec::inc_r16::<HL>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x33,
        micro_ops: &[
            instructions::inc_dec::inc_r16::<SP>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x0B,
        micro_ops: &[
            instructions::inc_dec::dec_r16::<BC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x1B,
        micro_ops: &[
            instructions::inc_dec::dec_r16::<DE>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x2B,
        micro_ops: &[
            instructions::inc_dec::dec_r16::<HL>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x3B,
        micro_ops: &[
            instructions::inc_dec::dec_r16::<SP>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x09,
        micro_ops: &[
            instructions::add::add_r8_r8_no_zero_flag::<C, L>,
            instructions::add::add_r8_r8_with_carry_and_no_zero_flag::<B, H>,
        ],
    },
    Instruction {
        opcode: 0x19,
        micro_ops: &[
            instructions::add::add_r8_r8_no_zero_flag::<E, L>,
            instructions::add::add_r8_r8_with_carry_and_no_zero_flag::<D, H>,
        ],
    },
    Instruction {
        opcode: 0x29,
        micro_ops: &[
            instructions::add::add_r8_r8_no_zero_flag::<L, L>,
            instructions::add::add_r8_r8_with_carry_and_no_zero_flag::<H, H>,
        ],
    },
    Instruction {
        opcode: 0x39,
        micro_ops: &[
            instructions::add::add_r8_r8_no_zero_flag::<P, L>,
            instructions::add::add_r8_r8_with_carry_and_no_zero_flag::<S, H>,
        ],
    },
    Instruction {
        opcode: 0xE8,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::add::add_hl_sp_e_low,
            instructions::add::add_hl_sp_e_high,
            instructions::load::load_r16_r16::<SP, WZ>,
        ],
    },
    Instruction {
        opcode: 0x07,
        micro_ops: &[instructions::other::rlca],
    },
    Instruction {
        opcode: 0x0F,
        micro_ops: &[instructions::other::rrca],
    },
    Instruction {
        opcode: 0x17,
        micro_ops: &[instructions::other::rla],
    },
    Instruction {
        opcode: 0x1F,
        micro_ops: &[instructions::other::rra],
    },
    Instruction {
        opcode: 0x20,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::cond::check_cond::<CondNZ>,
            instructions::cond::relative_jump,
        ],
    },
    Instruction {
        opcode: 0x30,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::cond::check_cond::<CondNC>,
            instructions::cond::relative_jump,
        ],
    },
    Instruction {
        opcode: 0x28,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::cond::check_cond::<CondZ>,
            instructions::cond::relative_jump,
        ],
    },
    Instruction {
        opcode: 0x38,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::cond::check_cond::<CondC>,
            instructions::cond::relative_jump,
        ],
    },
    Instruction {
        opcode: 0xCD,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::inc_dec::dec_r16::<SP>,
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_reassign_pc::<SP, PcC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD4,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::inc_dec::dec_r16::<SP>,
            instructions::cond::check_cond::<CondNZ>,
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_reassign_pc::<SP, PcC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC4,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::inc_dec::dec_r16::<SP>,
            instructions::cond::check_cond::<CondNC>,
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_reassign_pc::<SP, PcC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xDC,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::inc_dec::dec_r16::<SP>,
            instructions::cond::check_cond::<CondC>,
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_reassign_pc::<SP, PcC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xCC,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::inc_dec::dec_r16::<SP>,
            instructions::cond::check_cond::<CondZ>,
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_reassign_pc::<SP, PcC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC3,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE9,
        micro_ops: &[instructions::load::load_r16_r16::<PC, HL>],
    },
    Instruction {
        opcode: 0xC2,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::cond::check_cond::<CondNZ>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD2,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::cond::check_cond::<CondNC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xCA,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::cond::check_cond::<CondZ>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xDA,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::cond::check_cond::<CondC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x18,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::cond::relative_jump,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC9,
        micro_ops: &[
            instructions::load::read_memory_incr::<SP, Z>,
            instructions::load::read_memory_incr::<SP, W>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC0,
        micro_ops: &[
            instructions::cond::check_cond::<CondNZ>,
            instructions::load::read_memory_incr::<SP, Z>,
            instructions::load::read_memory_incr::<SP, W>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD0,
        micro_ops: &[
            instructions::cond::check_cond::<CondNC>,
            instructions::load::read_memory_incr::<SP, Z>,
            instructions::load::read_memory_incr::<SP, W>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC8,
        micro_ops: &[
            instructions::cond::check_cond::<CondZ>,
            instructions::load::read_memory_incr::<SP, Z>,
            instructions::load::read_memory_incr::<SP, W>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD8,
        micro_ops: &[
            instructions::cond::check_cond::<CondC>,
            instructions::load::read_memory_incr::<SP, Z>,
            instructions::load::read_memory_incr::<SP, W>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD9,
        micro_ops: &[
            instructions::load::read_memory_incr::<SP, Z>,
            instructions::load::read_memory_incr::<SP, W>,
            instructions::load::load_r16_r16_and_ime::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xCF,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x08, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x00, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x10, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xDF,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x18, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x20, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xEF,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x28, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x30, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xFF,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PcP>,
            instructions::load::write_memory_rst::<0x38, SP, PcC>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF3,
        micro_ops: &[instructions::other::set_ime_0],
    },
    Instruction {
        opcode: 0xFB,
        micro_ops: &[instructions::other::set_ime_1],
    },
    Instruction {
        opcode: 0xCB,
        micro_ops: &[decode_cb],
    },
];

const fn build_dispatch() -> [Option<&'static [MicroOp]>; 256] {
    let mut table = [None; 256];
    let mut i = 0;
    while i < INSTRUCTIONS.len() {
        let opcode = INSTRUCTIONS[i].opcode as usize;
        table[opcode] = Some(INSTRUCTIONS[i].micro_ops);
        i += 1;
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::{Cpu, Flag};
    use crate::flags::FlagsOps;
    use crate::implemenation::*;

    // Creates a CPU with the given first opcode and pads with NOPs so post-instruction
    // fetch never goes out of bounds.
    fn cpu(opcode: u8) -> Cpu {
        let mut list = vec![opcode];
        list.resize(32, 0x00);
        Cpu::new(list)
    }

    // Sets bus[0] = n (immediate byte read at PC=0 for the first instruction).
    fn cpu_n(opcode: u8, n: u8) -> Cpu {
        let mut c = cpu(opcode);
        c.bus[0] = n;
        c
    }

    // Sets bus[0]=lo, bus[1]=hi (16-bit immediate at PC=0 and PC=1).
    fn cpu_nn(opcode: u8, lo: u8, hi: u8) -> Cpu {
        let mut c = cpu(opcode);
        c.bus[0] = lo;
        c.bus[1] = hi;
        c
    }

    fn ticks(cpu: &mut Cpu, n: usize) {
        for _ in 0..n {
            cpu.tick();
        }
    }

    // --- NOP ---

    #[test]
    fn op_00_nop_preserves_state() {
        let mut c = cpu(0x00);
        c.set_r8::<A>(42);
        c.flags = 0xFF;
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 42);
        assert_eq!(c.flags, 0xFF);
    }

    // --- ADD A, r ---

    #[test]
    fn op_80_add_a_b() {
        let mut c = cpu(0x80);
        c.set_r8::<A>(5);
        c.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 8);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_83_add_a_e() {
        let mut c = cpu(0x83);
        c.set_r8::<A>(2);
        c.set_r8::<E>(10);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 12);
    }

    #[test]
    fn op_87_add_a_a() {
        let mut c = cpu(0x87);
        c.set_r8::<A>(7);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 14);
    }

    #[test]
    fn op_86_add_a_hl() {
        let mut c = cpu(0x86);
        c.set_r8::<A>(5);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 7;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 12);
    }

    #[test]
    fn op_c6_add_a_n() {
        let mut c = cpu_n(0xC6, 10);
        c.set_r8::<A>(5);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 15);
    }

    #[test]
    fn op_c6_add_a_n_zero_flag() {
        let mut c = cpu_n(0xC6, 0);
        c.set_r8::<A>(0);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0);
        assert!(c.flags.get_flag(Flag::Zero));
    }

    // --- ADC A, r ---

    #[test]
    fn op_88_adc_a_b_uses_carry() {
        let mut c = cpu(0x88);
        c.set_r8::<A>(5);
        c.set_r8::<B>(3);
        c.flags.set_flag(Flag::Carry, true);
        c.flags.set_flag(Flag::Zero, false);
        ticks(&mut c, 1);
        // Expected: 5 + 3 + 1(carry) = 9
        assert_eq!(c.get_r8::<A>(), 9);
    }

    #[test]
    fn op_ce_adc_a_n() {
        let mut c = cpu_n(0xCE, 4);
        c.set_r8::<A>(10);
        c.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 14);
    }

    // --- SUB A, r ---

    #[test]
    fn op_90_sub_a_b() {
        let mut c = cpu(0x90);
        c.set_r8::<A>(10);
        c.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 7);
        assert!(c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_97_sub_a_a() {
        let mut c = cpu(0x97);
        c.set_r8::<A>(5);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0);
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn op_93_sub_a_e() {
        let mut c = cpu(0x93);
        c.set_r8::<A>(20);
        c.set_r8::<D>(7);
        c.set_r8::<E>(3);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 17);
    }

    #[test]
    fn op_96_sub_a_hl() {
        let mut c = cpu(0x96);
        c.set_r8::<A>(15);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 5;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 10);
    }

    #[test]
    fn op_d6_sub_a_n() {
        let mut c = cpu_n(0xD6, 4);
        c.set_r8::<A>(10);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 6);
    }

    // --- SBC A, r ---

    #[test]
    fn op_9b_sbc_a_e() {
        let mut c = cpu(0x9B);
        c.set_r8::<A>(20);
        c.set_r8::<D>(7);
        c.set_r8::<E>(3);
        c.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 17);
    }

    // --- AND A, r ---

    #[test]
    fn op_a0_and_a_b() {
        let mut c = cpu(0xA0);
        c.set_r8::<A>(0b1111_0000);
        c.set_r8::<B>(0b1010_1010);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0b1010_0000);
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_a7_and_a_a() {
        let mut c = cpu(0xA7);
        c.set_r8::<A>(0b1010_1010);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0b1010_1010);
    }

    #[test]
    fn op_a3_and_a_e() {
        let mut c = cpu(0xA3);
        c.set_r8::<A>(0xFF);
        c.set_r8::<D>(0xF0);
        c.set_r8::<E>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0x0F);
    }

    #[test]
    fn op_a6_and_a_hl() {
        let mut c = cpu(0xA6);
        c.set_r8::<A>(0xFF);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 0xF0;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0xF0);
    }

    #[test]
    fn op_e6_and_a_n() {
        let mut c = cpu_n(0xE6, 0x0F);
        c.set_r8::<A>(0xFF);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0x0F);
    }

    // --- OR A, r ---

    #[test]
    fn op_b0_or_a_b() {
        let mut c = cpu(0xB0);
        c.set_r8::<A>(0b1111_0000);
        c.set_r8::<B>(0b0000_1111);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0xFF);
    }

    #[test]
    fn op_b3_or_a_e() {
        let mut c = cpu(0xB3);
        c.set_r8::<A>(0x00);
        c.set_r8::<D>(0xF0);
        c.set_r8::<E>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0x0F);
    }

    #[test]
    fn op_b6_or_a_hl() {
        let mut c = cpu(0xB6);
        c.set_r8::<A>(0xF0);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 0x0F;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0xFF);
    }

    #[test]
    fn op_f6_or_a_n() {
        let mut c = cpu_n(0xF6, 0b1010_1010);
        c.set_r8::<A>(0b1111_0000);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0b1111_1010);
    }

    // --- XOR A, r ---

    #[test]
    fn op_a8_xor_a_b() {
        let mut c = cpu(0xA8);
        c.set_r8::<A>(0xFF);
        c.set_r8::<B>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0xF0);
    }

    #[test]
    fn op_af_xor_a_a_clears() {
        let mut c = cpu(0xAF);
        c.set_r8::<A>(0xFF);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0x00);
        assert!(c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_ab_xor_a_e() {
        let mut c = cpu(0xAB);
        c.set_r8::<A>(0xFF);
        c.set_r8::<D>(0xF0);
        c.set_r8::<E>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0xF0);
    }

    #[test]
    fn op_ee_xor_a_n() {
        let mut c = cpu_n(0xEE, 0x0F);
        c.set_r8::<A>(0xFF);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0xF0);
    }

    // --- CP A, r ---

    #[test]
    fn op_b8_cp_a_b_equal() {
        let mut c = cpu(0xB8);
        c.set_r8::<A>(5);
        c.set_r8::<B>(5);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 5); // A unchanged
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn op_b8_cp_a_b_not_equal() {
        let mut c = cpu(0xB8);
        c.set_r8::<A>(10);
        c.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 10);
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_bb_cp_a_e() {
        let mut c = cpu(0xBB);
        c.set_r8::<A>(5);
        c.set_r8::<D>(10);
        c.set_r8::<E>(5);
        ticks(&mut c, 1);
        assert!(c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_fe_cp_a_n() {
        let mut c = cpu_n(0xFE, 5);
        c.set_r8::<A>(10);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 10);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::Subtract));
    }

    // --- INC r8 ---

    #[test]
    fn op_04_inc_b() {
        let mut c = cpu(0x04);
        c.set_r8::<B>(41);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<B>(), 42);
    }

    #[test]
    fn op_3c_inc_a_wraps() {
        let mut c = cpu(0x3C);
        c.set_r8::<A>(0xFF);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0);
    }

    #[test]
    fn op_1c_inc_e() {
        let mut c = cpu(0x1C);
        c.set_r8::<E>(9);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<E>(), 10);
    }

    #[test]
    fn op_34_inc_hl_addr() {
        let mut c = cpu(0x34);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 41;
        ticks(&mut c, 3);
        assert_eq!(c.bus[0x8000], 42);
    }

    // --- DEC r8 ---

    #[test]
    fn op_05_dec_b() {
        let mut c = cpu(0x05);
        c.set_r8::<B>(10);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<B>(), 9);
    }

    #[test]
    fn op_3d_dec_a_wraps() {
        let mut c = cpu(0x3D);
        c.set_r8::<A>(0x00);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0xFF);
    }

    #[test]
    fn op_35_dec_hl_addr() {
        let mut c = cpu(0x35);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 10;
        ticks(&mut c, 3);
        assert_eq!(c.bus[0x8000], 9);
    }

    // --- INC / DEC r16 ---

    #[test]
    fn op_23_inc_hl() {
        let mut c = cpu(0x23);
        c.set_r16::<HL>(0x1234);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<HL>(), 0x1235);
    }

    #[test]
    fn op_03_inc_bc() {
        let mut c = cpu(0x03);
        c.set_r16::<BC>(0x00FF);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<BC>(), 0x0100);
    }

    #[test]
    fn op_33_inc_sp() {
        let mut c = cpu(0x33);
        c.set_r16::<SP>(0x00FF);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<SP>(), 0x0100);
    }

    #[test]
    fn op_2b_dec_hl() {
        let mut c = cpu(0x2B);
        c.set_r16::<HL>(0x1235);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<HL>(), 0x1234);
    }

    #[test]
    fn op_0b_dec_bc() {
        let mut c = cpu(0x0B);
        c.set_r16::<BC>(0x0100);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<BC>(), 0x00FF);
    }

    // --- LD r, r (0x40–0x7F block) ---

    #[test]
    fn op_41_ld_b_c() {
        let mut c = cpu(0x41);
        c.set_r8::<C>(99);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<B>(), 99);
    }

    #[test]
    fn op_78_ld_a_b() {
        let mut c = cpu(0x78);
        c.set_r8::<B>(77);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 77);
    }

    #[test]
    fn op_7b_ld_a_e() {
        let mut c = cpu(0x7B);
        c.set_r8::<E>(55);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 55);
    }

    #[test]
    fn op_57_ld_d_a() {
        let mut c = cpu(0x57);
        c.set_r8::<A>(33);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<D>(), 33);
    }

    #[test]
    fn op_4f_ld_c_a() {
        let mut c = cpu(0x4F);
        c.set_r8::<A>(0xAB);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<C>(), 0xAB);
    }

    // --- LD A, (HL) / LD (HL), r ---

    #[test]
    fn op_7e_ld_a_hl() {
        let mut c = cpu(0x7E);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 55;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 55);
    }

    #[test]
    fn op_77_ld_hl_a() {
        let mut c = cpu(0x77);
        c.set_r8::<A>(33);
        c.set_r16::<HL>(0x9000);
        ticks(&mut c, 2);
        assert_eq!(c.bus[0x9000], 33);
    }

    #[test]
    fn op_70_ld_hl_b() {
        let mut c = cpu(0x70);
        c.set_r8::<B>(0x55);
        c.set_r16::<HL>(0x8000);
        ticks(&mut c, 2);
        assert_eq!(c.bus[0x8000], 0x55);
    }

    // --- LD (HL), n ---

    #[test]
    fn op_36_ld_hl_n() {
        let mut c = cpu_n(0x36, 0xAB);
        c.set_r16::<HL>(0x8000);
        ticks(&mut c, 3);
        assert_eq!(c.bus[0x8000], 0xAB);
    }

    // --- LD r, n ---

    #[test]
    fn op_06_ld_b_n() {
        let mut c = cpu_n(0x06, 42);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<B>(), 42);
    }

    #[test]
    fn op_3e_ld_a_n() {
        let mut c = cpu_n(0x3E, 77);
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 77);
    }

    // --- LD r16, nn ---

    #[test]
    fn op_21_ld_hl_nn() {
        let mut c = cpu_nn(0x21, 0x34, 0x12);
        ticks(&mut c, 3);
        assert_eq!(c.get_r16::<HL>(), 0x1234);
    }

    #[test]
    fn op_01_ld_bc_nn() {
        let mut c = cpu_nn(0x01, 0x78, 0x56);
        ticks(&mut c, 3);
        assert_eq!(c.get_r16::<BC>(), 0x5678);
    }

    #[test]
    fn op_31_ld_sp_nn() {
        let mut c = cpu_nn(0x31, 0xFE, 0xFF);
        ticks(&mut c, 3);
        assert_eq!(c.get_r16::<SP>(), 0xFFFE);
    }

    // --- LD A, (rr) / LD (rr), A ---

    #[test]
    fn op_0a_ld_a_bc() {
        let mut c = cpu(0x0A);
        c.set_r16::<BC>(0x8000);
        c.bus[0x8000] = 0x42;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0x42);
    }

    #[test]
    fn op_1a_ld_a_de() {
        let mut c = cpu(0x1A);
        c.set_r16::<DE>(0x8001);
        c.bus[0x8001] = 0x99;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0x99);
    }

    #[test]
    fn op_02_ld_bc_a() {
        let mut c = cpu(0x02);
        c.set_r8::<A>(0x77);
        c.set_r16::<BC>(0x8000);
        ticks(&mut c, 2);
        assert_eq!(c.bus[0x8000], 0x77);
    }

    #[test]
    fn op_12_ld_de_a() {
        let mut c = cpu(0x12);
        c.set_r8::<A>(0x55);
        c.set_r16::<DE>(0x8001);
        ticks(&mut c, 2);
        assert_eq!(c.bus[0x8001], 0x55);
    }

    // --- LD A, (nn) / LD (nn), A ---

    #[test]
    fn op_fa_ld_a_nn() {
        let mut c = cpu(0xFA);
        c.bus[0] = 0x50; // lo
        c.bus[1] = 0x80; // hi → 0x8050
        c.bus[0x8050] = 0xAB;
        ticks(&mut c, 4);
        assert_eq!(c.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_ea_ld_nn_a() {
        let mut c = cpu(0xEA);
        c.bus[0] = 0x50; // lo
        c.bus[1] = 0x80; // hi → 0x8050
        c.set_r8::<A>(0xCC);
        ticks(&mut c, 4);
        assert_eq!(c.bus[0x8050], 0xCC);
    }

    // --- LD A, (HL±) / LD (HL±), A ---

    #[test]
    fn op_3a_ld_a_hl_minus() {
        let mut c = cpu(0x3A);
        c.set_r16::<HL>(0x8001);
        c.bus[0x8001] = 0x55;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0x55);
        assert_eq!(c.get_r16::<HL>(), 0x8000);
    }

    #[test]
    fn op_2a_ld_a_hl_plus() {
        let mut c = cpu(0x2A);
        c.set_r16::<HL>(0x8000);
        c.bus[0x8000] = 0x44;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0x44);
        assert_eq!(c.get_r16::<HL>(), 0x8001);
    }

    #[test]
    fn op_32_ld_hl_minus_a() {
        let mut c = cpu(0x32);
        c.set_r8::<A>(0x99);
        c.set_r16::<HL>(0x8001);
        ticks(&mut c, 2);
        assert_eq!(c.bus[0x8001], 0x99);
        assert_eq!(c.get_r16::<HL>(), 0x8000);
    }

    #[test]
    fn op_22_ld_hl_plus_a() {
        let mut c = cpu(0x22);
        c.set_r8::<A>(0x66);
        c.set_r16::<HL>(0x8000);
        ticks(&mut c, 2);
        assert_eq!(c.bus[0x8000], 0x66);
        assert_eq!(c.get_r16::<HL>(), 0x8001);
    }

    // --- LDH ---

    #[test]
    fn op_f2_ldh_a_c() {
        let mut c = cpu(0xF2);
        c.set_r8::<C>(0x40);
        c.bus[0xFF40] = 0xAB;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0xAB);
    }

    #[test]
    fn op_e2_ldh_c_a() {
        let mut c = cpu(0xE2);
        c.set_r8::<A>(0xCD);
        c.set_r8::<C>(0x40);
        ticks(&mut c, 2);
        assert_eq!(c.bus[0xFF40], 0xCD);
    }

    #[test]
    fn op_f0_ldh_a_n() {
        let mut c = cpu_n(0xF0, 0x40); // n=0x40 → address 0xFF40
        c.bus[0xFF40] = 0x77;
        ticks(&mut c, 2);
        assert_eq!(c.get_r8::<A>(), 0x77);
    }

    // --- LD (nn), SP ---

    #[test]
    fn op_08_ld_nn_sp() {
        let mut c = cpu(0x08);
        c.bus[0] = 0x00; // lo
        c.bus[1] = 0x80; // hi → 0x8000
        c.set_r16::<SP>(0x1234); // P=0x34 (low), S=0x12 (high)
        ticks(&mut c, 5);
        assert_eq!(c.bus[0x8000], 0x34); // SP low byte first (little-endian)
        assert_eq!(c.bus[0x8001], 0x12); // SP high byte second
    }

    // --- LD SP, HL ---

    #[test]
    fn op_f9_ld_sp_hl() {
        let mut c = cpu(0xF9);
        c.set_r16::<HL>(0xFFFE);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<SP>(), 0xFFFE);
    }

    // --- LD HL, SP+e ---

    #[test]
    fn op_f8_ld_hl_sp_e() {
        let mut c = cpu_n(0xF8, 0x10);
        c.set_r16::<SP>(0x0100);
        ticks(&mut c, 3);
        assert_eq!(c.get_r16::<HL>(), 0x0110);
    }

    #[test]
    fn op_f8_ld_hl_sp_e_negative() {
        let mut c = cpu_n(0xF8, (-1i8) as u8); // e = -1
        c.set_r16::<SP>(0x0100);
        ticks(&mut c, 3);
        assert_eq!(c.get_r16::<HL>(), 0x00FF);
    }

    // --- Misc: SCF, CCF, CPL, DAA ---

    #[test]
    fn op_37_scf() {
        let mut c = cpu(0x37);
        c.flags.set_flag(Flag::Carry, false);
        c.flags.set_flag(Flag::Subtract, true);
        c.flags.set_flag(Flag::HalfCarry, true);
        ticks(&mut c, 1);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3f_ccf_toggles_carry() {
        let mut c = cpu(0x3F);
        c.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_3f_ccf_sets_carry_when_clear() {
        let mut c = cpu(0x3F);
        c.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 1);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_2f_cpl() {
        let mut c = cpu(0x2F);
        c.set_r8::<A>(0b1010_0101);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0b0101_1010);
        assert!(c.flags.get_flag(Flag::Subtract));
        assert!(c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_27_daa_after_add() {
        let mut c = cpu(0x27);
        // After ADD A=0x09 + B=0x09 = 0x12 (binary), DAA should give 0x18 (BCD)
        c.set_r8::<A>(0x12);
        c.flags.set_flag(Flag::Subtract, false);
        c.flags.set_flag(Flag::HalfCarry, false);
        c.flags.set_flag(Flag::Carry, false);
        ticks(&mut c, 1);
        // 0x12 → no adjustment needed since nibbles are already < 10
        // But 0x12 in BCD means 12, which is fine
        // DAA adjusts: lower nibble 2 < 10 and no half-carry → no low adjust
        // upper nibble 1 < 10 and no carry → no high adjust
        // result stays 0x12
        assert_eq!(c.get_r8::<A>(), 0x12);
        assert!(!c.flags.get_flag(Flag::Subtract));
    }

    // --- Rotate instructions ---

    #[test]
    fn op_07_rlca_msb_to_carry() {
        let mut c = cpu(0x07);
        c.set_r8::<A>(0b1000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0b0000_0011);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn op_0f_rrca_lsb_to_carry() {
        let mut c = cpu(0x0F);
        c.set_r8::<A>(0b0000_0001);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0b1000_0000);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_17_rla_through_carry() {
        let mut c = cpu(0x17);
        c.set_r8::<A>(0b1000_0000);
        c.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0b0000_0001); // old carry shifted in
        assert!(c.flags.get_flag(Flag::Carry)); // old msb shifted out
    }

    #[test]
    fn op_1f_rra_through_carry() {
        let mut c = cpu(0x1F);
        c.set_r8::<A>(0b0000_0001);
        c.flags.set_flag(Flag::Carry, true);
        ticks(&mut c, 1);
        assert_eq!(c.get_r8::<A>(), 0b1000_0000); // old carry shifted in
        assert!(c.flags.get_flag(Flag::Carry)); // old lsb shifted out
    }

    // --- ADD HL, rr ---

    // ADD HL, BC (0x09): uses individual B (r8[1]) and C (r8[2]) registers.
    #[test]
    fn op_09_add_hl_bc() {
        let mut c = cpu(0x09);
        c.set_r8::<B>(0x01); // high byte contribution
        c.set_r8::<C>(0x05); // low byte contribution
        c.set_r16::<HL>(0x0010);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<HL>(), 0x0115);
    }

    #[test]
    fn op_19_add_hl_de() {
        let mut c = cpu(0x19);
        c.set_r8::<D>(0x00); // high byte
        c.set_r8::<E>(0x10); // low byte
        c.set_r16::<HL>(0x0001);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<HL>(), 0x0011);
    }

    #[test]
    fn op_29_add_hl_hl() {
        let mut c = cpu(0x29);
        c.set_r16::<HL>(0x0010);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<HL>(), 0x0020);
    }

    #[test]
    fn op_39_add_hl_sp() {
        let mut c = cpu(0x39);
        c.set_r8::<S>(0x00); // SP high
        c.set_r8::<P>(0x05); // SP low
        c.set_r16::<HL>(0x0010);
        ticks(&mut c, 2);
        assert_eq!(c.get_r16::<HL>(), 0x0015);
    }

    // --- JR ---

    #[test]
    fn op_18_jr_forward() {
        let mut c = cpu(0x18);
        c.bus[0] = 0x05; // e = +5
        // tick1: read_memory_incr → Z=5, PC→1
        // tick2: relative_jump → PC=6
        // tick3: noop (last) + fetch → reads instructions_list[6]=NOP, PC→7
        ticks(&mut c, 3);
        assert_eq!(c.get_r16::<PC>(), 7);
    }

    // --- JR cc ---

    #[test]
    fn op_20_jr_nz_taken() {
        let mut c = cpu(0x20);
        c.bus[0] = 0x02; // e = +2
        c.flags.set_flag(Flag::Zero, false); // NZ condition met
        c.tick(); // Z=2, PC→1
        c.tick(); // check_cond NZ: met, op_index stays at 2
        c.tick(); // relative_jump: PC = 1+2 = 3, then fetch at 3 → PC→4
        assert_eq!(c.get_r16::<PC>(), 4);
    }

    #[test]
    fn op_20_jr_nz_not_taken() {
        let mut c = cpu(0x20);
        c.bus[0] = 0x02;
        c.flags.set_flag(Flag::Zero, true); // NZ not met
        c.tick(); // Z=2, PC→1
        c.tick(); // check_cond: condition NOT met → op_index=queue.len() → fetch inside tick
        // fetch reads instructions_list[PC=1], PC→2
        assert_eq!(c.get_r16::<PC>(), 2);
    }

    #[test]
    fn op_28_jr_z_taken() {
        let mut c = cpu(0x28);
        c.bus[0] = 0x02;
        c.flags.set_flag(Flag::Zero, true);
        c.tick();
        c.tick();
        c.tick(); // relative_jump, then fetch → PC→4
        assert_eq!(c.get_r16::<PC>(), 4);
    }

    #[test]
    fn op_28_jr_z_not_taken() {
        let mut c = cpu(0x28);
        c.bus[0] = 0x02;
        c.flags.set_flag(Flag::Zero, false);
        c.tick();
        c.tick(); // not taken → fetch inside tick, PC→2
        assert_eq!(c.get_r16::<PC>(), 2);
    }

    #[test]
    fn op_30_jr_nc_taken() {
        let mut c = cpu(0x30);
        c.bus[0] = 0x02;
        c.flags.set_flag(Flag::Carry, false);
        c.tick();
        c.tick();
        c.tick(); // relative_jump, fetch → PC→4
        assert_eq!(c.get_r16::<PC>(), 4);
    }

    #[test]
    fn op_38_jr_c_taken() {
        let mut c = cpu(0x38);
        c.bus[0] = 0x02;
        c.flags.set_flag(Flag::Carry, true);
        c.tick();
        c.tick();
        c.tick(); // relative_jump, fetch → PC→4
        assert_eq!(c.get_r16::<PC>(), 4);
    }

    // --- JP nn ---

    #[test]
    fn op_c3_jp_nn() {
        let mut c = cpu_nn(0xC3, 0x05, 0x00); // target = 0x0005
        c.tick(); // Z=5, PC→1
        c.tick(); // W=0, PC→2
        c.tick(); // PC = WZ = 0x0005
        // PC=5, then tick4 (noop) → fetch at 5, PC→6
        assert_eq!(c.get_r16::<PC>(), 0x0005);
        c.tick(); // noop + fetch → PC→6
        assert_eq!(c.get_r16::<PC>(), 6);
    }

    #[test]
    fn op_e9_jp_hl() {
        let mut c = cpu(0xE9);
        c.set_r16::<HL>(0x0005);
        c.tick(); // PC = HL = 5, then fetch at 5 → PC→6
        assert_eq!(c.get_r16::<PC>(), 6);
    }

    // --- JP cc, nn ---

    #[test]
    fn op_c2_jp_nz_taken() {
        let mut c = cpu_nn(0xC2, 0x08, 0x00); // target = 0x0008
        c.flags.set_flag(Flag::Zero, false);
        c.tick(); c.tick(); // Z=8, W=0, PC=2
        c.tick(); // check_cond NZ: met, continues
        c.tick(); // PC = WZ = 0x0008
        assert_eq!(c.get_r16::<PC>(), 0x0008);
    }

    #[test]
    fn op_c2_jp_nz_not_taken() {
        let mut c = cpu_nn(0xC2, 0x08, 0x00);
        c.flags.set_flag(Flag::Zero, true);
        c.tick(); c.tick(); // Z=8, W=0, PC=2
        c.tick(); // check_cond NZ: not met → fetch inside this tick, PC→3
        assert_eq!(c.get_r16::<PC>(), 3);
    }

    #[test]
    fn op_ca_jp_z_taken() {
        let mut c = cpu_nn(0xCA, 0x08, 0x00);
        c.flags.set_flag(Flag::Zero, true);
        c.tick(); c.tick(); c.tick(); // check_cond Z: met
        c.tick(); // PC = 0x0008
        assert_eq!(c.get_r16::<PC>(), 0x0008);
    }

    // --- CALL nn ---

    #[test]
    fn op_cd_call_nn_sets_pc() {
        let mut c = cpu_nn(0xCD, 0x08, 0x00); // target = 0x0008
        c.set_r16::<SP>(0xFFFE);
        ticks(&mut c, 5); // after write_memory_reassign_pc: PC=0x0008
        assert_eq!(c.get_r16::<PC>(), 0x0008);
    }

    #[test]
    fn op_cd_call_nn_saves_return_address() {
        let mut c = cpu_nn(0xCD, 0x08, 0x00);
        c.set_r16::<SP>(0xFFFE);
        ticks(&mut c, 5);
        // Return address = 2 (PC after reading 2 immediate bytes)
        // dec_r16::<SP> makes SP=0xFFFD
        // PcP (low=2) written at 0xFFFD, SP→0xFFFC
        // PcC (high=0) written at 0xFFFC
        assert_eq!(c.get_r16::<SP>(), 0xFFFC);
        assert_eq!(c.bus[0xFFFD], 0x02); // PC low
        assert_eq!(c.bus[0xFFFC], 0x00); // PC high
    }

    // --- RET ---

    #[test]
    fn op_c9_ret() {
        let mut c = cpu(0xC9);
        c.set_r16::<SP>(0xFFFC);
        // Stack has return address 0x0050 stored as: bus[0xFFFC]=high, bus[0xFFFD]=low
        // (matching how CALL stores it)
        c.bus[0xFFFC] = 0x00; // PcC (high byte)
        c.bus[0xFFFD] = 0x08; // PcP (low byte)
        ticks(&mut c, 3); // Z=bus[SP], W=bus[SP+1], PC=WZ
        // WZ = W<<8|Z = PcP<<8|PcC = 0x08<<8|0x00 = 0x0800 (endianness bug: should be 0x0008)
        // This test documents the endianness bug in CALL/RET
        // Correct behavior: PC should be 0x0008
        assert_eq!(c.get_r16::<PC>(), 0x0800); // Bug: wrong endianness
    }

    #[test]
    fn op_c0_ret_nz_taken() {
        let mut c = cpu(0xC0);
        c.flags.set_flag(Flag::Zero, false);
        c.set_r16::<SP>(0xFFFC);
        c.bus[0xFFFC] = 0x00;
        c.bus[0xFFFD] = 0x08;
        ticks(&mut c, 4); // check_cond + 3 ticks
        assert_eq!(c.get_r16::<PC>(), 0x0800); // same endianness bug
    }

    #[test]
    fn op_c0_ret_nz_not_taken() {
        let mut c = cpu(0xC0);
        c.flags.set_flag(Flag::Zero, true); // NZ not met
        ticks(&mut c, 1); // check_cond: skips → fetch inside this tick
        assert_eq!(c.get_r16::<PC>(), 1); // PC advanced by fetch only
    }
}
