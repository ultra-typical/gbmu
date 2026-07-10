#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct SampleBuffer {
    buffer: Arc<Mutex<VecDeque<(f32, f32)>>>,
    pub audio_starting: Arc<AtomicBool>,
}

impl SampleBuffer {
    pub fn new() -> Self {
        SampleBuffer {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            audio_starting: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn push(&self, sample: (f32, f32)) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push_back(sample);
    }

    pub fn pop(&self) -> Option<(f32, f32)> {
        if !self.audio_starting.load(Ordering::Relaxed) {
            self.audio_starting.store(true, Ordering::Relaxed);
        }

        let mut buffer = self.buffer.lock().unwrap();
        buffer.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let buffer = self.buffer.lock().unwrap();
        buffer.is_empty()
    }

    pub fn len(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }

    pub fn clear(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }
}
