pub mod emulation_ui_state;
use chrono::Local;
use egui::vec2;
use egui::{Color32, RichText};

use crate::GBMU_FILE;
use crate::communications::{CpuState, FRAME_SIZE_IN_U8, InstructionList, Mode, WatchedAdresses};
use crate::gui::egui::Id;
use crate::gui::{
    AppState, CoreGameDevice, CoreGameOptions, DebuggingDevice, EmulationDevice, GbType,
    SelectionDevice,
};

#[derive(Debug)]
pub struct SaveState {
    pub preview: [u8; FRAME_SIZE_IN_U8],
    pub name: String,
}

use crate::gui::views::emulation_view::emulation_ui_state::EmulationUiState;

use std::time::Instant;

impl EmulationDevice {
    pub fn emulation_view(mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) -> AppState {
        let debut = Instant::now();
        if self.core_game.update_and_size_image(ui).is_err() {
            eprintln!("Communication is cut : falling back to selection view.");
            return AppState::SelectionHub(self.into());
        }
        let duration = debut.elapsed();
        self.core_game.capture_and_send_input(ui);
        let fps = self.core_game.interface_ct.get_fps().unwrap();

        let mut open_debugger = false;
        let mut back_to_selection = false;

        const TOOLBAR_CONTENT_HEIGHT: f32 = 36.0;
        const TOOLBAR_PANEL_HEIGHT: f32 = 64.0;

        egui::CentralPanel::default().show_inside(ui, |ui| {
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

            egui::Panel::bottom(Id::new("EmulationViewBottomPanel"))
                .exact_size(TOOLBAR_PANEL_HEIGHT)
                .frame(
                    egui::Frame::NONE
                        .fill(ui.visuals().faint_bg_color)
                        .stroke(egui::Stroke::new(
                            1.0,
                            ui.visuals().widgets.noninteractive.bg_stroke.color,
                        ))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::symmetric(12, 8)),
                )
                .show_inside(ui, |ui| {
                    let available_height = ui.available_height();
                    let top_padding = ((available_height - TOOLBAR_CONTENT_HEIGHT) / 2.0).max(0.0);
                    ui.add_space(top_padding);

                    ui.horizontal_centered(|ui| {
                        let back_button = ui.add(
                            egui::Button::new(
                                RichText::new("◀ Back to menu")
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .corner_radius(egui::CornerRadius::same(6))
                            .min_size(vec2(32.0, 28.0)),
                        );
                        if back_button.clicked() {
                            back_to_selection = true;
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        let (pause_label, pause_color) = if self.ui_state.is_paused {
                            ("▶  Resume", Color32::from_rgb(120, 200, 120))
                        } else {
                            ("⏸  Pause", Color32::from_rgb(230, 170, 90))
                        };
                        let pause_button = ui.add(
                            egui::Button::new(
                                RichText::new(pause_label).color(Color32::WHITE).strong(),
                            )
                            .fill(pause_color)
                            .corner_radius(egui::CornerRadius::same(6))
                            .min_size(vec2(100.0, 28.0)),
                        );

                        if pause_button.clicked() {
                            self.ui_state.is_paused = !self.ui_state.is_paused;
                            if self.ui_state.is_paused {
                                let _ = self.core_game.interface_ct.set_mode(Mode::Stop);
                            } else {
                                let _ = self.core_game.interface_ct.set_mode(Mode::Game);
                            }
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        let save_state_button = ui.add(
                            egui::Button::new(
                                RichText::new("Save State").color(Color32::WHITE).strong(),
                            )
                            .corner_radius(egui::CornerRadius::same(6))
                            .min_size(vec2(110.0, 28.0)),
                        );
                        if save_state_button.clicked() {
                            let _ = self.core_game.interface_ct.set_mode(Mode::Stop);
                            self.ui_state.save_name.clear();
                            self.ui_state.save_name = format!(
                                "{} - {}",
                                self.core_game
                                    .options
                                    .rom_path
                                    .split('/')
                                    .next_back()
                                    .unwrap_or("Unknow ROM"),
                                Local::now().format("%Y-%m-%d %H:%M:%S")
                            );
                            self.ui_state.show_save_popup = true;
                        }

                        if self.ui_state.show_save_popup {
                            egui::Modal::new(egui::Id::new("save_state_modal")).show(
                                ui.ctx(),
                                |ui| {
                                    ui.heading("Name your save-state");

                                    ui.text_edit_singleline(&mut self.ui_state.save_name);

                                    ui.horizontal(|ui| {
                                        let ok_btn =
                                            egui::Button::new(RichText::new("Ok").strong())
                                                .corner_radius(egui::CornerRadius::same(6))
                                                .fill(Color32::DARK_GREEN);
                                        let cancel_btn =
                                            egui::Button::new(RichText::new("Cancel").strong())
                                                .corner_radius(egui::CornerRadius::same(6))
                                                .fill(Color32::DARK_RED);

                                        if ui.add(ok_btn).clicked() {
                                            self.ui_state.show_save_popup = false;
                                            let _ =
                                                self.core_game.interface_ct.set_mode(Mode::Game);
                                            let _ = self.core_game.interface_ct.request_save_state(
                                                SaveState {
                                                    preview: self.core_game.buffer,
                                                    name: self.ui_state.save_name.clone(),
                                                },
                                            );
                                        }
                                        if ui.add(cancel_btn).clicked() {
                                            self.ui_state.show_save_popup = false;
                                            let _ =
                                                self.core_game.interface_ct.set_mode(Mode::Game);
                                        }
                                    });
                                },
                            );
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        let debug_button = ui.add(
                            egui::Button::new(
                                RichText::new("Open Debugger")
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .corner_radius(egui::CornerRadius::same(6))
                            .min_size(vec2(120.0, 28.0)),
                        );
                        if debug_button.clicked() {
                            open_debugger = true;
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        ui.label(RichText::new("Game Speed").color(Color32::WHITE).strong());
                        let slider = egui::Slider::new(&mut self.ui_state.speed, 1.0..=16.0)
                            .step_by(1.0)
                            .suffix("x")
                            .show_value(true);
                        if ui.add_sized(vec2(140.0, 20.0), slider).changed() {
                            let _ = self
                                .core_game
                                .interface_ct
                                .set_speed(self.ui_state.speed as u8);
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);
                        let reset_button = ui.add(
                            egui::Button::new(
                                RichText::new("🔄  Reset").color(Color32::WHITE).strong(),
                            )
                            .corner_radius(egui::CornerRadius::same(6))
                            .min_size(vec2(90.0, 28.0)),
                        );
                        if reset_button.clicked() {
                            self.core_game.reset();
                            self.ui_state.is_paused = false;
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        ui.label(RichText::new("Volume").color(Color32::WHITE).strong());
                        let slider = egui::Slider::new(&mut self.ui_state.volume, 0.0..=200.0)
                            .step_by(10.0)
                            .suffix("%")
                            .show_value(true);
                        if ui.add_sized(vec2(140.0, 20.0), slider).changed() {
                            let _ = self
                                .core_game
                                .interface_ct
                                .set_volume(self.ui_state.volume as u8);
                            GBMU_FILE.lock().unwrap().settings.volume = self.ui_state.volume;
                            GBMU_FILE.lock().unwrap().persist();
                        }
                    });
                });
        });

        if back_to_selection {
            return AppState::SelectionHub(self.into());
        }

        if open_debugger {
            if self.ui_state.is_paused {
                return match self.core_game.interface_ct.set_mode(Mode::Stop) {
                    Ok(()) => AppState::DebuggingHub(self.into()),
                    Err(_) => {
                        eprintln!("Communication is cut : falling back to selection view.");
                        AppState::SelectionHub(self.into())
                    }
                };
            }
            return match self.core_game.interface_ct.set_mode(Mode::Debug) {
                Ok(()) => AppState::DebuggingHub(self.into()),
                Err(_) => {
                    eprintln!("Communication is cut : falling back to selection view.");
                    AppState::SelectionHub(self.into())
                }
            };
        }

        AppState::EmulationHub(self)
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
            ui_state: original.ui_state,
            instruction_to_exec: None,
            is_paused: false,
        }
    }
}

impl From<SelectionDevice> for EmulationDevice {
    fn from(original: SelectionDevice) -> Self {
        let rom_path = original.path;
        let gb_type = match original.launch_cgb {
            true => GbType::Cgb,
            false => GbType::Dmg,
        };
        let boot_rom_path = match gb_type {
            GbType::Cgb => Some("boot-roms/cgb.bin".into()),
            GbType::Dmg => Some("boot-roms/dmg.bin".into()),
        };
        let options = CoreGameOptions {
            gb_type: Some(gb_type),
            rom_path,
            boot_rom: true,
            boot_rom_path,
        };
        let mut core_game = CoreGameDevice::new(options);
        core_game.key_mapping = original.key_mapping;
        let _ = core_game
            .interface_ct
            .set_volume(GBMU_FILE.lock().unwrap().settings.volume as u8);
        Self {
            core_game,
            ui_state: EmulationUiState::default(),
        }
    }
}

impl From<DebuggingDevice> for EmulationDevice {
    fn from(original: DebuggingDevice) -> Self {
        Self {
            core_game: original.core_game,
            ui_state: original.ui_state,
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
        let key_mapping = value.core_game.key_mapping.clone();
        Self {
            key_mapping,
            ..Default::default()
        }
    }
}
