use super::registers::*;

#[derive(Default)]
pub struct ChannelThree {
    pub nr30_dac_enable: WaveDacEnableReg,
    pub nr31_ln_timer: WaveLengthTimerReg,
    pub nr32_output_level: OutputLevelReg,
    pub nr33_period_low: PeriodLowReg,
    pub nr34_period_high_crtl: PeriodHighCtrlReg,

    pub wave_ram: [u8; 16],

    freq_timer: u16,
    sample_position: u8,
    current_sample: u8,
    enabled: bool,
    length_counter: u16,
}

impl ChannelThree {
    fn period(&self) -> u16 {
        ((self.nr34_period_high_crtl.raw() as u16 & 0b0000_0111) << 8)
            | self.nr33_period_low.raw() as u16
    }

    pub fn trigger(&mut self) {
        self.enabled = true;

        self.length_counter = 256 - self.nr31_ln_timer.raw() as u16;

        self.freq_timer = (2048 - self.period()) * 2;
        self.sample_position = 0;

        if !self.dac_enabled() {
            self.enabled = false;
        }
    }    

    pub fn step(&mut self) {
        if self.freq_timer > 0 {
            self.freq_timer -= 1;
        }

        if self.freq_timer == 0 {
            self.freq_timer = (2048 - self.period()) * 2;
            self.sample_position = (self.sample_position + 1) % 32;

            let byte = self.wave_ram[(self.sample_position / 2) as usize];
            self.current_sample = if self.sample_position.is_multiple_of(2) {
                byte >> 4
            } else {
                byte & 0x0F
            };
        }
    }

    fn output_level(&self) -> u8 {
        (self.nr32_output_level.raw() >> 5) & 0b11
    }

    fn dac_enabled(&self) -> bool {
        self.nr30_dac_enable.raw() & 0b1000_0000 != 0
    }

    pub fn output(&self) -> f32 {
        if !self.enabled || !self.dac_enabled() {
            return 0.0;
        }

        let shift = match self.output_level() {
            0b00 => 4, // mute
            0b01 => 0, // 100%
            0b10 => 1, // 50%
            0b11 => 2, // 25%
            _ => 4,
        };

        let sample = self.current_sample >> shift;

        (sample as f32 / 7.5) - 1.0
    }

    pub fn tick_length(&mut self) {
        if self.nr34_period_high_crtl.raw() & 0b0100_0000 != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }
}