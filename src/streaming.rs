use std::sync::atomic::Ordering;
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device, Stream};

use crate::{compressor::Compressor, gui::{CURR_THRESHOLD, DEFAULT_ATTACK, DEFAULT_RELEASE}};

pub fn get_devices() -> Vec<Device> {
  cpal::default_host().devices().expect("Could not find devices").collect()
}

pub fn create_stream(input_device: &Device, output_device: &Device, threshold: f32) -> Option<(Stream, Stream)> {
  let input_config: cpal::StreamConfig = input_device.default_input_config().ok()?.into();
  let output_config: cpal::StreamConfig = output_device.default_output_config().ok()?.into();

  let mut comp = Compressor::new(input_config.sample_rate.0 as f32, threshold, DEFAULT_ATTACK, DEFAULT_RELEASE);
  
  let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

  let latency = 100.0;
  let latency_frames = (latency / 1000.0) * input_config.sample_rate.0 as f32;
  let latency_samples = latency_frames as usize * input_config.channels as usize;

  let ring = ringbuf::RingBuffer::new(latency_samples * 2);
  let (mut producer, mut consumer) = ring.split();

  for _ in 0..latency_samples {
    producer.push(0.0).unwrap(); // The ring buffer has twice as much space as necessary to add latency here, so this should never fail
  }

  let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
    comp.threshold = CURR_THRESHOLD.load(Ordering::SeqCst);

    for &sample in data {
      let compressed = comp.compress(sample);

      if producer.push(compressed).is_err() {
        // eprintln!("Output stream fell behind: try increasing latency");
      }
    }
  };

  let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
    for sample in data {
      *sample = match consumer.pop() {
        Some(s) => s,
        None => 0.0,
      };
    }
  };

  let input_stream = input_device.build_input_stream(&input_config, input_data_fn, err_fn).expect("Error building input stream");
  let output_stream = output_device.build_output_stream(&output_config, output_data_fn, err_fn).expect("Error building output stream");

  input_stream.play().expect("Could not play input stream");
  output_stream.play().expect("Could not play output stream");

  Some((input_stream, output_stream))
}
