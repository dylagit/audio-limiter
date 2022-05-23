#[derive(Copy, Clone, Debug)]
pub struct Compressor {
  pub peak_at: f32,
  pub peak_rt: f32,
  pub peak_average: f32,

  pub gain_at: f32,
  pub gain_rt: f32,
  pub gain_average: f32,

  pub threshold: f32
}

fn calc_tau(sample_rate: f32, time_ms: f32) -> f32 {
  1.0 - (-2200.0 / (time_ms * sample_rate)).exp()
}

fn limiter(input: f32, threshold: f32) -> f32 {
  let db = 20.0 * input.abs().log10();
  let gain = (threshold - db).min(0.0);
  10.0f32.powf(0.05 * gain)
}

fn ar_avg(avg: f32, at: f32, rt: f32, input: f32) -> f32 {
  let tau = if input > avg { at } else { rt };

  (1.0 - tau) * avg + tau * input
}

impl Compressor {
  pub fn compress(&mut self, input: f32) -> f32 {
    self.peak_average = ar_avg(self.peak_average, self.peak_at, self.peak_rt, input.abs());

    let gain = limiter(self.peak_average, self.threshold);

    self.gain_average = ar_avg(self.gain_average, self.gain_rt, self.gain_at, gain);

    self.gain_average * input
  }

  pub fn new(sample_rate: f32, threshold: f32, attack_ms: f32, release_ms: f32) -> Self {
    Self {
      peak_at: calc_tau(sample_rate, 0.01),
      peak_rt: calc_tau(sample_rate, 10.0),
      peak_average: 0.0,
      gain_at: calc_tau(sample_rate, attack_ms),
      gain_rt: calc_tau(sample_rate, release_ms),
      gain_average: 1.0,
      threshold,
    }
  }
}
