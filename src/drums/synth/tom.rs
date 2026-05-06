use super::utils::SynthUtils;
use crate::params::electric::ElectricParams;
use std::f32::consts::PI;

pub fn process(
    name: &str,
    params: &ElectricParams,
    pos: usize,
    vel: f32,
    sr: f32,
    phase: &mut f32,
) -> f32 {
    let t = pos as f32 / sr;

    // タムの種類によってベースピッチをスケーリング
    let base_scale = match name {
        "tom_high" => 2.0,
        "tom_low" => 1.4,
        _ => 0.9, // Floor Tom
    };

    let f_target = params.freq.value() * base_scale;
    let decay = params.decay.value();

    // 振幅とピッチのエンベロープ
    let amp_env = SynthUtils::exp_env(t, decay);
    let p_env = SynthUtils::exp_env(t, decay * 0.2); // ピッチが落ちる速度

    let sweep = params.sweep.value() * 1.5;
    let current_freq = f_target * (1.0 + sweep * p_env);

    *phase += 2.0 * PI * current_freq / sr;

    // メインのサイン波 + わずかにズレた第2倍音 (膜の共鳴感)
    let osc1 = phase.sin();
    let osc2 = (*phase * 1.02).sin() * 0.3; // 微妙なデチューン
    let osc3 = (*phase * 2.0).sin() * 0.1; // オクターブ上の成分

    let out = (osc1 + osc2 + osc3) * amp_env;

    // 打撃の瞬間（アタック）にスティックのコンタクト音を追加
    let click_env = SynthUtils::exp_env(t, 8.0);
    let click = SynthUtils::stable_noise(pos) * click_env * 0.2;

    (out + click) * vel
}
