use crate::params::clipper::ClipperParams;

pub struct Clipper {}

impl Clipper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(&mut self, input: f32, params: &ClipperParams, _sr: f32) -> f32 {
        let in_gain = 10.0f32.powf(params.input_gain.value() / 20.0);
        let threshold = 10.0f32.powf(params.threshold.value() / 20.0);
        let softness = params.softness.value() / 100.0;

        let x = input * in_gain;
        if softness <= 0.0 {
            x.clamp(-threshold, threshold)
        } else {
            let margin = threshold * softness;
            let inner = threshold - margin;
            if x.abs() < inner {
                x
            } else {
                let sign = x.signum();
                let excess = x.abs() - inner;
                sign * (inner + margin * (excess / margin).tanh())
            }
        }
    }
}
