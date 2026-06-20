use super::registers::*;

#[derive(Default)]
pub struct ChannelFour {
    pub nr41_length_timer: NoiseLengthTimer,
    pub nr42_volume_envelope: VolumeEnvReg,
    pub nr43_freq_and_randomness: FreqRandomnessReg,
    pub nr44_control: ChannelFourCtrlReg,
}