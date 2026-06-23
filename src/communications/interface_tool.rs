use std::{sync::{Arc, Mutex, atomic::{AtomicBool, AtomicIsize}}};
use std::sync::atomic::Ordering;
use tokio::sync::watch;
use crate::gui::KeyInput;
use tokio::sync::mpsc::Sender;

use super::FRAME_SIZE_IN_U8;
use super::CpuState;
use super::InstructionList;
use super::WatchedAdresses;
use super::Color;
use super::Mode;
use super::Request;

pub trait InterfaceCT {
    // Emulation 
    fn send_input(&self, input: KeyInput) -> Result<(), String>;
    fn get_new_image(&mut self, buffer: &mut [u8; FRAME_SIZE_IN_U8]) -> Result<Option<()>, String>;
    fn get_fps(&self) -> Result<u128, String>;

    // Debug
    fn get_cpu_state(&mut self, state: &mut CpuState) -> Result<(), String>;
    fn get_next_instructions(&mut self, list: &mut InstructionList) -> Result<(), String>;
    fn get_watched_adresses(&mut self, addresses: &mut WatchedAdresses) -> Result<(), String>;

    fn set_mode(&self, value: Mode) -> Result<(), String>;

    //// Execution instructions
    fn ask_fps_counter(&self) -> Result<(), String>;
    fn disable_fps_counter(&self) -> Result<(), String>;
    fn execute_instruction(&self, instruction: Vec<u8>) -> Result<(), String>;
    fn execute_next_instructions(&self, instruction_nb: usize) -> Result<(), String>;
    fn render_frame(&self) -> Result<(), String>;
    fn render_frames(&self, frame_nb: u16) -> Result<(), String>;

    //// Debug instructions
    fn watch_adress(&self, addr_to_watch: u16) -> Result<(), String>;
    fn set_instruction_list_len(&self, list_len: u8) -> Result<(), String>;
    fn remove_watch_address(&self, addr_to_delete: u16) -> Result<(), String>;
}

pub struct InterfaceCommunicationTool {
    input_sender: watch::Sender<KeyInput>,
    image: Arc<Mutex<Vec<Color>>>,
    image_has_changed: Arc<AtomicBool>,
    fps: Arc<AtomicIsize>,
    cpu_state_receiver: watch::Receiver<CpuState>,
    instructions_receiver: watch::Receiver<Arc<InstructionList>>,
    request_sender: Sender<Request>,
    watched_addresses_receiver: watch::Receiver<Arc<WatchedAdresses>>
}

impl InterfaceCommunicationTool {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        input_sender: watch::Sender<KeyInput>,
        image: Arc<Mutex<Vec<Color>>>,
        image_has_changed: Arc<AtomicBool>,
        fps: Arc<AtomicIsize>,
        cpu_state_receiver: watch::Receiver<CpuState>,
        instructions_receiver: watch::Receiver<Arc<InstructionList>>,
        request_sender: Sender<Request>,
        watched_addresses_receiver: watch::Receiver<Arc<WatchedAdresses>>
    ) -> Self {
        Self {
            input_sender,
            image_has_changed,
            image,
            fps,
            cpu_state_receiver,
            instructions_receiver,
            request_sender,
            watched_addresses_receiver
        }
    }

    pub fn try_send_query(&self, request:Request) -> Result<(), String> {
        self.request_sender.try_send(request).map_err(
            | _ |
            String::from("Request couldn't be sent")
        )
    }
}

impl InterfaceCT for InterfaceCommunicationTool {

    // Emulation 
    fn send_input(&self, input: KeyInput) -> Result<(), String>{
        self.input_sender.send(input).map_err(|err| format!("send impossible {}", err))
    }


    fn get_new_image(&mut self, buffer: &mut [u8; FRAME_SIZE_IN_U8]) -> Result<Option<()>, String> {
        if !self.image_has_changed.load(Ordering::Relaxed) {
            Ok(None)
        } else {
            let image_copy = if let Ok(image) = self.image.lock() {
                Ok(image.clone())
            } else {
                Err("Mutex lock was poisoned".to_string())
            }?;

            image_copy.into_iter().enumerate().for_each(|(index, color)| {
                buffer[index * 3..index * 3 + 3].copy_from_slice(color.to_rgb());
            });
            self.image_has_changed.store(false,Ordering::Relaxed);
            Ok(Some(()))
        }
    }

    fn get_fps(&self) -> Result<u128, String> {
        Ok(self.fps.load(Ordering::Relaxed) as u128)   
    }

    // Debug
    fn get_cpu_state(&mut self, state: &mut CpuState) -> Result<(), String> {
        if self.cpu_state_receiver.has_changed().map_err(|_| "really?".to_string())?{
            *state = *self.cpu_state_receiver.borrow_and_update();
        }
        Ok(())
    }

    fn get_next_instructions(&mut self, list: &mut InstructionList) -> Result<(), String>  {
        if self.instructions_receiver.has_changed().map_err(|_| "really ?".to_string())?{
            list.clear();
            list.extend_from_slice(&self.instructions_receiver.borrow_and_update());
        }
        Ok(())
    }

    fn get_watched_adresses(&mut self, _addresses: &mut WatchedAdresses)-> Result<(), String>  {
        if self.watched_addresses_receiver.has_changed().map_err(|_| "really ?".to_string())?{
            _addresses.clear();
            _addresses.extend_from_slice(&self.watched_addresses_receiver.borrow_and_update());
        }
        Ok(())
    }


    //// Execution instructions
    fn ask_fps_counter(&self)-> Result<(), String>  {
        self.try_send_query(Request::Fps(true))
    }

    fn disable_fps_counter(&self)-> Result<(), String>  {
        self.try_send_query(Request::Fps(false))
    }

    fn execute_instruction(&self, instruction: Vec<u8>) -> Result<(), String>  {
        self.try_send_query(Request::Execute(instruction))
    }

    fn execute_next_instructions(&self, instruction_nb: usize) -> Result<(), String>  {
        self.try_send_query(Request::Step(instruction_nb))
    }

    fn render_frame(&self) -> Result<(), String>  {
        self.try_send_query(Request::RenderFrame(1))
    }

    fn render_frames(&self, frame_nb: u16) -> Result<(), String>  {
        self.try_send_query(Request::RenderFrame(frame_nb))
    }

    fn set_mode(&self, mode: Mode) -> Result<(), String> {
        self.try_send_query(Request::Mode(mode))
    }

    //// Debug instructions
    fn watch_adress(&self, addr_to_watch: u16) -> Result<(), String>  {
        self.try_send_query(Request::Watch(addr_to_watch))
    }
    
    fn set_instruction_list_len(&self, list_len: u8) -> Result<(), String>  {
        self.try_send_query(Request::SetInstructionListLength(list_len))
    }

    fn remove_watch_address(&self, addr_to_delete: u16) -> Result<(), String>  {
        self.try_send_query(Request::StopWatch(addr_to_delete))
    }
}