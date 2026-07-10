#![allow(unused_variables, dead_code)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};

use crate::sound::start_audio;

pub mod channel3;
pub mod channel4;
pub mod channels_square;
pub mod registers;
pub mod sample_buffer;

use crate::mmu::apu::registers::*;
use channel3::ChannelThree;
use channel4::ChannelFour;
use channels_square::ChannelSquare;
use sample_buffer::SampleBuffer;

const T_CYCLES_PER_SEC: f64 = 4_194_304.0;
const SAMPLE_RATE: f64 = 48_000.0;
const BASE_CYCLE: f64 = T_CYCLES_PER_SEC / SAMPLE_RATE; // ~= 87.38

fn default_audio_running() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(true))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apu {
    nr50_master_vol_and_vin_panning: MasterVolVinPanningReg,
    nr51_sound_panning: SoundPanningReg,
    nr52_audio_master_control: AudioMasterControlReg,

    channel_one: ChannelSquare,
    channel_two: ChannelSquare,
    channel_three: ChannelThree,
    channel_four: ChannelFour,

    // Runtime-only handles to the audio playback thread: not meaningful to
    // persist, and reconstructed unlinked to any thread on deserialize (the
    // caller must re-hook audio playback afterwards, same as Apu::new does).
    #[serde(skip, default = "default_audio_running")]
    pub audio_running: Arc<AtomicBool>,
    sample_counter: f64,
    frame_seq_counter: u64,
    frame_seq_step: u8,
    #[serde(skip)]
    pub sample_buffer: SampleBuffer,
    pub volume: f32, // 1.0 = 100%
    speed: f64,
}

impl Apu {
    pub fn new() -> Self {
        let sample_buffer = SampleBuffer::new();
        let audio_running = Arc::new(AtomicBool::new(true));
        start_audio(sample_buffer.clone(), audio_running.clone());

        Self {
            nr50_master_vol_and_vin_panning: MasterVolVinPanningReg::default(),
            nr51_sound_panning: SoundPanningReg::default(),
            nr52_audio_master_control: AudioMasterControlReg::default(),
            channel_one: ChannelSquare::default(),
            channel_two: ChannelSquare::default(),
            channel_three: ChannelThree::default(),
            channel_four: ChannelFour::default(),
            audio_running,
            sample_counter: 0.0,
            frame_seq_counter: 0,
            frame_seq_step: 0,
            sample_buffer,
            volume: 1.0,
            speed: 1.0,
        }
    }

