use crate::communications::CpuState;
use crate::cpu::cb_operations::build_cb_instructions;
use crate::cpu::defines::Cpu;
use crate::cpu::defines::{r8, r16};
use crate::cpu::operations::build_instructions_set;
use crate::mmu::MemoryMapper;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum StepStatus {
    Continue,
    Halted,
}

impl<M: MemoryMapper> Cpu<M> {
    pub fn new() -> Self {
        Self {
            r8: [0; 14],
            flags: 0,
            queue: Vec::new(),
            op_index: 0,
            bus: [0; 65536],
            ime: false,
            ime_delay: false,
            halted: false,
            halt_bug: false,
            tick_to_wait: 0,
            instructions: build_instructions_set(),
            cb_instructions: build_cb_instructions(),
        }
    }

    pub fn first_read(&mut self, bus: &mut M) {
        let pc = self.get_r16::<PC>();
        let instruction_byte: u8 = bus.read_byte(pc);

        self.handle_halt_bug(bus);
        self.handle_ime_delay();

        self.set_r16::<PC>(self.get_r16::<PC>().wrapping_add(1));
        self.queue = self.instructions[instruction_byte as usize]
            .micro_ops
            .to_vec();
        self.op_index = 0;
    }

    pub fn tick(&mut self, bus: &mut M) {
        if Self::get_r16::<PC>(self) == 0 {
            Self::first_read(self, bus);
            return;
        }
        let micro_op = &self.queue[self.op_index];
        self.op_index += 1;
        micro_op(self, bus);

        if self.op_index == self.queue.len() {
            let pc = self.get_r16::<PC>();
            let instruction_byte: u8 = bus.read_byte(pc);

            self.handle_halt_bug(bus);
            self.handle_ime_delay();

            self.set_r16::<PC>(self.get_r16::<PC>().wrapping_add(1));
            println!("{}", instruction_byte);
            self.queue = self.instructions[instruction_byte as usize]
                .micro_ops
                .to_vec();
            self.op_index = 0;
        }
    }

    pub fn dump_state(&self) -> CpuState {
        CpuState {
            a: self.get_r8::<A>(),
            b: self.get_r8::<B>(),
            c: self.get_r8::<C>(),
            d: self.get_r8::<D>(),
            e: self.get_r8::<E>(),
            h: self.get_r8::<H>(),
            l: self.get_r8::<L>(),
            hl: self.get_r16::<HL>(),
            sp: self.get_r16::<SP>(),
            pc: self.get_r16::<PC>(),
        }
    }

    fn handle_halt_state(&mut self, bus: &mut M) -> StepStatus {
        if self.halted {
            let iflag = bus.read_interrupt_flag();
            let ienable = bus.read_interrupt_enable();

            if ienable & iflag == 0 {
                return StepStatus::Halted;
            }

            self.halted = false;

            if !self.ime {
                self.halt_bug = true;
            }
        }
        StepStatus::Continue
    }

    fn handle_ime_state(&mut self, bus: &mut M) -> StepStatus {
        if self.ime {
            if let Some(interrupt) = bus.interrupts_next_request() {
                self.ime = false;
                bus.interrupts_clear_request(interrupt);

                let ret_addr = self.get_r16::<PC>();

                let sp1 = self.get_r16::<SP>().wrapping_sub(1);
                self.set_r16::<SP>(sp1);
                bus.write_byte(sp1, (ret_addr >> 8) as u8);

                let sp2 = sp1.wrapping_sub(1);
                self.set_r16::<SP>(sp2);
                bus.write_byte(sp2, (ret_addr & 0xFF) as u8);

                self.set_r16::<PC>(interrupt.vector());
                StepStatus::Halted
            } else {
                StepStatus::Continue
            }
        } else {
            StepStatus::Continue
        }
    }

    fn handle_halt_bug(&mut self, bus: &mut M) {
        if self.halt_bug {
            Self::dec_r16::<PC>(self, bus);
            self.halt_bug = false;
        }
    }

    fn handle_ime_delay(&mut self) {
        if self.ime_delay {
            self.ime = true;
            self.ime_delay = false;
        }
    }
}

impl<M: MemoryMapper> fmt::Debug for Cpu<M> {
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
implreg8!(PcP);
implreg8!(PcC);
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

impl<M: MemoryMapper> Cpu<M> {
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
