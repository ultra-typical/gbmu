use crate::R8;
use crate::defines::Accumulator;
use crate::defines::{Cpu, R16, Registers};
use crate::operations::{DISPATCH, INSTRUCTIONS};
use std::fmt;

impl Registers {
    pub fn new() -> Self {
        Registers {
            r8: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            pc: 0,
            sp: 0,
            flags: 0,
        }
    }

    pub fn incr_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    pub fn incr_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}

impl Cpu {
    pub fn new(test: Vec<u8>) -> Self {
        {
            let first_opcode = test
                .first()
                .expect("Error in the fetch of the first instruction");
            let first_instr = INSTRUCTIONS
                .iter()
                .find(|e| e.opcode == *first_opcode)
                .expect("Unknown opcode");
            println!("First fetch");

            Cpu {
                registers: Registers::new(),
                instructions_list: test.clone(),
                queue: first_instr.micro_ops,
                op_index: 0,
                accumulator: Accumulator::new(),
                bus: [0; 65536],
            }
        }
    }

    pub fn tick(&mut self) {
        if self.op_index < self.queue.len() {
            let micro_op = &self.queue[self.op_index];
            self.op_index += 1;
            micro_op(self);

            if self.op_index == self.queue.len() {
                println!("Fetching...");
                let opcode = self
                    .instructions_list
                    .get(self.registers.pc as usize)
                    .expect("Could not fetch instructions");
                self.registers.pc = self.registers.pc.wrapping_add(1);

                self.queue = DISPATCH[*opcode as usize].expect("Unknown opcode");
                self.op_index = 0;
                self.accumulator.reset();
            }
        } else {
            panic!("No instruction left!");
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
            .field("Registers", &self.registers)
            .field("op_index", &self.op_index)
            .field("queue", &self.queue)
            .finish()
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
            .field("r8", &self.r8)
            .field("pc", &self.pc)
            .field("sp", &self.sp)
            .field("flags", &format!("{:08b}", self.flags))
            .finish()
    }
}

pub trait GetReg<R> {
    type Output;
    fn get(&self, reg: R) -> Self::Output;
    fn set(&mut self, reg: R, value: Self::Output);
}

impl GetReg<R8> for Registers {
    type Output = u8;
    fn get(&self, reg: R8) -> u8 {
        self.r8[reg as usize]
    }
    fn set(&mut self, reg: R8, value: u8) {
        self.r8[reg as usize] = value;
    }
}

impl GetReg<R16> for Registers {
    type Output = u16;
    fn get(&self, reg: R16) -> u16 {
        match reg {
            R16::AF => (self.r8[R8::A as usize] as u16) << 8 | self.r8[R8::F as usize] as u16,
            R16::BC => (self.r8[R8::B as usize] as u16) << 8 | self.r8[R8::C as usize] as u16,
            R16::DE => (self.r8[R8::D as usize] as u16) << 8 | self.r8[R8::E as usize] as u16,
            R16::HL => (self.r8[R8::H as usize] as u16) << 8 | self.r8[R8::L as usize] as u16,
            R16::PC => self.pc,
            R16::SP => self.sp,
        }
    }

    fn set(&mut self, reg: R16, value: u16) {
        match reg {
            R16::AF => {
                self.r8[R8::A as usize] = (value >> 8) as u8;
                self.r8[R8::F as usize] = (value & 0xFF) as u8;
            }
            R16::BC => {
                self.r8[R8::B as usize] = (value >> 8) as u8;
                self.r8[R8::C as usize] = (value & 0xFF) as u8;
            }
            R16::DE => {
                self.r8[R8::D as usize] = (value >> 8) as u8;
                self.r8[R8::E as usize] = (value & 0xFF) as u8;
            }
            R16::HL => {
                self.r8[R8::H as usize] = (value >> 8) as u8;
                self.r8[R8::L as usize] = (value & 0xFF) as u8;
            }
            R16::PC => self.pc = value,
            R16::SP => self.sp = value,
        }
    }
}
