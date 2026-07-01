use crate::GBMU_FILE;
use crate::gui::egui::Id;
use crate::gui::{AppState, SelectionDevice};
use eframe::egui;
use std::fs;
use std::path::{Path, PathBuf};

enum OutState {
    Emulation,
    Selection,
}

impl SelectionDevice {
    fn list_save_states(&self) -> Vec<PathBuf> {
        let Some(home) = dirs::home_dir() else {
            return Vec::new();
        };
        let gbmu_dir = home.join(".gbmu");

        let Ok(entries) = fs::read_dir(&gbmu_dir) else {
            return Vec::new();
        };

        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .collect()
    }

    fn get_or_load_preview(
        &mut self,
        ctx: &egui::Context,
        save_dir: &Path,
    ) -> Option<egui::TextureHandle> {
        let name = save_dir.file_name()?.to_string_lossy().to_string();

        if let Some(texture) = self.save_state_previews.get(&name) {
            return Some(texture.clone());
        }

        let preview_path = save_dir.join("preview");
        let bytes = fs::read(&preview_path).ok()?;

        const WIDTH: usize = 160;
        const HEIGHT: usize = 144;

        if bytes.len() != WIDTH * HEIGHT * 3 {
            eprintln!(
                "Preview size mismatch for '{name}': expected {} bytes, got {}",
                WIDTH * HEIGHT * 3,
                bytes.len()
            );
            return None;
        }

        let color_image = egui::ColorImage::from_rgb([WIDTH, HEIGHT], &bytes);
        let texture = ctx.load_texture(&name, color_image, egui::TextureOptions::NEAREST);

        self.save_state_previews.insert(name, texture.clone());
        Some(texture)
    }

    fn try_capture_key(&mut self, ctx: &egui::Context) {
        let Some(action) = self.listening else { return };

        let captured = ctx.input(|i| {
            i.events.iter().find_map(|event| match event {
                egui::Event::Key {
                    key,
                    pressed: true,
                    repeat: false,
                    ..
                } => Some(*key),
                _ => None,
            })
        });

        if let Some(key) = captured {
            if key != egui::Key::Escape {
                self.key_mapping.remap(action, key);
                GBMU_FILE.lock().unwrap().settings.keymapping = self.key_mapping.clone();
                GBMU_FILE.lock().unwrap().persist();
            }
            self.listening = None;
        }
    }

    fn key_cell(&mut self, ui: &mut egui::Ui, action: &'static str) {
        ui.vertical_centered(|ui| {
            let is_listening = self.listening == Some(action);

            let label = if is_listening {
                "...".to_string()
            } else {
                format!("{:?}", self.key_mapping.get(action))
            };

            let mut button = egui::Button::new(label);
            if is_listening {
                button = button.fill(ui.visuals().selection.bg_fill);
            }

            if ui.add_sized(egui::vec2(42.0, 30.0), button).clicked() {
                self.listening = Some(action);
            }

            ui.add_space(2.0);
            ui.label(egui::RichText::new(action).size(11.0).weak());
        });
    }

    pub fn selection_view(mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) -> AppState {
        self.display(ui, _frame);
        let next_state = self.next_state();
        self.update_view(next_state)
    }

    fn next_state(&mut self) -> OutState {
        let path = Path::new(&self.path);
        if path.is_file() {
            let rom_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string();
            let mut gbmu = GBMU_FILE.lock().unwrap();
            gbmu.record_launch(rom_name, PathBuf::from(&self.path));
            OutState::Emulation
        } else {
            OutState::Selection
        }
    }

    fn update_view(self, state: OutState) -> AppState {
        match state {
            OutState::Emulation => AppState::EmulationHub(self.into()),
            OutState::Selection => AppState::SelectionHub(self),
        }
    }

