#![allow(unused_variables)]
#![allow(dead_code)]

pub mod debbuger {

    use crate::gui::DebuggingDevice;

    pub fn update_info_struct(game: &mut DebuggingDevice) -> Result<(), String>{
        game.core_game.interface_ct.get_next_instructions(&mut game.next_instructions)?;
        game.core_game.interface_ct.get_watched_adresses(&mut game.watched_adress)?;
        game.core_game.interface_ct.get_cpu_state(&mut game.registers)
    }

    /*
    impl DebuggingDevice {
        pub fn execute_instruction(&self, instr: u8) {
            let _ = self
                .core_game
                .command_query_sender
                .try_send(DebugCommandQueries::ExecuteInstruction(instr));
        }

        pub fn get_next_instructions(&self, instr_nb: u8) {
            let _ = self
                .core_game
                .command_query_sender
                .try_send(DebugCommandQueries::GetNextInstructions(instr_nb));
        }

        pub fn request_registers(&self) {
            let _ = self
                .core_game
                .command_query_sender
                .try_send(DebugCommandQueries::GetRegisters);
        }

        pub fn request_step_mode(&self) {
            let _ = self
                .core_game
                .command_query_sender
                .try_send(DebugCommandQueries::SetStepMode);
        }

        pub fn executed_next_step(&self, nb_instru: usize) {
            let _ = self
                .core_game
                .command_query_sender
                .try_send(DebugCommandQueries::ExecuteNextInstructions(nb_instru));
        }

        pub fn request_watch_address(&self, address: u16) {
            let _ = self
                .core_game
                .command_query_sender
                .try_send(DebugCommandQueries::WatchAddress(address));
        }

        fn get_watched_addresses(&self) {
            let _ = self
                .core_game
                .command_query_sender
                .try_send(DebugCommandQueries::GetAddresses);
        }
    }
    */
}
