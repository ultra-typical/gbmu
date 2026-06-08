#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Zero = 7,      // bit 7
    Subtract = 6,  // bit 6
    HalfCarry = 5, // bit 5
    Carry = 4,     // bit 4
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: u16,
    pub micro_ops: &'static [MicroOp],
}

pub type MicroOp = fn(&mut Cpu);
pub type Flags = u8;

pub struct Cpu {
    pub queue: &'static [MicroOp],
    pub r8: [u8; 14],
    /*
     * A, B, C, D, E, F, H, L -> 0:7
     * SP, PC 8:11
     * WZ 12:14 -> Register used to store variables
     */
    pub flags: Flags,
    pub instructions_list: Vec<u8>,
    pub op_index: usize,
    pub bus: [u8; 0x10000],
}

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
    pub const PC_P: usize = 10;
    pub const PC_C: usize = 11;
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
