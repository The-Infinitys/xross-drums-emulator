use std::f32::consts::PI;

#[derive(Default, Clone)]
pub struct Biquad {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
    pub z1: f32,
    pub z2: f32,
}

impl Biquad {
    pub fn process(&mut self, input: f32) -> f32 {
        let output = input * self.b0 + self.z1;
        self.z1 = input * self.b1 - output * self.a1 + self.z2;
        self.z2 = input * self.b2 - output * self.a2;
        output
    }

    pub fn set_hpf(&mut self, freq: f32, q: f32, sr: f32) {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);
        let a0 = 1.0 + alpha;
        self.b0 = (1.0 + cos_w0) / 2.0 / a0;
        self.b1 = -(1.0 + cos_w0) / a0;
        self.b2 = (1.0 + cos_w0) / 2.0 / a0;
        self.a1 = -2.0 * cos_w0 / a0;
        self.a2 = (1.0 - alpha) / a0;
    }

    pub fn set_lpf(&mut self, freq: f32, q: f32, sr: f32) {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);
        let a0 = 1.0 + alpha;
        self.b0 = (1.0 - cos_w0) / 2.0 / a0;
        self.b1 = (1.0 - cos_w0) / a0;
        self.b2 = (1.0 - cos_w0) / 2.0 / a0;
        self.a1 = -2.0 * cos_w0 / a0;
        self.a2 = (1.0 - alpha) / a0;
    }

    pub fn set_peaking(&mut self, freq: f32, q: f32, gain_db: f32, sr: f32) {
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq / sr;
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();
        let a0 = 1.0 + alpha / a;
        self.b0 = (1.0 + alpha * a) / a0;
        self.b1 = -2.0 * cos_w0 / a0;
        self.b2 = (1.0 - alpha * a) / a0;
        self.a1 = -2.0 * cos_w0 / a0;
        self.a2 = (1.0 - alpha / a) / a0;
    }

    pub fn set_low_shelf(&mut self, freq: f32, q: f32, gain_db: f32, sr: f32) {
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq / sr;
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();
        let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;
        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + two_sqrt_a_alpha;
        self.b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + two_sqrt_a_alpha) / a0;
        self.b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0) / a0;
        self.b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - two_sqrt_a_alpha) / a0;
        self.a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0) / a0;
        self.a2 = ((a + 1.0) + (a - 1.0) * cos_w0 - two_sqrt_a_alpha) / a0;
    }

    pub fn set_high_shelf(&mut self, freq: f32, q: f32, gain_db: f32, sr: f32) {
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq / sr;
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();
        let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;
        let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + two_sqrt_a_alpha;
        self.b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + two_sqrt_a_alpha) / a0;
        self.b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0) / a0;
        self.b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - two_sqrt_a_alpha) / a0;
        self.a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0) / a0;
        self.a2 = ((a + 1.0) - (a - 1.0) * cos_w0 - two_sqrt_a_alpha) / a0;
    }
}
