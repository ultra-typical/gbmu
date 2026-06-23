use crate::cpu::defines::Cpu;
use crate::cpu::defines::Instruction;
use crate::cpu::*;
use crate::mmu::MemoryMapper;

pub fn build_cb_instructions<M: MemoryMapper>() -> Vec<Instruction<M>> {
    vec![
        Instruction {
            name: "RLC B".to_string(),
            opcode: 0x00,
            micro_ops: vec![Cpu::rlc::<B>],
        },
        Instruction {
            name: "RLC C".to_string(),
            opcode: 0x01,
            micro_ops: vec![Cpu::rlc::<C>],
        },
        Instruction {
            name: "RLC D".to_string(),
            opcode: 0x02,
            micro_ops: vec![Cpu::rlc::<D>],
        },
        Instruction {
            name: "RLC E".to_string(),
            opcode: 0x03,
            micro_ops: vec![Cpu::rlc::<E>],
        },
        Instruction {
            name: "RLC H".to_string(),
            opcode: 0x04,
            micro_ops: vec![Cpu::rlc::<H>],
        },
        Instruction {
            name: "RLC L".to_string(),
            opcode: 0x05,
            micro_ops: vec![Cpu::rlc::<L>],
        },
        Instruction {
            name: "RLC (HL)".to_string(),
            opcode: 0x06,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rlc_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RLC A".to_string(),
            opcode: 0x07,
            micro_ops: vec![Cpu::rlc::<A>],
        },
        Instruction {
            name: "RRC B".to_string(),
            opcode: 0x08,
            micro_ops: vec![Cpu::rrc::<B>],
        },
        Instruction {
            name: "RRC C".to_string(),
            opcode: 0x09,
            micro_ops: vec![Cpu::rrc::<C>],
        },
        Instruction {
            name: "RRC D".to_string(),
            opcode: 0x0A,
            micro_ops: vec![Cpu::rrc::<D>],
        },
        Instruction {
            name: "RRC E".to_string(),
            opcode: 0x0B,
            micro_ops: vec![Cpu::rrc::<E>],
        },
        Instruction {
            name: "RRC H".to_string(),
            opcode: 0x0C,
            micro_ops: vec![Cpu::rrc::<H>],
        },
        Instruction {
            name: "RRC L".to_string(),
            opcode: 0x0D,
            micro_ops: vec![Cpu::rrc::<L>],
        },
        Instruction {
            name: "RRC (HL)".to_string(),
            opcode: 0x0E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rrc_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RRC A".to_string(),
            opcode: 0x0F,
            micro_ops: vec![Cpu::rrc::<A>],
        },
        Instruction {
            name: "RL B".to_string(),
            opcode: 0x10,
            micro_ops: vec![Cpu::rl::<B>],
        },
        Instruction {
            name: "RL C".to_string(),
            opcode: 0x11,
            micro_ops: vec![Cpu::rl::<C>],
        },
        Instruction {
            name: "RL D".to_string(),
            opcode: 0x12,
            micro_ops: vec![Cpu::rl::<D>],
        },
        Instruction {
            name: "RL E".to_string(),
            opcode: 0x13,
            micro_ops: vec![Cpu::rl::<E>],
        },
        Instruction {
            name: "RL H".to_string(),
            opcode: 0x14,
            micro_ops: vec![Cpu::rl::<H>],
        },
        Instruction {
            name: "RL L".to_string(),
            opcode: 0x15,
            micro_ops: vec![Cpu::rl::<L>],
        },
        Instruction {
            name: "RL (HL)".to_string(),
            opcode: 0x16,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rl_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RL A".to_string(),
            opcode: 0x17,
            micro_ops: vec![Cpu::rl::<A>],
        },
        Instruction {
            name: "RR B".to_string(),
            opcode: 0x18,
            micro_ops: vec![Cpu::rr::<B>],
        },
        Instruction {
            name: "RR C".to_string(),
            opcode: 0x19,
            micro_ops: vec![Cpu::rr::<C>],
        },
        Instruction {
            name: "RR D".to_string(),
            opcode: 0x1A,
            micro_ops: vec![Cpu::rr::<D>],
        },
        Instruction {
            name: "RR E".to_string(),
            opcode: 0x1B,
            micro_ops: vec![Cpu::rr::<E>],
        },
        Instruction {
            name: "RR H".to_string(),
            opcode: 0x1C,
            micro_ops: vec![Cpu::rr::<H>],
        },
        Instruction {
            name: "RR L".to_string(),
            opcode: 0x1D,
            micro_ops: vec![Cpu::rr::<L>],
        },
        Instruction {
            name: "RR (HL)".to_string(),
            opcode: 0x1E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_rr_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RR A".to_string(),
            opcode: 0x1F,
            micro_ops: vec![Cpu::rr::<A>],
        },
        Instruction {
            name: "SLA B".to_string(),
            opcode: 0x20,
            micro_ops: vec![Cpu::sla::<B>],
        },
        Instruction {
            name: "SLA C".to_string(),
            opcode: 0x21,
            micro_ops: vec![Cpu::sla::<C>],
        },
        Instruction {
            name: "SLA D".to_string(),
            opcode: 0x22,
            micro_ops: vec![Cpu::sla::<D>],
        },
        Instruction {
            name: "SLA E".to_string(),
            opcode: 0x23,
            micro_ops: vec![Cpu::sla::<E>],
        },
        Instruction {
            name: "SLA H".to_string(),
            opcode: 0x24,
            micro_ops: vec![Cpu::sla::<H>],
        },
        Instruction {
            name: "SLA L".to_string(),
            opcode: 0x25,
            micro_ops: vec![Cpu::sla::<L>],
        },
        Instruction {
            name: "SLA (HL)".to_string(),
            opcode: 0x26,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_sla_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SLA A".to_string(),
            opcode: 0x27,
            micro_ops: vec![Cpu::sla::<A>],
        },
        Instruction {
            name: "SRA B".to_string(),
            opcode: 0x28,
            micro_ops: vec![Cpu::sra::<B>],
        },
        Instruction {
            name: "SRA C".to_string(),
            opcode: 0x29,
            micro_ops: vec![Cpu::sra::<C>],
        },
        Instruction {
            name: "SRA D".to_string(),
            opcode: 0x2A,
            micro_ops: vec![Cpu::sra::<D>],
        },
        Instruction {
            name: "SRA E".to_string(),
            opcode: 0x2B,
            micro_ops: vec![Cpu::sra::<E>],
        },
        Instruction {
            name: "SRA H".to_string(),
            opcode: 0x2C,
            micro_ops: vec![Cpu::sra::<H>],
        },
        Instruction {
            name: "SRA L".to_string(),
            opcode: 0x2D,
            micro_ops: vec![Cpu::sra::<L>],
        },
        Instruction {
            name: "SRA (HL)".to_string(),
            opcode: 0x2E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_sra_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SRA A".to_string(),
            opcode: 0x2F,
            micro_ops: vec![Cpu::sra::<A>],
        },
        Instruction {
            name: "SWAP B".to_string(),
            opcode: 0x30,
            micro_ops: vec![Cpu::swap::<B>],
        },
        Instruction {
            name: "SWAP C".to_string(),
            opcode: 0x31,
            micro_ops: vec![Cpu::swap::<C>],
        },
        Instruction {
            name: "SWAP D".to_string(),
            opcode: 0x32,
            micro_ops: vec![Cpu::swap::<D>],
        },
        Instruction {
            name: "SWAP E".to_string(),
            opcode: 0x33,
            micro_ops: vec![Cpu::swap::<E>],
        },
        Instruction {
            name: "SWAP H".to_string(),
            opcode: 0x34,
            micro_ops: vec![Cpu::swap::<H>],
        },
        Instruction {
            name: "SWAP L".to_string(),
            opcode: 0x35,
            micro_ops: vec![Cpu::swap::<L>],
        },
        Instruction {
            name: "SWAP (HL)".to_string(),
            opcode: 0x36,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_swap_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SWAP A".to_string(),
            opcode: 0x37,
            micro_ops: vec![Cpu::swap::<A>],
        },
        Instruction {
            name: "SRL B".to_string(),
            opcode: 0x38,
            micro_ops: vec![Cpu::srl::<B>],
        },
        Instruction {
            name: "SRL C".to_string(),
            opcode: 0x39,
            micro_ops: vec![Cpu::srl::<C>],
        },
        Instruction {
            name: "SRL D".to_string(),
            opcode: 0x3A,
            micro_ops: vec![Cpu::srl::<D>],
        },
        Instruction {
            name: "SRL E".to_string(),
            opcode: 0x3B,
            micro_ops: vec![Cpu::srl::<E>],
        },
        Instruction {
            name: "SRL H".to_string(),
            opcode: 0x3C,
            micro_ops: vec![Cpu::srl::<H>],
        },
        Instruction {
            name: "SRL L".to_string(),
            opcode: 0x3D,
            micro_ops: vec![Cpu::srl::<L>],
        },
        Instruction {
            name: "SRL (HL)".to_string(),
            opcode: 0x3E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_srl_mem::<HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SRL A".to_string(),
            opcode: 0x3F,
            micro_ops: vec![Cpu::srl::<A>],
        },
        Instruction {
            name: "BIT 0,B".to_string(),
            opcode: 0x40,
            micro_ops: vec![Cpu::bit::<0, B>],
        },
        Instruction {
            name: "BIT 0,C".to_string(),
            opcode: 0x41,
            micro_ops: vec![Cpu::bit::<0, C>],
        },
        Instruction {
            name: "BIT 0,D".to_string(),
            opcode: 0x42,
            micro_ops: vec![Cpu::bit::<0, D>],
        },
        Instruction {
            name: "BIT 0,E".to_string(),
            opcode: 0x43,
            micro_ops: vec![Cpu::bit::<0, E>],
        },
        Instruction {
            name: "BIT 0,H".to_string(),
            opcode: 0x44,
            micro_ops: vec![Cpu::bit::<0, H>],
        },
        Instruction {
            name: "BIT 0,L".to_string(),
            opcode: 0x45,
            micro_ops: vec![Cpu::bit::<0, L>],
        },
        Instruction {
            name: "BIT 0,(HL)".to_string(),
            opcode: 0x46,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<0, Z>],
        },
        Instruction {
            name: "BIT 0,A".to_string(),
            opcode: 0x47,
            micro_ops: vec![Cpu::bit::<0, A>],
        },
        Instruction {
            name: "BIT 1,B".to_string(),
            opcode: 0x48,
            micro_ops: vec![Cpu::bit::<1, B>],
        },
        Instruction {
            name: "BIT 1,C".to_string(),
            opcode: 0x49,
            micro_ops: vec![Cpu::bit::<1, C>],
        },
        Instruction {
            name: "BIT 1,D".to_string(),
            opcode: 0x4A,
            micro_ops: vec![Cpu::bit::<1, D>],
        },
        Instruction {
            name: "BIT 1,E".to_string(),
            opcode: 0x4B,
            micro_ops: vec![Cpu::bit::<1, E>],
        },
        Instruction {
            name: "BIT 1,H".to_string(),
            opcode: 0x4C,
            micro_ops: vec![Cpu::bit::<1, H>],
        },
        Instruction {
            name: "BIT 1,L".to_string(),
            opcode: 0x4D,
            micro_ops: vec![Cpu::bit::<1, L>],
        },
        Instruction {
            name: "BIT 1,(HL)".to_string(),
            opcode: 0x4E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<1, Z>],
        },
        Instruction {
            name: "BIT 1,A".to_string(),
            opcode: 0x4F,
            micro_ops: vec![Cpu::bit::<1, A>],
        },
        Instruction {
            name: "BIT 2,B".to_string(),
            opcode: 0x50,
            micro_ops: vec![Cpu::bit::<2, B>],
        },
        Instruction {
            name: "BIT 2,C".to_string(),
            opcode: 0x51,
            micro_ops: vec![Cpu::bit::<2, C>],
        },
        Instruction {
            name: "BIT 2,D".to_string(),
            opcode: 0x52,
            micro_ops: vec![Cpu::bit::<2, D>],
        },
        Instruction {
            name: "BIT 2,E".to_string(),
            opcode: 0x53,
            micro_ops: vec![Cpu::bit::<2, E>],
        },
        Instruction {
            name: "BIT 2,H".to_string(),
            opcode: 0x54,
            micro_ops: vec![Cpu::bit::<2, H>],
        },
        Instruction {
            name: "BIT 2,L".to_string(),
            opcode: 0x55,
            micro_ops: vec![Cpu::bit::<2, L>],
        },
        Instruction {
            name: "BIT 2,(HL)".to_string(),
            opcode: 0x56,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<2, Z>],
        },
        Instruction {
            name: "BIT 2,A".to_string(),
            opcode: 0x57,
            micro_ops: vec![Cpu::bit::<2, A>],
        },
        Instruction {
            name: "BIT 3,B".to_string(),
            opcode: 0x58,
            micro_ops: vec![Cpu::bit::<3, B>],
        },
        Instruction {
            name: "BIT 3,C".to_string(),
            opcode: 0x59,
            micro_ops: vec![Cpu::bit::<3, C>],
        },
        Instruction {
            name: "BIT 3,D".to_string(),
            opcode: 0x5A,
            micro_ops: vec![Cpu::bit::<3, D>],
        },
        Instruction {
            name: "BIT 3,E".to_string(),
            opcode: 0x5B,
            micro_ops: vec![Cpu::bit::<3, E>],
        },
        Instruction {
            name: "BIT 3,H".to_string(),
            opcode: 0x5C,
            micro_ops: vec![Cpu::bit::<3, H>],
        },
        Instruction {
            name: "BIT 3,L".to_string(),
            opcode: 0x5D,
            micro_ops: vec![Cpu::bit::<3, L>],
        },
        Instruction {
            name: "BIT 3,(HL)".to_string(),
            opcode: 0x5E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<3, Z>],
        },
        Instruction {
            name: "BIT 3,A".to_string(),
            opcode: 0x5F,
            micro_ops: vec![Cpu::bit::<3, A>],
        },
        Instruction {
            name: "BIT 4,B".to_string(),
            opcode: 0x60,
            micro_ops: vec![Cpu::bit::<4, B>],
        },
        Instruction {
            name: "BIT 4,C".to_string(),
            opcode: 0x61,
            micro_ops: vec![Cpu::bit::<4, C>],
        },
        Instruction {
            name: "BIT 4,D".to_string(),
            opcode: 0x62,
            micro_ops: vec![Cpu::bit::<4, D>],
        },
        Instruction {
            name: "BIT 4,E".to_string(),
            opcode: 0x63,
            micro_ops: vec![Cpu::bit::<4, E>],
        },
        Instruction {
            name: "BIT 4,H".to_string(),
            opcode: 0x64,
            micro_ops: vec![Cpu::bit::<4, H>],
        },
        Instruction {
            name: "BIT 4,L".to_string(),
            opcode: 0x65,
            micro_ops: vec![Cpu::bit::<4, L>],
        },
        Instruction {
            name: "BIT 4,(HL)".to_string(),
            opcode: 0x66,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<4, Z>],
        },
        Instruction {
            name: "BIT 4,A".to_string(),
            opcode: 0x67,
            micro_ops: vec![Cpu::bit::<4, A>],
        },
        Instruction {
            name: "BIT 5,B".to_string(),
            opcode: 0x68,
            micro_ops: vec![Cpu::bit::<5, B>],
        },
        Instruction {
            name: "BIT 5,C".to_string(),
            opcode: 0x69,
            micro_ops: vec![Cpu::bit::<5, C>],
        },
        Instruction {
            name: "BIT 5,D".to_string(),
            opcode: 0x6A,
            micro_ops: vec![Cpu::bit::<5, D>],
        },
        Instruction {
            name: "BIT 5,E".to_string(),
            opcode: 0x6B,
            micro_ops: vec![Cpu::bit::<5, E>],
        },
        Instruction {
            name: "BIT 5,H".to_string(),
            opcode: 0x6C,
            micro_ops: vec![Cpu::bit::<5, H>],
        },
        Instruction {
            name: "BIT 5,L".to_string(),
            opcode: 0x6D,
            micro_ops: vec![Cpu::bit::<5, L>],
        },
        Instruction {
            name: "BIT 5,(HL)".to_string(),
            opcode: 0x6E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<5, Z>],
        },
        Instruction {
            name: "BIT 5,A".to_string(),
            opcode: 0x6F,
            micro_ops: vec![Cpu::bit::<5, A>],
        },
        Instruction {
            name: "BIT 6,B".to_string(),
            opcode: 0x70,
            micro_ops: vec![Cpu::bit::<6, B>],
        },
        Instruction {
            name: "BIT 6,C".to_string(),
            opcode: 0x71,
            micro_ops: vec![Cpu::bit::<6, C>],
        },
        Instruction {
            name: "BIT 6,D".to_string(),
            opcode: 0x72,
            micro_ops: vec![Cpu::bit::<6, D>],
        },
        Instruction {
            name: "BIT 6,E".to_string(),
            opcode: 0x73,
            micro_ops: vec![Cpu::bit::<6, E>],
        },
        Instruction {
            name: "BIT 6,H".to_string(),
            opcode: 0x74,
            micro_ops: vec![Cpu::bit::<6, H>],
        },
        Instruction {
            name: "BIT 6,L".to_string(),
            opcode: 0x75,
            micro_ops: vec![Cpu::bit::<6, L>],
        },
        Instruction {
            name: "BIT 6,(HL)".to_string(),
            opcode: 0x76,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<6, Z>],
        },
        Instruction {
            name: "BIT 6,A".to_string(),
            opcode: 0x77,
            micro_ops: vec![Cpu::bit::<6, A>],
        },
        Instruction {
            name: "BIT 7,B".to_string(),
            opcode: 0x78,
            micro_ops: vec![Cpu::bit::<7, B>],
        },
        Instruction {
            name: "BIT 7,C".to_string(),
            opcode: 0x79,
            micro_ops: vec![Cpu::bit::<7, C>],
        },
        Instruction {
            name: "BIT 7,D".to_string(),
            opcode: 0x7A,
            micro_ops: vec![Cpu::bit::<7, D>],
        },
        Instruction {
            name: "BIT 7,E".to_string(),
            opcode: 0x7B,
            micro_ops: vec![Cpu::bit::<7, E>],
        },
        Instruction {
            name: "BIT 7,H".to_string(),
            opcode: 0x7C,
            micro_ops: vec![Cpu::bit::<7, H>],
        },
        Instruction {
            name: "BIT 7,L".to_string(),
            opcode: 0x7D,
            micro_ops: vec![Cpu::bit::<7, L>],
        },
        Instruction {
            name: "BIT 7,(HL)".to_string(),
            opcode: 0x7E,
            micro_ops: vec![Cpu::read_memory::<HL, Z>, Cpu::bit::<7, Z>],
        },
        Instruction {
            name: "BIT 7,A".to_string(),
            opcode: 0x7F,
            micro_ops: vec![Cpu::bit::<7, A>],
        },
        Instruction {
            name: "RES 0,B".to_string(),
            opcode: 0x80,
            micro_ops: vec![Cpu::res::<0, B>],
        },
        Instruction {
            name: "RES 0,C".to_string(),
            opcode: 0x81,
            micro_ops: vec![Cpu::res::<0, C>],
        },
        Instruction {
            name: "RES 0,D".to_string(),
            opcode: 0x82,   
            micro_ops: vec![Cpu::res::<0, D>],
        },
        Instruction {
            name: "RES 0,E".to_string(),
            opcode: 0x83,
            micro_ops: vec![Cpu::res::<0, E>],
        },
        Instruction {
            name: "RES 0,H".to_string(),
            opcode: 0x84,
            micro_ops: vec![Cpu::res::<0, H>],
        },
        Instruction {
            name: "RES 0,L".to_string(),
            opcode: 0x85,
            micro_ops: vec![Cpu::res::<0, L>],
        },
        Instruction {
            name: "RES 0,(HL)".to_string(),
            opcode: 0x86,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<0, HL, Z>,
                Cpu::noop
            ],
        },
        Instruction {
            name: "RES 0,A".to_string(),
            opcode: 0x87,
            micro_ops: vec![Cpu::res::<0, A>],
        },
        Instruction {
            name: "RES 1,B".to_string(),
            opcode: 0x88,
            micro_ops: vec![Cpu::res::<1, B>],
        },
        Instruction {
            name: "RES 1,C".to_string(),
            opcode: 0x89,
            micro_ops: vec![Cpu::res::<1, C>],
        },
        Instruction {
            name: "RES 1,D".to_string(),
            opcode: 0x8A,
            micro_ops: vec![Cpu::res::<1, D>],
        },
        Instruction {
            name: "RES 1,E".to_string(),
            opcode: 0x8B,
            micro_ops: vec![Cpu::res::<1, E>],
        },
        Instruction {
            name: "RES 1,H".to_string(),
            opcode: 0x8C,
            micro_ops: vec![Cpu::res::<1, H>],
        },
        Instruction {
            name: "RES 1,L".to_string(),
            opcode: 0x8D,
            micro_ops: vec![Cpu::res::<1, L>],
        },
        Instruction {
            name: "RES 1,(HL)".to_string(),
            opcode: 0x8E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<1, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RES 1,A".to_string(),
            opcode: 0x8F,
            micro_ops: vec![Cpu::res::<1, A>],
        },
        Instruction {
            name: "RES 2,B".to_string(),
            opcode: 0x90,
            micro_ops: vec![Cpu::res::<2, B>],
        },
        Instruction {
            name: "RES 2,C".to_string(),
            opcode: 0x91,
            micro_ops: vec![Cpu::res::<2, C>],
        },
        Instruction {
            name: "RES 2,D".to_string(),
            opcode: 0x92,
            micro_ops: vec![Cpu::res::<2, D>],
        },
        Instruction {
            name: "RES 2,E".to_string(),
            opcode: 0x93,
            micro_ops: vec![Cpu::res::<2, E>],
        },
        Instruction {
            name: "RES 2,H".to_string(),
            opcode: 0x94,
            micro_ops: vec![Cpu::res::<2, H>],
        },
        Instruction {
            name: "RES 2,L".to_string(),
            opcode: 0x95,
            micro_ops: vec![Cpu::res::<2, L>],
        },
        Instruction {
            name: "RES 2,(HL)".to_string(),
            opcode: 0x96,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<2, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RES 2,A".to_string(),
            opcode: 0x97,
            micro_ops: vec![Cpu::res::<2, A>],
        },
        Instruction {
            name: "RES 3,B".to_string(),
            opcode: 0x98,
            micro_ops: vec![Cpu::res::<3, B>],
        },
        Instruction {
            name: "RES 3,C".to_string(),
            opcode: 0x99,
            micro_ops: vec![Cpu::res::<3, C>],
        },
        Instruction {
            name: "RES 3,D".to_string(),
            opcode: 0x9A,
            micro_ops: vec![Cpu::res::<3, D>],
        },
        Instruction {
            name: "RES 3,E".to_string(),
            opcode: 0x9B,
            micro_ops: vec![Cpu::res::<3, E>],
        },
        Instruction {
            name: "RES 3,H".to_string(),
            opcode: 0x9C,
            micro_ops: vec![Cpu::res::<3, H>],
        },
        Instruction {
            name: "RES 3,L".to_string(),
            opcode: 0x9D,
            micro_ops: vec![Cpu::res::<3, L>],
        },
        Instruction {
            name: "RES 3,(HL)".to_string(),
            opcode: 0x9E,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<3, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RES 3,A".to_string(),
            opcode: 0x9F,
            micro_ops: vec![Cpu::res::<3, A>],
        },
        Instruction {
            name: "RES 4,B".to_string(),
            opcode: 0xA0,
            micro_ops: vec![Cpu::res::<4, B>],
        },
        Instruction {
            name: "RES 4,C".to_string(),
            opcode: 0xA1,
            micro_ops: vec![Cpu::res::<4, C>],
        },
        Instruction {
            name: "RES 4,D".to_string(),
            opcode: 0xA2,
            micro_ops: vec![Cpu::res::<4, D>],
        },
        Instruction {
            name: "RES 4,E".to_string(),
            opcode: 0xA3,
            micro_ops: vec![Cpu::res::<4, E>],
        },
        Instruction {
            name: "RES 4,H".to_string(),
            opcode: 0xA4,
            micro_ops: vec![Cpu::res::<4, H>],
        },
        Instruction {
            name: "RES 4,L".to_string(),
            opcode: 0xA5,
            micro_ops: vec![Cpu::res::<4, L>],
        },
        Instruction {
            name: "RES 4,(HL)".to_string(),
            opcode: 0xA6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<4, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RES 4,A".to_string(),
            opcode: 0xA7,
            micro_ops: vec![Cpu::res::<4, A>],
        },
        Instruction {
            name: "RES 5,B".to_string(),
            opcode: 0xA8,
            micro_ops: vec![Cpu::res::<5, B>],
        },
        Instruction {
            name: "RES 5,C".to_string(),
            opcode: 0xA9,
            micro_ops: vec![Cpu::res::<5, C>],
        },
        Instruction {
            name: "RES 5,D".to_string(),
            opcode: 0xAA,
            micro_ops: vec![Cpu::res::<5, D>],
        },
        Instruction {
            name: "RES 5,E".to_string(),
            opcode: 0xAB,
            micro_ops: vec![Cpu::res::<5, E>],
        },
        Instruction {
            name: "RES 5,H".to_string(),
            opcode: 0xAC,
            micro_ops: vec![Cpu::res::<5, H>],
        },
        Instruction {
            name: "RES 5,L".to_string(),
            opcode: 0xAD,
            micro_ops: vec![Cpu::res::<5, L>],
        },
        Instruction {
            name: "RES 5,(HL)".to_string(),
            opcode: 0xAE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<5, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RES 5,A".to_string(),
            opcode: 0xAF,
            micro_ops: vec![Cpu::res::<5, A>],
        },
        Instruction {
            name: "RES 6,B".to_string(),
            opcode: 0xB0,
            micro_ops: vec![Cpu::res::<6, B>],
        },
        Instruction {
            name: "RES 6,C".to_string(),
            opcode: 0xB1,
            micro_ops: vec![Cpu::res::<6, C>],
        },
        Instruction {
            name: "RES 6,D".to_string(),
            opcode: 0xB2,
            micro_ops: vec![Cpu::res::<6, D>],
        },
        Instruction {
            name: "RES 6,E".to_string(),
            opcode: 0xB3,
            micro_ops: vec![Cpu::res::<6, E>],
        },
        Instruction {
            name: "RES 6,H".to_string(),
            opcode: 0xB4,
            micro_ops: vec![Cpu::res::<6, H>],
        },
        Instruction {
            name: "RES 6,L".to_string(),
            opcode: 0xB5,
            micro_ops: vec![Cpu::res::<6, L>],
        },
        Instruction {
            name: "RES 6,(HL)".to_string(),
            opcode: 0xB6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<6, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RES 6,A".to_string(),
            opcode: 0xB7,
            micro_ops: vec![Cpu::res::<6, A>],
        },
        Instruction {
            name: "RES 7,B".to_string(),
            opcode: 0xB8,
            micro_ops: vec![Cpu::res::<7, B>],
        },
        Instruction {
            name: "RES 7,C".to_string(),
            opcode: 0xB9,
            micro_ops: vec![Cpu::res::<7, C>],
        },
        Instruction {
            name: "RES 7,D".to_string(),
            opcode: 0xBA,
            micro_ops: vec![Cpu::res::<7, D>],
        },
        Instruction {
            name: "RES 7,E".to_string(),
            opcode: 0xBB,
            micro_ops: vec![Cpu::res::<7, E>],
        },
        Instruction {
            name: "RES 7,H".to_string(),
            opcode: 0xBC,
            micro_ops: vec![Cpu::res::<7, H>],
        },
        Instruction {
            name: "RES 7,L".to_string(),
            opcode: 0xBD,
            micro_ops: vec![Cpu::res::<7, L>],
        },
        Instruction {
            name: "RES 7,(HL)".to_string(),
            opcode: 0xBE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_res_mem::<7, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "RES 7,A".to_string(),
            opcode: 0xBF,
            micro_ops: vec![Cpu::res::<7, A>],
        },
        Instruction {
            name: "SET 0,B".to_string(),
            opcode: 0xC0,
            micro_ops: vec![Cpu::set::<0, B>],
        },
        Instruction {
            name: "SET 0,C".to_string(),
            opcode: 0xC1,
            micro_ops: vec![Cpu::set::<0, C>],
        },
        Instruction {
            name: "SET 0,D".to_string(),
            opcode: 0xC2,
            micro_ops: vec![Cpu::set::<0, D>],
        },
        Instruction {
            name: "SET 0,E".to_string(),
            opcode: 0xC3,
            micro_ops: vec![Cpu::set::<0, E>],
        },
        Instruction {
            name: "SET 0,H".to_string(),
            opcode: 0xC4,
            micro_ops: vec![Cpu::set::<0, H>],
        },
        Instruction {
            name: "SET 0,L".to_string(),
            opcode: 0xC5,
            micro_ops: vec![Cpu::set::<0, L>],
        },
        Instruction {
            name: "SET 0,(HL)".to_string(),
            opcode: 0xC6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<0, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 0,A".to_string(),
            opcode: 0xC7,
            micro_ops: vec![Cpu::set::<0, A>],
        },
        Instruction {
            name: "SET 1,B".to_string(),
            opcode: 0xC8,
            micro_ops: vec![Cpu::set::<1, B>],
        },
        Instruction {
            name: "SET 1,C".to_string(),
            opcode: 0xC9,
            micro_ops: vec![Cpu::set::<1, C>],
        },
        Instruction {
            name: "SET 1,D".to_string(),
            opcode: 0xCA,
            micro_ops: vec![Cpu::set::<1, D>],
        },
        Instruction {
            name: "SET 1,E".to_string(),
            opcode: 0xCB,
            micro_ops: vec![Cpu::set::<1, E>],
        },
        Instruction {
            name: "SET 1,H".to_string(),
            opcode: 0xCC,
            micro_ops: vec![Cpu::set::<1, H>],
        },
        Instruction {
            name: "SET 1,L".to_string(),
            opcode: 0xCD,
            micro_ops: vec![Cpu::set::<1, L>],
        },
        Instruction {
            name: "SET 1,(HL)".to_string(),
            opcode: 0xCE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<1, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 1,A".to_string(),
            opcode: 0xCF,
            micro_ops: vec![Cpu::set::<1, A>],
        },
        Instruction {
            name: "SET 2,B".to_string(),
            opcode: 0xD0,
            micro_ops: vec![Cpu::set::<2, B>],
        },
        Instruction {
            name: "SET 2,C".to_string(),
            opcode: 0xD1,
            micro_ops: vec![Cpu::set::<2, C>],
        },
        Instruction {
            name: "SET 2,D".to_string(),
            opcode: 0xD2,
            micro_ops: vec![Cpu::set::<2, D>],
        },
        Instruction {
            name: "SET 2,E".to_string(),
            opcode: 0xD3,
            micro_ops: vec![Cpu::set::<2, E>],
        },
        Instruction {
            name: "SET 2,H".to_string(),
            opcode: 0xD4,
            micro_ops: vec![Cpu::set::<2, H>],
        },
        Instruction {
            name: "SET 2,L".to_string(),
            opcode: 0xD5,
            micro_ops: vec![Cpu::set::<2, L>],
        },
        Instruction {
            name: "SET 2,(HL)".to_string(),
            opcode: 0xD6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<2, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 2,A".to_string(),
            opcode: 0xD7,
            micro_ops: vec![Cpu::set::<2, A>],
        },
        Instruction {
            name: "SET 3,B".to_string(),
            opcode: 0xD8,
            micro_ops: vec![Cpu::set::<3, B>],
        },
        Instruction {
            name: "SET 3,C".to_string(),
            opcode: 0xD9,
            micro_ops: vec![Cpu::set::<3, C>],
        },
        Instruction {
            name: "SET 3,D".to_string(),
            opcode: 0xDA,
            micro_ops: vec![Cpu::set::<3, D>],
        },
        Instruction {
            name: "SET 3,E".to_string(),
            opcode: 0xDB,
            micro_ops: vec![Cpu::set::<3, E>],
        },
        Instruction {
            name: "SET 3,H".to_string(),
            opcode: 0xDC,
            micro_ops: vec![Cpu::set::<3, H>],
        },
        Instruction {
            name: "SET 3,L".to_string(),
            opcode: 0xDD,
            micro_ops: vec![Cpu::set::<3, L>],
        },
        Instruction {
            name: "SET 3,(HL)".to_string(),
            opcode: 0xDE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<3, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 3,A".to_string(),
            opcode: 0xDF,
            micro_ops: vec![Cpu::set::<3, A>],
        },
        Instruction {
            name: "SET 4,B".to_string(),
            opcode: 0xE0,
            micro_ops: vec![Cpu::set::<4, B>],
        },
        Instruction {
            name: "SET 4,C".to_string(),
            opcode: 0xE1,
            micro_ops: vec![Cpu::set::<4, C>],
        },
        Instruction {
            name: "SET 4,D".to_string(),
            opcode: 0xE2,
            micro_ops: vec![Cpu::set::<4, D>],
        },
        Instruction {
            name: "SET 4,E".to_string(),
            opcode: 0xE3,
            micro_ops: vec![Cpu::set::<4, E>],
        },
        Instruction {
            name: "SET 4,H".to_string(),
            opcode: 0xE4,
            micro_ops: vec![Cpu::set::<4, H>],
        },
        Instruction {
            name: "SET 4,L".to_string(),
            opcode: 0xE5,
            micro_ops: vec![Cpu::set::<4, L>],
        },
        Instruction {
            name: "SET 4,(HL)".to_string(),
            opcode: 0xE6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<4, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 4,A".to_string(),
            opcode: 0xE7,
            micro_ops: vec![Cpu::set::<4, A>],
        },
        Instruction {
            name: "SET 5,B".to_string(),
            opcode: 0xE8,
            micro_ops: vec![Cpu::set::<5, B>],
        },
        Instruction {
            name: "SET 5,C".to_string(),
            opcode: 0xE9,
            micro_ops: vec![Cpu::set::<5, C>],
        },
        Instruction {
            name: "SET 5,D".to_string(),
            opcode: 0xEA,
            micro_ops: vec![Cpu::set::<5, D>],
        },
        Instruction {
            name: "SET 5,E".to_string(),
            opcode: 0xEB,
            micro_ops: vec![Cpu::set::<5, E>],
        },
        Instruction {
            name: "SET 5,H".to_string(),
            opcode: 0xEC,
            micro_ops: vec![Cpu::set::<5, H>],
        },
        Instruction {
            name: "SET 5,L".to_string(),
            opcode: 0xED,
            micro_ops: vec![Cpu::set::<5, L>],
        },
        Instruction {
            name: "SET 5,(HL)".to_string(),
            opcode: 0xEE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<5, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 5,A".to_string(),
            opcode: 0xEF,
            micro_ops: vec![Cpu::set::<5, A>],
        },
        Instruction {
            name: "SET 6,B".to_string(),
            opcode: 0xF0,
            micro_ops: vec![Cpu::set::<6, B>],
        },
        Instruction {
            name: "SET 6,C".to_string(),
            opcode: 0xF1,
            micro_ops: vec![Cpu::set::<6, C>],
        },
        Instruction {
            name: "SET 6,D".to_string(),
            opcode: 0xF2,
            micro_ops: vec![Cpu::set::<6, D>],
        },
        Instruction {
            name: "SET 6,E".to_string(),
            opcode: 0xF3,
            micro_ops: vec![Cpu::set::<6, E>],
        },
        Instruction {
            name: "SET 6,H".to_string(),
            opcode: 0xF4,
            micro_ops: vec![Cpu::set::<6, H>],
        },
        Instruction {
            name: "SET 6,L".to_string(),
            opcode: 0xF5,
            micro_ops: vec![Cpu::set::<6, L>],
        },
        Instruction {
            name: "SET 6,(HL)".to_string(),
            opcode: 0xF6,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<6, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 6,A".to_string(),
            opcode: 0xF7,
            micro_ops: vec![Cpu::set::<6, A>],
        },
        Instruction {
            name: "SET 7,B".to_string(),
            opcode: 0xF8,
            micro_ops: vec![Cpu::set::<7, B>],
        },
        Instruction {
            name: "SET 7,C".to_string(),
            opcode: 0xF9,
            micro_ops: vec![Cpu::set::<7, C>],
        },
        Instruction {
            name: "SET 7,D".to_string(),
            opcode: 0xFA,
            micro_ops: vec![Cpu::set::<7, D>],
        },
        Instruction {
            name: "SET 7,E".to_string(),
            opcode: 0xFB,
            micro_ops: vec![Cpu::set::<7, E>],
        },
        Instruction {
            name: "SET 7,H".to_string(),
            opcode: 0xFC,
            micro_ops: vec![Cpu::set::<7, H>],
        },
        Instruction {
            name: "SET 7,L".to_string(),
            opcode: 0xFD,
            micro_ops: vec![Cpu::set::<7, L>],
        },
        Instruction {
            name: "SET 7,(HL)".to_string(),
            opcode: 0xFE,
            micro_ops: vec![
                Cpu::read_memory::<HL, Z>,
                Cpu::write_set_mem::<7, HL, Z>,
                Cpu::noop,
            ],
        },
        Instruction {
            name: "SET 7,A".to_string(),
            opcode: 0xFF,
            micro_ops: vec![Cpu::set::<7, A>],
        },
    ]
}

impl<M: MemoryMapper> Cpu<M> {
    pub fn decode_cb(&mut self, bus: &mut M) {
        let pc = self.get_r16::<PC>(); 
        let cb_opcode = bus.read_byte(pc);
        self.set_r16::<PC>(pc.wrapping_add(1));
        
        let ops = &self.cb_instructions[cb_opcode as usize].micro_ops;
        self.queue_len = ops.len();
        self.queue[..self.queue_len].copy_from_slice(&ops[..self.queue_len]);
        self.op_index = 0;
    }

    pub fn debug_step(&mut self, _instruction: u8, _bus: &mut M) -> bool {
        todo!()
    }
}