use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SampleFormat, SizedSample, SupportedStreamConfig};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::mmu::apu::sample_buffer;

fn choose_config(device: &Device) -> SupportedStreamConfig {
    let supported_configs: Vec<_> = device
        .supported_output_configs()
        .expect("error while querying configs")
        .collect();

    let preference = [
        SampleFormat::F32,
        SampleFormat::I32,
        SampleFormat::U32,
        SampleFormat::I16,
        SampleFormat::U16,
        SampleFormat::I8,
        SampleFormat::U8,
    ];

    for fmt in preference {
        if let Some(cfg) = supported_configs.iter().find(|c| c.sample_format() == fmt) {
            return cfg.clone().with_sample_rate(48000);
        }
    }

    supported_configs.into_iter()
        .next()
        .expect("no supported config")
        .with_sample_rate(48000)
}

// start audio on a thread, and play the audio stream
pub fn start_audio(buffer: sample_buffer::SampleBuffer, audio_running: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");

        let sound_config = choose_config(&device);

        let sample_format = sound_config.sample_format();
        let config: cpal::StreamConfig = sound_config.into();

        let stream = match sample_format {
            SampleFormat::U8 => build_stream::<u8>(&device, &config, buffer.clone()),
            SampleFormat::I8 => build_stream::<i8>(&device, &config, buffer.clone()),
            SampleFormat::U16 => build_stream::<u16>(&device, &config, buffer.clone()),
            SampleFormat::I16 => build_stream::<i16>(&device, &config, buffer.clone()),
            SampleFormat::U32 => build_stream::<u32>(&device, &config, buffer.clone()),
            SampleFormat::I32 => build_stream::<i32>(&device, &config, buffer.clone()),
            SampleFormat::F32 => build_stream::<f32>(&device, &config, buffer.clone()),
            _ => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        stream.play().unwrap();

        while audio_running.load(Ordering::Relaxed) {
            std::thread::park_timeout(Duration::from_millis(100));
        }
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
    let channels = config.channels as usize;

    device.build_output_stream(
        config,
        move |data: &mut [T], _| {
            for frame in data.chunks_mut(channels) {
                let value = T::from_sample(buffer.pop().unwrap_or(0.0));
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
        None,
    )
}
