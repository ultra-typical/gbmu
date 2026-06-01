#![allow(unreachable_code)]

use crate::cpu::{Cpu, CpuDTO};
use crate::gameboy::GameBoy;
use crate::gui::{DebugCommandQueries, DebugResponse, KeyInput, LaunchGameData, WatchedAdresses};
use crate::mmu::mbc::Mbc;
use crate::ppu::{Ppu, PpuDTO};
use crate::mmu::Mmu;
use std::cell::RefCell;
use std::sync::atomic::AtomicI16;
use std::sync::atomic::Ordering::Relaxed;
use std::fs;
use std::path::Path;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    Mutex
};
use std::io::{BufReader, Write};
use std::rc::Rc;
use std::error;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{Receiver, Sender};

const SAVE_CPU_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/gbmu_save_states/save_cpu.json");
const SAVE_PPU_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/gbmu_save_states/save_ppu.json");
const SAVE_BUS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/gbmu_save_states/save_bus.json");

pub struct GameApp<T: Mbc> {
    updated_image_boolean: Arc<AtomicBool>,
    is_debug_mode: Arc<AtomicBool>,
    gameboy: GameBoy<T>,
    input_receiver: Receiver<KeyInput>,
    debug_receiver: Receiver<DebugCommandQueries>,
    debug_sender: Sender<DebugResponse>,
    is_step_mode: bool,
    nb_next_intruction: u8,
    is_sending_registers: bool,
    watched_adress: WatchedAdresses,
    fps_counter: Arc<AtomicI16>,
}

impl<T: Mbc> GameApp<T> {
    pub fn ram_dump(self) -> Option<Vec<u8>> {
        self.gameboy.bus.borrow_mut().ram_dump()
    }

    pub fn new(
        rom_data: Vec<u8>,
        ram_data: Option<Vec<u8>>,
        game_data: LaunchGameData,
    ) -> Result<Self, String> {
        let boot_rom = if game_data.boot_rom {
            let boot_bytes = std::fs::read("boot-roms/dmg.bin").expect("cannot read boot rom");
            assert!(boot_bytes.len() == 0x100, "boot rom must be 256 bytes");

            let mut boot_rom = [0u8; 0x0100];
            boot_rom.copy_from_slice(&boot_bytes);
            Some(boot_rom)
        } else { None };
        let gameboy = GameBoy::<T>::new(rom_data, boot_rom, ram_data, game_data.actual_image)?;

        let mut app = Self {
            gameboy,
            updated_image_boolean: game_data.updated_image_boolean,
            debug_receiver: game_data.command_query_receiver,
            debug_sender: game_data.debug_response_sender,
            input_receiver: game_data.input_receiver,
            is_step_mode: false,
            is_debug_mode: game_data.global_is_debug,
            is_sending_registers: false,
            nb_next_intruction: 0,
            watched_adress: WatchedAdresses {
                addresses_n_values: Vec::new(),
            },
            fps_counter: game_data.fps_counter
        };
        if !game_data.boot_rom { 
            app.gameboy.simulate_boot_rom_effect()
        }
        Ok(app)
    }

