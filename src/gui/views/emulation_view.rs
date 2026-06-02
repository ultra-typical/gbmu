use crate::communications::{CpuState, InstructionList, Mode, WatchedAdresses};
use crate::gui::{
    AppState,
    CoreGameDevice,
    CoreGameOptions,
    DebuggingDevice,
    EmulationDevice,
    SelectionDevice
};


use std::time::{Instant};

impl EmulationDevice {
    pub fn emulation_view(mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) -> AppState {
        let debut = Instant::now();
        if self.core_game.update_and_size_image(ui).is_err() {
            eprintln!("Communication is cut : falling back to selection view.");
            return AppState::SelectionHub(self.into())
        }
        let duration = debut.elapsed();
        self.core_game.capture_and_send_input(ui);
        let fps = self.core_game.interface_ct.get_fps().unwrap();

        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() - 50.0);
                    ui.label(fps.to_string());
                });
                ui.vertical_centered(|ui| {
                    if let Some(texture) = self.core_game.sized_image {
                        ui.image(texture);
                    }
                    ui.add_space(10.0);
                });
                if ui.button("🐛 Open Debug Panel").clicked() {
                    if let Err(err) = self.core_game.interface_ct.set_mode(Mode::Debug) {
                        eprintln!("Communication is cut : falling back to selection view.");
                        AppState::SelectionHub(self.into())
                    } else {
                        AppState::DebuggingHub(self.into())
                    }
                } else {
                    AppState::EmulationHub(self)
                }
            })
            .inner 
    }
}

impl From<EmulationDevice> for DebuggingDevice {
    fn from(original: EmulationDevice) -> Self {
        Self {
            core_game: original.core_game,
            next_instructions: InstructionList::default(),
            watched_adress: WatchedAdresses::default(),
            registers: CpuState::default(),
            is_step: false,
            nb_instruction: 0,
            error_message: None,
            hex_string: String::new(),
        }
    }
}

impl From<SelectionDevice> for EmulationDevice {
    fn from(original: SelectionDevice) -> Self {
       let rom_path = original.path;
        let options = CoreGameOptions {
            rom_path,
            boot_rom: true,
        };
        let core_game = CoreGameDevice::new(options);
        Self { core_game}
    }
}

impl From<DebuggingDevice> for EmulationDevice {
    fn from(original: DebuggingDevice) -> Self {
        Self {
            core_game: original.core_game,
        }
    }
}

impl From<DebuggingDevice> for SelectionDevice {
    fn from(value: DebuggingDevice) -> Self {
        Self::default() 
    }
}

impl From<EmulationDevice> for SelectionDevice {
    fn from(value: EmulationDevice) -> Self {
        Self::default()
    }
}
