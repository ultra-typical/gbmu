#![cfg_attr(test, allow(clippy::all))]
#![allow(unused_variables)]

mod common;
mod views;

use std::path::{Path, PathBuf};
use std::time::Duration;
use std::thread;
use egui_file_dialog::{FileDialog, Filter};
use crate::communications::{CpuState, GameCT, InstructionList, InterfaceCT, WatchedAdresses, create_communication_tools};
use crate::gameboy::GameBoy;
use crate::mmu::DmgMmu;
use crate::mmu::mbc::{Mbc1, Mbc2, Mbc3, RomOnly};
use crate::mmu::timers::GbaTimers;
use crate::ppu::{self, GbaPpu};
use eframe::egui::{Key, TextureHandle};
use eframe::egui::{load::SizedTexture, vec2, ColorImage, TextureOptions};
use std::collections::HashSet;

use std::time::Instant;

#[derive(Default)]
pub struct GraphicalApp {
    app_state: AppState,
}

pub mod themes;
use eframe::egui;
use tokio::task::JoinHandle;

const UI_REFRESH_PERIOD_IN_MILLIS: u64 = 30;

impl eframe::App for GraphicalApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let debut = Instant::now();
        
        self.app_state = match std::mem::replace(&mut self.app_state, AppState::Default) {
            AppState::StartingHub(device) => device.starting_view(_ui, _frame),
            AppState::SelectionHub(device) => device.selection_view(_ui, _frame),
            AppState::EmulationHub(device) => device.emulation_view(_ui, _frame),
            AppState::DebuggingHub(device) => device.debug_view(_ui, _frame),
            AppState::Default => unreachable!(),
        };
        let keys_down = _ui.ctx().input(|i| { i.keys_down.clone()});
        if keys_down.contains(&Key::Escape) { 
            _ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }

        let wanted_duration = Duration::from_millis(UI_REFRESH_PERIOD_IN_MILLIS);
        let duration_elapsed = debut.elapsed();
        if wanted_duration > duration_elapsed {
            thread::sleep(wanted_duration - duration_elapsed);
        }   
        _ui.ctx().request_repaint();
    }
}

pub struct EmulationAppOptions {
    rom_path: String,
    boot_rom: bool,
}

#[derive(PartialEq, Debug)]
pub enum GbType {
    Cgb,
    Dmg
}

impl GbType {
    fn supported_types(code: u8) -> Vec<GbType> {
        match code {
            0x80 => vec![GbType::Cgb, GbType::Dmg],
            _ => vec![GbType::Cgb],
        }
    }
}

pub struct CoreGameOptions {
    pub boot_rom_path: String,
    pub rom_path: String,
    pub boot_rom: bool,
    pub gbtype: GbType,
}

impl From<EmulationAppOptions> for CoreGameOptions {
    fn from(value: EmulationAppOptions) -> Self {
        Self {
            boot_rom_path: "boot-roms/dmg.bin".into(),
            rom_path: value.rom_path,
            boot_rom: value.boot_rom,
            gbtype: GbType::Cgb, // TODO -> permettre le choix du type par EmulationAppOptions
        }
    }
}

impl EmulationAppOptions {
    pub fn new(rom_path: String, boot_rom: bool) -> Self{
        Self {
            rom_path, boot_rom
        }
    }
}


impl GraphicalApp {
    pub fn create_emulation_app(options: EmulationAppOptions) -> Self {

        Self {
            app_state: AppState::EmulationHub(
                EmulationDevice {
                    core_game: CoreGameDevice::new(options.into())
                }
            ),
        }
    }
}

#[derive(Default)]
pub struct StartingHubDevice {}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct KeyInput{
    pub a_pushed: bool,
    pub b_pushed: bool,
    pub select_pushed: bool,
    pub start_pushed: bool,
    pub up_pushed: bool,
    pub down_pushed: bool,
    pub left_pushed: bool,
    pub right_pushed: bool,
}

impl From<&KeyInput> for bool {
    fn from(val: &KeyInput) -> Self {
        val.a_pushed ||
        val.b_pushed ||
        val.select_pushed ||
        val.start_pushed ||
        val.up_pushed ||
        val.down_pushed ||
        val.left_pushed ||
        val.right_pushed 
    }
}

pub struct KeyMapping{
    pub a: Key,
    pub b: Key,
    pub select: Key,
    pub start: Key,
    pub up: Key,
    pub down: Key,
    pub left: Key,
    pub right: Key,
}

