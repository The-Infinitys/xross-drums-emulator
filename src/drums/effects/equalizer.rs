use super::utils::Biquad;
use crate::params::equalizer::EqualizerParams;

pub struct Equalizer {
    hp: Biquad,
    low: Biquad,
    mid: Biquad,
    high: Biquad,
    lp: Biquad,
}

impl Equalizer {
    pub fn new() -> Self {
        Self {
            hp: Biquad::default(),
            low: Biquad::default(),
            mid: Biquad::default(),
            high: Biquad::default(),
            lp: Biquad::default(),
        }
    }

    pub fn process(&mut self, input: f32, params: &EqualizerParams, sr: f32) -> f32 {
        self.hp
            .set_hpf(params.hp.freq.value(), params.hp.q.value(), sr);
        self.low.set_low_shelf(
            params.low.freq.value(),
            params.low.q.value(),
            params.low.gain.value(),
            sr,
        );
        self.mid.set_peaking(
            params.mid.freq.value(),
            params.mid.q.value(),
            params.mid.gain.value(),
            sr,
        );
        self.high.set_high_shelf(
            params.high.freq.value(),
            params.high.q.value(),
            params.high.gain.value(),
            sr,
        );
        self.lp
            .set_lpf(params.lp.freq.value(), params.lp.q.value(), sr);

        let mut x = input;
        x = self.hp.process(x);
        x = self.low.process(x);
        x = self.mid.process(x);
        x = self.high.process(x);
        x = self.lp.process(x);
        x
    }
}