    fn display(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.try_capture_key(&ui.ctx().clone());
        if let Some(path) = ui.ctx().input(|i| {
            i.raw
                .dropped_files
                .first()
                .and_then(|file| file.path.clone())
        }) {
            self.path = path.to_string_lossy().to_string();
        }

        egui::Panel::bottom(Id::new("bot"))
            .resizable(true)
            .default_size(220.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    let keymappings_width = 150.0;
                    ui.allocate_ui(egui::vec2(keymappings_width, ui.available_height()), |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Keymappings");
                            ui.add_space(12.0);

                            ui.horizontal(|ui| {
                                ui.add_space(10.0);

                                egui::Grid::new("dpad_grid")
                                    .spacing(egui::vec2(6.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.label("");
                                        self.key_cell(ui, "Up");
                                        ui.label("");
                                        ui.end_row();

                                        self.key_cell(ui, "Left");
                                        ui.label("");
                                        self.key_cell(ui, "Right");
                                        ui.end_row();

                                        ui.label("");
                                        self.key_cell(ui, "Down");
                                        ui.label("");
                                        ui.end_row();
                                    });

                                ui.add_space(28.0);

                                ui.vertical(|ui| {
                                    egui::Grid::new("ab_grid")
                                        .spacing(egui::vec2(3.0, 6.0))
                                        .show(ui, |ui| {
                                            ui.label("");
                                            ui.label("");
                                            self.key_cell(ui, "B");
                                            ui.end_row();

                                            self.key_cell(ui, "A");
                                            ui.label("");
                                            ui.end_row();
                                            self.key_cell(ui, "Select");
                                            ui.label("");
                                            self.key_cell(ui, "Start");
                                            ui.label("");
                                            ui.end_row();
                                        });
                                });
                            });
                        });
                    });

                    ui.separator();
                    ui.allocate_ui(
                        egui::vec2(ui.available_width(), ui.available_height()),
                        |ui| {
                            ui.vertical(|ui| {
                                ui.heading("Save States selector");
                                ui.add_space(4.0);

                                let save_states = self.list_save_states();

                                egui::ScrollArea::vertical()
                                    .id_salt("save_states_scroll")
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        for save_dir in &save_states {
                                            let name = save_dir
                                                .file_name()
                                                .map(|n| n.to_string_lossy().to_string())
                                                .unwrap_or_else(|| "Unknown".to_string());

                                            ui.horizontal(|ui| {
                                                if let Some(texture) =
                                                    self.get_or_load_preview(ui.ctx(), save_dir)
                                                {
                                                    ui.add(
                                                        egui::Image::new(&texture)
                                                            .fit_to_exact_size(egui::vec2(
                                                                64.0, 57.6,
                                                            )),
                                                    );
                                                } else {
                                                    let (rect, _) = ui.allocate_exact_size(
                                                        egui::vec2(64.0, 57.6),
                                                        egui::Sense::hover(),
                                                    );
                                                    ui.painter().rect_filled(
                                                        rect,
                                                        2.0,
                                                        ui.visuals().extreme_bg_color,
                                                    );
                                                }

                                                ui.vertical(|ui| {
                                                    ui.label(&name);
                                                    if ui.small_button("Load").clicked() {
                                                        todo!("Load save state")
                                                    }
                                                    if ui.small_button("Delete").clicked() {
                                                        if let Err(e) = fs::remove_dir_all(save_dir)
                                                        {
                                                            eprintln!(
                                                                "Could not delete '{name}': {e}"
                                                            );
                                                        } else {
                                                            self.save_state_previews.remove(&name);
                                                        }
                                                    }
                                                });
                                            });
                                            ui.add_space(4.0);
                                        }
                                    });
                            });
                        },
                    );
                });
                ui.separator();
            });
        ui.separator();

        ui.vertical(|ui| {
            ui.heading("Settings");
            ui.add_space(4.0);
        });

        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                    ui.checkbox(&mut self.launch_cgb, "Launch Gameboy Color");
                });
            });
        });

        egui::Panel::right("history_panel")
            .resizable(true)
            .default_size(250.0)
            .show_inside(ui, |ui| {
                ui.heading("History");

                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.text_edit_singleline(&mut self.search);
                });
                ui.add_space(6.0);
                ui.separator();

                let search_lower = self.search.to_lowercase();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let gbmu = GBMU_FILE.lock().unwrap();
                    for entry in gbmu.history.iter().filter(|entry| {
                        search_lower.is_empty()
                            || entry.rom_name.to_lowercase().contains(&search_lower)
                    }) {
                        let subtitle = format!(
                            "Launches: {} \nLast: {}",
                            entry.launch_count,
                            entry.last_launched.format("%d/%m/%Y %H:%M")
                        );
                        let text = format!("▶ {}\n{}", entry.rom_name, subtitle);
                        let button = egui::Button::new(egui::RichText::new(text).size(16.0))
                            .min_size(egui::vec2(220.0, 48.0))
                            .corner_radius(5.0);
                        if ui.add(button).clicked() {
                            self.path = entry.rom_path.to_string_lossy().to_string();
                        }
                        ui.add_space(6.0);
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(ui.style()).inner_margin(0.0))
            .show_inside(ui, |ui| {
                let drop_size = ui.available_size();
                let is_hovering_file = ui.ctx().input(|i| !i.raw.hovered_files.is_empty());

                let frame = egui::Frame::canvas(ui.style()).stroke(egui::Stroke::new(
                    2.0,
                    if is_hovering_file {
                        ui.visuals().selection.stroke.color
                    } else {
                        ui.visuals().widgets.inactive.bg_stroke.color
                    },
                ));

                frame.show(ui, |ui| {
                    ui.set_min_size(drop_size);
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("+").size(64.0).weak());
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("Drag and drop a ROM here")
                                    .size(16.0)
                                    .weak(),
                            );
                            ui.add_space(12.0);

                            if ui.button("Choose rom").clicked() {
                                self.file_dialog.pick_file();
                            }

                            self.file_dialog.update(ui.ctx());

                            if let Some(path) = self.file_dialog.take_picked() {
                                self.path = path.into_os_string().into_string().unwrap();
                            }
                        });
                    });
                });
            });
    }
}
