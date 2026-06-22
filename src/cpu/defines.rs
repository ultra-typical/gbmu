use crate::mmu::MemoryMapper;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Zero = 7,      // bit 7
    Subtract = 6,  // bit 6
    HalfCarry = 5, // bit 5
    Carry = 4,     // bit 4
}

#[derive(Debug, Clone)]
pub struct Instruction<M: MemoryMapper> {
    pub opcode: u8,
    pub micro_ops: Vec<MicroOp<M>>,
}

pub type MicroOp<M> = fn(&mut Cpu<M>, bus: &mut M);
pub type Flags = u8;

pub struct Cpu<M: MemoryMapper> {
    pub queue: [MicroOp<M>; 8],
    pub r8: [u8; 14],

    pub queue_len: usize,
    pub instructions: Vec<Instruction<M>>,
    pub cb_instructions: Vec<Instruction<M>>,
    /*
     * A, B, C, D, E, F, H, L -> 0:7
     * SP, PC 8:11
     * WZ 12:14 -> Register used to store variables
     */
    pub flags: Flags,
    pub op_index: usize,
    pub ime: bool,
    pub ime_delay: bool, // mimic hardware delay in EI
    pub halted: bool,    // for HALT instruction
    pub halt_bug: bool,
}

#[allow(non_upper_case_globals)]
pub mod r8 {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const C: usize = 2;
    pub const D: usize = 3;
    pub const E: usize = 4;
    pub const F: usize = 5;
    pub const H: usize = 6;
    pub const L: usize = 7;
    pub const S: usize = 8;
    pub const P: usize = 9;
    pub const PcC: usize = 10;
    pub const PcP: usize = 11;
    pub const W: usize = 12;
    pub const Z: usize = 13;
}

pub mod r16 {
    pub const AF: usize = 0;
    pub const BC: usize = 1;
    pub const DE: usize = 2;
    pub const HL: usize = 3;
    pub const SP: usize = 4;
    pub const PC: usize = 5;
    pub const WZ: usize = 6;
}
