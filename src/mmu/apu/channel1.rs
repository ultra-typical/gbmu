use super::registers::*;

#[derive(Default)]
pub struct ChannelOne {
    pub nr10_sweep: SweepReg,
    pub nr11_ln_timer_duty_cycle: LnTimerDutyCycleReg,
    pub nr12_volume_envelope: VolumeEnvReg,
    pub nr13_period_low: PeriodLowReg,
    pub nr14_period_high_ctrl: PeriodHighCtrlReg,
}