#![allow(unused_variables)]

use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::communications::InstructionList;
use crate::communications::WatchedAdresses;
use crate::communications::{GameCT, Mode, Request};
use crate::cpu::Cpu;
use crate::cpu::registers::{R8};
use crate::gui::KeyInput;
use crate::mmu::apu::sample_buffer::SampleBuffer;
use crate::mmu::MemoryMapper;

const FRAME_CYCLES: u32 = 70224;
const GAME_REFRESH_PERIOD_IN_MILLIS: u64 = 15000; //8000 pour 120 fps
const CUT_TIME_FOR_CAP_FRAMES: u32 = 30; // A faire varier. TODO: Verifier si la meilleur version
pub struct GameBoy<M: MemoryMapper> {
    pub cpu: Cpu,
    pub bus: M,

    step_to_execute: usize,
    should_get_fps: bool,
    instructions_to_send: u16,

    watched_address: HashSet<u16>,
    cycles_elapsed: u32,
}

type GBMode<M> = fn(&mut GameBoy<M>, &KeyInput, &mut Box<dyn GameCT>);

impl<M: MemoryMapper>  GameBoy<M> {
    fn send_watched_adress(&mut self, ct: &mut Box<dyn GameCT>) {
        ct.send_watched_adresses(
            WatchedAdresses(self.watched_address.iter().map(
                    |addr| (*addr, self.bus.read_byte(*addr))
                ).collect::<Vec<(u16, u8)>>())
        )
    }

    fn send_registers(&self, ct: &mut Box<dyn GameCT>) {
        ct.send_cpu_state(
            &self.cpu.dump_state()
        );
    }

    fn send_next_instructions(&mut self, ct: &mut Box<dyn GameCT>) {
        ct.send_next_instructions(
            InstructionList((0..self.instructions_to_send).map(
                    |index: u16| self.cpu.pc as usize + index as usize
                ).map(
                    |addr: usize|  self.bus.read_byte(addr as u16) as u16
                ).collect())
        );
    }

    pub fn ram_dump(mut self) -> Option<Vec<u8>> {
        self.bus.ram_dump()
    }

    pub fn new(
        boot_rom_data: Option<[u8; 0x100]>,
        rom_data: Vec<u8>,
        ram_data: Option<Vec<u8>>,
        sample_buffer: SampleBuffer,
    ) -> Result<GameBoy<M>, String> {
        let skip_boot = boot_rom_data.is_none();
        let bus = M::new(boot_rom_data, rom_data, ram_data, sample_buffer)?;

        let cpu = Cpu::new();
        let mut gb = GameBoy {
            cpu,
            bus,
            step_to_execute: 0,
            should_get_fps: true,
            instructions_to_send: 0,

            cycles_elapsed: 0,
            watched_address: HashSet::new(),
        };

        if skip_boot {gb.simulate_boot_rom_effect()}

        Ok(gb)
    }

    fn calculate_fps(before: &mut Instant) -> u128 {
        let now = Instant::now();
        let frame: Duration = now.duration_since(*before);
        *before = now;
        Duration::from_secs(1).as_nanos() / frame.as_nanos()
    }

    #[allow(unused_mut)]
    pub fn launch(mut self, mut ct: Box<dyn GameCT>) -> Result<Option<Vec<u8>>, String>{
        let mut input = KeyInput::default();
        let mut before = Instant::now();
        let mut debut: Instant;
        let mut mode: GBMode<M> = Self::game_mode;
        loop {
            debut = Instant::now();
            ct.poll_requests()
                .into_iter()
                .for_each(|request| {
                    self.treat_request(request, &mut mode)
                }
                );

            if let Err(msg) = ct.update_input(&mut input) {
                eprintln!("Gameboy must stop : {msg}");
                break 
            }
            mode(&mut self, &input, &mut ct);
            if self.should_get_fps {
                ct.update_fps(Self::calculate_fps(&mut before))?;
            }
            let wanted_duration = Duration::from_micros(GAME_REFRESH_PERIOD_IN_MILLIS);
            let duration_elapsed = debut.elapsed();
            if wanted_duration > duration_elapsed {
                Self::cap_frame(wanted_duration, duration_elapsed);
            }
        }
        Ok(self.ram_dump())
    }

    fn cap_frame(wanted_duration: Duration, duration_elapsed: Duration) {
        let duration_of_the_sleep = wanted_duration - duration_elapsed;
        for mut i in 0..CUT_TIME_FOR_CAP_FRAMES {
            thread::sleep(duration_of_the_sleep / CUT_TIME_FOR_CAP_FRAMES);
            i += 1;
        }
    }

    fn treat_request(&mut self, request: Request,  mode: &mut GBMode<M>) {
        match request {
            Request::Mode(new_mode) => {
                match new_mode {
                    Mode::Game => { *mode = Self::game_mode; },
                    Mode::Debug => { *mode = Self::debug_mode },
                    Mode::Stop => { *mode = Self::stopped_mode },
                }
            },
            Request::Execute(instructions) => {
                for instruction in instructions {
                    self.cpu.debug_step(instruction, &mut self.bus);
                }
            },
            Request::RenderFrame(frame) => {
                if std::ptr::fn_addr_eq(*mode, Self::stopped_mode as GBMode<M>) {
                    todo!()
                }
            },
            Request::Watch(address) => {
                self.watched_address.insert(address);
            },
            Request::Step(step) => {
                if std::ptr::fn_addr_eq(*mode, Self::stopped_mode as GBMode<M>) {
                    self.step_to_execute = step;
                }
            },
            Request::SetInstructionListLength(length) => {
                self.instructions_to_send = length as u16;
            },
            _ => unreachable!()
        }
    }

