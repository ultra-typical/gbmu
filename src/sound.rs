use std::sync::Mutex;
use cpal::{SizedSample, SampleFormat, FromSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc};

use crate::mmu::apu::sample_buffer;

// start audio on a thread, and play the audio stream
pub fn start_audio(buffer: sample_buffer::SampleBuffer) {
    std::thread::spawn(move || {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");
        let first_supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let sample_format = first_supported_config.sample_format();
        let config: cpal::StreamConfig = first_supported_config.into();

        let stream = match sample_format {
            SampleFormat::U8 => { build_stream::<u8>(&device, &config, buffer.clone()) },
            SampleFormat::I8 => { build_stream::<i8>(&device, &config, buffer.clone()) },
            SampleFormat::U16 => { build_stream::<u16>(&device, &config, buffer.clone()) },
            SampleFormat::I16 => { build_stream::<i16>(&device, &config, buffer.clone()) },
            SampleFormat::U32 => { build_stream::<u32>(&device, &config, buffer.clone()) },
            SampleFormat::I32 => { build_stream::<i32>(&device, &config, buffer.clone()) },
            SampleFormat::F32 => { build_stream::<f32>(&device, &config, buffer.clone()) },
            _ => panic!("Unsupported sample format '{sample_format}'")
        }.unwrap();

        stream.play().unwrap();
        std::thread::park(); // Park the thread to keep it alive indefinitely
        });
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    buffer: sample_buffer::SampleBuffer,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: SizedSample + FromSample<f32>,
{
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    device.build_output_stream(
        config,
        move |data: &mut [T], _| {
            for sample in data.iter_mut() {
                *sample = T::from_sample(buffer.pop().unwrap_or(0.0));
            }
        },
        err_fn,
        None,
    )
}

fn write_sine<T: SizedSample + FromSample<f32>>(
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
