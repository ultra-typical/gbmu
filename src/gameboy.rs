#![allow(unused_variables)]

use crate::gameboy::instructions::get_instruction_length;
use crate::gui::GbType;
use crate::gui::keymapping::KeyInput;
use crate::ppu::PpuMode;
use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::communications::InstructionList;
use crate::communications::WatchedAdresses;
use crate::communications::{GameCT, Mode, Request};
use crate::cpu::defines::Cpu;
use crate::cpu::*;
use crate::file::GbmuFile;
use crate::gameboy::defines::MicroOp;
use crate::mmu::MemoryMapper;

const FRAME_CYCLES: u32 = 70224;
const GAME_REFRESH_PERIOD_IN_MILLIS: u64 = 16667; //8000 pour 120 fps
const CUT_TIME_FOR_CAP_FRAMES: u32 = 30; // A faire varier. TODO: Verifier si la meilleur version
const SLEEP_MARGIN: Duration = Duration::from_micros(50);
#[derive(Serialize, Deserialize, Debug)]
pub struct GameBoy<M: MemoryMapper> {
    pub cpu: Cpu<M>,
    pub bus: M,

    step_to_execute: usize,
    should_get_fps: bool,
    instructions_to_send: u16,

    watched_address: HashSet<u16>,
    cycles_elapsed: u32,
    speed: u64,
    is_paused: bool,

    path: Option<PathBuf>,
}

type GBMode<M> = fn(&mut GameBoy<M>, &KeyInput, &mut Box<dyn GameCT>);

impl<M: MemoryMapper + Serialize + std::fmt::Debug> GameBoy<M> {
    fn send_watched_adress(&mut self, ct: &mut Box<dyn GameCT>) {
        ct.send_watched_adresses(WatchedAdresses(
            self.watched_address
                .iter()
                .map(|addr| (*addr, self.bus.read_byte(*addr)))
                .collect::<Vec<(u16, u8)>>(),
        ))
    }

    fn send_registers(&self, ct: &mut Box<dyn GameCT>) {
        ct.send_cpu_state(&self.cpu.dump_state());
    }

    fn send_next_instructions(&mut self, ct: &mut Box<dyn GameCT>, mut current_pc: u16) {
        let mut instructions: Vec<(u16, String)> = Vec::new();

        for _ in 0..self.instructions_to_send {
            let opcode = self.bus.read_byte(current_pc);

            let (opcode_to_push, name, instr_len) = if opcode == 0xCB {
                let cb_opcode = self.bus.read_byte(current_pc.wrapping_add(1));
                let name = self.cpu.cb_instructions[cb_opcode as usize].name.clone();
                (((opcode as u16) << 8) | cb_opcode as u16, name, 2)
            } else {
                let name = self.cpu.instructions[opcode as usize].name.clone();
                let len = get_instruction_length(opcode);
                (opcode as u16, name, len)
            };

            instructions.push((opcode_to_push, name));

            current_pc = current_pc.wrapping_add(instr_len);
        }

        ct.send_next_instructions(InstructionList(instructions));
    }

    pub fn ram_dump(mut self) -> Option<Vec<u8>> {
        self.bus.ram_dump()
    }

    pub fn new(
        boot_rom_data: Option<[u8; 0x900]>,
        rom_data: Vec<u8>,
        ram_data: Option<Vec<u8>>,
        rom_compatibility: bool,
        gb_type: GbType,
    ) -> Result<GameBoy<M>, String> {
        println!("new gameboy");
        let with_boot_rom = boot_rom_data.is_some();
        let bus = M::new(boot_rom_data, rom_data, ram_data, rom_compatibility)?;

        let cpu = Cpu::new();
        let mut gb = GameBoy::<M> {
            cpu,
            bus,
            step_to_execute: 0,
            should_get_fps: true,
            instructions_to_send: 0,

            cycles_elapsed: 0,
            watched_address: HashSet::new(),
            speed: 1,
            is_paused: false,

            path: None,
        };

        if !with_boot_rom {
            match gb_type {
                GbType::Cgb => gb.simulate_boot_rom_effect_cgb(),
                GbType::Dmg => gb.simulate_boot_rom_effect(),
            }
        }

        gb.cpu.first_read(&mut gb.bus);

        Ok(gb)
    }

