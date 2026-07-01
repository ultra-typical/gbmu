use crate::{GBMU_FILE, gui::CoreGameDevice};

pub struct EmulationUiState {
    pub is_paused: bool,
    pub speed: f32,
    pub volume: f32,
    pub show_save_popup: bool,
    pub save_name: String,
}

impl Default for EmulationUiState {
    fn default() -> Self {
        Self {
            is_paused: false,
            speed: 1.0,
            volume: GBMU_FILE.lock().unwrap().settings.volume,
            show_save_popup: false,
            save_name: String::new(),
        }
    }
}

pub struct EmulationDevice {
    pub core_game: CoreGameDevice,
    pub ui_state: EmulationUiState,
}
