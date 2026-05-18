use crate::params::fx::FxParams;
use truce::params::FloatParamReadF32;

pub struct DelayEffect {
    buffer: Vec<f32>,
    write_pos: usize,
}

impl DelayEffect {
    pub fn new(max_delay_samples: usize) -> Self {
        Self {
            buffer: vec![0.0; max_delay_samples],
            write_pos: 0,
        }
    }

    pub fn process(&mut self, input: f32, params: &FxParams, sr: f32) -> f32 {
        let mix = params.delay_mix.value() / 100.0;
        if mix <= 0.0 {
            return input;
        }

        let delay_ms = params.delay_time.value();
        let delay_samples = (delay_ms * sr / 1000.0) as usize;
        let fb = params.delay_fb.value() / 100.0;

        let read_pos = (self.write_pos + self.buffer.len()
            - delay_samples.min(self.buffer.len() - 1))
            % self.buffer.len();
        let output = self.buffer[read_pos];

        self.buffer[self.write_pos] = input + output * fb;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        input * (1.0 - mix) + output * mix
    }
}