    pub fn snapshot(save_state_path: String) -> Result<GameBoy<M>, String>
    where
        M: for<'de> Deserialize<'de>,
    {
        let json = std::fs::read_to_string(&save_state_path)
            .map_err(|e| format!("Could not read save state {save_state_path}: {e}"))?;
        serde_json::from_str(&json)
            .map_err(|e| format!("Could not deserialize save state {save_state_path}: {e}"))
    }

    fn calculate_fps(before: &mut Instant) -> u128 {
        let now = Instant::now();
        let frame: Duration = now.duration_since(*before);
        *before = now;
        Duration::from_secs(1).as_nanos() / frame.as_nanos()
    }

    pub fn launch(mut self, ct: &mut Box<dyn GameCT>) -> Result<Option<Vec<u8>>, String> {
        let mut input = KeyInput::default();
        let mut before = Instant::now();
        let mut debut: Instant;
        let mut mode: GBMode<M> = Self::game_mode;
        loop {
            debut = Instant::now();
            ct.poll_requests()
                .into_iter()
                .for_each(|request| self.treat_request(request, &mut mode));

            if let Err(msg) = ct.update_input(&mut input) {
                eprintln!("Gameboy must stop : {msg}");
                break;
            }
            mode(&mut self, &input, ct);
            if self.should_get_fps {
                ct.update_fps(Self::calculate_fps(&mut before))?;
            }
            let mut wanted_duration =
                Duration::from_micros(GAME_REFRESH_PERIOD_IN_MILLIS / self.speed);
            if self.cpu.is_in_fast_mode {
                wanted_duration /= 2
            }
            let duration_elapsed = debut.elapsed();
            self.cap_frame(wanted_duration, duration_elapsed);
        }
        Ok(GameBoy::<M>::ram_dump(self))
    }

    fn cap_frame(&self, wanted_duration: Duration, duration_elapsed: Duration) {
        let target = wanted_duration;
        let start = Instant::now();

        let duration_of_the_wait = target.saturating_sub(duration_elapsed);

        if duration_of_the_wait > SLEEP_MARGIN {
            thread::sleep(duration_of_the_wait - SLEEP_MARGIN);
        }

        while start.elapsed() < duration_of_the_wait {
            std::hint::spin_loop();
        }
    }
    fn treat_request(&mut self, request: Request, mode: &mut GBMode<M>) {
        match request {
            Request::Mode(new_mode) => match new_mode {
                Mode::Game => {
                    println!("game mode set");
                    *mode = Self::game_mode;
                }
                Mode::Debug => {
                    println!("debug mode set");
                    *mode = Self::debug_mode;
                }
                Mode::Stop => {
                    println!("stopped mode set");
                    *mode = Self::stopped_mode;
                }
                Mode::ByFrame => {
                    println!("frame by frame mode");
                    self.step_to_execute = 1;
                    *mode = Self::frame_by_frame_mode;
                }
                Mode::ByTick => {
                    println!("tick by tick mode set");
                    self.step_to_execute = 1;
                    *mode = Self::tick_by_tick_mode;
                }
                Mode::Snapshot => {
                    println!("snapshot mode set");
                    self.step_to_execute = 1;
                    *mode = Self::snapshot_mode;
                }
            },
            Request::Execute(instructions) => {
                if let Some(instr) = self.cpu.find_instruction(&instructions) {
                    println!(
                        "Found instructions {:#?}. Executing....",
                        (instr.name.clone(), instr.opcode)
                    );
                    let snapshot: ([MicroOp<M>; 8], usize, usize) =
                        (self.cpu.queue, self.cpu.queue_len, self.cpu.op_index);
                    self.cpu.load_instruction(instr.opcode);
                    let mut i = self.cpu.queue_len;

                    while i > 0 {
                        let micro_op = &self.cpu.queue[self.cpu.op_index];
                        self.cpu.op_index += 1;
                        micro_op(&mut self.cpu, &mut self.bus);
                        i -= 1;
                    }
                    self.cpu.queue = snapshot.0;
                    self.cpu.queue_len = snapshot.1;
                    self.cpu.op_index = snapshot.2;
                } else {
                    eprintln!("Haven't found any instructions named {:?}", instructions);
                }
            }
            Request::RenderFrame(frame) => {
                if std::ptr::fn_addr_eq(*mode, Self::stopped_mode as GBMode<M>) {
                    todo!()
                }
            }
            Request::Watch(address) => {
                self.watched_address.insert(address);
            }
            Request::Step(step) => {
                self.step_to_execute = step;
            }
            Request::SetInstructionListLength(length) => {
                self.instructions_to_send = length as u16;
            }
            Request::StopWatch(address) => {
                self.watched_address.remove(&address);
            }
            Request::SetSpeed(speed) => {
                self.speed = speed as u64;
                self.bus.get_apu().set_speed(speed as f64);
            }
            Request::SetVolume(volume) => {
                self.bus.get_apu().set_volume(volume);
            }
            Request::SaveState(path) => {
                GbmuFile::create_save_state(&path, self);
                *mode = Self::game_mode;
            }
            _ => unreachable!(),
        }
    }

