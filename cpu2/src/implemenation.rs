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
            .field("op_index", &self.op_index)
            .field("r8", &self.r8)
            .field("flags", &format!("{:08b}", self.flags))
            .field("queue", &self.queue)
            .finish()
    }
}

pub trait  GetReg {
    type Output;
    fn get(&self) -> Self::Output;
    fn set(&mut self, value: Self::Output);
}

trait Reg8 { const USIZE: usize; }

macro_rules! implreg8 {
    ($name:ident) => {
        struct $name {}
        impl Reg8 for $name {
            const USIZE: usize = R8::$name as usize;
        }
    };
}

implreg8!(A);
implreg8!(B);
implreg8!(C);
implreg8!(D);
implreg8!(E);
implreg8!(F);
implreg8!(H);
implreg8!(L);

macro_rules! implreg16 {
    ($name:ident) => {
        struct $name {}
        impl Reg16 for $name {
            const USIZE: usize = R16::$name as usize;
        }
    };
}

implreg16!(AF);
implreg16!(BC);
implreg16!(DE);
implreg16!(HL);
implreg16!(SP);
implreg16!(PC);


impl Cpu{
    fn get_r8<R:Reg8> (&self) -> u8{
        self.r8[R::USIZE]
    }

    fn set_r8<R:Reg8>(&mut self, value: u8) {
        self.r8[R::USIZE] = value;
    }

    fn get_r16<R: Reg16>(&self) -> u16 {
        (self.r8[R::USIZE * 2] as u16) << 8 | self.r8[R::USIZE * 2 + 1] as u16
    }

    fn set_r16<R: Reg16>(&mut self, value: u16) {
        self.r8[R::USIZE] = (value >> 8) as u8;
        self.r8[R::USIZE] = (value & 0xFF) as u8;
    }
}



trait Reg16 { const USIZE: usize; }

struct RegAF;

impl Reg16 for RegAF {
    const USIZE: usize = 0;
}

