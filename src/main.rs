#![allow(dead_code)]
mod cli;
mod cpu;
mod gameboy;
mod gui;
mod mmu;
mod ppu;
mod file;
mod sound;
mod communications;

use sound::start_audio;
use gui::GraphicalApp;
use crate::{cli::EmulatorArguments, file::{GbmuFile}, gui::EmulationAppOptions};
use std::sync::{LazyLock, Mutex};
use std::f32::consts::PI;

use crate::mmu::apu::sample_buffer;

static GBMU_FILE: LazyLock<Mutex<GbmuFile>> =
    LazyLock::new(|| Mutex::new(GbmuFile::get_existing_or_new()));

#[tokio::main]
async fn main() {
    let arguments = match EmulatorArguments::get() {
        Ok(args) => args,
        Err(errors) => {
            eprintln!("Enable to open emulator : {errors}");
            return 
        }
    };

    if arguments.sound {
        let buffer = sample_buffer::SampleBuffer::new();

        let mut phase = 0.0;
        for _ in 0..48000*3 {
            phase += 2.0 * PI * 261.63 / 48000.0;
            buffer.push(phase.sin() * 0.5);
        }
        start_audio(buffer.clone());
        std::thread::sleep(std::time::Duration::from_secs(4));
    }

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    let app = if let Some(rom_path) = arguments.rom_path {
        let options = EmulationAppOptions::new(
            rom_path,
            arguments.boot_rom
        );
        GraphicalApp::create_emulation_app(options)
    } else {
        GraphicalApp::default()
    };

    let _ = eframe::run_native(
        "egui Demo",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );

}
