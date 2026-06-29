#![cfg_attr(test, allow(clippy::all))]
#![allow(unused_variables)]

mod common;
mod views;
pub mod keymapping;

use crate::communications::{
    CpuState, GameCT, InstructionList, InterfaceCT, WatchedAdresses, create_communication_tools,
};
use crate::gui::keymapping::KeyMapping;
use crate::gameboy::GameBoy;
use crate::gui::views::emulation_view::emulation_ui_state::EmulationUiState;
use crate::mmu::DmgMmu;
use crate::mmu::mbc::{Mbc1, Mbc2, Mbc3, Mbc5, RomOnly};
use crate::mmu::timers::DmgTimers;
use crate::ppu::{self, DmgPpu};
use egui::load::SizedTexture;
use egui::{ColorImage, TextureOptions, vec2};
use egui_file_dialog::{FileDialog, Filter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

use eframe::egui::{Key, TextureHandle};
use std::str::FromStr;
use std::time::Instant;

#[derive(Default)]
pub struct GraphicalApp {
    app_state: AppState,
}

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
        let keys_down = _ui.ctx().input(|i| i.keys_down.clone());
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

#[derive(PartialEq, Debug, Clone)]
pub enum GbType {
    Cgb,
    Dmg,
}

impl GbType {
    fn supported_types(code: u8) -> Vec<GbType> {
        match code {
            0x80 => vec![GbType::Cgb, GbType::Dmg],
            0xC0 => vec![GbType::Cgb],
            _ => vec![GbType::Dmg],
        }
    }
}

impl FromStr for GbType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cgb" => Ok(GbType::Cgb),
            "dmg" => Ok(GbType::Dmg),
            _ => Err(format!(
                "Unknown GameBoy type: {}. Choose between Cgb and Dmg",
                s
            )),
        }
    }
}

#[derive(Clone)]
pub struct CoreGameOptions {
    // pub boot_rom_path: String,
    pub rom_path: String,
    pub boot_rom: bool,
    pub gbtype: GbType,
}

impl From<EmulationAppOptions> for CoreGameOptions {
    fn from(value: EmulationAppOptions) -> Self {
        Self {
            // boot_rom_path: "boot-roms/dmg.bin".into(),
            rom_path: value.rom_path,
            boot_rom: value.boot_rom,
            gbtype: GbType::Dmg, // TODO -> need to do feature to choose which type
        }
    }
}

impl EmulationAppOptions {
    pub fn new(rom_path: String, boot_rom: bool) -> Self {
        Self { rom_path, boot_rom }
    }
}

impl GraphicalApp {
    pub fn create_emulation_app(options: EmulationAppOptions) -> Self {
        Self {
            app_state: AppState::EmulationHub(EmulationDevice {
                core_game: CoreGameDevice::new(options.into()),
                ui_state: EmulationUiState::default(),
            }),
        }
    }
}

#[derive(Default)]
pub struct StartingHubDevice {}


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
    DmgOnlyRom(GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>>),
    CgbOnlyRom(GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>>),
    DmgMbc1(GameBoy<DmgMmu<Mbc1, DmgTimers, DmgPpu>>),
    CgbMbc1(GameBoy<DmgMmu<Mbc1, DmgTimers, DmgPpu>>),
    DmgMbc2(GameBoy<DmgMmu<Mbc2, DmgTimers, DmgPpu>>),
    CgbMbc2(GameBoy<DmgMmu<Mbc2, DmgTimers, DmgPpu>>),
    DmgMbc3(GameBoy<DmgMmu<Mbc3, DmgTimers, DmgPpu>>),
    CgbMbc3(GameBoy<DmgMmu<Mbc3, DmgTimers, DmgPpu>>),
    DmgMbc5(GameBoy<DmgMmu<Mbc5, DmgTimers, DmgPpu>>),
    CgbMbc5(GameBoy<DmgMmu<Mbc5, DmgTimers, DmgPpu>>),
}