    pub fn simulate_boot_rom_effect(&mut self) {
        self.cpu.set_r8::<A>(0x01);
        self.cpu.set_r8::<B>(0xFF);
        self.cpu.set_r8::<C>(0x13);
        self.cpu.set_r8::<D>(0x00);
        self.cpu.set_r8::<E>(0xC1);
        self.cpu.set_r8::<H>(0x84);
        self.cpu.set_r8::<L>(0x03);
        self.cpu.set_r8::<F>(0x00);
        self.cpu.set_r16::<PC>(0x0100);
        self.cpu.set_r16::<SP>(0xFFFE);

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

    pub fn simulate_boot_rom_effect_cgb(&mut self) {
        // CGB flag from the cartridge header: bit 7 set means the game runs
        // in CGB mode, otherwise the boot ROM enables DMG compatibility mode.
        let cgb_flag = self.bus.read_byte(0x0143);
        let compatibility = cgb_flag & 0x80 == 0;

        self.cpu.set_r8::<A>(0x11);
        self.cpu.set_r8::<F>(0x80);
        self.cpu.set_r8::<B>(0x00);
        self.cpu.set_r8::<C>(0x00);
        if compatibility {
            self.cpu.set_r8::<D>(0x00);
            self.cpu.set_r8::<E>(0x08);
            self.cpu.set_r8::<H>(0x00);
            self.cpu.set_r8::<L>(0x7C);
        } else {
            self.cpu.set_r8::<D>(0xFF);
            self.cpu.set_r8::<E>(0x56);
            self.cpu.set_r8::<H>(0x00);
            self.cpu.set_r8::<L>(0x0D);
        }
        self.cpu.set_r16::<PC>(0x0100);
        self.cpu.set_r16::<SP>(0xFFFE);

        self.bus.write_byte(0xFF00, 0xC7);
        self.bus.write_byte(0xFF01, 0x00);
        self.bus.write_byte(0xFF02, 0x7F);
        self.bus.write_byte(0xFF04, 0xFF);
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
        self.bus.write_byte(0xFF42, 0x00);
        self.bus.write_byte(0xFF43, 0x00);
        self.bus.write_byte(0xFF45, 0x00);
        self.bus.write_byte(0xFF47, 0xFC);
        self.bus.write_byte(0xFF48, 0xFF);
        self.bus.write_byte(0xFF49, 0xFF);
        self.bus.write_byte(0xFF4A, 0x00);
        self.bus.write_byte(0xFF4B, 0x00);
        // The KEY0 write also drives OPRI through the PPU handler: bit 2 set
        // (DMG compatibility) selects X-coordinate sprite priority.
        if compatibility {
            self.bus.write_byte(0xFF4C, 0x04);
        } else {
            self.bus.write_byte(0xFF4C, cgb_flag);
        }
        self.bus.write_byte(0xFF4D, 0x7E);
        self.bus.write_byte(0xFF4F, 0xFE);
        self.bus.write_byte(0xFF51, 0xFF);
        self.bus.write_byte(0xFF52, 0xFF);
        self.bus.write_byte(0xFF53, 0xFF);
        self.bus.write_byte(0xFF54, 0xFF);
        self.bus.write_byte(0xFF56, 0x3E);
        // The CGB boot ROM fills palette RAM before handing control to the
        // game; DMG-only games never write CRAM themselves, so without this
        // the whole screen stays black in compatibility mode.
        const GRAYSCALE: [u16; 4] = [0x7FFF, 0x6318, 0x318C, 0x0000];
        self.bus.write_byte(0xFF68, 0x80);
        for color in GRAYSCALE {
            self.bus.write_byte(0xFF69, (color & 0xFF) as u8);
            self.bus.write_byte(0xFF69, (color >> 8) as u8);
        }
        self.bus.write_byte(0xFF6A, 0x80);
        for _ in 0..2 {
            for color in GRAYSCALE {
                self.bus.write_byte(0xFF6B, (color & 0xFF) as u8);
                self.bus.write_byte(0xFF6B, (color >> 8) as u8);
            }
        }
        self.bus.write_byte(0xFF68, 0xFF);
        self.bus.write_byte(0xFF6A, 0xFF);
        self.bus.write_byte(0xFF70, 0xF8);
        self.bus.write_byte(0xFFFF, 0x00);
    }

    pub fn manage_input(&mut self, key_input: &KeyInput) {
        let mut dpad = 0x0F;
        if key_input.down_pushed {
            dpad &= 0b1111_0111;
        }
        if key_input.up_pushed {
            dpad &= 0b1111_1011;
        }
        if key_input.left_pushed {
            dpad &= 0b1111_1101;
        }
        if key_input.right_pushed {
            dpad &= 0b1111_1110;
        }

        let mut buttons = 0x0F;
        if key_input.start_pushed {
            buttons &= 0b1111_0111;
        }
        if key_input.select_pushed {
            buttons &= 0b1111_1011;
        }
        if key_input.b_pushed {
            buttons &= 0b1111_1101;
        }
        if key_input.a_pushed {
            buttons &= 0b1111_1110;
        }

        self.bus.update_keys(dpad, buttons)
    }

    pub fn tick_gb(
        &mut self,
        key_input: &KeyInput,
        ct: &mut Box<dyn GameCT>,
    ) -> (CpuQueueState, PpuMode) {
        self.manage_input(key_input);
        self.bus.tick_timers();
        self.cycles_elapsed += 1;
        let mut queue_state = CpuQueueState::ExecutingInstructions;
        if self.cycles_elapsed.is_multiple_of(4) {
            if self.bus.get_dma_index() != 0xFF {
                self.bus.tick_dma();
            }
            queue_state = self.cpu.tick(&mut self.bus);
            self.cycles_elapsed = 0;
        }
        let mut ppu_state = PpuMode::OamSearch;
        if !self.cpu.is_in_fast_mode || self.cycles_elapsed.is_multiple_of(2) {
            ppu_state = self.bus.tick_ppu(ct, self.cpu.halted);
            self.bus.tick_apu();
        }
        (queue_state, ppu_state)
    }

    fn game_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        for _ in 0..FRAME_CYCLES {
            self.tick_gb(key_input, ct);
        }
    }