impl Default for KeyMapping {
    fn default() -> Self {
        KeyMapping {
            a: Key::J,
            b: Key::K,
            select: Key::N,
            start: Key::M,
            up: Key::W,
            down: Key::S,
            left: Key::A,
            right: Key::D,
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum AppState {
    StartingHub(StartingHubDevice),
    SelectionHub(SelectionDevice),
    EmulationHub(EmulationDevice),
    DebuggingHub(DebuggingDevice),
    Default,
}

use std::fs;
use std::process;


pub enum AnyGameApp {
    GbaOnlyRom(GameBoy<GbaMmu<RomOnly, GbaTimers, GbaPpu>>),
    CgbOnlyRom(GameBoy<GbaMmu<RomOnly, GbaTimers, GbaPpu>>),
    GbaMbc1(GameBoy<GbaMmu<Mbc1, GbaTimers, GbaPpu>>),
    CgbMbc1(GameBoy<GbaMmu<Mbc1, GbaTimers, GbaPpu>>),
    GbaMbc2(GameBoy<GbaMmu<Mbc2, GbaTimers, GbaPpu>>),
    CgbMbc2(GameBoy<GbaMmu<Mbc2, GbaTimers, GbaPpu>>),
    GbaMbc3(GameBoy<GbaMmu<Mbc3, GbaTimers, GbaPpu>>),
    CgbMbc3(GameBoy<GbaMmu<Mbc3, GbaTimers, GbaPpu>>),
}



impl AnyGameApp {
    pub fn new(game_data: CoreGameOptions) -> Result<Self, String> {
        let rom_data: Vec<u8> = Self::read_rom(&game_data.rom_path);
        let ram_path = game_data.rom_path.to_owned() + ".save";
        let ram_data: Option<Vec<u8>> = Self::read_ram(&ram_path);
        if ram_data.is_some() { println!("Backup detected") };
        let boot_rom_data = if game_data.boot_rom {
            let boot_bytes = std::fs::read(game_data.boot_rom_path).expect("cannot read boot rom");
            assert!(boot_bytes.len() == 0x100, "boot rom must be 256 bytes");

            let mut boot_rom = [0u8; 0x0100];
            boot_rom.copy_from_slice(&boot_bytes);
            Some(boot_rom)
        } else { None };

        println!("new AnyGameApp");

        let mbc_code = rom_data[0x0147];
        let supported_gb_types = GbType::supported_types(rom_data[0x0143]);

        if !supported_gb_types.contains(&game_data.gbtype) {
            return Err(format!("Cartridge doesn't support type {:#?}", game_data.gbtype));
        }

        match game_data.gbtype {
            GbType::Cgb => {
                match mbc_code {
                    0x00 | 0x08 | 0x09 =>  {
                        println!("OnlyRom detected");
                        Ok(
                            AnyGameApp::CgbOnlyRom(GameBoy::new(
                                boot_rom_data,
                                rom_data,
                                ram_data,
                            )?)
                        )
                    }
                    0x01..=0x03 => {
                        println!("Mbc1 detected");
                        Ok(
                            AnyGameApp::CgbMbc1(GameBoy::new(
                                boot_rom_data,
                                rom_data,
                                ram_data,
                            )?)
                        )
                    }
                    0x05 | 0x06 => {
                        println!("Mbc2 detected");
                        Ok(
                            AnyGameApp::CgbMbc2(GameBoy::new(
                                boot_rom_data,
                                rom_data,
                                ram_data,
                            )?)
                        )
                    }
                    0x0F..=0x13 => {
                        println!("Mbc3 detected");
                        Ok(
                            AnyGameApp::CgbMbc3(GameBoy::new(
                                boot_rom_data,
                                rom_data,
                                ram_data,
                            )?)
                        )
                    }
                    /*
                    0x0B | 0x0C | 0x0D => Ok(todo!()), // MMM01 pas dans le sujet
                    0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E => Ok(todo!()), // Mbc5
                    0x20 => Ok(todo!()), // Mbc6
                    0x22 => Ok(todo!()),// MBC7+SENSOR+RUMBLE+RAM+BATTERY
                    */
                    _ => Err("Unmanaged cartridge type".into())

                }
            }
            GbType::Dmg => todo!()
        }

    }

    fn read_ram(ram_path: &String) -> Option<Vec<u8>> {
        fs::read(ram_path).ok()
    }

    fn read_rom(rom_path: &String) -> Vec<u8> {
        if !rom_path.is_empty() {
            match fs::read(rom_path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Failed to read the file: {e}");
                    process::exit(1);
                }
            }
        } else {
            eprintln!("Failed to read the file: {rom_path} : path is empty");
            process::exit(1);
        }
    }

    pub fn launch(self, ct: Box<dyn GameCT>) -> Result<Option<Vec<u8>>, String>{
        match self {
            AnyGameApp::DmgOnlyRom(g) => g.launch(ct),
            AnyGameApp::CgbOnlyRom(g) => g.launch(ct),
            AnyGameApp::DmgMbc1(g) => g.launch(ct),
            AnyGameApp::CgbMbc1(g) => g.launch(ct),
            AnyGameApp::DmgMbc2(g) => g.launch(ct),
            AnyGameApp::CgbMbc2(g) => g.launch(ct),
            AnyGameApp::DmgMbc3(g) => g.launch(ct),
            AnyGameApp::CgbMbc3(g) => g.launch(ct),
        }
    }
}


async fn async_launch_game(
    game_data: CoreGameOptions,
    ct: Box<dyn GameCT>
) -> Result<(), String> {
    let rom_path = game_data.rom_path.clone();
    let app = AnyGameApp::new(game_data)?;
    if let Some(value) = app.launch(ct)? {
        let save_path = rom_path.clone() + ".save";
        eprintln!("attempting to save game ram to {}", save_path);
        fs::write(save_path, value).unwrap_or_else(
            |err| eprintln!("backup was unsucessfull {:?}", err)
        );
    }
    Ok(())
}

use crate::communications::FRAME_SIZE_IN_U8;

pub struct CoreGameDevice {
    pub handler: JoinHandle<Result<(), String>>,
    buffer: [u8; FRAME_SIZE_IN_U8],
    pub sized_image: Option<SizedTexture>,
    texture_handler: Option<TextureHandle>,
    key_mapping: KeyMapping,
    pub interface_ct: Box<dyn InterfaceCT>,
}

impl KeyMapping {
    pub fn generate_key_input(&self, keys_down: HashSet<Key>) -> KeyInput {
        KeyInput {
            a_pushed: keys_down.contains(&self.a),
            b_pushed: keys_down.contains(&self.b),
            select_pushed: keys_down.contains(&self.select),
            start_pushed: keys_down.contains(&self.start),
            up_pushed: keys_down.contains(&self.up),
            down_pushed: keys_down.contains(&self.down),
            left_pushed: keys_down.contains(&self.left),
            right_pushed: keys_down.contains(&self.right),
        }
    }
}

impl Drop for CoreGameDevice {
    fn drop(&mut self) {
        println!("this was droped");
        self.handler.abort();
    }
}

impl CoreGameDevice {
    pub fn update_and_size_image(&mut self, ui: &mut egui::Ui) -> Result<(), String> {

        let Some(()) = self.interface_ct.get_new_image(&mut self.buffer)? else {
            return Ok(());
        };

        let loaded_image =  ColorImage::from_rgb([ppu::WIN_SIZE_X, ppu::WIN_SIZE_Y], &self.buffer);       
        if let Some(th) = &mut self.texture_handler {
            th.set(loaded_image, TextureOptions::NEAREST);
        } else {
            self.texture_handler = Some(ui.ctx().load_texture("gb_frame", loaded_image, TextureOptions::NEAREST));
        }

        if let Some(th) = &self.texture_handler {
            let scaled_size = vec2(ppu::WIN_SIZE_X as f32 * 4., ppu::WIN_SIZE_Y as f32 * 4.);
            let sized_texture = SizedTexture::new(th.id(), scaled_size);
            self.sized_image = Some(sized_texture);
        }
        Ok(())
    }

    pub fn capture_and_send_input(&self, ui: &mut egui::Ui) {
        let keys_down= ui.ctx().input(|i| { i.keys_down.clone() });
        let input = self.key_mapping.generate_key_input(keys_down);
        _ = self.interface_ct.send_input(input);
    }

    fn new(options: CoreGameOptions) -> Self {
        let (game_ct, interface_ct) = create_communication_tools();

        Self {
            interface_ct,
            handler: tokio::spawn(async_launch_game(
                options,
                game_ct,
            )),
            buffer: [0; FRAME_SIZE_IN_U8],
            texture_handler: None,
            sized_image: None,
            key_mapping: KeyMapping::default(),
        }
    }
}

pub struct SelectionDevice {
    path: String,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
}

impl Default for SelectionDevice {
    fn default() -> Self {
        Self {
            path: String::from("./"),
            picked_file: None,
            file_dialog: FileDialog::new()
                .default_size([600.0, 400.0])
                .set_file_icon(
                    "🎮",
                    Filter::new(
                        |path: &Path|
                        path.extension().unwrap_or_default() == "gb"
                        || path.extension().unwrap_or_default() == "gbc"
                    )
                )
                .add_file_filter(
                    "GameBoy ROMS",
                    Filter::new(
                        |path: &Path|
                        path.extension().unwrap_or_default() == "gb"
                        || path.extension().unwrap_or_default() == "gbc"
                    )
                )
                .default_file_filter(
                    "GameBoy ROMS"
                )
        }
    }
}

pub struct EmulationDevice {
    pub core_game: CoreGameDevice,
}


pub struct DebuggingDevice {
    pub core_game: CoreGameDevice,
/*
    Info stored for the GUI to use them;
    These are the responses from the sending/receiving operation
*/
    pub next_instructions: InstructionList,
    pub watched_adress: WatchedAdresses,
    pub registers: CpuState,
    pub is_step: bool,
    pub nb_instruction: usize,

    pub error_message: Option<String>,
    pub hex_string: String,
}

impl Default for AppState {
    fn default() -> Self { Self::StartingHub(Default::default()) }
}
