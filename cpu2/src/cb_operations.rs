use crate::Cpu;
use crate::defines::Instruction;
use crate::defines::MicroOp;
use crate::implemenation::*;
use crate::instructions;

pub static CB_DISPATCH: [Option<&'static [MicroOp]>; 256] = build_dispatch();

pub static CB_INSTRUCTIONS: &[Instruction] = &[
    Instruction {
        opcode: 0x00,
        micro_ops: &[instructions::other::rlc::<B>],
    },
    Instruction {
        opcode: 0x01,
        micro_ops: &[instructions::other::rlc::<C>],
    },
    Instruction {
        opcode: 0x02,
        micro_ops: &[instructions::other::rlc::<D>],
    },
    Instruction {
        opcode: 0x03,
        micro_ops: &[instructions::other::rlc::<E>],
    },
    Instruction {
        opcode: 0x04,
        micro_ops: &[instructions::other::rlc::<H>],
    },
    Instruction {
        opcode: 0x05,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_rlc_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x06,
        micro_ops: &[instructions::other::rlc::<B>],
    },
    Instruction {
        opcode: 0x07,
        micro_ops: &[instructions::other::rlc::<A>],
    },
    //RRC
    Instruction {
        opcode: 0x08,
        micro_ops: &[instructions::other::rrc::<B>],
    },
    Instruction {
        opcode: 0x09,
        micro_ops: &[instructions::other::rrc::<C>],
    },
    Instruction {
        opcode: 0x0A,
        micro_ops: &[instructions::other::rrc::<D>],
    },
    Instruction {
        opcode: 0x0B,
        micro_ops: &[instructions::other::rrc::<E>],
    },
    Instruction {
        opcode: 0x0C,
        micro_ops: &[instructions::other::rrc::<H>],
    },
    Instruction {
        opcode: 0x0D,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_rrc_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x0E,
        micro_ops: &[instructions::other::rrc::<B>],
    },
    Instruction {
        opcode: 0x0F,
        micro_ops: &[instructions::other::rrc::<A>],
    },
    //RL
    Instruction {
        opcode: 0x10,
        micro_ops: &[instructions::other::rl::<B>],
    },
    Instruction {
        opcode: 0x11,
        micro_ops: &[instructions::other::rl::<C>],
    },
    Instruction {
        opcode: 0x12,
        micro_ops: &[instructions::other::rl::<D>],
    },
    Instruction {
        opcode: 0x13,
        micro_ops: &[instructions::other::rl::<E>],
    },
    Instruction {
        opcode: 0x14,
        micro_ops: &[instructions::other::rl::<H>],
    },
    Instruction {
        opcode: 0x15,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_rl_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x16,
        micro_ops: &[instructions::other::rl::<B>],
    },
    Instruction {
        opcode: 0x17,
        micro_ops: &[instructions::other::rl::<A>],
    },
    //RR
    Instruction {
        opcode: 0x18,
        micro_ops: &[instructions::other::rr::<B>],
    },
    Instruction {
        opcode: 0x19,
        micro_ops: &[instructions::other::rr::<C>],
    },
    Instruction {
        opcode: 0x1A,
        micro_ops: &[instructions::other::rr::<D>],
    },
    Instruction {
        opcode: 0x1B,
        micro_ops: &[instructions::other::rr::<E>],
    },
    Instruction {
        opcode: 0x1C,
        micro_ops: &[instructions::other::rr::<H>],
    },
    Instruction {
        opcode: 0x1D,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_rr_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x1E,
        micro_ops: &[instructions::other::rr::<B>],
    },
    Instruction {
        opcode: 0x1F,
        micro_ops: &[instructions::other::rr::<A>],
    },
    //SLA
    Instruction {
        opcode: 0x20,
        micro_ops: &[instructions::other::sla::<B>],
    },
    Instruction {
        opcode: 0x21,
        micro_ops: &[instructions::other::sla::<C>],
    },
    Instruction {
        opcode: 0x22,
        micro_ops: &[instructions::other::sla::<D>],
    },
    Instruction {
        opcode: 0x23,
        micro_ops: &[instructions::other::sla::<E>],
    },
    Instruction {
        opcode: 0x24,
        micro_ops: &[instructions::other::sla::<H>],
    },
    Instruction {
        opcode: 0x25,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_sla_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x26,
        micro_ops: &[instructions::other::sla::<B>],
    },
    Instruction {
        opcode: 0x27,
        micro_ops: &[instructions::other::sla::<A>],
    },
    //SRA
    Instruction {
        opcode: 0x28,
        micro_ops: &[instructions::other::sra::<B>],
    },
    Instruction {
        opcode: 0x29,
        micro_ops: &[instructions::other::sra::<C>],
    },
    Instruction {
        opcode: 0x2A,
        micro_ops: &[instructions::other::sra::<D>],
    },
    Instruction {
        opcode: 0x2B,
        micro_ops: &[instructions::other::sra::<E>],
    },
    Instruction {
        opcode: 0x2C,
        micro_ops: &[instructions::other::sra::<H>],
    },
    Instruction {
        opcode: 0x2D,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_sra_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x2E,
        micro_ops: &[instructions::other::sra::<B>],
    },
    Instruction {
        opcode: 0x2F,
        micro_ops: &[instructions::other::sra::<A>],
    },
    //SWAP
    Instruction {
        opcode: 0x30,
        micro_ops: &[instructions::other::swap::<B>],
    },
    Instruction {
        opcode: 0x31,
        micro_ops: &[instructions::other::swap::<C>],
    },
    Instruction {
        opcode: 0x32,
        micro_ops: &[instructions::other::swap::<D>],
    },
    Instruction {
        opcode: 0x33,
        micro_ops: &[instructions::other::swap::<E>],
    },
    Instruction {
        opcode: 0x34,
        micro_ops: &[instructions::other::swap::<H>],
    },
    Instruction {
        opcode: 0x35,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_swap_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x36,
        micro_ops: &[instructions::other::swap::<B>],
    },
    Instruction {
        opcode: 0x37,
        micro_ops: &[instructions::other::swap::<A>],
    },
    //SRL
    Instruction {
        opcode: 0x38,
        micro_ops: &[instructions::other::srl::<B>],
    },
    Instruction {
        opcode: 0x39,
        micro_ops: &[instructions::other::srl::<C>],
    },
    Instruction {
        opcode: 0x3A,
        micro_ops: &[instructions::other::srl::<D>],
    },
    Instruction {
        opcode: 0x3B,
        micro_ops: &[instructions::other::srl::<E>],
    },
    Instruction {
        opcode: 0x3C,
        micro_ops: &[instructions::other::srl::<H>],
    },
    Instruction {
        opcode: 0x3D,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_srl_mem::<HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x3E,
        micro_ops: &[instructions::other::srl::<B>],
    },
    Instruction {
        opcode: 0x3F,
        micro_ops: &[instructions::other::srl::<A>],
    },
    //BIT 0
    Instruction {
        opcode: 0x40,
        micro_ops: &[instructions::other::bit::<0, B>],
    },
    Instruction {
        opcode: 0x41,
        micro_ops: &[instructions::other::bit::<0, C>],
    },
    Instruction {
        opcode: 0x42,
        micro_ops: &[instructions::other::bit::<0, D>],
    },
    Instruction {
        opcode: 0x43,
        micro_ops: &[instructions::other::bit::<0, E>],
    },
    Instruction {
        opcode: 0x44,
        micro_ops: &[instructions::other::bit::<0, H>],
    },
    Instruction {
        opcode: 0x45,
        micro_ops: &[instructions::other::bit::<0, L>],
    },
    Instruction {
        opcode: 0x46,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<0, Z>,
        ],
    },
    Instruction {
        opcode: 0x47,
        micro_ops: &[instructions::other::bit::<0, A>],
    },
    //BIT 1
    Instruction {
        opcode: 0x48,
        micro_ops: &[instructions::other::bit::<1, B>],
    },
    Instruction {
        opcode: 0x49,
        micro_ops: &[instructions::other::bit::<1, C>],
    },
    Instruction {
        opcode: 0x4A,
        micro_ops: &[instructions::other::bit::<1, D>],
    },
    Instruction {
        opcode: 0x4B,
        micro_ops: &[instructions::other::bit::<1, E>],
    },
    Instruction {
        opcode: 0x4C,
        micro_ops: &[instructions::other::bit::<1, H>],
    },
    Instruction {
        opcode: 0x4D,
        micro_ops: &[instructions::other::bit::<1, L>],
    },
    Instruction {
        opcode: 0x4E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<1, Z>,
        ],
    },
    Instruction {
        opcode: 0x4F,
        micro_ops: &[instructions::other::bit::<1, A>],
    },
    //BIT 2
    Instruction {
        opcode: 0x50,
        micro_ops: &[instructions::other::bit::<2, B>],
    },
    Instruction {
        opcode: 0x51,
        micro_ops: &[instructions::other::bit::<2, C>],
    },
    Instruction {
        opcode: 0x52,
        micro_ops: &[instructions::other::bit::<2, D>],
    },
    Instruction {
        opcode: 0x53,
        micro_ops: &[instructions::other::bit::<2, E>],
    },
    Instruction {
        opcode: 0x54,
        micro_ops: &[instructions::other::bit::<2, H>],
    },
    Instruction {
        opcode: 0x55,
        micro_ops: &[instructions::other::bit::<2, L>],
    },
    Instruction {
        opcode: 0x56,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<2, Z>,
        ],
    },
    Instruction {
        opcode: 0x57,
        micro_ops: &[instructions::other::bit::<2, A>],
    },
    //BIT 3
    Instruction {
        opcode: 0x58,
        micro_ops: &[instructions::other::bit::<3, B>],
    },
    Instruction {
        opcode: 0x59,
        micro_ops: &[instructions::other::bit::<3, C>],
    },
    Instruction {
        opcode: 0x5A,
        micro_ops: &[instructions::other::bit::<3, D>],
    },
    Instruction {
        opcode: 0x5B,
        micro_ops: &[instructions::other::bit::<3, E>],
    },
    Instruction {
        opcode: 0x5C,
        micro_ops: &[instructions::other::bit::<3, H>],
    },
    Instruction {
        opcode: 0x5D,
        micro_ops: &[instructions::other::bit::<3, L>],
    },
    Instruction {
        opcode: 0x5E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<3, Z>,
        ],
    },
    Instruction {
        opcode: 0x5F,
        micro_ops: &[instructions::other::bit::<3, A>],
    },
    //BIT 4
    Instruction {
        opcode: 0x60,
        micro_ops: &[instructions::other::bit::<4, B>],
    },
    Instruction {
        opcode: 0x61,
        micro_ops: &[instructions::other::bit::<4, C>],
    },
    Instruction {
        opcode: 0x62,
        micro_ops: &[instructions::other::bit::<4, D>],
    },
    Instruction {
        opcode: 0x63,
        micro_ops: &[instructions::other::bit::<4, E>],
    },
    Instruction {
        opcode: 0x64,
        micro_ops: &[instructions::other::bit::<4, H>],
    },
    Instruction {
        opcode: 0x65,
        micro_ops: &[instructions::other::bit::<4, L>],
    },
    Instruction {
        opcode: 0x66,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<4, Z>,
        ],
    },
    Instruction {
        opcode: 0x67,
        micro_ops: &[instructions::other::bit::<4, A>],
    },
    //BIT 5
    Instruction {
        opcode: 0x68,
        micro_ops: &[instructions::other::bit::<5, B>],
    },
    Instruction {
        opcode: 0x69,
        micro_ops: &[instructions::other::bit::<5, C>],
    },
    Instruction {
        opcode: 0x6A,
        micro_ops: &[instructions::other::bit::<5, D>],
    },
    Instruction {
        opcode: 0x6B,
        micro_ops: &[instructions::other::bit::<5, E>],
    },
    Instruction {
        opcode: 0x6C,
        micro_ops: &[instructions::other::bit::<5, H>],
    },
    Instruction {
        opcode: 0x6D,
        micro_ops: &[instructions::other::bit::<5, L>],
    },
    Instruction {
        opcode: 0x6E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<5, Z>,
        ],
    },
    Instruction {
        opcode: 0x6F,
        micro_ops: &[instructions::other::bit::<5, A>],
    },
    //BIT 6
    Instruction {
        opcode: 0x70,
        micro_ops: &[instructions::other::bit::<6, B>],
    },
    Instruction {
        opcode: 0x71,
        micro_ops: &[instructions::other::bit::<6, C>],
    },
    Instruction {
        opcode: 0x72,
        micro_ops: &[instructions::other::bit::<6, D>],
    },
    Instruction {
        opcode: 0x73,
        micro_ops: &[instructions::other::bit::<6, E>],
    },
    Instruction {
        opcode: 0x74,
        micro_ops: &[instructions::other::bit::<6, H>],
    },
    Instruction {
        opcode: 0x75,
        micro_ops: &[instructions::other::bit::<6, L>],
    },
    Instruction {
        opcode: 0x76,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<6, Z>,
        ],
    },
    Instruction {
        opcode: 0x77,
        micro_ops: &[instructions::other::bit::<6, A>],
    },
    //BIT 7
    Instruction {
        opcode: 0x78,
        micro_ops: &[instructions::other::bit::<7, B>],
    },
    Instruction {
        opcode: 0x79,
        micro_ops: &[instructions::other::bit::<7, C>],
    },
    Instruction {
        opcode: 0x7A,
        micro_ops: &[instructions::other::bit::<7, D>],
    },
    Instruction {
        opcode: 0x7B,
        micro_ops: &[instructions::other::bit::<7, E>],
    },
    Instruction {
        opcode: 0x7C,
        micro_ops: &[instructions::other::bit::<7, H>],
    },
    Instruction {
        opcode: 0x7D,
        micro_ops: &[instructions::other::bit::<7, L>],
    },
    Instruction {
        opcode: 0x7E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::bit::<7, Z>,
        ],
    },
    Instruction {
        opcode: 0x7F,
        micro_ops: &[instructions::other::bit::<7, A>],
    },
    //RES 0
    Instruction {
        opcode: 0x80,
        micro_ops: &[instructions::other::res::<0, B>],
    },
    Instruction {
        opcode: 0x81,
        micro_ops: &[instructions::other::res::<0, C>],
    },
    Instruction {
        opcode: 0x82,
        micro_ops: &[instructions::other::res::<0, D>],
    },
    Instruction {
        opcode: 0x83,
        micro_ops: &[instructions::other::res::<0, E>],
    },
    Instruction {
        opcode: 0x84,
        micro_ops: &[instructions::other::res::<0, H>],
    },
    Instruction {
        opcode: 0x85,
        micro_ops: &[instructions::other::res::<0, L>],
    },
    Instruction {
        opcode: 0x86,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<0, HL, Z>,
            instructions::other::res::<0, Z>,
        ],
    },
    Instruction {
        opcode: 0x87,
        micro_ops: &[instructions::other::res::<0, A>],
    },
    //RES 1
    Instruction {
        opcode: 0x88,
        micro_ops: &[instructions::other::res::<1, B>],
    },
    Instruction {
        opcode: 0x89,
        micro_ops: &[instructions::other::res::<1, C>],
    },
    Instruction {
        opcode: 0x8A,
        micro_ops: &[instructions::other::res::<1, D>],
    },
    Instruction {
        opcode: 0x8B,
        micro_ops: &[instructions::other::res::<1, E>],
    },
    Instruction {
        opcode: 0x8C,
        micro_ops: &[instructions::other::res::<1, H>],
    },
    Instruction {
        opcode: 0x8D,
        micro_ops: &[instructions::other::res::<1, L>],
    },
    Instruction {
        opcode: 0x8E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<1, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x8F,
        micro_ops: &[instructions::other::res::<1, A>],
    },
    //RES 2
    Instruction {
        opcode: 0x90,
        micro_ops: &[instructions::other::res::<2, B>],
    },
    Instruction {
        opcode: 0x91,
        micro_ops: &[instructions::other::res::<2, C>],
    },
    Instruction {
        opcode: 0x92,
        micro_ops: &[instructions::other::res::<2, D>],
    },
    Instruction {
        opcode: 0x93,
        micro_ops: &[instructions::other::res::<2, E>],
    },
    Instruction {
        opcode: 0x94,
        micro_ops: &[instructions::other::res::<2, H>],
    },
    Instruction {
        opcode: 0x95,
        micro_ops: &[instructions::other::res::<2, L>],
    },
    Instruction {
        opcode: 0x96,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<2, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x97,
        micro_ops: &[instructions::other::res::<2, A>],
    },
    //RES 3
    Instruction {
        opcode: 0x98,
        micro_ops: &[instructions::other::res::<3, B>],
    },
    Instruction {
        opcode: 0x99,
        micro_ops: &[instructions::other::res::<3, C>],
    },
    Instruction {
        opcode: 0x9A,
        micro_ops: &[instructions::other::res::<3, D>],
    },
    Instruction {
        opcode: 0x9B,
        micro_ops: &[instructions::other::res::<3, E>],
    },
    Instruction {
        opcode: 0x9C,
        micro_ops: &[instructions::other::res::<3, H>],
    },
    Instruction {
        opcode: 0x9D,
        micro_ops: &[instructions::other::res::<3, L>],
    },
    Instruction {
        opcode: 0x9E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<3, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x9F,
        micro_ops: &[instructions::other::res::<3, A>],
    },
    //RES 4
    Instruction {
        opcode: 0xA0,
        micro_ops: &[instructions::other::res::<4, B>],
    },
    Instruction {
        opcode: 0xA1,
        micro_ops: &[instructions::other::res::<4, C>],
    },
    Instruction {
        opcode: 0xA2,
        micro_ops: &[instructions::other::res::<4, D>],
    },
    Instruction {
        opcode: 0xA3,
        micro_ops: &[instructions::other::res::<4, E>],
    },
    Instruction {
        opcode: 0xA4,
        micro_ops: &[instructions::other::res::<4, H>],
    },
    Instruction {
        opcode: 0xA5,
        micro_ops: &[instructions::other::res::<4, L>],
    },
    Instruction {
        opcode: 0xA6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<4, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xA7,
        micro_ops: &[instructions::other::res::<4, A>],
    },
    //RES 5
    Instruction {
        opcode: 0xA8,
        micro_ops: &[instructions::other::res::<5, B>],
    },
    Instruction {
        opcode: 0xA9,
        micro_ops: &[instructions::other::res::<5, C>],
    },
    Instruction {
        opcode: 0xAA,
        micro_ops: &[instructions::other::res::<5, D>],
    },
    Instruction {
        opcode: 0xAB,
        micro_ops: &[instructions::other::res::<5, E>],
    },
    Instruction {
        opcode: 0xAC,
        micro_ops: &[instructions::other::res::<5, H>],
    },
    Instruction {
        opcode: 0xAD,
        micro_ops: &[instructions::other::res::<5, L>],
    },
    Instruction {
        opcode: 0xAE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<5, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xAF,
        micro_ops: &[instructions::other::res::<5, A>],
    },
    //RES 6
    Instruction {
        opcode: 0xB0,
        micro_ops: &[instructions::other::res::<6, B>],
    },
    Instruction {
        opcode: 0xB1,
        micro_ops: &[instructions::other::res::<6, C>],
    },
    Instruction {
        opcode: 0xB2,
        micro_ops: &[instructions::other::res::<6, D>],
    },
    Instruction {
        opcode: 0xB3,
        micro_ops: &[instructions::other::res::<6, E>],
    },
    Instruction {
        opcode: 0xB4,
        micro_ops: &[instructions::other::res::<6, H>],
    },
    Instruction {
        opcode: 0xB5,
        micro_ops: &[instructions::other::res::<6, L>],
    },
    Instruction {
        opcode: 0xB6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<6, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xB7,
        micro_ops: &[instructions::other::res::<6, A>],
    },
    //RES 7
    Instruction {
        opcode: 0xB8,
        micro_ops: &[instructions::other::res::<7, B>],
    },
    Instruction {
        opcode: 0xB9,
        micro_ops: &[instructions::other::res::<7, C>],
    },
    Instruction {
        opcode: 0xBA,
        micro_ops: &[instructions::other::res::<7, D>],
    },
    Instruction {
        opcode: 0xBB,
        micro_ops: &[instructions::other::res::<7, E>],
    },
    Instruction {
        opcode: 0xBC,
        micro_ops: &[instructions::other::res::<7, H>],
    },
    Instruction {
        opcode: 0xBD,
        micro_ops: &[instructions::other::res::<7, L>],
    },
    Instruction {
        opcode: 0xBE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_res_mem::<7, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xBF,
        micro_ops: &[instructions::other::res::<7, A>],
    },
    //SET 0
    Instruction {
        opcode: 0xC0,
        micro_ops: &[instructions::other::set::<0, B>],
    },
    Instruction {
        opcode: 0xC1,
        micro_ops: &[instructions::other::set::<0, C>],
    },
    Instruction {
        opcode: 0xC2,
        micro_ops: &[instructions::other::set::<0, D>],
    },
    Instruction {
        opcode: 0xC3,
        micro_ops: &[instructions::other::set::<0, E>],
    },
    Instruction {
        opcode: 0xC4,
        micro_ops: &[instructions::other::set::<0, H>],
    },
    Instruction {
        opcode: 0xC5,
        micro_ops: &[instructions::other::set::<0, L>],
    },
    Instruction {
        opcode: 0xC6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<0, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xC7,
        micro_ops: &[instructions::other::set::<0, A>],
    },
    //SET 1
    Instruction {
        opcode: 0xC8,
        micro_ops: &[instructions::other::set::<1, B>],
    },
    Instruction {
        opcode: 0xC9,
        micro_ops: &[instructions::other::set::<1, C>],
    },
    Instruction {
        opcode: 0xCA,
        micro_ops: &[instructions::other::set::<1, D>],
    },
    Instruction {
        opcode: 0xCB,
        micro_ops: &[instructions::other::set::<1, E>],
    },
    Instruction {
        opcode: 0xCC,
        micro_ops: &[instructions::other::set::<1, H>],
    },
    Instruction {
        opcode: 0xCD,
        micro_ops: &[instructions::other::set::<1, L>],
    },
    Instruction {
        opcode: 0xCE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<1, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xCF,
        micro_ops: &[instructions::other::set::<1, A>],
    },
    //SET 2
    Instruction {
        opcode: 0xD0,
        micro_ops: &[instructions::other::set::<2, B>],
    },
    Instruction {
        opcode: 0xD1,
        micro_ops: &[instructions::other::set::<2, C>],
    },
    Instruction {
        opcode: 0xD2,
        micro_ops: &[instructions::other::set::<2, D>],
    },
    Instruction {
        opcode: 0xD3,
        micro_ops: &[instructions::other::set::<2, E>],
    },
    Instruction {
        opcode: 0xD4,
        micro_ops: &[instructions::other::set::<2, H>],
    },
    Instruction {
        opcode: 0xD5,
        micro_ops: &[instructions::other::set::<2, L>],
    },
    Instruction {
        opcode: 0xD6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<2, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD7,
        micro_ops: &[instructions::other::set::<2, A>],
    },
    //SET 3
    Instruction {
        opcode: 0xD8,
        micro_ops: &[instructions::other::set::<3, B>],
    },
    Instruction {
        opcode: 0xD9,
        micro_ops: &[instructions::other::set::<3, C>],
    },
    Instruction {
        opcode: 0xDA,
        micro_ops: &[instructions::other::set::<3, D>],
    },
    Instruction {
        opcode: 0xDB,
        micro_ops: &[instructions::other::set::<3, E>],
    },
    Instruction {
        opcode: 0xDC,
        micro_ops: &[instructions::other::set::<3, H>],
    },
    Instruction {
        opcode: 0xDD,
        micro_ops: &[instructions::other::set::<3, L>],
    },
    Instruction {
        opcode: 0xDE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<3, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xDF,
        micro_ops: &[instructions::other::set::<3, A>],
    },
    //SET 4
    Instruction {
        opcode: 0xE0,
        micro_ops: &[instructions::other::set::<4, B>],
    },
    Instruction {
        opcode: 0xE1,
        micro_ops: &[instructions::other::set::<4, C>],
    },
    Instruction {
        opcode: 0xE2,
        micro_ops: &[instructions::other::set::<4, D>],
    },
    Instruction {
        opcode: 0xE3,
        micro_ops: &[instructions::other::set::<4, E>],
    },
    Instruction {
        opcode: 0xE4,
        micro_ops: &[instructions::other::set::<4, H>],
    },
    Instruction {
        opcode: 0xE5,
        micro_ops: &[instructions::other::set::<4, L>],
    },
    Instruction {
        opcode: 0xE6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<4, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE7,
        micro_ops: &[instructions::other::set::<4, A>],
    },
    //SET 5
    Instruction {
        opcode: 0xE8,
        micro_ops: &[instructions::other::set::<5, B>],
    },
    Instruction {
        opcode: 0xE9,
        micro_ops: &[instructions::other::set::<5, C>],
    },
    Instruction {
        opcode: 0xEA,
        micro_ops: &[instructions::other::set::<5, D>],
    },
    Instruction {
        opcode: 0xEB,
        micro_ops: &[instructions::other::set::<5, E>],
    },
    Instruction {
        opcode: 0xEC,
        micro_ops: &[instructions::other::set::<5, H>],
    },
    Instruction {
        opcode: 0xED,
        micro_ops: &[instructions::other::set::<5, L>],
    },
    Instruction {
        opcode: 0xEE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<5, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xEF,
        micro_ops: &[instructions::other::set::<5, A>],
    },
    //SET 6
    Instruction {
        opcode: 0xF0,
        micro_ops: &[instructions::other::set::<6, B>],
    },
    Instruction {
        opcode: 0xF1,
        micro_ops: &[instructions::other::set::<6, C>],
    },
    Instruction {
        opcode: 0xF2,
        micro_ops: &[instructions::other::set::<6, D>],
    },
    Instruction {
        opcode: 0xF3,
        micro_ops: &[instructions::other::set::<6, E>],
    },
    Instruction {
        opcode: 0xF4,
        micro_ops: &[instructions::other::set::<6, H>],
    },
    Instruction {
        opcode: 0xF5,
        micro_ops: &[instructions::other::set::<6, L>],
    },
    Instruction {
        opcode: 0xF6,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<6, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF7,
        micro_ops: &[instructions::other::set::<6, A>],
    },
    //SET 7
    Instruction {
        opcode: 0xF8,
        micro_ops: &[instructions::other::set::<7, B>],
    },
    Instruction {
        opcode: 0xF9,
        micro_ops: &[instructions::other::set::<7, C>],
    },
    Instruction {
        opcode: 0xFA,
        micro_ops: &[instructions::other::set::<7, D>],
    },
    Instruction {
        opcode: 0xFB,
        micro_ops: &[instructions::other::set::<7, E>],
    },
    Instruction {
        opcode: 0xFC,
        micro_ops: &[instructions::other::set::<7, H>],
    },
    Instruction {
        opcode: 0xFD,
        micro_ops: &[instructions::other::set::<7, L>],
    },
    Instruction {
        opcode: 0xFE,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::other::write_set_mem::<7, HL, Z>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xFF,
        micro_ops: &[instructions::other::set::<7, A>],
    },
];

