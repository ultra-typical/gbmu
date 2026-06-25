use super::registers::*;

const DUTY_PATTERNS: [u8; 4] = [
    0b0000_0001, // 12.5%
    0b0000_0011, // 25%
    0b0000_1111, // 50%
    0b0011_1111, // 75%
];

#[derive(Default)]
pub struct ChannelSquare {
    pub nr0_sweep: SweepReg,
    pub nr1_ln_timer_duty_cycle: LnTimerDutyCycleReg,
    pub nr2_volume_envelope: VolumeEnvReg,
    pub nr3_period_low: PeriodLowReg,
    pub nr4_period_high_ctrl: PeriodHighCtrlReg,

    freq_timer: u16,
    duty_step: u8,
    enabled: bool,
    length_counter: u8,
    volume: u8,
    envelope_timer: u8,
    shadow_frequency: u16,
    sweep_timer: u8,
    sweep_enabled: bool,
}

impl ChannelSquare {
    fn dac_enabled(&self) -> bool {
        (self.nr2_volume_envelope.raw() & 0b1111_1000) != 0
    }

    fn sweep_shift(&self) -> u8 {
        self.nr0_sweep.raw() & 0b0000_0111
    }

    fn sweep_period(&self) -> u8 {
        (self.nr0_sweep.raw() >> 4) & 0b0000_0111
    }

    fn sweep_negate(&self) -> bool {
        self.nr0_sweep.raw() & 0b0000_1000 != 0
    }

    fn calculate_sweep(&self) -> u16 {
        let delta = self.shadow_frequency >> self.sweep_shift();

        if self.sweep_negate() {
            self.shadow_frequency - delta
        } else {
            self.shadow_frequency + delta
        }
    }

    fn write_frequency(&mut self, freq: u16) {
        self.nr3_period_low.write((freq & 0xFF) as u8);

        let high = ((freq >> 8) & 0b0111) as u8;
        let nr14 = self.nr4_period_high_ctrl.raw() & 0b1111_1000;
        self.nr4_period_high_ctrl.write(nr14 | high);
    }

    fn tick_sweep(&mut self) {
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }

        if self.sweep_timer == 0 {
            self.sweep_timer = if self.sweep_period() != 0 {
                self.sweep_period()
            } else {
                8
            };

            if self.sweep_enabled && self.sweep_period() != 0 {
                let new_frequency = self.calculate_sweep();

                if new_frequency > 2047 {
                    self.enabled = false;
                } else if self.sweep_shift() != 0 {
                    self.shadow_frequency = new_frequency;

                    self.write_frequency(new_frequency);
                    if self.calculate_sweep() > 2047 { self.enabled = false; }
                }
            }
        }
    }

    pub fn tick_length(&mut self) {
        if self.nr4_period_high_ctrl.raw() & 0b0100_0000 != 0
            && self.length_counter > 0 {
                self.length_counter -= 1;
                if self.length_counter == 0 {
                    self.enabled = false;
                }
            }
    }

    pub fn tick_envelope(&mut self) {
        let period = self.nr2_volume_envelope.raw() & 0b0000_0111;
        
        if period == 0 { return ; }

        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }

        if self.envelope_timer == 0 {
            self.envelope_timer = period;

            if (self.nr2_volume_envelope.raw() & 0b0000_1000) != 0
                && self.volume < 15 {
                self.volume += 1;
            } else if (self.nr2_volume_envelope.raw() & 0b0000_1000) == 0
                && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    pub fn trigger(&mut self) {
        self.enabled = true;
        self.freq_timer = (2048 - self.period()) * 4;

        let length_load = self.nr1_ln_timer_duty_cycle.raw() & 0b0011_1111;
        self.length_counter = 64 - length_load;
        if self.length_counter == 0 {
            self.length_counter = 64;
        }

        self.volume = (self.nr2_volume_envelope.raw() & 0b1111_0000) >> 4;
        self.envelope_timer = self.nr2_volume_envelope.raw() & 0b0000_0111;

        self.shadow_frequency = self.period();
        self.sweep_timer = self.sweep_period();
        if self.sweep_timer == 0 { self.sweep_timer = 8; }
        self.sweep_enabled = self.sweep_period() != 0 || self.sweep_shift() != 0;

        // if shift != 0, immediately calculate then check overflow at trigger (hardware behavior)
        if self.sweep_shift() != 0 {
            let new_freq = self.calculate_sweep();
            if new_freq > 2047 { self.enabled = false; }
        }
    }

    fn period(&self) -> u16 {
        ((self.nr4_period_high_ctrl.raw() as u16 & 0b0000_0111) << 8)
            | self.nr3_period_low.raw() as u16
    }

    pub fn step(&mut self) {
        if self.freq_timer > 0 {
            self.freq_timer -= 1;
        }
        if self.freq_timer == 0 {
            self.freq_timer = (2048 - self.period()) * 4;
            self.duty_step = (self.duty_step + 1) % 8;
        }
    }

    fn duty_output(&self) -> u8 {
        let duty = (self.nr1_ln_timer_duty_cycle.raw() >> 6) & 0b11;
        let pattern = DUTY_PATTERNS[duty as usize];

        (pattern >> self.duty_step) & 1
    }

    pub fn output(&self) -> f32 {
        if !self.enabled || !self.dac_enabled() {
            return 0.0;
        }

        let amplitude = self.volume as f32 / 15.0;
        if self.duty_output() == 1 {
            amplitude
        } else {
            -amplitude
        }
    }
}
