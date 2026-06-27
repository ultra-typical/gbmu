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
    }    
}