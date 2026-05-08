use super::utils::SynthUtils;
use crate::params::electric::ElectricParams;
use std::f32::consts::PI;

pub fn process(params: &ElectricParams, pos: usize, vel: f32, sr: f32, phase: &mut f32) -> f32 {
    let t = pos as f32 / sr;

    // 振幅エンベロープ
    let amp_env = SynthUtils::exp_env(t, params.decay.value());

    // ピッチエンベロープ: モダンなキックは最初の一瞬だけ超高域から落ちる
    let f_base = params.freq.value();
    let sweep_amount = params.sweep.value() * 8.0;
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
    let noise = SynthUtils::stable_noise(pos) * noise_env * params.noise_level.value();

    (osc * amp_env + noise) * vel
}

pub fn process_808(params: &ElectricParams, pos: usize, vel: f32, sr: f32, phase: &mut f32) -> f32 {
    let t = pos as f32 / sr;
    let f_base = params.freq.value() * 0.8; // 808は少し低めが気持ちいい

    // 808ピッチスイープ: 非常に速い
    let p_env = (-(t / 0.008)).exp();
    let sweep = params.sweep.value() * 4.0;
    let freq = f_base * (1.0 + sweep * p_env);

    *phase += 2.0 * PI * freq / sr;
    if *phase > 2.0 * PI {
        *phase -= 2.0 * PI;
    }

    let osc = phase.sin();
    let amp_env = SynthUtils::exp_env(t, params.decay.value() * 1.5); // 808は余韻が長い

    // わずかなクリック（トリガーパルスのシミュレーション）
    let click = (-(t / 0.002)).exp() * 0.5 * SynthUtils::stable_noise(pos).signum();

    (osc * amp_env + click * params.noise_level.value()) * vel
}

pub fn process_909(params: &ElectricParams, pos: usize, vel: f32, sr: f32, phase: &mut f32) -> f32 {
    let t = pos as f32 / sr;
    let f_base = params.freq.value() * 1.1;

    // 909ピッチスイープ: 808より少し複雑で「アタック感」がある
    let p_env = (-(t / 0.015)).exp();
    let sweep = params.sweep.value() * 6.0;
    let freq = f_base * (1.0 + sweep * p_env);

    *phase += 2.0 * PI * freq / sr;
    if *phase > 2.0 * PI {
        *phase -= 2.0 * PI;
    }

    // 909は少し歪んでいるのが特徴
    let osc = (phase.sin() * 1.5).tanh();
    let amp_env = SynthUtils::exp_env(t, params.decay.value() * 0.8);

    // アタックの「ドン」という成分（少し長めのノイズ）
    let attack_noise =
        (-(t / 0.01)).exp() * params.noise_level.value() * SynthUtils::stable_noise(pos);

    (osc * amp_env + attack_noise) * vel
}
