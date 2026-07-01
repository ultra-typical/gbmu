mod display;

use crate::communications::{CpuState, InstructionList, Mode};
use crate::gui::{AppState, DebuggingDevice, WatchedAdresses};

use eframe::egui::load::SizedTexture;

use display::display_interface;

#[derive(Debug)]
struct DebuggingDataIn<'a> {
    is_step: bool,
    watched_address: &'a WatchedAdresses,
    registers: &'a CpuState,
    nb_instruction: u8,
    next_instructions: &'a InstructionList,
    hex_string: &'a String,
    error_message: Option<&'a String>,
    sized_texture: Option<SizedTexture>,
    instruction_to_exec: Option<String>,
}

#[derive(Debug)]
struct DebuggingDataOut {
    close_btn_clicked: bool,
    step_clicked: bool,
    step_mode_clicked: bool,
    instruction_to_exec: Option<String>,
    refresh_register_clicked: bool,
    nb_instruction_requested: u8,
    hex_string: String,
    register_new_addr: bool,
    delete_new_addr: Option<u16>,
}

enum OutState {
    Emulating,
    Debugging,
    Selection,
}

impl DebuggingDevice {
    pub fn update_info_struct(&mut self) -> Result<(), String> {
        self.core_game
            .interface_ct
            .get_next_instructions(&mut self.next_instructions)?;
        self.core_game
            .interface_ct
            .get_watched_adresses(&mut self.watched_adress)?;
        self.core_game
            .interface_ct
            .get_cpu_state(&mut self.registers)
    }

    fn execute_changes(&mut self, data: DebuggingDataOut) -> Result<OutState, String> {
        if data.close_btn_clicked {
            if self.ui_state.is_paused {
                self.core_game.interface_ct.set_mode(Mode::Stop)?;
                return Ok(OutState::Emulating);
            } else {
                self.core_game.interface_ct.set_mode(Mode::Game)?;
                return Ok(OutState::Emulating);
            }
        }

        if data.step_mode_clicked {
            if self.is_step {
                self.core_game.interface_ct.set_mode(Mode::Debug)?;
            } else {
                self.core_game.interface_ct.set_mode(Mode::Stop)?;
            }
            self.is_step = !self.is_step;
        }

        if data.refresh_register_clicked {
            self.core_game
                .interface_ct
                .get_cpu_state(&mut self.registers)?;
        }

        if data.step_clicked {
            self.core_game.interface_ct.execute_next_instructions(1)?;
        }

        if self.nb_instruction != data.nb_instruction_requested as usize {
            self.core_game
                .interface_ct
                .set_instruction_list_len(data.nb_instruction_requested)?;
            self.nb_instruction = data.nb_instruction_requested as usize;
        }

        self.hex_string = data.hex_string;
        if data.register_new_addr {
            self.error_message = match u16::from_str_radix(self.hex_string.as_ref(), 16) {
                Ok(value) => {
                    self.core_game.interface_ct.watch_adress(value)?;
                    None
                }
                Err(err) => Some(format!("{} is not a valid hex addr", self.hex_string)),
            }
        }

        if let Some(instr) = data.instruction_to_exec {
            self.core_game.interface_ct.execute_instruction(instr)?;
        }

        if let Some(addr) = data.delete_new_addr {
            self.core_game.interface_ct.remove_watch_address(addr)?;
        }

        if self.is_paused {
            self.core_game.interface_ct.set_mode(Mode::Stop)?;
        }

        Ok(OutState::Debugging)
    }

    pub fn debug_view(mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) -> AppState {
        self.core_game.capture_and_send_input(ui);
        let debugging_data_in = self.update_and_get_debugging_data(ui);
        let Ok(debugging_data_in) = debugging_data_in else {
            eprintln!("Unexpected issue : Communication between threads were cut");
            return AppState::SelectionHub(self.into());
        };
        let actions_to_perform = display_interface(ui, _frame, debugging_data_in);
        let state = match self.execute_changes(actions_to_perform) {
            Ok(state) => state,
            Err(error) => {
                eprintln!("Unexpected issue : {error}");
                OutState::Selection
            }
        };
        self.switch_state(state)
    }

    fn update_and_get_debugging_data(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<DebuggingDataIn<'_>, String> {
        self.core_game.update_and_size_image(ui)?;
        self.update_info_struct()?;

        let error_message = if let Some(value) = &self.error_message {
            Some(value)
        } else {
            None
        };

        Ok(DebuggingDataIn {
            is_step: (self.is_step || self.ui_state.is_paused),
            sized_texture: self.core_game.sized_image,
            watched_address: &self.watched_adress,
            registers: &self.registers,
            nb_instruction: self.nb_instruction as u8,
            next_instructions: &self.next_instructions,
            error_message,
            hex_string: &self.hex_string,
            instruction_to_exec: self.instruction_to_exec.clone(),
        })
    }

    fn switch_state(self, next_state: OutState) -> AppState {
        match next_state {
            OutState::Debugging => AppState::DebuggingHub(self),
            OutState::Emulating => AppState::EmulationHub(self.into()),
            OutState::Selection => AppState::SelectionHub(self.into()),
        }
    }
}