    fn debug_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        for _ in 0..FRAME_CYCLES {
            if let (CpuQueueState::NewInstructionFetched(addr), _) = self.tick_gb(key_input, ct) {
                self.send_next_instructions(ct, addr);
            }
            self.send_watched_adress(ct);
            self.send_registers(ct);
        }
    }

    fn tick_by_tick_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        while self.step_to_execute > 0 {
            loop {
                if let (CpuQueueState::NewInstructionFetched(addr), _) = self.tick_gb(key_input, ct)
                {
                    self.send_next_instructions(ct, addr);
                    break;
                }
                self.send_watched_adress(ct);
                self.send_registers(ct);
            }
            self.step_to_execute -= 1;
        }
        self.send_watched_adress(ct);
        self.send_registers(ct);
    }

    fn frame_by_frame_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        while self.step_to_execute > 0 {
            println!("frame by frame");
            loop {
                if let (CpuQueueState::NewInstructionFetched(addr), PpuMode::HBlank) =
                    self.tick_gb(key_input, ct)
                {
                    self.send_next_instructions(ct, addr);
                    break;
                }
                self.send_watched_adress(ct);
                self.send_registers(ct);
            }
            println!("Hblank");
            loop {
                if let (CpuQueueState::NewInstructionFetched(addr), PpuMode::VBlank) =
                    self.tick_gb(key_input, ct)
                {
                    self.send_next_instructions(ct, addr);
                    break;
                }
                self.send_watched_adress(ct);
                self.send_registers(ct);
            }
            println!("vblank");
            self.step_to_execute -= 1;
        }
    }

    fn stopped_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        while self.step_to_execute > 0 {
            loop {
                if let (CpuQueueState::NewInstructionFetched(addr), _) = self.tick_gb(key_input, ct)
                {
                    self.send_next_instructions(ct, addr);
                    break;
                }
            }
            self.step_to_execute -= 1;
        }
        self.send_watched_adress(ct);
        self.send_registers(ct);
        self.step_to_execute = 0;
    }

    fn snapshot_mode(&mut self, key_input: &KeyInput, ct: &mut Box<dyn GameCT>) {
        if let Some(path) = self.path.take() {
            while self.step_to_execute > 0 {
                loop {
                    if let (CpuQueueState::NewInstructionFetched(addr), PpuMode::VBlank) =
                        self.tick_gb(key_input, ct)
                    {
                        self.cpu.set_r16::<PC>(addr);
                        break;
                    }
                }
                self.step_to_execute -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mmu::CgbMmu;
    use crate::mmu::DmgMmu;
    use crate::mmu::mbc::RomOnly;
    use crate::mmu::timers::DmgTimers;
    use crate::ppu::{CgbPpu, DmgPpu};

    #[test]
    fn dmg_gameboy_round_trips_through_json_with_same_generics() {
        let mut gb: GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>> =
            GameBoy::new(None, vec![], None, false, GbType::Dmg).expect("Failed to create gb");

        gb.cpu.set_r8::<A>(0x11);
        gb.bus.write_byte(0xC000, 0x42);
        gb.watched_address.insert(0x1234);
        gb.cycles_elapsed = 7;
        gb.speed = 2;
        gb.is_paused = true;

        let json = serde_json::to_string(&gb).expect("failed to serialize GameBoy");

        let mut restored: GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>> = serde_json::from_str(&json)
            .expect("failed to deserialize GameBoy back into the same generic instantiation");

        assert_eq!(restored.cpu.get_r8::<A>(), 0x11);
        assert_eq!(restored.bus.read_byte(0xC000), 0x42);
        assert!(restored.watched_address.contains(&0x1234));
        assert_eq!(restored.cycles_elapsed, 7);
        assert_eq!(restored.speed, 2);
        assert!(restored.is_paused);
    }

    #[test]
    fn cgb_gameboy_round_trips_through_json_with_same_generics() {
        let mut gb: GameBoy<CgbMmu<RomOnly, DmgTimers, CgbPpu>> =
            GameBoy::new(None, vec![], None, false, GbType::Dmg).expect("Failed to create gb");

        gb.cpu.set_r8::<A>(0x22);
        gb.bus.write_byte(0xC010, 0x99);
        gb.watched_address.insert(0x5678);

        let json = serde_json::to_string(&gb).expect("failed to serialize GameBoy");

        let mut restored: GameBoy<CgbMmu<RomOnly, DmgTimers, CgbPpu>> = serde_json::from_str(&json)
            .expect("failed to deserialize GameBoy back into the same generic instantiation");

        assert_eq!(restored.cpu.get_r8::<A>(), 0x22);
        assert_eq!(restored.bus.read_byte(0xC010), 0x99);
        assert!(restored.watched_address.contains(&0x5678));
    }

    #[test]
    fn snapshot_reports_an_error_for_a_missing_file() {
        let result: Result<GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>>, String> =
            GameBoy::snapshot("/nonexistent/gbmu_snapshot.json".to_string());
        assert!(result.is_err());
    }
}
