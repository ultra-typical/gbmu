use std::{sync::{Arc, Mutex, atomic::{AtomicBool, AtomicIsize}}};
use tokio::sync::watch;
use crate::gui::KeyInput;
use tokio::sync::mpsc::Receiver;


use super::CpuState;
use super::InstructionList;
use super::WatchedAdresses;
use super::Color;
use super::Request;

pub trait GameCT: Send {
    // Emulation
    fn update_input(&mut self, input_ref: &mut KeyInput) -> Result<(), String>;
    fn update_fps(&mut self, fps: u128)-> Result<(), String>;
    fn put_pixel_to_frame(&mut self, offset: usize, color: Color);

    // Debug
    fn send_cpu_state(&mut self, state: &CpuState);
    fn send_next_instructions(&mut self, list: InstructionList);
    fn send_watched_adresses(&mut self, addresses: WatchedAdresses);
    fn poll_requests(&mut self) -> Vec<Request>;
}

pub struct GameCommunicationTool {
    input_receiver: watch::Receiver<KeyInput>,
    fps: Arc<AtomicIsize>,
    image: Arc<Mutex<Vec<Color>>>,
    image_has_changed: Arc<AtomicBool>,
    cpu_state_sender: watch::Sender<CpuState>,
    instructions_sender: watch::Sender<Arc<InstructionList>>,
    request_receiver: Receiver<Request>,
    watched_addresses_sender: watch::Sender<Arc<WatchedAdresses>>
}

impl GameCommunicationTool {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        input_receiver: watch::Receiver<KeyInput>,
        fps: Arc<AtomicIsize>,
        image: Arc<Mutex<Vec<Color>>>,
        image_has_changed: Arc<AtomicBool>,
        cpu_state_sender: watch::Sender<CpuState>,
        instructions_sender: watch::Sender<Arc<InstructionList>>,
        request_receiver: Receiver<Request>,
        watched_addresses_sender: watch::Sender<Arc<WatchedAdresses>>
    ) -> Self {
        Self {
            input_receiver,
            fps,
            image,
            image_has_changed,
            cpu_state_sender,
            instructions_sender,
            request_receiver,
            watched_addresses_sender
        }
    }
}

impl GameCT for GameCommunicationTool {
    fn update_input(&mut self, input_ref: &mut KeyInput) -> Result<(), String> {
        match self.input_receiver.has_changed() {
            Ok(true) => {*input_ref = *self.input_receiver.borrow_and_update(); Ok(())}
            Ok(false) => {Ok(())}
            Err(_) => {Err("Unexpected error during input update".to_string())}
        }
    }

    fn put_pixel_to_frame(
        &mut self,
        offset: usize,
        pixel_color: Color
    ) {
        if let Ok(mut image) = self.image.lock() {
            image[offset] = pixel_color;
            drop(image);
            self.image_has_changed.store(true, std::sync::atomic::Ordering::Relaxed);
        } else  {
            panic!("Lock was poisoned")
        }
    }

    fn update_fps(&mut self, fps: u128) -> Result<(), String> {
        self.fps.store(fps as isize, std::sync::atomic::Ordering::Relaxed); Ok(())
    }

    // Debug
    fn send_cpu_state(&mut self, state: &CpuState) { 
        let _ = self.cpu_state_sender.send(*state);
    }
    
    fn send_next_instructions(&mut self, list: InstructionList) { 
        let _ = self.instructions_sender.send(Arc::from(list));
    }

    fn send_watched_adresses(&mut self, addresses: WatchedAdresses) { 
        let _ = self.watched_addresses_sender.send(Arc::from(addresses));
    }

    fn poll_requests(&mut self) -> Vec<Request> {
        let mut request_vec = Vec::<Request>::new();
        while let Ok(request) = self.request_receiver.try_recv().map_err(|err| format!("State of the request receiver : {:?}", err)) {
            request_vec.push(request);
        }
        request_vec
    }
}
