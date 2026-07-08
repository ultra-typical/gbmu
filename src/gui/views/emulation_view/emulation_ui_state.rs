use crate::{CROSSEMU_FILE, gui::CoreGameDevice};

pub const AUTHORIZED_SPEEDS_PERCENTS: [u16; 18] = [
    25, 50, 100, 150, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500,
];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameModeState {
    Running,
    Paused,
    Tick,
    Frame,
}

pub struct EmulationUiState {
    pub game_state: GameModeState,
    pub speed_indice: usize,
    pub volume: f32,
    pub show_save_popup: bool,
    pub save_name: String,
}

impl Default for EmulationUiState {
    fn default() -> Self {
        Self {
            game_state: GameModeState::Running,
            speed_indice: AUTHORIZED_SPEEDS_PERCENTS
                .iter()
                .position(|&speed| speed == 100)
                .expect("AUTHORIZED_SPEEDS_PERCENTS must contain 100"),
            volume: CROSSEMU_FILE.lock().unwrap().settings.volume,
            show_save_popup: false,
            save_name: String::new(),
        }
    }
}

pub struct EmulationDevice {
    pub core_game: CoreGameDevice,
    pub ui_state: EmulationUiState,
}