impl AnyGameApp {
    pub fn new(game_data: CoreGameOptions) -> Result<Self, String> {
        let rom_data: Vec<u8> = Self::read_rom(&game_data.rom_path);
        let ram_path = game_data.rom_path.to_owned() + ".save";
        let ram_data: Option<Vec<u8>> = Self::read_ram(&ram_path);
        if ram_data.is_some() {
            println!("Backup detected")
        };
        let boot_rom_data = if game_data.boot_rom {
            let boot_bytes = match game_data.gbtype {
                GbType::Dmg => {
                    println!("dmg");
                    let boot_rom_path: String = "boot-roms/dmg.bin".to_string();
                    let boot_bytes = std::fs::read(boot_rom_path).expect("cannot read boot rom");
                    assert!(boot_bytes.len() == 0x100, "boot rom must be 256 bytes");
                    boot_bytes
                },
                GbType::Cgb => {
                    println!("cgb");
                    let boot_rom_path: String = "boot-roms/cgb.bin".to_string();
                    let boot_bytes = std::fs::read(boot_rom_path).expect("cannot read boot rom");
                    assert!(boot_bytes.len() == 0x900, "boot rom must be 2304 bytes");
                    boot_bytes
                }
            };
            let mut boot_rom = [0u8; 0x0900];
            boot_rom.copy_from_slice(&boot_bytes);
            Some(boot_rom)
        } else {
            None
        };

        println!("new AnyGameApp");

        let mbc_code = rom_data[0x0147];
        let supported_gb_types = GbType::supported_types(rom_data[0x0143]);

        if !supported_gb_types.contains(&game_data.gbtype) {
            return Err(format!(
                "Cartridge doesn't support type {:#?}",
                game_data.gbtype
            ));
        }

        match game_data.gbtype {
            GbType::Cgb => {
                match mbc_code {
                    0x00 | 0x08 | 0x09 => {
                        println!("Cgb OnlyRom detected");
                        Ok(AnyGameApp::CgbOnlyRom(GameBoy::new(
                            boot_rom_data,
                            rom_data,
                            ram_data,
                        )?))
                    }
                    0x01..=0x03 => {
                        println!("Cgb Mbc1 detected");
                        Ok(AnyGameApp::CgbMbc1(GameBoy::new(
                            boot_rom_data,
                            rom_data,
                            ram_data,
                        )?))
                    }
                    0x05 | 0x06 => {
                        println!("Cgb Mbc2 detected");
                        Ok(AnyGameApp::CgbMbc2(GameBoy::new(
                            boot_rom_data,
                            rom_data,
                            ram_data,
                        )?))
                    }
                    0x0F..=0x13 => {
                        println!("Cgb Mbc3 detected");
                        Ok(AnyGameApp::CgbMbc3(GameBoy::new(
                            boot_rom_data,
                            rom_data,
                            ram_data,
                        )?))
                    }
                    0x19..=0x1E => {
                        println!("Cgb Mbc5 detected");
                        Ok(AnyGameApp::CgbMbc5(GameBoy::new(
                            boot_rom_data,
                            rom_data,
                            ram_data,
                        )?))
                    }
                    /*
                    0x0B | 0x0C | 0x0D => Ok(todo!()), // MMM01 pas dans le sujet
                    0x20 => Ok(todo!()), // Mbc6
                    0x22 => Ok(todo!()),// MBC7+SENSOR+RUMBLE+RAM+BATTERY
                    */
                    _ => Err("Unmanaged cartridge type".into()),
                }
            }
            GbType::Dmg => match mbc_code {
                0x00 | 0x08 | 0x09 => {
                    println!("Dmg OnlyRom detected");
                    Ok(AnyGameApp::DmgOnlyRom(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                    )?))
                }
                0x01..=0x03 => {
                    println!("Dmg Mbc1 detected");
                    Ok(AnyGameApp::DmgMbc1(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                    )?))
                }
                0x05 | 0x06 => {
                    println!("Dmg Mbc2 detected");
                    Ok(AnyGameApp::DmgMbc2(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                    )?))
                }
                0x0F..=0x13 => {
                    println!("Dmg Mbc3 detected");
                    Ok(AnyGameApp::DmgMbc3(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                    )?))
                }
                0x19..=0x1E => {
                    println!("Dmg Mbc5 detected");
                    Ok(AnyGameApp::DmgMbc5(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                    )?))
                }
                _ => Err("Unmanaged cartridge type".into()),
            },
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

    pub fn launch(self, ct: Box<dyn GameCT>) -> Result<Option<Vec<u8>>, String> {
        match self {
            AnyGameApp::DmgOnlyRom(g) => g.launch(ct),
            AnyGameApp::CgbOnlyRom(g) => g.launch(ct),
            AnyGameApp::DmgMbc1(g) => g.launch(ct),
            AnyGameApp::CgbMbc1(g) => g.launch(ct),
            AnyGameApp::DmgMbc2(g) => g.launch(ct),
            AnyGameApp::CgbMbc2(g) => g.launch(ct),
            AnyGameApp::DmgMbc3(g) => g.launch(ct),
            AnyGameApp::CgbMbc3(g) => g.launch(ct),
            AnyGameApp::DmgMbc5(g) => g.launch(ct),
            AnyGameApp::CgbMbc5(g) => g.launch(ct),
        }
    }
}

