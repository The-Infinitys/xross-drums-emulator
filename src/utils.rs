use std::f32::consts::PI;
use truce::params::FloatParam;

pub trait FloatParamNormalizedExt {
    fn value_normalized(&self) -> f64;
    fn set_value_normalized(&self, norm: f64);
}

impl FloatParamNormalizedExt for FloatParam {
    fn value_normalized(&self) -> f64 {
        let val = self.value() as f64;
        let range = &self.info.range;
        range.normalize(val)
    }

    fn set_value_normalized(&self, norm: f64) {
        let range = &self.info.range;
        let val = range.denormalize(norm);
        self.set_value(val);
    }
}

pub fn freq_to_norm(freq: f32) -> f32 {
    let min_log = 20.0f32.ln();
    let max_log = 20000.0f32.ln();
    ((freq.clamp(20.0, 20000.0)).ln() - min_log) / (max_log - min_log)
}

pub fn norm_to_freq(norm: f32) -> f32 {
    let min_log = 20.0f32.ln();
    let max_log = 20000.0f32.ln();
    (min_log + norm * (max_log - min_log)).exp()
}

pub enum FilterType {
    LowShelf,
    HighShelf,
    Peaking,
}

pub fn get_filter_gain(
    f: f32,
    f0: f32,
    gain_db: f32,
    q: f32,
    filter_type: FilterType,
    sample_rate: f32,
) -> f32 {
    if gain_db.abs() < 0.01 {
        return 0.0;
    }

    let a = 10.0f32.powf(gain_db / 40.0);
    let w0 = 2.0 * PI * f0 / sample_rate;
    let cos_w0 = w0.cos();
    let sin_w0 = w0.sin();
    let alpha = sin_w0 / (2.0 * q.max(0.01));

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

    let w = 2.0 * PI * f / sample_rate;
    let (sw, cw) = w.sin_cos();
    let (sw2, cw2) = (2.0 * w).sin_cos();

    let n_re = b0 + b1 * cw + b2 * cw2;
    let n_im = -(b1 * sw + b2 * sw2);
    let d_re = a0 + a1 * cw + a2 * cw2;
    let d_im = -(a1 * sw + a2 * sw2);

    let n_mag_sq = n_re * n_re + n_im * n_im;
    let d_mag_sq = d_re * d_re + d_im * d_im;

    10.0 * (n_mag_sq / d_mag_sq.max(1e-10)).log10()
}
