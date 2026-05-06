use super::utils::SynthUtils;
use crate::params::electric::ElectricParams;
use std::f32::consts::PI;

pub fn process(params: &ElectricParams, pos: usize, vel: f32, sr: f32, phase: &mut f32) -> f32 {
    let t = pos as f32 / sr;

    // 振幅エンベロープ
    let amp_env = SynthUtils::exp_env(t, params.decay.value() as f32);

    // ピッチエンベロープ: モダンなキックは最初の一瞬だけ超高域から落ちる
    let f_base = params.freq.value() as f32;
    let sweep_amount = params.sweep.value() as f32 * 8.0;
    let p_env = (-(t / 0.025)).exp(); // 非常に速いピッチ降下
    let freq = f_base * (1.0 + sweep_amount * p_env);

    *phase += 2.0 * PI * freq / sr;
    if *phase > 2.0 * PI {
        *phase -= 2.0 * PI;
    }

    // 正弦波にわずかな倍音（非対称な歪み）を加えて太さを出す
    let osc = (phase.sin() + 0.1 * (*phase * 2.0).sin()).tanh();

    // アタックノイズ
    let noise_env = (-(t / 0.004)).exp();
    let noise = SynthUtils::stable_noise(pos) * noise_env * params.noise_level.value() as f32;

    (osc * amp_env + noise) * vel
}
