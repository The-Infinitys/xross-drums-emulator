use crate::params::comp::CompressorParams;

pub struct Compressor {
    envelope: f32,
}

impl Compressor {
    pub fn new() -> Self {
        Self { envelope: 0.0 }
    }

    pub fn process(&mut self, input: f32, params: &CompressorParams, sr: f32) -> f32 {
        let abs_input = input.abs();
        let threshold = 10.0f32.powf(params.threshold.value() / 20.0);
        let ratio = params.ratio.value();
        let attack = (-1.0 / (params.attack.value() * sr / 1000.0)).exp();
        let release = (-1.0 / (params.release.value() * sr / 1000.0)).exp();

        // Envelope follower
        if abs_input > self.envelope {
            self.envelope = attack * self.envelope + (1.0 - attack) * abs_input;
        } else {
            self.envelope = release * self.envelope + (1.0 - release) * abs_input;
        }

        if self.envelope <= threshold || ratio <= 1.0 {
            return input * 10.0f32.powf(params.makeup.value() / 20.0);
        }

        // Gain reduction
        let knee = params.knee.value();
        let mut gr_db = 0.0;
        let env_db = 20.0 * self.envelope.log10().max(-100.0);
        let thresh_db = params.threshold.value();

        if knee > 0.0 && env_db > thresh_db - knee / 2.0 && env_db < thresh_db + knee / 2.0 {
            // Soft knee
            let x = env_db - thresh_db + knee / 2.0;
            gr_db = (1.0 / ratio - 1.0) * x * x / (2.0 * knee);
        } else if env_db > thresh_db {
            // Hard knee
            gr_db = (thresh_db - env_db) * (1.0 - 1.0 / ratio);
        }

        let gain = 10.0f32.powf((gr_db + params.makeup.value()) / 20.0);
        input * gain
    }
}
