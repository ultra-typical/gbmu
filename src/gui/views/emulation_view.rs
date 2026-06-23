pub mod emulation_ui_state;
use egui::{Color32, RichText};
use egui::vec2;

use crate::communications::{CpuState, InstructionList, Mode, WatchedAdresses};
use crate::gui::egui::Id;
use crate::gui::{
    AppState, CoreGameDevice, CoreGameOptions, DebuggingDevice, EmulationDevice, GbType, SelectionDevice
};

use crate::gui::views::emulation_view::emulation_ui_state::EmulationUiState;


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

        let mut open_debugger = false;
        let mut back_to_selection = false; // <-- nouveau flag

        const TOOLBAR_CONTENT_HEIGHT: f32 = 36.0;
        const TOOLBAR_PANEL_HEIGHT: f32 = 64.0;

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

                egui::Panel::bottom(Id::new("EmulationViewBottomPanel"))
                    .exact_size(TOOLBAR_PANEL_HEIGHT)
                    .frame(
                        egui::Frame::NONE
                            .fill(ui.visuals().faint_bg_color)
                            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                            .corner_radius(egui::CornerRadius::same(8))
                            .inner_margin(egui::Margin::symmetric(12, 8)),
                    )
                    .show_inside(ui, |ui| {
                        let available_height = ui.available_height();
                        let top_padding = ((available_height - TOOLBAR_CONTENT_HEIGHT) / 2.0).max(0.0);
                        ui.add_space(top_padding);

                        ui.horizontal_centered(|ui| {
                            let back_button = ui.add(
                                egui::Button::new(RichText::new("◀ Back to menu").color(Color32::WHITE).strong())
                                    .corner_radius(egui::CornerRadius::same(6))
                                    .min_size(vec2(32.0, 28.0)),
                            );
                            if back_button.clicked() {
                                back_to_selection = true;
                            }

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            // --- Pause / Resume, coloré selon l'état ---
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
                                println!(
                                    "{}",
                                    if self.ui_state.is_paused { "pausing!" } else { "resuming!" }
                                );
                                //CommunicationTool::shit() ...
                            }

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);



                            // --- Save State ---
                            let save_state_button = ui.add(
                                egui::Button::new(RichText::new("Save State").color(Color32::WHITE).strong())
                                    .corner_radius(egui::CornerRadius::same(6))
                                    .min_size(vec2(110.0, 28.0)),
                            );
                            if save_state_button.clicked() {
                                todo!("envoyer une requête de sauvegarde d'état au core")
                            }

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            let debug_button = ui.add(
                                egui::Button::new(RichText::new("Open Debugger").color(Color32::WHITE).strong())
                                    .corner_radius(egui::CornerRadius::same(6))
                                    .min_size(vec2(120.0, 28.0)),
                            );
                            if debug_button.clicked() {
                                open_debugger = true;
                            }

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            let slider = egui::Slider::new(&mut self.ui_state.speed, 0.5..=8.0)
                                .step_by(0.5)
                                .suffix("x")
                                .show_value(true);
                            if ui.add_sized(vec2(140.0, 20.0), slider).changed() {
                                println!("googoogaga")
                            }

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);
                            let reset_button = ui.add(
                                egui::Button::new(RichText::new("🔄  Reset").color(Color32::WHITE).strong())
                                    .corner_radius(egui::CornerRadius::same(6))
                                    .min_size(vec2(90.0, 28.0)),
                            );
                            if reset_button.clicked() {
                                self.core_game.reset();
                            }

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);
                        });
                    });
            });

        if back_to_selection {
            return AppState::SelectionHub(self.into());
        }

        if open_debugger {
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
            ui_state: original.ui_state
            instruction_to_exec: None
        }
    }
}

impl From<SelectionDevice> for EmulationDevice {
    fn from(original: SelectionDevice) -> Self {
        let rom_path = original.path;
        let options = CoreGameOptions {
            gbtype: GbType::Dmg,
            rom_path,
            boot_rom: true,
            boot_rom_path: "boot-roms/dmg.bin".into(),
        };
        let core_game = CoreGameDevice::new(options);
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
        Self::default()
    }
}
