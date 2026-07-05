#![cfg_attr(test, allow(clippy::all))]
#![allow(unused_variables)]

mod common;
pub mod keymapping;
pub mod views;

use crate::communications::{
    CpuState, GameCT, InstructionList, InterfaceCT, WatchedAdresses, create_communication_tools,
};
use crate::mmu::apu::Apu;
use crate::mmu::apu::sample_buffer::SampleBuffer;
use crate::{GBMU_FILE, update_presence};

use crate::file::{SAVE_STATE_FILE, SAVE_STATE_TYPES_FILE, SaveStateTypes};
use crate::gameboy::GameBoy;
use crate::gui::keymapping::KeyMapping;
use crate::gui::views::emulation_view::emulation_ui_state::EmulationUiState;
use crate::mmu::mbc::{Mbc1, Mbc2, Mbc3, Mbc5, MbcType, RomOnly};
use crate::mmu::timers::{DmgTimers};
use crate::mmu::{CgbMmu, DmgMmu, HardwareKind};
use egui::load::SizedTexture;
use egui::{ColorImage, TextureOptions, vec2};
use egui_file_dialog::{FileDialog, Filter};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

use crate::ppu::{self, CgbPpu, DmgPpu};
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
            AppState::SelectionHub(device) => device.selection_view(_ui, _frame),
            AppState::EmulationHub(device) => device.emulation_view(_ui, _frame),
            AppState::DebuggingHub(device) => device.debug_view(_ui, _frame),
            AppState::Error(device) => device.error_view(_ui, _frame),
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
    boot_rom_path: Option<String>,
    rom_path: String,
    boot_rom: bool,
    pub filename: String,
    gb_type: Option<GbType>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum GbType {
    Cgb,
    Dmg,
}

use std::fmt::Display;

impl Display for GbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GbType::Cgb => "Gameboy Color (Cgb)",
                GbType::Dmg => "Game boy (Dmg)",
            }
        )
    }
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
    pub boot_rom_path: Option<String>,
    pub rom_path: String,
    pub boot_rom: bool,
    pub filename: String,
    pub gb_type: Option<GbType>,
}

impl CoreGameOptions {
    pub fn define_gb_type(&self, supported_gb_types: &[GbType]) -> GbType {
        match &self.gb_type {
            Some(gb_type) => gb_type.clone(),
            None => supported_gb_types.first().unwrap().clone(),
        }
    }

    fn from_snapshot(path: PathBuf) -> Self {
        let rom_path = fs::read_to_string(path.join("name.txt")).unwrap_or_default();
        let filename = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();

        Self {
            boot_rom_path: None,
            rom_path,
            boot_rom: false,
            filename,
            gb_type: None,
        }
    }
}

impl From<EmulationAppOptions> for CoreGameOptions {
    fn from(value: EmulationAppOptions) -> Self {
        Self {
            boot_rom_path: value.boot_rom_path,
            rom_path: value.rom_path,
            boot_rom: value.boot_rom,
            filename: value.filename,
            gb_type: value.gb_type,
        }
    }
}

impl EmulationAppOptions {
    pub fn new(
        boot_rom_path: Option<String>,
        rom_path: String,
        boot_rom: bool,
        filename: String,
        gb_type: Option<GbType>,
    ) -> Self {
        Self {
            boot_rom_path,
            rom_path,
            boot_rom,
            filename,
            gb_type,
        }
    }
}

impl GraphicalApp {
    pub fn create_emulation_app(options: EmulationAppOptions) -> Self {
        Self {
            app_state: AppState::EmulationHub(EmulationDevice {
                core_game: CoreGameDevice::new(options.into()),
                rom_path: String::from(""),
                filename: String::from(""),
                ui_state: EmulationUiState::default(),
            }),
        }
    }
}

#[derive(Default)]
pub struct StartingHubDevice {}

#[allow(clippy::large_enum_variant)]
pub enum AppState {
    SelectionHub(SelectionDevice),
    EmulationHub(EmulationDevice),
    DebuggingHub(DebuggingDevice),
    Error(ErrorDevice),
    Default,
}

use std::fs;

pub enum AnyGameApp {
    DmgOnlyRom(GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>>),
    CgbOnlyRom(GameBoy<CgbMmu<RomOnly, DmgTimers, CgbPpu>>),
    DmgMbc1(GameBoy<DmgMmu<Mbc1, DmgTimers, DmgPpu>>),
    CgbMbc1(GameBoy<CgbMmu<Mbc1, DmgTimers, CgbPpu>>),
    DmgMbc2(GameBoy<DmgMmu<Mbc2, DmgTimers, DmgPpu>>),
    CgbMbc2(GameBoy<CgbMmu<Mbc2, DmgTimers, CgbPpu>>),
    DmgMbc3(GameBoy<DmgMmu<Mbc3, DmgTimers, DmgPpu>>),
    CgbMbc3(GameBoy<CgbMmu<Mbc3, DmgTimers, CgbPpu>>),
    DmgMbc5(GameBoy<DmgMmu<Mbc5, DmgTimers, DmgPpu>>),
    CgbMbc5(GameBoy<CgbMmu<Mbc5, DmgTimers, CgbPpu>>),
}

