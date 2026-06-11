use crate::cpu::defines::Cpu;
use crate::cpu::defines::Instruction;
use crate::cpu_def::*;
use crate::mmu::MemoryMapper;

pub fn build_cb_instructions<M: MemoryMapper>() -> Vec<Instruction<M>> {
    vec![
        Instruction {
            opcode: 0x00,
            micro_ops: vec![Cpu::rlc::<B>],
        },
        Instruction {
            opcode: 0x01,
            micro_ops: vec![Cpu::rlc::<C>],
        },
        Instruction {
            opcode: 0x02,
            micro_ops: vec![Cpu::rlc::<D>],
        },
        Instruction {
            opcode: 0x03,
            micro_ops: vec![Cpu::rlc::<E>],
        },
        Instruction {
            opcode: 0x04,
            micro_ops: vec![Cpu::rlc::<H>],
        },
        Instruction {
            opcode: 0x05,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rlc_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x06,
            micro_ops: vec![Cpu::rlc::<B>],
        },
        Instruction {
            opcode: 0x07,
            micro_ops: vec![Cpu::rlc::<A>],
        },
        //RRC
        Instruction {
            opcode: 0x08,
            micro_ops: vec![Cpu::rrc::<B>],
        },
        Instruction {
            opcode: 0x09,
            micro_ops: vec![Cpu::rrc::<C>],
        },
        Instruction {
            opcode: 0x0A,
            micro_ops: vec![Cpu::rrc::<D>],
        },
        Instruction {
            opcode: 0x0B,
            micro_ops: vec![Cpu::rrc::<E>],
        },
        Instruction {
            opcode: 0x0C,
            micro_ops: vec![Cpu::rrc::<H>],
        },
        Instruction {
            opcode: 0x0D,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rrc_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x0E,
            micro_ops: vec![Cpu::rrc::<B>],
        },
        Instruction {
            opcode: 0x0F,
            micro_ops: vec![Cpu::rrc::<A>],
        },
        //RL
        Instruction {
            opcode: 0x10,
            micro_ops: vec![Cpu::rl::<B>],
        },
        Instruction {
            opcode: 0x11,
            micro_ops: vec![Cpu::rl::<C>],
        },
        Instruction {
            opcode: 0x12,
            micro_ops: vec![Cpu::rl::<D>],
        },
        Instruction {
            opcode: 0x13,
            micro_ops: vec![Cpu::rl::<E>],
        },
        Instruction {
            opcode: 0x14,
            micro_ops: vec![Cpu::rl::<H>],
        },
        Instruction {
            opcode: 0x15,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rl_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x16,
            micro_ops: vec![Cpu::rl::<B>],
        },
        Instruction {
            opcode: 0x17,
            micro_ops: vec![Cpu::rl::<A>],
        },
        //RR
        Instruction {
            opcode: 0x18,
            micro_ops: vec![Cpu::rr::<B>],
        },
        Instruction {
            opcode: 0x19,
            micro_ops: vec![Cpu::rr::<C>],
        },
        Instruction {
            opcode: 0x1A,
            micro_ops: vec![Cpu::rr::<D>],
        },
        Instruction {
            opcode: 0x1B,
            micro_ops: vec![Cpu::rr::<E>],
        },
        Instruction {
            opcode: 0x1C,
            micro_ops: vec![Cpu::rr::<H>],
        },
        Instruction {
            opcode: 0x1D,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rr_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x1E,
            micro_ops: vec![Cpu::rr::<B>],
        },
        Instruction {
            opcode: 0x1F,
            micro_ops: vec![Cpu::rr::<A>],
        },
        //SLA
        Instruction {
            opcode: 0x20,
            micro_ops: vec![Cpu::sla::<B>],
        },
        Instruction {
            opcode: 0x21,
            micro_ops: vec![Cpu::sla::<C>],
        },
        Instruction {
            opcode: 0x22,
            micro_ops: vec![Cpu::sla::<D>],
        },
        Instruction {
            opcode: 0x23,
            micro_ops: vec![Cpu::sla::<E>],
        },
        Instruction {
            opcode: 0x24,
            micro_ops: vec![Cpu::sla::<H>],
        },
        Instruction {
            opcode: 0x25,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_sla_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x26,
            micro_ops: vec![Cpu::sla::<B>],
        },
        Instruction {
            opcode: 0x27,
            micro_ops: vec![Cpu::sla::<A>],
        },
        //SRA
        Instruction {
            opcode: 0x28,
            micro_ops: vec![Cpu::sra::<B>],
        },
        Instruction {
            opcode: 0x29,
            micro_ops: vec![Cpu::sra::<C>],
        },
        Instruction {
            opcode: 0x2A,
            micro_ops: vec![Cpu::sra::<D>],
        },
        Instruction {
            opcode: 0x2B,
            micro_ops: vec![Cpu::sra::<E>],
        },
        Instruction {
            opcode: 0x2C,
            micro_ops: vec![Cpu::sra::<H>],
        },
        Instruction {
            opcode: 0x2D,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_sra_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x2E,
            micro_ops: vec![Cpu::sra::<B>],
        },
        Instruction {
            opcode: 0x2F,
            micro_ops: vec![Cpu::sra::<A>],
        },
        //SWAP
        Instruction {
            opcode: 0x30,
            micro_ops: vec![Cpu::swap::<B>],
        },
        Instruction {
            opcode: 0x31,
            micro_ops: vec![Cpu::swap::<C>],
        },
        Instruction {
            opcode: 0x32,
            micro_ops: vec![Cpu::swap::<D>],
        },
        Instruction {
            opcode: 0x33,
            micro_ops: vec![Cpu::swap::<E>],
        },
        Instruction {
            opcode: 0x34,
            micro_ops: vec![Cpu::swap::<H>],
        },
        Instruction {
            opcode: 0x35,
            micro_ops: vec![Cpu::swap::<L>],
        },
        Instruction {
            opcode: 0x36,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_swap_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x37,
            micro_ops: vec![Cpu::swap::<A>],
        },
        //SRL
        Instruction {
            opcode: 0x38,
            micro_ops: vec![Cpu::srl::<B>],
        },
        Instruction {
            opcode: 0x39,
            micro_ops: vec![Cpu::srl::<C>],
        },
        Instruction {
            opcode: 0x3A,
            micro_ops: vec![Cpu::srl::<D>],
        },
        Instruction {
            opcode: 0x3B,
            micro_ops: vec![Cpu::srl::<E>],
        },
        Instruction {
            opcode: 0x3C,
            micro_ops: vec![Cpu::srl::<H>],
        },
        Instruction {
            opcode: 0x3D,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_srl_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x3E,
            micro_ops: vec![Cpu::srl::<B>],
        },
        Instruction {
            opcode: 0x3F,
            micro_ops: vec![Cpu::srl::<A>],
        },
        //BIT 0
        Instruction {
            opcode: 0x40,
            micro_ops: vec![Cpu::bit::<0, B>],
        },
        Instruction {
            opcode: 0x41,
            micro_ops: vec![Cpu::bit::<0, C>],
        },
        Instruction {
            opcode: 0x42,
            micro_ops: vec![Cpu::bit::<0, D>],
        },
        Instruction {
            opcode: 0x43,
            micro_ops: vec![Cpu::bit::<0, E>],
        },
        Instruction {
            opcode: 0x44,
            micro_ops: vec![Cpu::bit::<0, H>],
        },
        Instruction {
            opcode: 0x45,
            micro_ops: vec![Cpu::bit::<0, L>],
        },
        Instruction {
            opcode: 0x46,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<0, Z>],
        },
        Instruction {
            opcode: 0x47,
            micro_ops: vec![Cpu::bit::<0, A>],
        },
        //BIT 1
        Instruction {
            opcode: 0x48,
            micro_ops: vec![Cpu::bit::<1, B>],
        },
        Instruction {
            opcode: 0x49,
            micro_ops: vec![Cpu::bit::<1, C>],
        },
        Instruction {
            opcode: 0x4A,
            micro_ops: vec![Cpu::bit::<1, D>],
        },
        Instruction {
            opcode: 0x4B,
            micro_ops: vec![Cpu::bit::<1, E>],
        },
        Instruction {
            opcode: 0x4C,
            micro_ops: vec![Cpu::bit::<1, H>],
        },
        Instruction {
            opcode: 0x4D,
            micro_ops: vec![Cpu::bit::<1, L>],
        },
        Instruction {
            opcode: 0x4E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<1, Z>],
        },
        Instruction {
            opcode: 0x4F,
            micro_ops: vec![Cpu::bit::<1, A>],
        },
        //BIT 2
        Instruction {
            opcode: 0x50,
            micro_ops: vec![Cpu::bit::<2, B>],
        },
        Instruction {
            opcode: 0x51,
            micro_ops: vec![Cpu::bit::<2, C>],
        },
        Instruction {
            opcode: 0x52,
            micro_ops: vec![Cpu::bit::<2, D>],
        },
        Instruction {
            opcode: 0x53,
            micro_ops: vec![Cpu::bit::<2, E>],
        },
        Instruction {
            opcode: 0x54,
            micro_ops: vec![Cpu::bit::<2, H>],
        },
        Instruction {
            opcode: 0x55,
            micro_ops: vec![Cpu::bit::<2, L>],
        },
        Instruction {
            opcode: 0x56,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<2, Z>],
        },
        Instruction {
            opcode: 0x57,
            micro_ops: vec![Cpu::bit::<2, A>],
        },
        //BIT 3
        Instruction {
            opcode: 0x58,
            micro_ops: vec![Cpu::bit::<3, B>],
        },
        Instruction {
            opcode: 0x59,
            micro_ops: vec![Cpu::bit::<3, C>],
        },
        Instruction {
            opcode: 0x5A,
            micro_ops: vec![Cpu::bit::<3, D>],
        },
        Instruction {
            opcode: 0x5B,
            micro_ops: vec![Cpu::bit::<3, E>],
        },
        Instruction {
            opcode: 0x5C,
            micro_ops: vec![Cpu::bit::<3, H>],
        },
        Instruction {
            opcode: 0x5D,
            micro_ops: vec![Cpu::bit::<3, L>],
        },
        Instruction {
            opcode: 0x5E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<3, Z>],
        },
        Instruction {
            opcode: 0x5F,
            micro_ops: vec![Cpu::bit::<3, A>],
        },
        //BIT 4
        Instruction {
            opcode: 0x60,
            micro_ops: vec![Cpu::bit::<4, B>],
        },
        Instruction {
            opcode: 0x61,
            micro_ops: vec![Cpu::bit::<4, C>],
        },
        Instruction {
            opcode: 0x62,
            micro_ops: vec![Cpu::bit::<4, D>],
        },
        Instruction {
            opcode: 0x63,
            micro_ops: vec![Cpu::bit::<4, E>],
        },
        Instruction {
            opcode: 0x64,
            micro_ops: vec![Cpu::bit::<4, H>],
        },
        Instruction {
            opcode: 0x65,
            micro_ops: vec![Cpu::bit::<4, L>],
        },
        Instruction {
            opcode: 0x66,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<4, Z>],
        },
        Instruction {
            opcode: 0x67,
            micro_ops: vec![Cpu::bit::<4, A>],
        },
        //BIT 5
        Instruction {
            opcode: 0x68,
            micro_ops: vec![Cpu::bit::<5, B>],
        },
        Instruction {
            opcode: 0x69,
            micro_ops: vec![Cpu::bit::<5, C>],
        },
        Instruction {
            opcode: 0x6A,
            micro_ops: vec![Cpu::bit::<5, D>],
        },
        Instruction {
            opcode: 0x6B,
            micro_ops: vec![Cpu::bit::<5, E>],
        },
        Instruction {
            opcode: 0x6C,
            micro_ops: vec![Cpu::bit::<5, H>],
        },
        Instruction {
            opcode: 0x6D,
            micro_ops: vec![Cpu::bit::<5, L>],
        },
        Instruction {
            opcode: 0x6E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<5, Z>],
        },
        Instruction {
            opcode: 0x6F,
            micro_ops: vec![Cpu::bit::<5, A>],
        },
        //BIT 6
        Instruction {
            opcode: 0x70,
            micro_ops: vec![Cpu::bit::<6, B>],
        },
        Instruction {
            opcode: 0x71,
            micro_ops: vec![Cpu::bit::<6, C>],
        },
        Instruction {
            opcode: 0x72,
            micro_ops: vec![Cpu::bit::<6, D>],
        },
        Instruction {
            opcode: 0x73,
            micro_ops: vec![Cpu::bit::<6, E>],
        },
        Instruction {
            opcode: 0x74,
            micro_ops: vec![Cpu::bit::<6, H>],
        },
        Instruction {
            opcode: 0x75,
            micro_ops: vec![Cpu::bit::<6, L>],
        },
        Instruction {
            opcode: 0x76,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<6, Z>],
        },
        Instruction {
            opcode: 0x77,
            micro_ops: vec![Cpu::bit::<6, A>],
        },
        //BIT 7
        Instruction {
            opcode: 0x78,
            micro_ops: vec![Cpu::bit::<7, B>],
        },
        Instruction {
            opcode: 0x79,
            micro_ops: vec![Cpu::bit::<7, C>],
        },
        Instruction {
            opcode: 0x7A,
            micro_ops: vec![Cpu::bit::<7, D>],
        },
        Instruction {
            opcode: 0x7B,
            micro_ops: vec![Cpu::bit::<7, E>],
        },
        Instruction {
            opcode: 0x7C,
            micro_ops: vec![Cpu::bit::<7, H>],
        },
        Instruction {
            opcode: 0x7D,
            micro_ops: vec![Cpu::bit::<7, L>],
        },
        Instruction {
            opcode: 0x7E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<7, Z>],
        },
        Instruction {
            opcode: 0x7F,
            micro_ops: vec![Cpu::bit::<7, A>],
        },
        //RES 0
        Instruction {
            opcode: 0x80,
            micro_ops: vec![Cpu::res::<0, B>],
        },
        Instruction {
            opcode: 0x81,
            micro_ops: vec![Cpu::res::<0, C>],
        },
        Instruction {
            opcode: 0x82,
            micro_ops: vec![Cpu::res::<0, D>],
        },
        Instruction {
            opcode: 0x83,
            micro_ops: vec![Cpu::res::<0, E>],
        },
        Instruction {
            opcode: 0x84,
            micro_ops: vec![Cpu::res::<0, H>],
        },
        Instruction {
            opcode: 0x85,
            micro_ops: vec![Cpu::res::<0, L>],
        },
        Instruction {
            opcode: 0x86,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<0, HL, Z>,
                Cpu::res::<0, Z>,
            ],
        },
        Instruction {
            opcode: 0x87,
            micro_ops: vec![Cpu::res::<0, A>],
        },
        //RES 1
        Instruction {
            opcode: 0x88,
            micro_ops: vec![Cpu::res::<1, B>],
        },
        Instruction {
            opcode: 0x89,
            micro_ops: vec![Cpu::res::<1, C>],
        },
        Instruction {
            opcode: 0x8A,
            micro_ops: vec![Cpu::res::<1, D>],
        },
        Instruction {
            opcode: 0x8B,
            micro_ops: vec![Cpu::res::<1, E>],
        },
        Instruction {
            opcode: 0x8C,
            micro_ops: vec![Cpu::res::<1, H>],
        },
        Instruction {
            opcode: 0x8D,
            micro_ops: vec![Cpu::res::<1, L>],
        },
        Instruction {
            opcode: 0x8E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<1, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x8F,
            micro_ops: vec![Cpu::res::<1, A>],
        },
        //RES 2
        Instruction {
            opcode: 0x90,
            micro_ops: vec![Cpu::res::<2, B>],
        },
        Instruction {
            opcode: 0x91,
            micro_ops: vec![Cpu::res::<2, C>],
        },
        Instruction {
            opcode: 0x92,
            micro_ops: vec![Cpu::res::<2, D>],
        },
        Instruction {
            opcode: 0x93,
            micro_ops: vec![Cpu::res::<2, E>],
        },
        Instruction {
            opcode: 0x94,
            micro_ops: vec![Cpu::res::<2, H>],
        },
        Instruction {
            opcode: 0x95,
            micro_ops: vec![Cpu::res::<2, L>],
        },
        Instruction {
            opcode: 0x96,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<2, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x97,
            micro_ops: vec![Cpu::res::<2, A>],
        },
        //RES 3
        Instruction {
            opcode: 0x98,
            micro_ops: vec![Cpu::res::<3, B>],
        },
        Instruction {
            opcode: 0x99,
            micro_ops: vec![Cpu::res::<3, C>],
        },
        Instruction {
            opcode: 0x9A,
            micro_ops: vec![Cpu::res::<3, D>],
        },
        Instruction {
            opcode: 0x9B,
            micro_ops: vec![Cpu::res::<3, E>],
        },
        Instruction {
            opcode: 0x9C,
            micro_ops: vec![Cpu::res::<3, H>],
        },
        Instruction {
            opcode: 0x9D,
            micro_ops: vec![Cpu::res::<3, L>],
        },
        Instruction {
            opcode: 0x9E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<3, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x9F,
            micro_ops: vec![Cpu::res::<3, A>],
        },
        //RES 4
        Instruction {
            opcode: 0xA0,
            micro_ops: vec![Cpu::res::<4, B>],
        },
        Instruction {
            opcode: 0xA1,
            micro_ops: vec![Cpu::res::<4, C>],
        },
        Instruction {
            opcode: 0xA2,
            micro_ops: vec![Cpu::res::<4, D>],
        },
        Instruction {
            opcode: 0xA3,
            micro_ops: vec![Cpu::res::<4, E>],
        },
        Instruction {
            opcode: 0xA4,
            micro_ops: vec![Cpu::res::<4, H>],
        },
        Instruction {
            opcode: 0xA5,
            micro_ops: vec![Cpu::res::<4, L>],
        },
        Instruction {
            opcode: 0xA6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<4, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xA7,
            micro_ops: vec![Cpu::res::<4, A>],
        },
        //RES 5
        Instruction {
            opcode: 0xA8,
            micro_ops: vec![Cpu::res::<5, B>],
        },
        Instruction {
            opcode: 0xA9,
            micro_ops: vec![Cpu::res::<5, C>],
        },
        Instruction {
            opcode: 0xAA,
            micro_ops: vec![Cpu::res::<5, D>],
        },
        Instruction {
            opcode: 0xAB,
            micro_ops: vec![Cpu::res::<5, E>],
        },
        Instruction {
            opcode: 0xAC,
            micro_ops: vec![Cpu::res::<5, H>],
        },
        Instruction {
            opcode: 0xAD,
            micro_ops: vec![Cpu::res::<5, L>],
        },
        Instruction {
            opcode: 0xAE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<5, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xAF,
            micro_ops: vec![Cpu::res::<5, A>],
        },
        //RES 6
        Instruction {
            opcode: 0xB0,
            micro_ops: vec![Cpu::res::<6, B>],
        },
        Instruction {
            opcode: 0xB1,
            micro_ops: vec![Cpu::res::<6, C>],
        },
        Instruction {
            opcode: 0xB2,
            micro_ops: vec![Cpu::res::<6, D>],
        },
        Instruction {
            opcode: 0xB3,
            micro_ops: vec![Cpu::res::<6, E>],
        },
        Instruction {
            opcode: 0xB4,
            micro_ops: vec![Cpu::res::<6, H>],
        },
        Instruction {
            opcode: 0xB5,
            micro_ops: vec![Cpu::res::<6, L>],
        },
        Instruction {
            opcode: 0xB6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<6, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xB7,
            micro_ops: vec![Cpu::res::<6, A>],
        },
        //RES 7
        Instruction {
            opcode: 0xB8,
            micro_ops: vec![Cpu::res::<7, B>],
        },
        Instruction {
            opcode: 0xB9,
            micro_ops: vec![Cpu::res::<7, C>],
        },
        Instruction {
            opcode: 0xBA,
            micro_ops: vec![Cpu::res::<7, D>],
        },
        Instruction {
            opcode: 0xBB,
            micro_ops: vec![Cpu::res::<7, E>],
        },
        Instruction {
            opcode: 0xBC,
            micro_ops: vec![Cpu::res::<7, H>],
        },
        Instruction {
            opcode: 0xBD,
            micro_ops: vec![Cpu::res::<7, L>],
        },
        Instruction {
            opcode: 0xBE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<7, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xBF,
            micro_ops: vec![Cpu::res::<7, A>],
        },
        //SET 0
        Instruction {
            opcode: 0xC0,
            micro_ops: vec![Cpu::set::<0, B>],
        },
        Instruction {
            opcode: 0xC1,
            micro_ops: vec![Cpu::set::<0, C>],
        },
        Instruction {
            opcode: 0xC2,
            micro_ops: vec![Cpu::set::<0, D>],
        },
        Instruction {
            opcode: 0xC3,
            micro_ops: vec![Cpu::set::<0, E>],
        },
        Instruction {
            opcode: 0xC4,
            micro_ops: vec![Cpu::set::<0, H>],
        },
        Instruction {
            opcode: 0xC5,
            micro_ops: vec![Cpu::set::<0, L>],
        },
        Instruction {
            opcode: 0xC6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<0, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC7,
            micro_ops: vec![Cpu::set::<0, A>],
        },
        //SET 1
        Instruction {
            opcode: 0xC8,
            micro_ops: vec![Cpu::set::<1, B>],
        },
        Instruction {
            opcode: 0xC9,
            micro_ops: vec![Cpu::set::<1, C>],
        },
        Instruction {
            opcode: 0xCA,
            micro_ops: vec![Cpu::set::<1, D>],
        },
        Instruction {
            opcode: 0xCB,
            micro_ops: vec![Cpu::set::<1, E>],
        },
        Instruction {
            opcode: 0xCC,
            micro_ops: vec![Cpu::set::<1, H>],
        },
        Instruction {
            opcode: 0xCD,
            micro_ops: vec![Cpu::set::<1, L>],
        },
        Instruction {
            opcode: 0xCE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<1, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCF,
            micro_ops: vec![Cpu::set::<1, A>],
        },
        //SET 2
        Instruction {
            opcode: 0xD0,
            micro_ops: vec![Cpu::set::<2, B>],
        },
        Instruction {
            opcode: 0xD1,
            micro_ops: vec![Cpu::set::<2, C>],
        },
        Instruction {
            opcode: 0xD2,
            micro_ops: vec![Cpu::set::<2, D>],
        },
        Instruction {
            opcode: 0xD3,
            micro_ops: vec![Cpu::set::<2, E>],
        },
        Instruction {
            opcode: 0xD4,
            micro_ops: vec![Cpu::set::<2, H>],
        },
        Instruction {
            opcode: 0xD5,
            micro_ops: vec![Cpu::set::<2, L>],
        },
        Instruction {
            opcode: 0xD6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<2, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD7,
            micro_ops: vec![Cpu::set::<2, A>],
        },
        //SET 3
        Instruction {
            opcode: 0xD8,
            micro_ops: vec![Cpu::set::<3, B>],
        },
        Instruction {
            opcode: 0xD9,
            micro_ops: vec![Cpu::set::<3, C>],
        },
        Instruction {
            opcode: 0xDA,
            micro_ops: vec![Cpu::set::<3, D>],
        },
        Instruction {
            opcode: 0xDB,
            micro_ops: vec![Cpu::set::<3, E>],
        },
        Instruction {
            opcode: 0xDC,
            micro_ops: vec![Cpu::set::<3, H>],
        },
        Instruction {
            opcode: 0xDD,
            micro_ops: vec![Cpu::set::<3, L>],
        },
        Instruction {
            opcode: 0xDE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<3, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xDF,
            micro_ops: vec![Cpu::set::<3, A>],
        },
        //SET 4
        Instruction {
            opcode: 0xE0,
            micro_ops: vec![Cpu::set::<4, B>],
        },
        Instruction {
            opcode: 0xE1,
            micro_ops: vec![Cpu::set::<4, C>],
        },
        Instruction {
            opcode: 0xE2,
            micro_ops: vec![Cpu::set::<4, D>],
        },
        Instruction {
            opcode: 0xE3,
            micro_ops: vec![Cpu::set::<4, E>],
        },
        Instruction {
            opcode: 0xE4,
            micro_ops: vec![Cpu::set::<4, H>],
        },
        Instruction {
            opcode: 0xE5,
            micro_ops: vec![Cpu::set::<4, L>],
        },
        Instruction {
            opcode: 0xE6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<4, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xE7,
            micro_ops: vec![Cpu::set::<4, A>],
        },
        //SET 5
        Instruction {
            opcode: 0xE8,
            micro_ops: vec![Cpu::set::<5, B>],
        },
        Instruction {
            opcode: 0xE9,
            micro_ops: vec![Cpu::set::<5, C>],
        },
        Instruction {
            opcode: 0xEA,
            micro_ops: vec![Cpu::set::<5, D>],
        },
        Instruction {
            opcode: 0xEB,
            micro_ops: vec![Cpu::set::<5, E>],
        },
        Instruction {
            opcode: 0xEC,
            micro_ops: vec![Cpu::set::<5, H>],
        },
        Instruction {
            opcode: 0xED,
            micro_ops: vec![Cpu::set::<5, L>],
        },
        Instruction {
            opcode: 0xEE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<5, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xEF,
            micro_ops: vec![Cpu::set::<5, A>],
        },
        //SET 6
        Instruction {
            opcode: 0xF0,
            micro_ops: vec![Cpu::set::<6, B>],
        },
        Instruction {
            opcode: 0xF1,
            micro_ops: vec![Cpu::set::<6, C>],
        },
        Instruction {
            opcode: 0xF2,
            micro_ops: vec![Cpu::set::<6, D>],
        },
        Instruction {
            opcode: 0xF3,
            micro_ops: vec![Cpu::set::<6, E>],
        },
        Instruction {
            opcode: 0xF4,
            micro_ops: vec![Cpu::set::<6, H>],
        },
        Instruction {
            opcode: 0xF5,
            micro_ops: vec![Cpu::set::<6, L>],
        },
        Instruction {
            opcode: 0xF6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<6, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xF7,
            micro_ops: vec![Cpu::set::<6, A>],
        },
        //SET 7
        Instruction {
            opcode: 0xF8,
            micro_ops: vec![Cpu::set::<7, B>],
        },
        Instruction {
            opcode: 0xF9,
            micro_ops: vec![Cpu::set::<7, C>],
        },
        Instruction {
            opcode: 0xFA,
            micro_ops: vec![Cpu::set::<7, D>],
        },
        Instruction {
            opcode: 0xFB,
            micro_ops: vec![Cpu::set::<7, E>],
        },
        Instruction {
            opcode: 0xFC,
            micro_ops: vec![Cpu::set::<7, H>],
        },
        Instruction {
            opcode: 0xFD,
            micro_ops: vec![Cpu::set::<7, L>],
        },
        Instruction {
            opcode: 0xFE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<7, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xFF,
            micro_ops: vec![Cpu::set::<7, A>],
        },
    ]
}

impl<M: MemoryMapper> Cpu<M> {
    pub fn decode_cb(&mut self, bus: &mut M) {
        let pc = Self::get_r16::<PC>(self);
        let cb_opcode = bus.read_byte(pc);

        self.set_r16::<PC>(pc.wrapping_add(1));
        self.queue = self.cb_instructions[cb_opcode as usize].micro_ops.clone();
        self.op_index = 0;
    }
    pub fn debug_step(&mut self, _instruction: u8, _bus: &mut M) -> bool {
        todo!()
    }
}