    pub fn launch(mut self) -> Result<Option<Vec<u8>>, String>{
        let mut input = KeyInput::default();
        let debut = Instant::now();
        let mut old_instant_frame_in_ms = debut.elapsed().as_millis();
        let mut cycle = 0;
        //TODO: A changer / enlever
        if Path::new(SAVE_BUS_PATH).exists() &&
            Path::new(SAVE_CPU_PATH).exists() &&
            Path::new(SAVE_PPU_PATH).exists() {
                let res_load_state = self.load_state();
                match res_load_state {
                    Ok(()) => (),
                    Err(err) => eprintln!("{}", err)
                }
        }
        let debut = Instant::now();
        let mut old_instant_frame_in_ms = debut.elapsed().as_millis();
        loop {
            use tokio::sync::mpsc::error::TryRecvError;
            match self.input_receiver.try_recv(){
                Ok(new_input) => input = new_input,
                Err(TryRecvError::Empty) => {},
                Err(TryRecvError::Disconnected) => break,
            }
            let buffer_was_updated = self.update(&input);
            if buffer_was_updated {
                self.updated_image_boolean.store(true, Ordering::Relaxed);
            }
            get_fps(debut, &mut old_instant_frame_in_ms, &self.fps_counter);
            if cycle == 1000 {
                self.save_state().unwrap();

            //TODO: Enlever
            if cycle == 500 {
                self.save_state();
                break;
            }
            cycle += 1;
        }
        Ok(self.ram_dump())
    }

    fn send_watched_address(&mut self) {
        if !self.watched_adress.addresses_n_values.is_empty() {
            let mut values = WatchedAdresses {
                addresses_n_values: Vec::new(),
            };
            for (addr, _) in self.watched_adress.addresses_n_values.iter() {
                let bus = self.gameboy.bus.borrow_mut();
                let value = bus.read_byte(*addr);
                values.addresses_n_values.push((*addr, value as u16));
            }
            let _ = self
                .debug_sender
                .try_send(DebugResponse::AddressesWatched(values));
        }
    }

    fn send_registers(&mut self) {
        let _ = self.debug_sender.try_send(DebugResponse::Registers(
            self.gameboy.cpu.registers.get_a(),
            self.gameboy.cpu.registers.get_b(),
            self.gameboy.cpu.registers.get_c(),
            self.gameboy.cpu.registers.get_d(),
            self.gameboy.cpu.registers.get_e(),
            self.gameboy.cpu.registers.get_h(),
            self.gameboy.cpu.registers.get_l(),
            self.gameboy.cpu.registers.get_flags_u8() as u16,
            self.gameboy.cpu.registers.get_sp(),
            self.gameboy.cpu.pc,
        ));
    }

    fn send_next_instructions(&mut self) {
        let mut v = vec![self.gameboy.cpu.pc];
        for current_instruction in 1..self.nb_next_intruction {
            v.push(
                self.gameboy
                    .bus
                    .borrow_mut()
                    .read_byte(self.gameboy.cpu.pc.wrapping_add(current_instruction as u16))
                    as u16,
            );
        }
        let _ = self
            .debug_sender
            .try_send(DebugResponse::NextInstructions(v));
    }

    pub fn update(&mut self, keys_down: &KeyInput) -> bool {
        let mut instruction_to_execute = !self.is_step_mode as usize;
        let is_debug = self.is_debug_mode.load(Ordering::Relaxed);
        if is_debug {
            while let Ok(debug) = self.debug_receiver.try_recv() {
                match debug {
                    DebugCommandQueries::ExecuteInstruction(instruction) => {
                        println!("execute instruction received! {instruction}");
                        self.gameboy.cpu.debug_step(instruction);
                        let _ = self
                            .debug_sender
                            .try_send(DebugResponse::InstructionsExecuted(instruction as usize));
                    }
                    DebugCommandQueries::GetNextInstructions(instr_nb) => {
                        println!("get next instruction received! {instr_nb}");
                        self.nb_next_intruction = instr_nb;
                        self.send_next_instructions();
                    }
                    DebugCommandQueries::GetRegisters => {
                        println!("get registers received!");
                        self.is_sending_registers = !self.is_sending_registers;
                        self.send_registers();
                    }
                    DebugCommandQueries::SetStepMode => {
                        println!("set step mode rs received!");
                        self.is_step_mode = !self.is_step_mode;
                        let _ = self
                            .debug_sender
                            .try_send(DebugResponse::StepModeSet(self.is_step_mode));
                    }
                    DebugCommandQueries::WatchAddress(addr) => {
                        if !self
                            .watched_adress
                            .addresses_n_values
                            .iter()
                            .any(|(a, _)| *a == addr)
                        {
                            self.watched_adress.addresses_n_values.push((addr, 0));
                        } else if let Some(index) = self
                            .watched_adress
                            .addresses_n_values
                            .iter()
                            .position(|(address, _)| *address == addr)
                        {
                            self.watched_adress.addresses_n_values.remove(index);
                        }
                        self.send_watched_address();
                    }
                    DebugCommandQueries::ExecuteNextInstructions(nb_instruction) => {
                        instruction_to_execute = nb_instruction;
                        println!("execute next instruction received! {nb_instruction}");
                    }
                    DebugCommandQueries::GetAddresses => {
                        self.send_watched_address();
                    }
                }
            }
        }

        let mut frame_was_edited = false;
        if is_debug {
            for _ in 0..instruction_to_execute {
                frame_was_edited = self.gameboy.run_frame(keys_down);
                self.send_next_instructions();
                self.send_watched_address();
                self.send_registers();
            }
            frame_was_edited
        } else {
            self.gameboy.run_frame(keys_down)
        }
    }

    // fn rgb_to_rgba(rgb_frame: &[u8]) -> Vec<u8> {
    //     let mut rgba_frame = Vec::with_capacity(ppu::WIN_SIZE_X * ppu::WIN_SIZE_Y * 4);
    //     for chunk in rgb_frame.chunks(3) {
    //         rgba_frame.extend_from_slice(chunk);
    //         rgba_frame.push(255);
    //     }
    //     rgba_frame
    // }
    pub fn save_state(&self) {
        struct StringByComponents {
            path_file: String, error_file: String, json_content: String, error_json: String
        }
        let string_by_components = [
            StringByComponents {
                path_file: SAVE_CPU_PATH.to_string(),
                error_file: String::from("failed to create save_cpu.json"),
                json_content: String::from("cpu"),
                error_json: String::from("failed to serialize cpu")
            },
            StringByComponents {
                path_file: SAVE_PPU_PATH.to_string(),
                error_file: String::from("failed to create save_ppu.json"),
                json_content: String::from("ppu"),
                error_json: String::from("failed to serialize ppu")
            },
            StringByComponents {
                path_file: SAVE_BUS_PATH.to_string(),
                error_file: String::from("failed to create save_bus.json"),
                json_content: String::from("bus"),
                error_json: String::from("failed to serialize bus")
            },
        ];
        let save_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/gbmu_save_states");
        fs::create_dir_all(save_dir).expect("failed to create save directory");
        for component in string_by_components {
            let file: fs::File;
            let json: String;
            file = fs::File::create(component.path_file).expect(&component.error_file);
            json = self.create_json(&component.json_content).expect(&component.error_json);
            let ret_save_file = self.write_save_to_file(file, json);
            match ret_save_file {
                Ok(()) => (),
                Err(error) => println!("{}", error)
            }
        }
    }

    fn write_save_to_file(&self, mut file: std::fs::File, json: String) -> Result<(), String> {
        let ret = file.write_all(json.as_bytes());
        let _: () = ret.unwrap();
        Ok(())
    }

    fn create_json(&self, content: &str) -> Result<String, serde_json::Error> {
        match content {
            "cpu" => serde_json::to_string_pretty(&self.gameboy.cpu),
            "ppu" => serde_json::to_string_pretty(&self.gameboy.ppu),
            "bus" => serde_json::to_string_pretty(&self.gameboy.bus),
            _ => todo!()
        }
    }

    pub fn load_state(&mut self) -> Result<(), Box<dyn error::Error>> {
        let components: [(&str, &str); 3] = [
            ("bus", SAVE_BUS_PATH),
            ("cpu", SAVE_CPU_PATH),
            ("ppu", SAVE_PPU_PATH)
        ];
        for component in components {
            self.load_component(component.0, component.1);
        }
        Ok(())
    }

    fn load_component(&mut self, component: &str, path: &str) {
        let file = fs::File::open(path).expect(&format!("failed to open {}", path));
        let reader = BufReader::new(file);
        match component {
            "cpu" => {
                let cpu_dto: CpuDTO = serde_json::from_reader(reader).expect("failed to deserialize cpu file");
                self.gameboy.cpu = Cpu::from_dto(cpu_dto, self.gameboy.bus.clone());
            },
            "ppu" => {
                let ppu_dto: PpuDTO = serde_json::from_reader(reader).expect("failed to deserialize ppu file");
                self.gameboy.ppu = Ppu::from_dto(ppu_dto, self.gameboy.bus.clone());
            },
            "bus" => {
                let bus: Rc<RefCell<Mmu<T>>> = serde_json::from_reader(reader).expect("failed to deserialize bus file");
                self.gameboy.bus = bus;
            }
            _ => todo!()
        }
    }

}

fn get_fps(base_instant: Instant, old_frame_ms: &mut u128, fps: &AtomicI16) {
    let one_second = Duration::from_secs(1);
    let now_ms = base_instant.elapsed().as_millis();
    let diff = now_ms.saturating_sub(*old_frame_ms).max(1);
    let current_fps = one_second.as_millis() / diff;
    *old_frame_ms = now_ms;
    fps.store(current_fps.try_into().unwrap(), Relaxed);
}
