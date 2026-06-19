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
}

impl ChannelSquare {
    fn dac_enabled(&self) -> bool {
        (self.nr2_volume_envelope.raw() & 0b1111_1000) != 0
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
