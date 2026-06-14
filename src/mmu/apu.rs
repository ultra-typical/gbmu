#![allow(unused_variables, dead_code)]


#[derive(Default)]
struct ChannelOne {
    nr10_sweep: SweepReg,
    nr11_ln_timer_duty_cycle: LnTimerDutyCycleReg,
    nr12_volume_envelope: VolumeEnvReg,
    nr13_period_low: PeriodLowReg,
    nr14_period_high_ctrl: PeriodHighCtrlReg,
}

#[derive(Default)]
struct ChannelTwo {
    nr21_ln_timer_duty_cycle: LnTimerDutyCycleReg,
    nr22_volume_envelope: VolumeEnvReg,
    nr23_period_low: PeriodLowReg,
    nr24_period_high_ctrl: PeriodHighCtrlReg,
}

#[derive(Default)]
struct ChannelThree {
    nr30_dac_enable: WaveDacEnableReg,
    nr31_ln_timer: WaveLengthTimerReg,
    nr32_output_level: OutputLevelReg,
    nr33_period_low: PeriodLowReg,
    nr34_period_high_crtl: PeriodHighCtrlReg,
}

#[derive(Default)]
struct ChannelFour {
    nr41_length_timer: NoiseLengthTimer,
    nr42_volume_envelope: VolumeEnvReg,
    nr43_freq_and_randomness: FreqRandomnessReg,
    nr44_control: ChannelFourCtrlReg,
}

#[derive(Default)]
pub struct Apu {
    nr50_master_vol_and_vin_panning: MasterVolVinPanningReg,
    nr51_sound_panning: SoundPanningReg,
    nr52_audio_master_control: AudioMasterControlReg,

    wave_ram: [u8; 16],

    channel_one: ChannelOne,
    channel_two: ChannelTwo,
    channel_three: ChannelThree,
    channel_four: ChannelFour,
}

trait Channel {}

macro_rules! define_register {
    ($name:ident) => {
        #[derive(Default, Debug, Copy, Clone)]
        struct $name { byte: u8, }
    };
}

macro_rules! read_write_register {
    ($name:ident) => {
        define_register!($name);
        impl Register for $name {
            fn read(&self) -> u8 { self.byte }
            fn write(&mut self, value: u8) { self.byte = value}
        }
    };
}

macro_rules! write_only_register {
    ($name:ident) => {
        define_register!($name);
        impl Register for $name {
            fn read(&self) -> u8 { 0xFF }
            fn write(&mut self, value: u8) { self.byte = value}
        }
    };
}

read_write_register!(AudioMasterControlReg);
read_write_register!(SoundPanningReg);
read_write_register!(MasterVolVinPanningReg);
read_write_register!(SweepReg);

define_register!(LnTimerDutyCycleReg);
impl Register for LnTimerDutyCycleReg {
    fn read(&self) -> u8 { (self.byte & 0b1100_0000) | 0b0011_1111 }
    fn write(&mut self, value: u8) { self.byte = value;}
}

read_write_register!(VolumeEnvReg);
write_only_register!(PeriodHighCtrlReg);
write_only_register!(PeriodLowReg);
read_write_register!(WaveDacEnableReg);
write_only_register!(WaveLengthTimerReg);

read_write_register!(OutputLevelReg);
read_write_register!(NoiseLengthTimer);
read_write_register!(FreqRandomnessReg);
define_register!(ChannelFourCtrlReg);
impl Register for ChannelFourCtrlReg {
    fn read(&self) -> u8 { (self.byte & 0b0110_0000) | 0b1001_1111 }
    fn write(&mut self, value: u8) { self.byte = value }
}

trait Register {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}

impl Apu {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF10 => self.channel_one.nr10_sweep.read(),
            0xFF11 => self.channel_one.nr11_ln_timer_duty_cycle.read(),
            0xFF12 => self.channel_one.nr12_volume_envelope.read(),
            0xFF13 => self.channel_one.nr13_period_low.read(),
            0xFF14 => self.channel_one.nr14_period_high_ctrl.read(),
            0xFF16 => self.channel_two.nr21_ln_timer_duty_cycle.read(),
            0xFF17 => self.channel_two.nr22_volume_envelope.read(),
            0xFF18 => self.channel_two.nr23_period_low.read(),
            0xFF19 => self.channel_two.nr24_period_high_ctrl.read(),
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
            _ => 0xFF,
        }
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF10 => self.channel_one.nr10_sweep.write(value),
            0xFF11 => self.channel_one.nr11_ln_timer_duty_cycle.write(value),
            0xFF12 => self.channel_one.nr12_volume_envelope.write(value),
            0xFF13 => self.channel_one.nr13_period_low.write(value),
            0xFF14 => self.channel_one.nr14_period_high_ctrl.write(value),
            0xFF16 => self.channel_two.nr21_ln_timer_duty_cycle.write(value),
            0xFF17 => self.channel_two.nr22_volume_envelope.write(value),
            0xFF18 => self.channel_two.nr23_period_low.write(value),
            0xFF19 => self.channel_two.nr24_period_high_ctrl.write(value),
            0xFF1A => self.channel_three.nr30_dac_enable.write(value),
            0xFF1B => self.channel_three.nr31_ln_timer.write(value),
            0xFF1C => self.channel_three.nr32_output_level.write(value),
            0xFF1D => self.channel_three.nr33_period_low.write(value),
            0xFF1E => self.channel_three.nr34_period_high_crtl.write(value),
            0xFF20 => self.channel_four.nr41_length_timer.write(value),
            0xFF21 => self.channel_four.nr42_volume_envelope.write(value),
            0xFF22 => self.channel_four.nr43_freq_and_randomness.write(value),
            0xFF23 => self.channel_four.nr44_control.write(value),
            0xFF24 => self.nr50_master_vol_and_vin_panning.write(value),
            0xFF25 => self.nr51_sound_panning.write(value),
            0xFF26 => self.nr52_audio_master_control.write(value),
            _ => {}
        }
    }
}