    pub fn step(&mut self) {
        self.channel_one.step();
        self.channel_two.step();
        self.channel_three.step();
        self.channel_four.step();

        self.sample_counter += 1.0;
        self.frame_seq_counter += 1;
        if self.frame_seq_counter >= 8192 {
            self.frame_seq_counter -= 8192;
            self.frame_seq_step = (self.frame_seq_step + 1) % 8;

            match self.frame_seq_step {
                0 | 4 => {
                    self.channel_one.tick_length();
                    self.channel_two.tick_length();
                    self.channel_three.tick_length();
                    self.channel_four.tick_length();
                }
                2 | 6 => {
                    self.channel_one.tick_length();
                    self.channel_two.tick_length();
                    self.channel_three.tick_length();
                    self.channel_four.tick_length();
                    self.channel_one.tick_sweep();
                }
                7 => {
                    self.channel_one.tick_envelope();
                    self.channel_two.tick_envelope();
                    self.channel_four.tick_envelope();
                }
                _ => {}
            }
        }

        let cycles_per_sample = BASE_CYCLE * self.speed;
        if self.sample_counter >= cycles_per_sample {
            self.sample_counter -= cycles_per_sample;

            let mixed = (self.channel_one.output()
                + self.channel_two.output()
                + self.channel_three.output()
                + self.channel_four.output())
                / 4.0;
            let sample = (mixed * self.volume).clamp(-1.0, 1.0);

            if self.sample_buffer.audio_starting.load(Ordering::Relaxed) {
                self.sample_buffer.push((sample, sample));
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF10 => self.channel_one.nr0_sweep.read(),
            0xFF11 => self.channel_one.nr1_ln_timer_duty_cycle.read(),
            0xFF12 => self.channel_one.nr2_volume_envelope.read(),
            0xFF13 => self.channel_one.nr3_period_low.read(),
            0xFF14 => self.channel_one.nr4_period_high_ctrl.read(),
            0xFF16 => self.channel_two.nr1_ln_timer_duty_cycle.read(),
            0xFF17 => self.channel_two.nr2_volume_envelope.read(),
            0xFF18 => self.channel_two.nr3_period_low.read(),
            0xFF19 => self.channel_two.nr4_period_high_ctrl.read(),
            0xFF1A => self.channel_three.nr30_dac_enable.read(),
            0xFF1B => self.channel_three.nr31_ln_timer.read(),
            0xFF1C => self.channel_three.nr32_output_level.read(),
            0xFF1D => self.channel_three.nr33_period_low.read(),
            0xFF1E => self.channel_three.nr34_period_high_crtl.read(),
            0xFF20 => self.channel_four.nr41_length_timer.read(),
            0xFF21 => self.channel_four.nr42_volume_envelope.read(),
            0xFF22 => self.channel_four.nr43_freq_and_randomness.read(),
            0xFF23 => self.channel_four.nr44_control.read(),
            0xFF24 => self.nr50_master_vol_and_vin_panning.read(),
            0xFF25 => self.nr51_sound_panning.read(),
            0xFF26 => self.nr52_audio_master_control.read(),
            0xFF30..=0xFF3F => {
                let index = (addr - 0xFF30) as usize;
                self.channel_three.wave_ram[index]
            }
            _ => 0xFF,
        }
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF10 => self.channel_one.nr0_sweep.write(value),
            0xFF11 => self.channel_one.nr1_ln_timer_duty_cycle.write(value),
            0xFF12 => self.channel_one.nr2_volume_envelope.write(value),
            0xFF13 => self.channel_one.nr3_period_low.write(value),
            0xFF14 => {
                self.channel_one.nr4_period_high_ctrl.write(value);
                if value & 0b1000_0000 != 0 {
                    self.channel_one.trigger();
                }
            }
            0xFF16 => self.channel_two.nr1_ln_timer_duty_cycle.write(value),
            0xFF17 => self.channel_two.nr2_volume_envelope.write(value),
            0xFF18 => self.channel_two.nr3_period_low.write(value),
            0xFF19 => {
                self.channel_two.nr4_period_high_ctrl.write(value);
                if value & 0b1000_0000 != 0 {
                    self.channel_two.trigger();
                }
            }
            0xFF1A => self.channel_three.nr30_dac_enable.write(value),
            0xFF1B => self.channel_three.nr31_ln_timer.write(value),
            0xFF1C => self.channel_three.nr32_output_level.write(value),
            0xFF1D => self.channel_three.nr33_period_low.write(value),
            0xFF1E => {
                self.channel_three.nr34_period_high_crtl.write(value);
                if value & 0b1000_0000 != 0 {
                    self.channel_three.trigger();
                }
            }
            0xFF20 => self.channel_four.nr41_length_timer.write(value),
            0xFF21 => self.channel_four.nr42_volume_envelope.write(value),
            0xFF22 => self.channel_four.nr43_freq_and_randomness.write(value),
            0xFF23 => {
                self.channel_four.nr44_control.write(value);
                if value & 0b1000_0000 != 0 {
                    self.channel_four.trigger();
                }
            }
            0xFF24 => self.nr50_master_vol_and_vin_panning.write(value),
            0xFF25 => self.nr51_sound_panning.write(value),
            0xFF26 => self.nr52_audio_master_control.write(value),
            0xFF30..=0xFF3F => {
                let index = (addr - 0xFF30) as usize;
                self.channel_three.wave_ram[index] = value;
            }
            _ => {}
        }
    }

    pub fn set_volume(&mut self, percent: u8) {
        self.volume = percent as f32 / 100.0;
    }

    pub fn set_speed(&mut self, percent: u16) {
        self.speed = percent as f64 / 100.0;
    }
}

impl Drop for Apu {
    fn drop(&mut self) {
        self.audio_running.store(false, Ordering::Relaxed);
    }
}

trait Channel {}
