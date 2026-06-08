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
        opcode: 0x08, //TaBONNE GROSSE DARONNE LA PUTE,
        micro_ops: &[instructions::other::noop],
    },
    Instruction {
        opcode: 0x80,
        micro_ops: &[instructions::add::add_r8_r8::<A, B>],
    },
    Instruction {
        opcode: 0x81,
        micro_ops: &[instructions::add::add_r8_r8::<A, C>],
    },
    Instruction {
        opcode: 0x82,
        micro_ops: &[instructions::add::add_r8_r8::<A, D>],
    },
    Instruction {
        opcode: 0x83,
        micro_ops: &[instructions::add::add_r8_r8::<A, E>],
    },
    Instruction {
        opcode: 0x84,
        micro_ops: &[instructions::add::add_r8_r8::<A, H>],
    },
    Instruction {
        opcode: 0x85,
        micro_ops: &[instructions::add::add_r8_r8::<A, L>],
    },
    Instruction {
        opcode: 0x86,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::add::add_r8_r8::<A, Z>,
        ],
    },
    Instruction {
        opcode: 0xC6,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::add::add_r8_r8::<A, Z>,
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
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, B>],
    },
    //ADC C
    Instruction {
        opcode: 0x89,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, C>],
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
        micro_ops: &[instructions::sub::sub_r8_r8::<A, D>],
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
        opcode: 0x97,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<A, B>],
    },
    //SBC A, C
    Instruction {
        opcode: 0x98,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<A, C>],
    },
    //SBC A, D
    Instruction {
        opcode: 0x99,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<A, D>],
    },
    //SBC A, E
    Instruction {
        opcode: 0x9A,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<A, D>],
    },
    //SBC A, H
    Instruction {
        opcode: 0x9B,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<A, H>],
    },
    //SBC A, L
    Instruction {
        opcode: 0x9C,
        micro_ops: &[instructions::sub::sub_r8_r8_with_carry::<A, L>],
    },
    //SBC A, HL
    Instruction {
        opcode: 0x9E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::sub::sub_r8_r8_with_carry::<A, Z>,
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
            instructions::sub::sub_r8_r8_with_carry::<A, Z>,
        ],
    },
    //                      ADC
    // ADC B
    Instruction {
        opcode: 0x88,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, B>],
    },
    // ADC C
    Instruction {
        opcode: 0x89,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, C>],
    },
    // ADC D
    Instruction {
        opcode: 0x8A,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, D>],
    },
    //ADC E
    Instruction {
        opcode: 0x8B,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, E>],
    },
    //ADC H
    Instruction {
        opcode: 0x8C,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, H>],
    },
    //ADC L
    Instruction {
        opcode: 0x8D,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, L>],
    },
    //ADC HL
    Instruction {
        opcode: 0x8E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::add::add_r8_r8_with_carry::<A, Z>,
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
            instructions::add::add_r8_r8_with_carry::<A, Z>,
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
        micro_ops: &[instructions::cp::cp_r8_r8::<A, D>],
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
        micro_ops: &[instructions::and_or_xor::and_r8_r8::<A, D>],
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
        micro_ops: &[instructions::and_or_xor::or_r8_r8::<A, D>],
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
            instructions::and_or_xor::xor_r8_r8::<A, Z>,
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
        micro_ops: &[instructions::and_or_xor::xor_r8_r8::<A, D>],
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
            instructions::cp::cp_r8_r8::<A, Z>,
        ],
    },
    // CCF
    Instruction {
        opcode: 0x3F,
        micro_ops: &[instructions::other::ccf],
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
            instructions::load::write_memory_0xff::<C, Z>,
            instructions::other::noop,
        ],
    },
    //LDH A, (n)
    Instruction {
        opcode: 0xF0,
        micro_ops: &[
            instructions::load::read_memory_0xff::<C, Z>,
            instructions::load::load_r8_r8::<A, Z>,
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
            instructions::load::read_memory_decr::<HL, Z>,
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
            instructions::load::write_memory_incr::<HL, Z>,
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
        opcode: 0x08,
        micro_ops: &[
            instructions::inc_dec::dec_r16::<BC>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x18,
        micro_ops: &[
            instructions::inc_dec::dec_r16::<DE>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x28,
        micro_ops: &[
            instructions::inc_dec::dec_r16::<HL>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x38,
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
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_reassign_pc::<SP, PC_C>,
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
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_reassign_pc::<SP, PC_C>,
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
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_reassign_pc::<SP, PC_C>,
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
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_reassign_pc::<SP, PC_C>,
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
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_reassign_pc::<SP, PC_C>,
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
        opcode: 0xD0,
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
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_1::<SP, PC_C>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_0::<SP, PC_C>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_2::<SP, PC_C>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xDF,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_3::<SP, PC_C>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_4::<SP, PC_C>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xEF,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_5::<SP, PC_C>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF7,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_6::<SP, PC_C>,
            instructions::load::load_r16_r16::<PC, WZ>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xFF,
        micro_ops: &[
            instructions::load::write_memory_decr::<SP, PC_P>,
            instructions::load::write_memory_rst_7::<SP, PC_C>,
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
