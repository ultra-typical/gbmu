#![allow(dead_code)]

mod game_tool;
mod interface_tool;
#[cfg(test)]
mod tests;

use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicIsize},
    },
};
use tokio::sync::mpsc::channel;
use tokio::sync::watch;

pub use game_tool::GameCT;
pub use game_tool::GameCommunicationTool;
pub use interface_tool::InterfaceCT;
pub use interface_tool::InterfaceCommunicationTool;

#[derive(Default, Debug)]
pub struct InstructionList(pub Vec<(u16, String)>);

use std::ops::{Deref, DerefMut};

impl DerefMut for InstructionList {
    fn deref_mut(&mut self) -> &mut Vec<(u16, String)> {
        &mut self.0
    }
}

impl Deref for InstructionList {
    type Target = Vec<(u16, String)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct CpuState {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Default, Debug)]
pub struct WatchedAdresses(pub Vec<(u16, u8)>);

impl DerefMut for WatchedAdresses {
    fn deref_mut(&mut self) -> &mut Vec<(u16, u8)> {
        &mut self.0
    }
}

impl Deref for WatchedAdresses {
    type Target = Vec<(u16, u8)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::gui::keymapping::KeyInput;

pub const FRAME_SIZE_IN_U8: usize = FRAME_SIZE * 3;

#[derive(Debug)]
pub enum Mode {
    Game,
    Debug,
    Stop,
}

#[derive(Debug)]
pub enum Request {
    Mode(Mode),
    Fps(bool),
    Execute(String),
    RenderFrame(u16),
    Watch(u16),
    SaveState(PathBuf),
    StopWatch(u16),
    Step(usize),
    SetInstructionListLength(u8),
    SetSpeed(u8),
    SetVolume(u8),
}

pub const FRAME_SIZE: usize = 160 * 144;

pub fn create_communication_tools() -> (Box<dyn GameCT>, Box<dyn InterfaceCT>) {
    let (input_sender, input_receiver) = watch::channel(KeyInput::default());
    let image = Arc::new(Mutex::new(vec![[0u8; 3]; FRAME_SIZE]));
    let image_has_changed = Arc::new(AtomicBool::new(false));
    let fps = Arc::new(AtomicIsize::new(0));
    let (cpu_state_sender, cpu_state_receiver) = watch::channel(CpuState::default());
    let (instructions_sender, instructions_receiver) =
        watch::channel(Arc::new(InstructionList::default()));
    let (watched_addresses_sender, watched_addresses_receiver) =
        watch::channel(Arc::new(WatchedAdresses::default()));
    let (request_sender, request_receiver) = channel::<Request>(50);

    (
        Box::new(GameCommunicationTool::new(
            input_receiver,
            fps.clone(),
            image.clone(),
            image_has_changed.clone(),
            cpu_state_sender,
            instructions_sender,
            request_receiver,
            watched_addresses_sender,
        )),
        Box::new(InterfaceCommunicationTool::new(
            input_sender,
            image,
            image_has_changed,
            fps,
            cpu_state_receiver,
            instructions_receiver,
            request_sender,
            watched_addresses_receiver,
        )),
    )
}
