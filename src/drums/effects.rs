use crate::params::equalizer::EqualizerParams;
use std::f32::consts::PI;

pub struct Biquad {
    z1: f32,
    z2: f32,
}

impl Biquad {
    pub fn new() -> Self {
        Self { z1: 0.0, z2: 0.0 }
    }

    pub fn process(&mut self, input: f32, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> f32 {
        let output = input * b0 + self.z1;
        self.z1 = input * b1 - output * a1 + self.z2;
        self.z2 = input * b2 - output * a2;
        output
    }
}

pub struct EffectChain {
    pub low: Biquad,
    pub mid: Biquad,
    pub high: Biquad,
    // Transient
    pub last_envelope: f32,
    // Comp
    pub comp_envelope: f32,
}

impl EffectChain {
    pub fn new() -> Self {
        Self {
            low: Biquad::new(),
            mid: Biquad::new(),
            high: Biquad::new(),
            last_envelope: 0.0,
            comp_envelope: 0.0,
        }
    }

    pub fn process(
        &mut self,
        input: f32,
        params: &crate::params::PartParams,
        sample_rate: f32,
    ) -> f32 {
        let mut x = input;

        // 1. Transient Shaper (Simplified)
        let ts = &params.transient;
        let att_time = ts.attack_time.value() / 1000.0;
        let sus_time = ts.sustain_time.value() / 1000.0;
        let alpha_att = (-(1.0 / (sample_rate * att_time))).exp();
        let alpha_sus = (-(1.0 / (sample_rate * sus_time))).exp();

        let env = input.abs();
        let fast_env = env.max(self.last_envelope * alpha_att);
        let slow_env = env.max(self.last_envelope * alpha_sus);
        self.last_envelope = fast_env;

        let diff = fast_env - slow_env;
        let ts_gain = 10.0f32
            .powf((diff * ts.attack_gain.value() + slow_env * ts.sustain_gain.value()) / 20.0);
        x *= ts_gain;

        // 2. Saturation
        let sat = &params.saturation;
        let drive = 10.0f32.powf(sat.drive.value() / 20.0);
        let sat_in = x * drive;
        let sat_out = sat_in.tanh(); // Simple soft clipping
        let mix = sat.mix.value() / 100.0;
        x = sat_out * mix + x * (1.0 - mix);
        x *= 10.0f32.powf(sat.output_gain.value() / 20.0);

        // 3. Compressor (Simplified)
        let comp = &params.comp;
        let threshold = 10.0f32.powf(comp.threshold.value() / 20.0);
        let ratio = comp.ratio.value();
        let env_comp = x.abs();
        let alpha_comp = (-(1.0 / (sample_rate * 0.01))).exp(); // Fixed 10ms for simplicity
        self.comp_envelope = env_comp.max(self.comp_envelope * alpha_comp);

        if self.comp_envelope > threshold {
            let over_db = 20.0 * (self.comp_envelope / threshold).log10();
            let reduction_db = over_db * (1.0 - 1.0 / ratio);
            let gain_reduction = 10.0f32.powf(-reduction_db / 20.0);
            x *= gain_reduction;
        }
        x *= 10.0f32.powf(comp.makeup.value() / 20.0);

        // 4. EQ
        x = self.apply_eq(x, &params.eq, sample_rate);

        x
    }

    fn apply_eq(&mut self, input: f32, eq: &EqualizerParams, sample_rate: f32) -> f32 {
        let mut x = input;

        // Low Shelf
        let (b0, b1, b2, a1, a2) = get_coeffs(
            eq.low.freq.value(),
            eq.low.gain.value(),
            eq.low.q.value(),
            FilterType::LowShelf,
            sample_rate,
        );
        x = self.low.process(x, b0, b1, b2, a1, a2);

        // Mid Peaking
        let (b0, b1, b2, a1, a2) = get_coeffs(
            eq.mid.freq.value(),
            eq.mid.gain.value(),
            eq.mid.q.value(),
            FilterType::Peaking,
            sample_rate,
        );
        x = self.mid.process(x, b0, b1, b2, a1, a2);

        // High Shelf
        let (b0, b1, b2, a1, a2) = get_coeffs(
            eq.high.freq.value(),
            eq.high.gain.value(),
            eq.high.q.value(),
            FilterType::HighShelf,
            sample_rate,
        );
        x = self.high.process(x, b0, b1, b2, a1, a2);

        x
    }
}

enum FilterType {
    LowShelf,
    HighShelf,
    Peaking,
}

fn get_coeffs(
    freq: f32,
    gain_db: f32,
    q: f32,
    filter_type: FilterType,
    sample_rate: f32,
) -> (f32, f32, f32, f32, f32) {
    let q = q.max(0.01);
    let a = 10.0f32.powf(gain_db / 40.0);
    let w0 = 2.0 * PI * freq / sample_rate;
    let cos_w0 = w0.cos();
    let sin_w0 = w0.sin();
    let alpha = sin_w0 / (2.0 * q);

    let (b0, b1, b2, a0, a1, a2) = match filter_type {
        FilterType::LowShelf => {
            let a_plus = a + 1.0;
            let a_minus = a - 1.0;
            let s = 2.0 * a.sqrt() * alpha;
            (
                a * ((a_plus - a_minus * cos_w0) + s),
                2.0 * a * (a_minus - a_plus * cos_w0),
                a * ((a_plus - a_minus * cos_w0) - s),
                a_plus + a_minus * cos_w0 + s,
                -2.0 * (a_minus + a_plus * cos_w0),
                a_plus + a_minus * cos_w0 - s,
            )
        }
        FilterType::Peaking => (
            1.0 + alpha * a,
            -2.0 * cos_w0,
            1.0 - alpha * a,
            1.0 + alpha / a,
            -2.0 * cos_w0,
            1.0 - alpha / a,
        ),
        FilterType::HighShelf => {
            let a_plus = a + 1.0;
            let a_minus = a - 1.0;
            let s = 2.0 * a.sqrt() * alpha;
            (
                a * ((a_plus + a_minus * cos_w0) + s),
                -2.0 * a * (a_minus + a_plus * cos_w0),
                a * ((a_plus + a_minus * cos_w0) - s),
                a_plus - a_minus * cos_w0 + s,
                2.0 * (a_minus - a_plus * cos_w0),
                a_plus - a_minus * cos_w0 - s,
            )
        }
    };

    (b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
}