async fn async_launch_game(game_data: CoreGameOptions, ct: Box<dyn GameCT>) -> Result<(), String> {
    let rom_path = game_data.rom_path.clone();
    let app = AnyGameApp::new(game_data)?;
    if let Some(value) = app.launch(ct)? {
        let save_path = rom_path.clone() + ".save";
        eprintln!("attempting to save game ram to {}", save_path);
        fs::write(save_path, value)
            .unwrap_or_else(|err| eprintln!("backup was unsucessfull {:?}", err));
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
    options: CoreGameOptions,
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

        let loaded_image = ColorImage::from_rgb([ppu::WIN_SIZE_X, ppu::WIN_SIZE_Y], &self.buffer);
        if let Some(th) = &mut self.texture_handler {
            th.set(loaded_image, TextureOptions::NEAREST);
        } else {
            self.texture_handler = Some(ui.ctx().load_texture(
                "gb_frame",
                loaded_image,
                TextureOptions::NEAREST,
            ));
        }

        if let Some(th) = &self.texture_handler {
            let scaled_size = vec2(ppu::WIN_SIZE_X as f32 * 4., ppu::WIN_SIZE_Y as f32 * 4.);
            let sized_texture = SizedTexture::new(th.id(), scaled_size);
            self.sized_image = Some(sized_texture);
        }
        Ok(())
    }

    pub fn capture_and_send_input(&self, ui: &mut egui::Ui) {
        let keys_down = ui.ctx().input(|i| i.keys_down.clone());
        let input = self.key_mapping.generate_key_input(keys_down);
        _ = self.interface_ct.send_input(input);
    }

    fn new(options: CoreGameOptions) -> Self {
        let (game_ct, interface_ct) = create_communication_tools();

        let audio_running = Arc::new(AtomicBool::new(true));

        Self {
            interface_ct,
            handler: tokio::spawn(async_launch_game(options.clone(), game_ct)),
            buffer: [0; FRAME_SIZE_IN_U8],
            texture_handler: None,
            sized_image: None,
            key_mapping: KeyMapping::default(),
            options,
        }
    }

    pub fn reset(&mut self) {
        self.handler.abort();

        let (game_ct, interface_ct) = create_communication_tools();
        self.interface_ct = interface_ct;
        self.handler = tokio::spawn(async_launch_game(self.options.clone(), game_ct));

        self.buffer = [0; FRAME_SIZE_IN_U8];
        self.sized_image = None;
    }

    
}

pub struct SelectionDevice {
    path: String,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    search: String,
    listening: Option<&'static str>,
    key_mapping: KeyMapping,
    launch_cgb: bool
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
                    Filter::new(|path: &Path| {
                        path.extension().unwrap_or_default() == "gb"
                            || path.extension().unwrap_or_default() == "gbc"
                    }),
                )
                .add_file_filter(
                    "GameBoy ROMS",
                    Filter::new(|path: &Path| {
                        path.extension().unwrap_or_default() == "gb"
                            || path.extension().unwrap_or_default() == "gbc"
                    }),
                )
                .default_file_filter("GameBoy ROMS"),
            search: String::new(),
            listening: None,
            key_mapping: KeyMapping::default(),
            launch_cgb: false,
        }
    }
}

pub struct EmulationDevice {
    pub core_game: CoreGameDevice,
    pub ui_state: EmulationUiState,
}

pub struct DebuggingDevice {
    pub core_game: CoreGameDevice,
    pub ui_state: EmulationUiState,

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
    pub instruction_to_exec: Option<String>,
    pub is_paused: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self::StartingHub(Default::default())
    }
}
