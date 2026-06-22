use crate::gui::{AppState, SelectionDevice};
use crate::{GBMU_FILE};
use crate::gui::egui::Id;
use eframe::egui;
use std::path::{PathBuf, Path};

enum OutState {
    Emulation,
    Selection,
}

impl SelectionDevice {
    pub fn selection_view(
        mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) -> AppState {
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
        if let Some(path) = ui.ctx().input(|i| {
            i.raw.dropped_files
                .first()
                .and_then(|file| file.path.clone())
        }) {
            self.path = path.to_string_lossy().to_string();
        }

        egui::Panel::bottom(Id::new("toppannel"))
            .show_inside(ui, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT),|ui| {
                egui::widgets::global_theme_preference_switch(ui);
                });
            });

        egui::Panel::right("history_panel")
            .resizable(true)
            .default_size(270.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("History");
                let gbmu = GBMU_FILE.lock().unwrap();
                for entry in &gbmu.history {

                let subtitle = format!(
                    "Launches: {} \nLast: {}",
                    entry.launch_count,
                    entry.last_launched.format("%d/%m/%Y %H:%M")
                );

                let text = format!(
                    "▶ {}\n{}",
                    entry.rom_name,
                    subtitle
                );

                let button = egui::Button::new(
                    egui::RichText::new(text)
                        .size(16.0)
                )
                .min_size(egui::vec2(220.0, 48.0))
                .corner_radius(5.0);

                if ui.add(button).clicked() {
                    self.path = entry.rom_path.to_string_lossy().to_string();
                }

                ui.add_space(6.0);
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.centered_and_justified(|ui| {
                if ui.button("Pick file").clicked() {
                    self.file_dialog.pick_file();
                }

                ui.label(format!("Picked file: {:?}", self.picked_file));

                self.file_dialog.update(ui.ctx());

                if let Some(path) = self.file_dialog.take_picked() {
                    self.path = path.into_os_string().into_string().unwrap();
                }
            })
        });
    }
}
