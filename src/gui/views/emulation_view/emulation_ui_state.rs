use crate::gui::CoreGameDevice;


pub struct EmulationUiState {
    pub is_paused: bool,
    pub speed: f32,
}

impl Default for EmulationUiState {
    fn default() -> Self {
        Self {
            is_paused: false,
            speed: 1.0,
        }
    }
}

pub struct EmulationDevice {
    pub core_game: CoreGameDevice,
    pub ui_state: EmulationUiState,
}