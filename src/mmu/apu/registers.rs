pub trait Register {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
    fn raw(&self) -> u8;
}

macro_rules! define_register {
    ($name:ident) => {
        #[derive(Default, Debug, Copy, Clone)]
        pub struct $name { byte: u8, }
    };
}

macro_rules! read_write_register {
    ($name:ident) => {
        define_register!($name);
        impl Register for $name {
            fn read(&self) -> u8 { self.byte }
            fn write(&mut self, value: u8) { self.byte = value}
            fn raw(&self) -> u8 { self.byte }
        }
    };
}

macro_rules! write_only_register {
    ($name:ident) => {
        define_register!($name);
        impl Register for $name {
            fn read(&self) -> u8 { 0xFF }
            fn write(&mut self, value: u8) { self.byte = value}
            fn raw(&self) -> u8 { 0xFF }
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
    fn raw(&self) -> u8 { self.byte }
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
    fn raw(&self) -> u8 { self.byte }
}