use crate::params::PartParams;

pub trait Effect: Send {
    fn process(&mut self, input: f32, params: &PartParams, sample_rate: f32) -> f32;
}

pub struct EffectChain {
    pub delay: DelayEffect,
    pub reverb: ReverbEffect,
}

impl EffectChain {
    pub fn new() -> Self {
        Self {
            delay: DelayEffect::new(44100),
            reverb: ReverbEffect::new(),
        }
    }

    pub fn process(&mut self, input: f32, params: &PartParams, sample_rate: f32) -> f32 {
        let x = self.delay.process(input, params, sample_rate);
        self.reverb.process(x, params, sample_rate)
    }
}

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
}

impl Effect for DelayEffect {
    fn process(&mut self, input: f32, params: &PartParams, sample_rate: f32) -> f32 {
        let mix = params.fx.delay_mix.value() / 100.0;
        if mix <= 0.0 {
            return input;
        }

        let delay_ms = params.fx.delay_time.value();
        let delay_samples = (delay_ms * sample_rate / 1000.0) as usize;
        let fb = params.fx.delay_fb.value() / 100.0;

        let read_pos = (self.write_pos + self.buffer.len() - delay_samples.min(self.buffer.len()))
            % self.buffer.len();
        let output = self.buffer[read_pos];

        self.buffer[self.write_pos] = input + output * fb;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();

        input * (1.0 - mix) + output * mix
    }
}

pub struct ReverbEffect {
    comb_filters: [Vec<f32>; 4],
    write_pos: [usize; 4],
}

impl ReverbEffect {
    pub fn new() -> Self {
        Self {
            comb_filters: [
                vec![0.0; 1116],
                vec![0.0; 1188],
                vec![0.0; 1277],
                vec![0.0; 1356],
            ],
            write_pos: [0; 4],
        }
    }
}

impl Effect for ReverbEffect {
    fn process(&mut self, input: f32, params: &PartParams, _sample_rate: f32) -> f32 {
        let mix = params.fx.reverb_mix.value() / 100.0;
        if mix <= 0.0 {
            return input;
        }

        let decay = 0.5 + params.fx.reverb_decay.value() / 200.0; // 0.5 - 1.0

        let mut output = 0.0;
        for i in 0..4 {
            let buf = &mut self.comb_filters[i];
            let read_pos = self.write_pos[i];
            let val = buf[read_pos];
            buf[read_pos] = input + val * decay;
            self.write_pos[i] = (read_pos + 1) % buf.len();
            output += val;
        }
        input * (1.0 - mix) + (output * 0.1) * mix
    }
}