pub fn decode_cb(cpu: &mut Cpu) {
    let pc = cpu.get_r16::<PC>();

    let cb_opcode = *cpu
        .instructions_list
        .get(pc as usize)
        .expect("Could not fetch CB opcode");

    cpu.set_r16::<PC>(pc.wrapping_add(1));
    cpu.queue = CB_DISPATCH[cb_opcode as usize].expect("Unknown CB opcode");
    cpu.op_index = 0;
}

const fn build_dispatch() -> [Option<&'static [MicroOp]>; 256] {
    let mut table = [None; 256];
    let mut i = 0;
    while i < CB_INSTRUCTIONS.len() {
        let opcode = CB_INSTRUCTIONS[i].opcode as usize;
        table[opcode] = Some(CB_INSTRUCTIONS[i].micro_ops);
        i += 1;
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defines::{Cpu, Flag, r8};
    use crate::flags::FlagsOps;
    use crate::implemenation::{A, B, C, D, E, H, L, HL};

    fn cpu_cb(cb_opcode: u8) -> Cpu {
        let mut list = vec![0xCB, cb_opcode];
        list.resize(32, 0x00);
        let mut regs = [0u8; 14];
        regs[r8::PcP] = 2;
        Cpu {
            queue: CB_DISPATCH[cb_opcode as usize].expect("unknown CB opcode"),
            r8: regs,
            flags: 0,
            instructions_list: list,
            op_index: 0,
            bus: [0; 0x10000],
        }
    }

    // === RLC (0x00-0x07): rotate left circular, bit7 → carry, wraps to bit0 ===

    #[test]
    fn rlc_b_0x00() {
        let mut c = cpu_cb(0x00);
        c.set_r8::<B>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x03);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Subtract));
        assert!(!c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn rlc_b_zero_0x00() {
        let mut c = cpu_cb(0x00);
        c.set_r8::<B>(0x00);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(!c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rlc_c_0x01() {
        let mut c = cpu_cb(0x01);
        c.set_r8::<C>(0x40);
        c.tick();
        assert_eq!(c.get_r8::<C>(), 0x80);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rlc_d_0x02() {
        let mut c = cpu_cb(0x02);
        c.set_r8::<D>(0x40);
        c.tick();
        assert_eq!(c.get_r8::<D>(), 0x80);
    }

    #[test]
    fn rlc_e_0x03() {
        let mut c = cpu_cb(0x03);
        c.set_r8::<E>(0x01);
        c.tick();
        assert_eq!(c.get_r8::<E>(), 0x02);
    }

    #[test]
    fn rlc_h_0x04() {
        let mut c = cpu_cb(0x04);
        c.set_r8::<H>(0x01);
        c.tick();
        assert_eq!(c.get_r8::<H>(), 0x02);
    }

    #[test]
    fn rlc_l_0x05() {
        let mut c = cpu_cb(0x05);
        c.set_r8::<L>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x03);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rlc_hl_0x06() {
        let mut c = cpu_cb(0x06);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x81;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x03);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rlc_a_0x07() {
        let mut c = cpu_cb(0x07);
        c.set_r8::<A>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<A>(), 0x03);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    // === RRC (0x08-0x0F): rotate right circular, bit0 → carry, wraps to bit7 ===

    #[test]
    fn rrc_b_0x08() {
        let mut c = cpu_cb(0x08);
        c.set_r8::<B>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0xC0);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rrc_b_zero_0x08() {
        let mut c = cpu_cb(0x08);
        c.set_r8::<B>(0x00);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(!c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rrc_l_0x0d() {
        let mut c = cpu_cb(0x0D);
        c.set_r8::<L>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0xC0);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rrc_hl_0x0e() {
        let mut c = cpu_cb(0x0E);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x81;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0xC0);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    // === RL (0x10-0x17): rotate left through carry, bit7 → carry, old_carry → bit0 ===

    #[test]
    fn rl_b_carry_out_0x10() {
        let mut c = cpu_cb(0x10);
        c.set_r8::<B>(0x80);
        c.flags.set_flag(Flag::Carry, false);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rl_b_carry_in_0x10() {
        let mut c = cpu_cb(0x10);
        c.set_r8::<B>(0x01);
        c.flags.set_flag(Flag::Carry, true);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x03);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rl_l_0x15() {
        let mut c = cpu_cb(0x15);
        c.set_r8::<L>(0x01);
        c.flags.set_flag(Flag::Carry, false);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x02);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rl_hl_0x16() {
        let mut c = cpu_cb(0x16);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x80;
        c.flags.set_flag(Flag::Carry, false);
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x00);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Zero));
    }

    // === RR (0x18-0x1F): rotate right through carry, bit0 → carry, old_carry → bit7 ===

    #[test]
    fn rr_b_carry_out_0x18() {
        let mut c = cpu_cb(0x18);
        c.set_r8::<B>(0x01);
        c.flags.set_flag(Flag::Carry, false);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn rr_b_carry_in_0x18() {
        let mut c = cpu_cb(0x18);
        c.set_r8::<B>(0x80);
        c.flags.set_flag(Flag::Carry, true);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0xC0);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rr_l_0x1d() {
        let mut c = cpu_cb(0x1D);
        c.set_r8::<L>(0x02);
        c.flags.set_flag(Flag::Carry, false);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x01);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn rr_hl_0x1e() {
        let mut c = cpu_cb(0x1E);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x01;
        c.flags.set_flag(Flag::Carry, false);
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x00);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Zero));
    }

    // === SLA (0x20-0x27): shift left arithmetic, bit7 → carry, bit0 = 0 ===

    #[test]
    fn sla_b_0x20() {
        let mut c = cpu_cb(0x20);
        c.set_r8::<B>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x02);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn sla_l_0x25() {
        let mut c = cpu_cb(0x25);
        c.set_r8::<L>(0x40);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x80);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn sla_hl_0x26() {
        let mut c = cpu_cb(0x26);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x80;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x00);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(c.flags.get_flag(Flag::Zero));
    }

    // === SRA (0x28-0x2F): shift right arithmetic, bit0 → carry, bit7 preserved ===

    #[test]
    fn sra_b_0x28() {
        let mut c = cpu_cb(0x28);
        c.set_r8::<B>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0xC0);
        assert!(c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn sra_l_0x2d() {
        let mut c = cpu_cb(0x2D);
        c.set_r8::<L>(0x80);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0xC0);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn sra_hl_0x2e() {
        let mut c = cpu_cb(0x2E);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x82;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0xC1);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    // === SWAP (0x30-0x37): swap upper and lower nibbles ===

    #[test]
    fn swap_b_0x30() {
        let mut c = cpu_cb(0x30);
        c.set_r8::<B>(0xAB);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0xBA);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn swap_b_zero_0x30() {
        let mut c = cpu_cb(0x30);
        c.set_r8::<B>(0x00);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x00);
        assert!(c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn swap_l_0x35() {
        let mut c = cpu_cb(0x35);
        c.set_r8::<L>(0x12);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x21);
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn swap_hl_0x36() {
        let mut c = cpu_cb(0x36);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x34;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x43);
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    // === SRL (0x38-0x3F): shift right logical, bit0 → carry, bit7 = 0 ===

    #[test]
    fn srl_b_0x38() {
        let mut c = cpu_cb(0x38);
        c.set_r8::<B>(0x81);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x40);
        assert!(c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn srl_l_0x3d() {
        let mut c = cpu_cb(0x3D);
        c.set_r8::<L>(0x02);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x01);
        assert!(!c.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn srl_hl_0x3e() {
        let mut c = cpu_cb(0x3E);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x80;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x40);
        assert!(!c.flags.get_flag(Flag::Carry));
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    // === BIT (0x40-0x7F): test bit n, sets Z/H, clears N, does not modify register ===

    #[test]
    fn bit_0_b_set_0x40() {
        let mut c = cpu_cb(0x40);
        c.set_r8::<B>(0x01);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x01);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::HalfCarry));
        assert!(!c.flags.get_flag(Flag::Subtract));
    }

    #[test]
    fn bit_0_b_clear_0x40() {
        let mut c = cpu_cb(0x40);
        c.set_r8::<B>(0x00);
        c.tick();
        assert!(c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn bit_0_l_0x45() {
        let mut c = cpu_cb(0x45);
        c.set_r8::<L>(0x01);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x01);
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn bit_0_hl_0x46() {
        let mut c = cpu_cb(0x46);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x01;
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x01);
        assert!(!c.flags.get_flag(Flag::Zero));
        assert!(c.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn bit_7_b_set_0x78() {
        let mut c = cpu_cb(0x78);
        c.set_r8::<B>(0x80);
        c.tick();
        assert!(!c.flags.get_flag(Flag::Zero));
    }

    #[test]
    fn bit_7_b_clear_0x78() {
        let mut c = cpu_cb(0x78);
        c.set_r8::<B>(0x7F);
        c.tick();
        assert!(c.flags.get_flag(Flag::Zero));
    }

    // === RES (0x80-0xBF): clear bit n, no flags modified ===

    #[test]
    fn res_0_b_0x80() {
        let mut c = cpu_cb(0x80);
        c.set_r8::<B>(0xFF);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0xFE);
    }

    #[test]
    fn res_0_l_0x85() {
        let mut c = cpu_cb(0x85);
        c.set_r8::<L>(0xFF);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0xFE);
    }

    #[test]
    fn res_0_hl_0x86() {
        let mut c = cpu_cb(0x86);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0xFF;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0xFE);
    }

    #[test]
    fn res_7_a_0xbf() {
        let mut c = cpu_cb(0xBF);
        c.set_r8::<A>(0xFF);
        c.tick();
        assert_eq!(c.get_r8::<A>(), 0x7F);
    }

    #[test]
    fn res_3_hl_0x9e() {
        let mut c = cpu_cb(0x9E);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0xFF;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0xF7);
    }

    // === SET (0xC0-0xFF): set bit n, no flags modified ===

    #[test]
    fn set_0_b_0xc0() {
        let mut c = cpu_cb(0xC0);
        c.set_r8::<B>(0x00);
        c.tick();
        assert_eq!(c.get_r8::<B>(), 0x01);
    }

    #[test]
    fn set_0_l_0xc5() {
        let mut c = cpu_cb(0xC5);
        c.set_r8::<L>(0x00);
        c.tick();
        assert_eq!(c.get_r8::<L>(), 0x01);
    }

    #[test]
    fn set_0_hl_0xc6() {
        let mut c = cpu_cb(0xC6);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x00;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x01);
    }

    #[test]
    fn set_7_a_0xff() {
        let mut c = cpu_cb(0xFF);
        c.set_r8::<A>(0x00);
        c.tick();
        assert_eq!(c.get_r8::<A>(), 0x80);
    }

    #[test]
    fn set_3_hl_0xde() {
        let mut c = cpu_cb(0xDE);
        c.set_r16::<HL>(0x2000);
        c.bus[0x2000] = 0x00;
        c.tick();
        c.tick();
        c.tick();
        assert_eq!(c.bus[0x2000], 0x08);
    }
}
