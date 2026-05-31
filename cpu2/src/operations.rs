use std::ops::Bound::Included;

use crate::defines::Instruction;
use crate::defines::MicroOp;
use crate::instructions;
use crate::instructions::add::{add_a_b, add_a_c, add_a_d, add_a_e, add_a_h, add_a_hl, add_a_l};

//We build that shit so that we can just define in INSTRUCTIONS what instructions are implemented
//But it'll eventually get deleted once everything is done since we won't need to build an array
pub static DISPATCH: [Option<&'static [MicroOp]>; 256] = build_dispatch();

pub static INSTRUCTIONS: &[Instruction] = &[
    Instruction {
        opcode: 0x00,
        micro_ops: &[],
    },
    Instruction {
        opcode: 0x80,
        micro_ops: &[add_a_b],
    },
    Instruction {
        opcode: 0x81,
        micro_ops: &[add_a_c],
    },
    Instruction {
        opcode: 0x82,
        micro_ops: &[add_a_d],
    },
    Instruction {
        opcode: 0x83,
        micro_ops: &[add_a_e],
    },
    Instruction {
        opcode: 0x84,
        micro_ops: &[add_a_h],
    },
    Instruction {
        opcode: 0x85,
        micro_ops: &[add_a_l],
    },
    Instruction {
        opcode: 0x86,
        micro_ops: &[instructions::load_r16::load_tmp_hl, add_a_hl],
    },
    //LD (HL), m
    Instruction {
        opcode: 0x36,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::write_n_in_accu,
            instructions::other::noop,
        ],
    },
    //LD A, (BC)
    Instruction {
        opcode: 0x0a,
        micro_ops: &[
            instructions::load_r16::load_tmp_bc,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD A, (DE)
    Instruction {
        opcode: 0x1a,
        micro_ops: &[
            instructions::load_r16::load_tmp_de,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD (BC), A
    Instruction {
        opcode: 0x02,
        micro_ops: &[
            instructions::load_r16::write_a_in_bc,
            instructions::other::noop,
        ],
    },
    //LD (DE), A
    Instruction {
        opcode: 0x12,
        micro_ops: &[
            instructions::load_r16::write_a_in_de,
            instructions::other::noop,
        ],
    },
    //LD A, (nn)
    Instruction {
        opcode: 0xfa,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load::load_mem_in_accu,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD (nn), A
    Instruction {
        opcode: 0xea,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_a::load_a_in_mem,
            instructions::other::noop,
        ],
    },
    //LDH (C), A
    Instruction {
        opcode: 0xE2,
        micro_ops: &[
            instructions::load::write_a_in_c0x_ff,
            instructions::other::noop,
        ],
    },
    //LDH A, (n)
    Instruction {
        opcode: 0xF0,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_mem0x_ff_in_accu,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD A, (HL-)
    Instruction {
        opcode: 0x3A,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl_decr,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD (HL-), A
    Instruction {
        opcode: 0x32,
        micro_ops: &[
            instructions::load_r16::write_a_in_mem_decr,
            instructions::other::noop,
        ],
    },
    //LD A, (HL+)
    Instruction {
        opcode: 0x2A,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl_incr,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD (HL+), A
    Instruction {
        opcode: 0x22,
        micro_ops: &[
            instructions::load_r16::write_a_in_mem_incr,
            instructions::other::noop,
        ],
    },
    //LD BC, NN
    Instruction {
        opcode: 0x01,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_bc,
        ],
    },
    //LD DE, NN
    Instruction {
        opcode: 0x11,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_de,
        ],
    },
    //LD HL, NN
    Instruction {
        opcode: 0x21,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_hl,
        ],
    },
    //LD SP, NN
    Instruction {
        opcode: 0x31,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_sp,
        ],
    },
    //LD NN, SP
    Instruction {
        opcode: 0x08,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_lsb_sp_in_mem,
            instructions::load_r16::write_msb_sp_in_mem,
        ],
    },
    //LD SP, HL
    Instruction {
        opcode: 0xF9,
        micro_ops: &[
            instructions::load_r16::load_hl_in_sp,
            instructions::other::noop,
        ],
    },
    //PUSH, rr
    Instruction {
        opcode: 0xC5,
        micro_ops: &[
            instructions::other::sp_decr,
            instructions::load_r16::write_msb_bc_in_mem,
            instructions::load_r16::write_lsb_bc_in_mem,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD5,
        micro_ops: &[
            instructions::other::sp_decr,
            instructions::load_r16::write_msb_de_in_mem,
            instructions::load_r16::write_lsb_de_in_mem,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE5,
        micro_ops: &[
            instructions::other::sp_decr,
            instructions::load_r16::write_msb_hl_in_mem,
            instructions::load_r16::write_lsb_hl_in_mem,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF5,
        micro_ops: &[
            instructions::other::sp_decr,
            instructions::load_r16::write_msb_af_in_mem,
            instructions::load_r16::write_lsb_af_in_mem,
            instructions::other::noop,
        ],
    },
    //POP, rr
    Instruction {
        opcode: 0xC1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_bc,
        ],
    },
    Instruction {
        opcode: 0xD1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_de,
        ],
    },
    Instruction {
        opcode: 0xE1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_hl,
        ],
    },
    Instruction {
        opcode: 0xF1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_af,
        ],
    },
    //LD HL,SP+e
    Instruction {
        opcode: 0xF8,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load_r16::put_spe_in_h,
            instructions::load_r16::write_tmp_in_hl,
        ],
    },
    //LD B, r
    Instruction {
        opcode: 0x40,
        micro_ops: &[instructions::load_b::load_b_b],
    },
    Instruction {
        opcode: 0x41,
        micro_ops: &[instructions::load_b::load_b_c],
    },
    Instruction {
        opcode: 0x42,
        micro_ops: &[instructions::load_b::load_b_d],
    },
    Instruction {
        opcode: 0x43,
        micro_ops: &[instructions::load_b::load_b_e],
    },
    Instruction {
        opcode: 0x44,
        micro_ops: &[instructions::load_b::load_b_h],
    },
    Instruction {
        opcode: 0x45,
        micro_ops: &[instructions::load_b::load_b_l],
    },
    Instruction {
        opcode: 0x46,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl,
            instructions::load_b::load_b_tmp,
        ],
    },
    Instruction {
        opcode: 0x47,
        micro_ops: &[instructions::load_a::load_a_b],
    },
    //LD C, r
    Instruction {
        opcode: 0x48,
        micro_ops: &[instructions::load_c::load_c_b],
    },
    Instruction {
        opcode: 0x49,
        micro_ops: &[instructions::load_c::load_c_c],
    },
    Instruction {
        opcode: 0x4a,
        micro_ops: &[instructions::load_c::load_c_d],
    },
    Instruction {
        opcode: 0x4b,
        micro_ops: &[instructions::load_c::load_c_e],
    },
    Instruction {
        opcode: 0x4c,
        micro_ops: &[instructions::load_c::load_c_h],
    },
    Instruction {
        opcode: 0x4d,
        micro_ops: &[instructions::load_c::load_c_l],
    },
    Instruction {
        opcode: 0x4e,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl,
            instructions::load_c::load_c_tmp,
        ],
    },
    Instruction {
        opcode: 0x4f,
        micro_ops: &[instructions::load_a::load_a_c],
    },
    //LD D, r
    Instruction {
        opcode: 0x50,
        micro_ops: &[instructions::load_d::load_d_b],
    },
    Instruction {
        opcode: 0x51,
        micro_ops: &[instructions::load_d::load_d_c],
    },
    Instruction {
        opcode: 0x52,
        micro_ops: &[instructions::load_d::load_d_d],
    },
    Instruction {
        opcode: 0x53,
        micro_ops: &[instructions::load_d::load_d_e],
    },
    Instruction {
        opcode: 0x54,
        micro_ops: &[instructions::load_d::load_d_h],
    },
    Instruction {
        opcode: 0x55,
        micro_ops: &[instructions::load_d::load_d_l],
    },
    Instruction {
        opcode: 0x56,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl,
            instructions::load_d::load_d_tmp,
        ],
    },
    Instruction {
        opcode: 0x57,
        micro_ops: &[instructions::load_a::load_a_d],
    },
    //LD E, r
    Instruction {
        opcode: 0x58,
        micro_ops: &[instructions::load_e::load_e_b],
    },
    Instruction {
        opcode: 0x59,
        micro_ops: &[instructions::load_e::load_e_c],
    },
    Instruction {
        opcode: 0x5a,
        micro_ops: &[instructions::load_e::load_e_d],
    },
    Instruction {
        opcode: 0x5b,
        micro_ops: &[instructions::load_e::load_e_e],
    },
    Instruction {
        opcode: 0x5c,
        micro_ops: &[instructions::load_e::load_e_h],
    },
    Instruction {
        opcode: 0x5d,
        micro_ops: &[instructions::load_e::load_e_l],
    },
    Instruction {
        opcode: 0x5e,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl,
            instructions::load_e::load_e_tmp,
        ],
    },
    Instruction {
        opcode: 0x5f,
        micro_ops: &[instructions::load_a::load_a_e],
    },
    //LD H, r
    Instruction {
        opcode: 0x60,
        micro_ops: &[instructions::load_h::load_h_b],
    },
    Instruction {
        opcode: 0x61,
        micro_ops: &[instructions::load_h::load_h_c],
    },
    Instruction {
        opcode: 0x62,
        micro_ops: &[instructions::load_h::load_h_d],
    },
    Instruction {
        opcode: 0x63,
        micro_ops: &[instructions::load_h::load_h_e],
    },
    Instruction {
        opcode: 0x64,
        micro_ops: &[instructions::load_h::load_h_h],
    },
    Instruction {
        opcode: 0x65,
        micro_ops: &[instructions::load_h::load_h_l],
    },
    Instruction {
        opcode: 0x66,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl,
            instructions::load_h::load_h_tmp,
        ],
    },
    Instruction {
        opcode: 0x67,
        micro_ops: &[instructions::load_a::load_a_h],
    },
    //LD L, r
    Instruction {
        opcode: 0x68,
        micro_ops: &[instructions::load_l::load_l_b],
    },
    Instruction {
        opcode: 0x69,
        micro_ops: &[instructions::load_l::load_l_c],
    },
    Instruction {
        opcode: 0x6a,
        micro_ops: &[instructions::load_l::load_l_d],
    },
    Instruction {
        opcode: 0x6b,
        micro_ops: &[instructions::load_l::load_l_e],
    },
    Instruction {
        opcode: 0x6c,
        micro_ops: &[instructions::load_l::load_l_h],
    },
    Instruction {
        opcode: 0x6d,
        micro_ops: &[instructions::load_l::load_l_l],
    },
    Instruction {
        opcode: 0x6e,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl,
            instructions::load_l::load_l_tmp,
        ],
    },
    Instruction {
        opcode: 0x6f,
        micro_ops: &[instructions::load_a::load_a_l],
    },
    //LD HL, r
    Instruction {
        opcode: 0x70,
        micro_ops: &[instructions::load_b::load_hl_b, instructions::other::noop],
    },
    Instruction {
        opcode: 0x71,
        micro_ops: &[instructions::load_c::load_hl_c, instructions::other::noop],
    },
    Instruction {
        opcode: 0x72,
        micro_ops: &[instructions::load_d::load_hl_d, instructions::other::noop],
    },
    Instruction {
        opcode: 0x73,
        micro_ops: &[instructions::load_e::load_hl_e, instructions::other::noop],
    },
    Instruction {
        opcode: 0x74,
        micro_ops: &[instructions::load_h::load_hl_h, instructions::other::noop],
    },
    Instruction {
        opcode: 0x75,
        micro_ops: &[instructions::load_l::load_hl_l, instructions::other::noop],
    },
    Instruction {
        opcode: 0x77,
        micro_ops: &[instructions::load_a::load_hl_a, instructions::other::noop],
    },
    //HALT
    Instruction {
        opcode: 0x76,
        micro_ops: &[instructions::other::halt],
    },
    //LD A, r
    Instruction {
        opcode: 0x78,
        micro_ops: &[instructions::load_a::load_a_b],
    },
    Instruction {
        opcode: 0x79,
        micro_ops: &[instructions::load_a::load_a_c],
    },
    Instruction {
        opcode: 0x7a,
        micro_ops: &[instructions::load_a::load_a_d],
    },
    Instruction {
        opcode: 0x7b,
        micro_ops: &[instructions::load_a::load_a_e],
    },
    Instruction {
        opcode: 0x7c,
        micro_ops: &[instructions::load_a::load_a_h],
    },
    Instruction {
        opcode: 0x7d,
        micro_ops: &[instructions::load_a::load_a_l],
    },
    Instruction {
        opcode: 0x7e,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl,
            instructions::load_a::load_a_accu,
        ],
    },
    Instruction {
        opcode: 0x7f,
        micro_ops: &[instructions::load_a::load_a_a],
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
