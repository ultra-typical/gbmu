#![allow(dead_code)]
mod cli;
mod communications;
mod cpu;
mod file;
mod gameboy;
mod gui;
mod mmu;
mod ppu;
mod sound;

use crate::{cli::EmulatorArguments, file::CrossemuFile, gui::EmulationAppOptions};
use discord_presence::{Client, DiscordError};
use gui::GraphicalApp;
use std::{
    sync::{LazyLock, Mutex},
    thread,
};

static CROSSEMU_FILE: LazyLock<Mutex<CrossemuFile>> =
    LazyLock::new(|| Mutex::new(CrossemuFile::get_existing_or_new()));

static DISCORD_CLIENT: LazyLock<Mutex<Client>> = LazyLock::new(|| {
    let mut drpc = Client::new(1197937661176987798);
    drpc.start();
    thread::sleep(std::time::Duration::from_millis(500));
    Mutex::new(drpc)
});

pub fn setup_rich_presence(
    arguments: &EmulatorArguments,
) -> Result<(), Box<dyn std::error::Error>> {
    let details = if let Some(rom_path) = &arguments.rom_path {
        format!(
            "Playing {}",
            rom_path.split('/').next_back().unwrap_or("Unknown")
        )
    } else {
        String::from("In Menu")
    };
    let state = arguments
        .gb_type
        .as_ref()
        .map(|gb| format!("Emulating a {}", gb));

    update_presence(details, state)?;
    println!("Rich presence setup complete.");
    Ok(())
}

pub fn update_presence(
    details: String,
    state: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let details = if details.len() > 128 {
        details.chars().take(128).collect::<String>()
    } else {
        details
    };

    let state = if let Some(s) = state {
        if s.len() > 128 {
            Some(
                s.chars()
                    .take(128)
                    .collect::<String>()
                    .split(".")
                    .next()
                    .unwrap_or("Unknown")
                    .to_string(),
            )
        } else {
            Some(s.split(".").next().unwrap_or("Unknown").to_string())
        }
    } else {
        None
    };

    if let Ok(mut drpc) = DISCORD_CLIENT.lock() {
        drpc.set_activity(|act| {
            let act = act.details(details);
            if let Some(s) = state {
                act.state(s)
            } else {
                act
            }
        })?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let arguments = match EmulatorArguments::get() {
        Ok(args) => args,
        Err(errors) => {
            eprintln!("Enable to open emulator : {errors}");
            return;
        }
    };

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    let arguments_clone = arguments.clone();
    tokio::spawn(async move {
        if let Err(setup_rich_presence) = setup_rich_presence(&arguments_clone) {
            match setup_rich_presence.downcast_ref::<DiscordError>() {
                Some(DiscordError::NotStarted) => {
                    eprintln!(
                        "Discord client not started. Please ensure Discord is running and try again for the presence to work."
                    );
                }
                _ => {
                    eprintln!("Failed to setup rich presence: {:?}", setup_rich_presence);
                }
            }
        }
    });

    let app = if let Some(rom_path) = arguments.rom_path {
        let options = EmulationAppOptions::new(
            arguments.boot_rom_path,
            rom_path,
            "".into(),
            arguments.gb_type,
        );
        GraphicalApp::create_emulation_app(options)
    } else {
        GraphicalApp::default()
    };

    let _ = eframe::run_native("CROSS-EMU", options, Box::new(|_cc| Ok(Box::new(app))));
}