    pub fn simulate_boot_rom_effect(&mut self) {
        self.cpu.registers.set_r8_value(R8::A, 0x01);
        self.cpu.registers.set_r8_value(R8::B, 0xFF);
        self.cpu.registers.set_r8_value(R8::C, 0x13);
        self.cpu.registers.set_r8_value(R8::D, 0x00);
        self.cpu.registers.set_r8_value(R8::E, 0xC1);
        self.cpu.registers.set_r8_value(R8::H, 0x84);
        self.cpu.registers.set_r8_value(R8::L, 0x03);
        self.cpu.pc = 0x0100;
        self.cpu.registers.set_sp(0xFFFE);

        self.bus.write_byte(0xFF00, 0xCF);
        self.bus.write_byte(0xFF01, 0x00);
        self.bus.write_byte(0xFF02, 0x7E);
        self.bus.write_byte(0xFF04, 0x18);
        self.bus.write_byte(0xFF05, 0x00);
        self.bus.write_byte(0xFF06, 0x00);
        self.bus.write_byte(0xFF07, 0xF8);
        self.bus.write_byte(0xFF0F, 0xE1);
        self.bus.write_byte(0xFF10, 0x80);
        self.bus.write_byte(0xFF11, 0xBF);
        self.bus.write_byte(0xFF12, 0xF3);
        self.bus.write_byte(0xFF13, 0xFF);
        self.bus.write_byte(0xFF14, 0xBF);
        self.bus.write_byte(0xFF16, 0x3F);
        self.bus.write_byte(0xFF17, 0x00);
        self.bus.write_byte(0xFF18, 0xFF);
        self.bus.write_byte(0xFF19, 0xBF);
        self.bus.write_byte(0xFF1A, 0x7F);
        self.bus.write_byte(0xFF1B, 0xFF);
        self.bus.write_byte(0xFF1C, 0x9F);
        self.bus.write_byte(0xFF1D, 0xFF);
        self.bus.write_byte(0xFF1E, 0xBF);
        self.bus.write_byte(0xFF20, 0xFF);
        self.bus.write_byte(0xFF21, 0x00);
        self.bus.write_byte(0xFF22, 0x00);
        self.bus.write_byte(0xFF23, 0xBF);
        self.bus.write_byte(0xFF24, 0x77);
        self.bus.write_byte(0xFF25, 0xF3);
        self.bus.write_byte(0xFF26, 0xF1);
        self.bus.write_byte(0xFF40, 0x91);
        self.bus.write_byte(0xFF41, 0x81);
        self.bus.write_byte(0xFF42, 0x00);
        self.bus.write_byte(0xFF43, 0x00);
        self.bus.write_byte(0xFF44, 0x91);
        self.bus.write_byte(0xFF45, 0x00);
        self.bus.write_byte(0xFF46, 0xFF);
        self.bus.write_byte(0xFF47, 0xFC);
        self.bus.write_byte(0xFF4A, 0x00);
        self.bus.write_byte(0xFF4B, 0x00);
        self.bus.write_byte(0xFFFF, 0x00);
    }


    pub fn manage_input(&mut self, key_input: &KeyInput) {
        let mut dpad = 0x0F;
        if key_input.down_pushed    { dpad &= 0b1111_0111; }
        if key_input.up_pushed      { dpad &= 0b1111_1011; }
        if key_input.left_pushed    { dpad &= 0b1111_1101; }
        if key_input.right_pushed   { dpad &= 0b1111_1110; }

        let mut buttons = 0x0F;
        if key_input.start_pushed   { buttons &= 0b1111_0111; }
        if key_input.select_pushed  { buttons &= 0b1111_1011; }
        if key_input.b_pushed       { buttons &= 0b1111_1101; }
        if key_input.a_pushed       { buttons &= 0b1111_1110; }

        self.bus.update_keys(dpad, buttons)
    }

    pub fn tick_gb(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        self.manage_input(key_input);
        if self.cycles_elapsed.is_multiple_of(4) {
            if self.bus.get_dma_index() != 0xFF {
                self.bus.tick_dma();
            }
            self.cpu.machine_cycle(&mut self.bus);
            self.bus.tick_timers();
            self.cycles_elapsed = 0;
        }
        self.cycles_elapsed += 1;
        
        self.bus.tick_ppu(ct);
    }


    fn game_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        for _ in 0..FRAME_CYCLES {
            self.tick_gb(key_input, ct);
        }
    }

    fn debug_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        if !self.watched_address.is_empty() { self.send_watched_adress(ct); }
        if self.instructions_to_send != 0 { self.send_next_instructions(ct); }
        self.send_registers(ct); 
        self.tick_gb(key_input, ct)
    }

    fn stopped_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        for step in 0..self.step_to_execute {
            self.debug_mode(key_input, ct);
        }
        self.step_to_execute = 0;
    }
}
