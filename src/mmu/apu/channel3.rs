use super::registers::*;

#[derive(Default)]
pub struct ChannelThree {
    pub nr30_dac_enable: WaveDacEnableReg,
    pub nr31_ln_timer: WaveLengthTimerReg,
    pub nr32_output_level: OutputLevelReg,
    pub nr33_period_low: PeriodLowReg,
    pub nr34_period_high_crtl: PeriodHighCtrlReg,
}