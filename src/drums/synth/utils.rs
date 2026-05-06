use rand::prelude::*;
use rand_pcg::Pcg64;

pub struct SynthUtils;

impl SynthUtils {
    /// 指数関数的なエンベロープ
    pub fn exp_env(t: f32, decay_ms: f32) -> f32 {
        let decay_s = decay_ms / 1000.0;
        (-(t / decay_s.max(0.001))).exp()
    }

    /// 高品質で安定したノイズ
    pub fn stable_noise(pos: usize) -> f32 {
        let mut rng = Pcg64::seed_from_u64(pos as u64);
        rng.random_range(-1.0..1.0)
    }

    /// 簡易ハイパスフィルタ（アタックのキレを出す用）
    pub fn hp_filter(current: f32, previous: f32, coefficient: f32) -> f32 {
        current - previous * coefficient
    }
}
