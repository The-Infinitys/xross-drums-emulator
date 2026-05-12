use crate::params::fx::FxParams;

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

    pub fn process(&mut self, input: f32, params: &FxParams, _sr: f32) -> f32 {
        let mix = params.reverb_mix.value() / 100.0;
        if mix <= 0.0 {
            return input;
        }

        let decay = 0.5 + params.reverb_decay.value() / 200.0;

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
