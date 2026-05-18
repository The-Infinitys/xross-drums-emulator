use crate::params::transient::TransientParams;
use truce::params::FloatParamReadF32;

pub struct TransientShaper {
    fast_env: f32,
    slow_env: f32,
}

impl TransientShaper {
    pub fn new() -> Self {
        Self {
            fast_env: 0.0,
            slow_env: 0.0,
        }
    }

    pub fn process(&mut self, input: f32, params: &TransientParams, sr: f32) -> f32 {
        let abs_input = input.abs();
        let att_time = params.attack_time.value();
        let sus_time = params.sustain_time.value();
        let sensitivity = params.sensitivity.value() / 100.0;

        let att_coeff = (-1.0 / (att_time * sr / 1000.0)).exp();
        let sus_coeff = (-1.0 / (sus_time * sr / 1000.0)).exp();

        self.fast_env = att_coeff * self.fast_env + (1.0 - att_coeff) * abs_input;
        self.slow_env = sus_coeff * self.slow_env + (1.0 - sus_coeff) * abs_input;

        let attack_gain = 10.0f32.powf(params.attack_gain.value() / 20.0 * sensitivity);
        let sustain_gain = 10.0f32.powf(params.sustain_gain.value() / 20.0 * sensitivity);

        let mut gain = 1.0;
        if self.slow_env > 1e-6 {
            let ratio = self.fast_env / (self.slow_env + 1e-6);
            if ratio > 1.0 {
                gain *= 1.0 + (attack_gain - 1.0) * (ratio - 1.0).min(1.0);
            } else {
                gain *= 1.0 + (sustain_gain - 1.0) * (1.0 - ratio).min(1.0);
            }
        }

        input * gain
    }
}