impl AnyGameApp {
    pub fn new(game_data: CoreGameOptions) -> Result<Self, String> {
        let rom_data: Vec<u8> = Self::read_rom(&game_data.rom_path)?;

        let mbc_code = rom_data[0x0147];
        let cgb_flag = rom_data[0x0143];
        let supported_gb_types = GbType::supported_types(cgb_flag);

        let gb_type = game_data.define_gb_type(&supported_gb_types);

        let boot_rom_path = match game_data.boot_rom_path {
            Some(path) => path,
            None => match gb_type {
                GbType::Cgb => "boot-roms/cgb.bin".into(),
                GbType::Dmg => "boot-roms/dmg.bin".into(),
            },
        };

        let _ = update_presence(
            format!("In a {} Game", gb_type),
            Some(format!(
                "Playing {}",
                game_data
                    .rom_path
                    .split('/')
                    .next_back()
                    .unwrap_or("Unknown")
            )),
        );

        let rom_compatibility = gb_type == GbType::Cgb && !matches!(cgb_flag, 0x80 | 0xC0);

        // ram_path
        let ram_path = game_data.rom_path.to_owned() + ".save";
        let ram_data: Option<Vec<u8>> = Self::read_ram(&ram_path);
        if ram_data.is_some() {
            println!("Backup detected")
        };

        let boot_rom_data: Option<[u8; 0x0900]> = if game_data.boot_rom {
            let mut boot_rom = [0u8; 0x0900];
            if let Ok(boot_bytes) = std::fs::read(boot_rom_path) {
                match gb_type {
                    GbType::Dmg => {
                        assert!(boot_bytes.len() == 0x100, "boot rom must be 256 bytes");
                        boot_rom[..0x100].copy_from_slice(&boot_bytes);
                    }
                    GbType::Cgb => {
                        assert!(boot_bytes.len() == 0x900, "boot rom must be 2304 bytes");
                        boot_rom.copy_from_slice(&boot_bytes);
                    }
                };
                Some(boot_rom)
            } else {
                eprintln!("Boot rom can't be read. Forcing boot rom simulation.");
                None
            }
        } else {
            None
        };

        println!("new AnyGameApp");

        match gb_type {
            GbType::Cgb => match mbc_code {
                0x00 | 0x08 | 0x09 => {
                    println!("Cgb OnlyRom detected");
                    Ok(AnyGameApp::CgbOnlyRom(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x01..=0x03 => {
                    println!("Cgb Mbc1 detected");
                    Ok(AnyGameApp::CgbMbc1(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x05 | 0x06 => {
                    println!("Cgb Mbc2 detected");
                    Ok(AnyGameApp::CgbMbc2(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x0F..=0x13 => {
                    println!("Cgb Mbc3 detected");
                    Ok(AnyGameApp::CgbMbc3(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x19..=0x1E => {
                    println!("Cgb Mbc5 detected");
                    Ok(AnyGameApp::CgbMbc5(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                _ => Err("Unmanaged cartridge type".into()),
            },
            GbType::Dmg => match mbc_code {
                0x00 | 0x08 | 0x09 => {
                    println!("Dmg OnlyRom detected");
                    Ok(AnyGameApp::DmgOnlyRom(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x01..=0x03 => {
                    println!("Dmg Mbc1 detected");
                    Ok(AnyGameApp::DmgMbc1(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x05 | 0x06 => {
                    println!("Dmg Mbc2 detected");
                    Ok(AnyGameApp::DmgMbc2(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x0F..=0x13 => {
                    println!("Dmg Mbc3 detected");
                    Ok(AnyGameApp::DmgMbc3(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                0x19..=0x1E => {
                    println!("Dmg Mbc5 detected");
                    Ok(AnyGameApp::DmgMbc5(GameBoy::new(
                        boot_rom_data,
                        rom_data,
                        ram_data,
                        rom_compatibility,
                        gb_type,
                    )?))
                }
                _ => Err("Unmanaged cartridge type".into()),
            },
        }
    }

    fn from_snapshot(path: PathBuf) -> Result<Self, String> {
        let types_path = path.join(SAVE_STATE_TYPES_FILE);
        let types_json = fs::read_to_string(&types_path)
            .map_err(|e| format!("Could not read {:?}: {e}", types_path))?;
        let types: SaveStateTypes = serde_json::from_str(&types_json)
            .map_err(|e| format!("Could not parse {:?}: {e}", types_path))?;

        let save_state_path = path
            .join(SAVE_STATE_FILE)
            .to_str()
            .ok_or_else(|| format!("Save state path {:?} isn't valid UTF-8", path))?
            .to_string();

        match (types.hardware, types.cart) {
            (HardwareKind::Dmg, MbcType::RomOnly) => {
                let mut gb: GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::DmgOnlyRom(gb))
            }
            (HardwareKind::Dmg, MbcType::Mbc1) => {
                let mut gb: GameBoy<DmgMmu<Mbc1, DmgTimers, DmgPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::DmgMbc1(gb))
            }
            (HardwareKind::Dmg, MbcType::Mbc2) => {
                let mut gb: GameBoy<DmgMmu<Mbc2, DmgTimers, DmgPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::DmgMbc2(gb))
            }
            (HardwareKind::Dmg, MbcType::Mbc3) => {
                let mut gb: GameBoy<DmgMmu<Mbc3, DmgTimers, DmgPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::DmgMbc3(gb))
            }
            (HardwareKind::Dmg, MbcType::Mbc5) => {
                let mut gb: GameBoy<DmgMmu<Mbc5, DmgTimers, DmgPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::DmgMbc5(gb))
            }
            (HardwareKind::Cgb, MbcType::RomOnly) => {
                let mut gb: GameBoy<CgbMmu<RomOnly, DmgTimers, CgbPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::CgbOnlyRom(gb))
            }
            (HardwareKind::Cgb, MbcType::Mbc1) => {
                let mut gb: GameBoy<CgbMmu<Mbc1, DmgTimers, CgbPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::CgbMbc1(gb))
            }
            (HardwareKind::Cgb, MbcType::Mbc2) => {
                let mut gb: GameBoy<CgbMmu<Mbc2, DmgTimers, CgbPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::CgbMbc2(gb))
            }
            (HardwareKind::Cgb, MbcType::Mbc3) => {
                let mut gb: GameBoy<CgbMmu<Mbc3, DmgTimers, CgbPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::CgbMbc3(gb))
            }
            (HardwareKind::Cgb, MbcType::Mbc5) => {
                let mut gb: GameBoy<CgbMmu<Mbc5, DmgTimers, CgbPpu>> = GameBoy::snapshot(save_state_path)?;
                let volume = gb.bus.apu.volume;
                gb.bus.apu = Apu::new();
                gb.bus.apu.volume = volume;
                Ok(AnyGameApp::CgbMbc5(gb))
            }
        }
    }

    fn read_ram(ram_path: &String) -> Option<Vec<u8>> {
        fs::read(ram_path).ok()
    }

    fn read_rom(rom_path: &String) -> Result<Vec<u8>, String> {
        if !rom_path.is_empty() {
            fs::read(rom_path).map_err(|e| {
                eprintln!("Failed to read the file: {e}");
                "Failed to read the file: {e}".into()
            })
        } else {
            Err("Failed to read the file: {rom_path} : path is empty".into())
        }
    }

    pub fn launch(self, ct: &mut Box<dyn GameCT>) -> Result<Option<Vec<u8>>, String> {
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

async fn launch_snapshot(path: PathBuf, mut ct: Box<dyn GameCT>) {
    let result = match AnyGameApp::from_snapshot(path.clone()) {
        Ok(app) => match app.launch(&mut ct) {
            Ok(Some(saved_ram)) => {
                let save_path = format!("{}{}", path.to_str().unwrap(), ".save");
                eprintln!("attempting to save game ram to {}", save_path);
                fs::write(save_path, saved_ram).map_err(|err| {
                    let formated = format!("backup was unsucessfull {:?}", err);
                    eprintln!("{}", formated);
                    formated
                })
            }
            Err(e) => {
                eprintln!("Error: during launch : {:?}", &e);
                Err(e)
            }
            _ => Ok(()),
        },
        Err(e) => {
            eprintln!("Error: during init : {:?}", &e);
            Err(e)
        }
    };
    ct.send_end_result(result)
}

async fn async_launch_game(game_data: CoreGameOptions, mut ct: Box<dyn GameCT>) {
    let rom_path = game_data.rom_path.clone();
    let result = match AnyGameApp::new(game_data) {
        Ok(app) => match app.launch(&mut ct) {
            Ok(Some(saved_ram)) => {
                let save_path = rom_path.clone() + ".save";
                eprintln!("attempting to save game ram to {}", save_path);
                fs::write(save_path, saved_ram).map_err(|err| {
                    let formated = format!("backup was unsucessfull {:?}", err);
                    eprintln!("{}", formated);
                    formated
                })
            }
            Err(e) => {
                eprintln!("Error: during launch : {:?}", &e);
                Err(e)
            }
            _ => Ok(()),
        },
        Err(e) => {
            eprintln!("Error: during init : {:?}", &e);
            Err(e)
        }
    };
    ct.send_end_result(result)
}

use crate::communications::FRAME_SIZE_IN_U8;

pub struct CoreGameDevice {
    game_name: String,

    pub handler: Option<JoinHandle<()>>,
    buffer: [u8; FRAME_SIZE_IN_U8],
    pub sized_image: Option<SizedTexture>,
    texture_handler: Option<TextureHandle>,
    key_mapping: KeyMapping,
    pub interface_ct: Box<dyn InterfaceCT>,
    options: CoreGameOptions,
}

impl Drop for CoreGameDevice {
    fn drop(&mut self) {
        self.handler.as_ref().inspect(|handler| {
            println!("Dropping Core Game Device : aborting the game task handler");
            handler.abort()
        });
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

    fn from_snapshot(path: PathBuf) -> Self {
        let (game_ct, interface_ct) = create_communication_tools();

        let options = CoreGameOptions::from_snapshot(path.clone());

        let game_name = options.rom_path.clone();
        Self {
            game_name,
            interface_ct,
            handler: Some(tokio::spawn(launch_snapshot(path, game_ct))),
            buffer: [0; FRAME_SIZE_IN_U8],
            texture_handler: None,
            sized_image: None,
            key_mapping: GBMU_FILE.lock().unwrap().settings.keymapping.clone(),
            options,
        }
    }

    fn new(options: CoreGameOptions) -> Self {
        let (game_ct, interface_ct) = create_communication_tools();

        let audio_running = Arc::new(AtomicBool::new(true));

        let game_name = options.rom_path.clone();
        Self {
            game_name,
            interface_ct,
            handler: Some(tokio::spawn(async_launch_game(options.clone(), game_ct))),
            buffer: [0; FRAME_SIZE_IN_U8],
            texture_handler: None,
            sized_image: None,
            key_mapping: GBMU_FILE.lock().unwrap().settings.keymapping.clone(),
            options,
        }
    }

    pub fn reset(&mut self) {
        self.handler.take().inspect(|h| h.abort());

        let (game_ct, interface_ct) = create_communication_tools();
        self.interface_ct = interface_ct;
        self.handler = Some(tokio::spawn(async_launch_game(
            self.options.clone(),
            game_ct,
        )));

        self.buffer = [0; FRAME_SIZE_IN_U8];
        self.sized_image = None;
    }

    fn is_finished(&self) -> bool {
        let mut is_finished = true;
        self.handler
            .as_ref()
            .inspect(|handler| is_finished = handler.is_finished());
        is_finished
    }

    fn return_value(mut self) -> String {
        if let Some(handler) = self.handler.take() {
            match tokio::runtime::Handle::current().block_on(handler) {
                Ok(_) => format!("{} quitted unexpectedly", &self.game_name),
                Err(err) => format!("{} quitted : {}", &self.game_name, err),
            }
        } else {
            "".into()
        }
    }
}

pub struct SelectionDevice {
    path: String,
    filename: String,
    save_state_path: Option<String>,
    file_dialog: FileDialog,
    directory_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    search: String,
    listening: Option<&'static str>,
    key_mapping: KeyMapping,
    forced_launch: Option<GbType>,
    forced_launch_text: String,
    save_state_previews: HashMap<String, egui::TextureHandle>,
}

impl Default for SelectionDevice {
    fn default() -> Self {
        Self {
            path: String::from("./"),
            filename: String::from(""),
            save_state_path: None,
            picked_file: None,
            directory_dialog: FileDialog::new(),
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
            key_mapping: GBMU_FILE.lock().unwrap().settings.keymapping.clone(),
            forced_launch: None,
            forced_launch_text: "None".to_string(),
            save_state_previews: HashMap::new(),
        }
    }
}

pub struct EmulationDevice {
    pub core_game: CoreGameDevice,
    pub filename: String,
    pub rom_path: String,
    pub ui_state: EmulationUiState,
}

pub struct DebuggingDevice {
    pub core_game: CoreGameDevice,
    pub rom_path: String,
    /*
        Info stored for the GUI to use them;
        These are the responses from the sending/receiving operation
    */
    pub ui_state: EmulationUiState,

    pub next_instructions: InstructionList,
    pub watched_adress: WatchedAdresses,
    pub registers: CpuState,
    pub is_step: bool,
    pub nb_instruction: usize,

    pub error_message: Option<String>,
    pub hex_string: String,
    pub instruction_to_exec: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::SelectionHub(SelectionDevice::default())
    }
}
pub struct ErrorDevice {
    formated_error: String,
}

impl ErrorDevice {
    fn new(formated_error: String) -> Self {
        Self { formated_error }
    }
}
