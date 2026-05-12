use super::utils::Biquad;
use crate::params::saturation::{SaturationParams, SaturationType};

pub struct Saturation {
    hpf: Biquad,
    lpf: Biquad,
}

impl Saturation {
    pub fn new() -> Self {
        Self {
            hpf: Biquad::default(),
            lpf: Biquad::default(),
        }
    }

    pub fn process(&mut self, input: f32, params: &SaturationParams, sr: f32) -> f32 {
        let drive_gain = 10.0f32.powf(params.drive.value() / 20.0);
        let mut x = input * drive_gain;

        // Pre-filters
        self.hpf.set_hpf(params.input_high_pass.value(), 0.707, sr);
        self.lpf.set_lpf(params.input_low_pass.value(), 0.707, sr);
        x = self.hpf.process(x);
        x = self.lpf.process(x);

        let saturated = match params.sat_type.value() {
            SaturationType::Soft => x.tanh(),
            SaturationType::Hard => x.clamp(-1.0, 1.0),
            SaturationType::Tape => if x > 0.0 {
                x - 0.25 * x * x
            } else {
                x + 0.25 * x * x
            }
            .clamp(-1.0, 1.0),
            SaturationType::Tube => (x + 0.2 * x * x).tanh(),
        };

        let mix = params.mix.value() / 100.0;
        let out_gain = 10.0f32.powf(params.output_gain.value() / 20.0);
        (input * (1.0 - mix) + saturated * mix) * out_gain
    }
}
