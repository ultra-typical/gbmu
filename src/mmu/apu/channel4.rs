use super::registers::*;

pub struct ChannelFour {
    pub nr41_length_timer: NoiseLengthTimer,
    pub nr42_volume_envelope: VolumeEnvReg,
    pub nr43_freq_and_randomness: FreqRandomnessReg,
    pub nr44_control: ChannelFourCtrlReg,

    enabled: bool,
    length_counter: u8,
    volume: u8,
    freq_timer: u32,
    envelope_timer: u8,
    lfsr: u16, // Linear-feedback shift register
}

impl Default for ChannelFour {
    fn default() -> Self {
        Self {
            nr41_length_timer: Default::default(),
            nr42_volume_envelope: Default::default(),
            nr43_freq_and_randomness: Default::default(),
            nr44_control: Default::default(),

            enabled: Default::default(),
            length_counter: Default::default(),
            volume: Default::default(),
            freq_timer: Default::default(),
            envelope_timer: Default::default(),
            lfsr: 0b0111_1111_1111_1111,
        }
    }
}

impl ChannelFour {
    fn dac_enabled(&self) -> bool {
        (self.nr42_volume_envelope.raw() & 0b1111_1000) != 0
    }

    fn width_mode(&self) -> u8 {
        (self.nr43_freq_and_randomness.raw() & 0b0000_1000) >> 3
    }

    fn clock_shift(&self) -> u8 {
        (self.nr43_freq_and_randomness.raw() & 0b1111_0000) >> 4
    }

    fn divisor(&self) -> u32 {
        let clock_divider = self.nr43_freq_and_randomness.raw() & 0b0000_0111;

        if clock_divider == 0 {
            8 // divider = 0 is treated as divider = 0.5 instead, 16 * 0.5 = 8
        } else {
            16 * clock_divider as u32 // T-cycles / frequency = 16
        }
    }

    fn reloading_period(&self) -> u32 {
        self.divisor() << self.clock_shift()
    }

    pub fn tick_length(&mut self) {
        if self.nr44_control.raw() & 0b0100_0000 != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn output(&self) -> f32 {
        if !self.enabled || !self.dac_enabled() {
            return 0.0;
        }

        let amplitude = self.volume as f32 / 15.0;
        if self.lfsr & 0b0000_0000_0000_0001 == 0 {
            amplitude
        } else {
            -amplitude
        }
    }

    pub fn tick_envelope(&mut self) {
        let period = self.nr42_volume_envelope.raw() & 0b0000_0111;

        if period == 0 {
            return;
        }

        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }

        if self.envelope_timer == 0 {
            self.envelope_timer = period;

            if (self.nr42_volume_envelope.raw() & 0b0000_1000) != 0 && self.volume < 15 {
                self.volume += 1;
            } else if (self.nr42_volume_envelope.raw() & 0b0000_1000) == 0 && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    pub fn trigger(&mut self) {
        self.enabled = true;
        self.freq_timer = self.reloading_period();

        let length_load = self.nr41_length_timer.raw() & 0b0011_1111;
        self.length_counter = 64 - length_load;
        if self.length_counter == 0 {
            self.length_counter = 64;
        }

        self.volume = (self.nr42_volume_envelope.raw() & 0b1111_0000) >> 4;
        self.envelope_timer = self.nr42_volume_envelope.raw() & 0b0000_0111;

        self.lfsr = 0b0111_1111_1111_1111;

        if !self.dac_enabled() {
            self.enabled = false;
        }
    }

    pub fn step(&mut self) {
        if self.freq_timer > 0 {
            self.freq_timer -= 1;
        }
        if self.freq_timer == 0 && self.clock_shift() < 14 {
            self.freq_timer = self.reloading_period();
            let new_bit = (self.lfsr ^ (self.lfsr >> 1)) & 1;
            self.lfsr >>= 1;
            self.lfsr &= 0b0011_1111_1111_1111;
            self.lfsr |= new_bit << 14;
            if self.width_mode() != 0 {
                self.lfsr &= 0b0111_1111_1011_1111;
                self.lfsr |= new_bit << 6;
            }
        }
    }
}
