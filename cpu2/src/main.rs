use crate::defines::{Cpu, R8};

mod accumulator;
mod defines;
mod flags;
mod implemenation;
mod instructions;
mod operations;

fn main() {
    let instr: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80];
    let mut cpu = Cpu::new(instr);

    cpu.registers.r8[R8::B as usize] = 5;
    for _ in 1..6 {
        cpu.tick();
        println!("{:?}", cpu);
    }
}

