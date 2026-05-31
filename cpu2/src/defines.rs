
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Zero      = 7,  // bit 7
    Subtract  = 6,  // bit 6
    HalfCarry = 5,  // bit 5
    Carry     = 4,  // bit 4
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum R8 {
    A, B, C, D, E, F, H, L, 
}


#[derive(Debug, Clone)]
pub enum R16 {
    AF, BC,  DE,
    HL, SP,  PC
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub micro_ops: &'static [MicroOp]
}

pub type MicroOp = fn(&mut Cpu);
pub type Flags = u8;

pub struct Accumulator {
    pub value: u32,
    pub pos: u8
}

pub struct Cpu {
    pub queue: &'static [MicroOp],
    pub registers : Registers,
    pub instructions_list: Vec<u8>,
    pub op_index: usize,
    pub accumulator : Accumulator,
    pub bus: [u8; 0x10000]
}

pub struct Registers {
    pub r8 : [u8; 8],
    pub sp: u16,
    pub pc: u16,
    pub flags: Flags,
}