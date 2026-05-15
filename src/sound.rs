
use std::sync::Mutex;
use cpal::{Sample, SampleFormat, FromSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc};

pub fn sound_test() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device");
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let first_supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let err_fn = |err| eprintln!("an error occured on the output audio stream: {}", err);
    let sample_format = first_supported_config.sample_format();
    let config: cpal::StreamConfig = first_supported_config.into();

    let frequency = 261.63_f32; // Do central (C4)
    let sample_rate = config.sample_rate as f32;
    let phase = Arc::new(Mutex::new(0f32));

    let stream = match sample_format {
        SampleFormat::F32 => {
            let phase = Arc::clone(&phase);
            device.build_output_stream(&config, move |data: &mut [f32], _| {
                write_sine::<f32>(data, frequency, sample_rate, &phase);
            }, err_fn, None)
        },
        SampleFormat::I16 => {
            let phase = Arc::clone(&phase);
            device.build_output_stream(&config, move |data: &mut [i16], _| {
                write_sine::<i16>(data, frequency, sample_rate, &phase);
            }, err_fn, None)
        },
        SampleFormat::U16 => {
            let phase = Arc::clone(&phase);
            device.build_output_stream(&config, move |data: &mut [u16], _| {
                write_sine::<u16>(data, frequency, sample_rate, &phase);
            }, err_fn, None)
        },
        SampleFormat::U8 => {
            let phase = Arc::clone(&phase);
            device.build_output_stream(&config, move |data: &mut [u8], _| {
                write_sine::<u8>(data, frequency, sample_rate, &phase);
            }, err_fn, None)
        },
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    }.unwrap();

    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
}

fn write_sine<T: Sample + FromSample<f32>>(
    data: &mut [T],
    frequency: f32,
    sample_rate: f32,
    phase: &Arc<Mutex<f32>>,
) {
    let mut phase_lock = phase.lock().unwrap();
    let phase_increment = 2.0 * std::f32::consts::PI * frequency / sample_rate;

    for sample in data.iter_mut() {
        let value = phase_lock.sin() * 0.5;
        *sample = T::from_sample(value);
        *phase_lock += phase_increment;

        if *phase_lock > 2.0 * std::f32::consts::PI {
            *phase_lock -= 2.0 * std::f32::consts::PI;
        }
    }
}
