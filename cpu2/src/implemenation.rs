use crate::defines::Cpu;
use crate::defines::{r8, r16};
use crate::operations::{DISPATCH, INSTRUCTIONS};
use std::fmt;

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
                r8: [0; 14],
                flags: 0,
                instructions_list: test.clone(),
                queue: first_instr.micro_ops,
                op_index: 0,
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
                let pc = self.get_r16::<PC>();
                let opcode = *self
                    .instructions_list
                    .get(pc as usize)
                    .expect("Could not fetch instructions");

                self.set_r16::<PC>(self.get_r16::<PC>().wrapping_add(1));
                self.queue = DISPATCH[opcode as usize].expect("Unknown opcode");
                self.op_index = 0;
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

pub trait Reg8 {
    const USIZE: usize;
}

pub trait Reg16 {
    const USIZE: usize;
}

macro_rules! implreg8 {
    ($name:ident) => {
        pub struct $name {}
        impl Reg8 for $name {
            const USIZE: usize = r8::$name as usize;
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
implreg8!(S);
implreg8!(PC_P);
implreg8!(PC_C);
implreg8!(P);
implreg8!(W);
implreg8!(Z);

macro_rules! implreg16 {
    ($name:ident) => {
        pub struct $name {}
        impl Reg16 for $name {
            const USIZE: usize = r16::$name as usize;
        }
    };
}

implreg16!(AF);
implreg16!(BC);
implreg16!(DE);
implreg16!(HL);
implreg16!(SP);
implreg16!(PC);
implreg16!(WZ);

impl Cpu {
    pub fn get_r8<R: Reg8>(&self) -> u8 {
        self.r8[R::USIZE]
    }

    pub fn set_r8<R: Reg8>(&mut self, value: u8) {
        self.r8[R::USIZE] = value;
    }

    pub fn get_r16<R: Reg16>(&self) -> u16 {
        (self.r8[R::USIZE * 2] as u16) << 8 | self.r8[R::USIZE * 2 + 1] as u16
    }

    pub fn set_r16<R: Reg16>(&mut self, value: u16) {
        self.r8[R::USIZE * 2] = (value >> 8) as u8;
        self.r8[R::USIZE * 2 + 1] = (value & 0xFF) as u8;
    }
}
