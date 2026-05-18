use super::utils::SynthUtils;
use crate::drums::PartId;
use crate::params::electric::ElectricParams;
use std::f32::consts::PI;
use truce::params::FloatParamReadF32;

pub fn process_cymbal(
    part_id: PartId,
    params: &ElectricParams,
    pos: usize,
    vel: f32,
    sr: f32,
) -> f32 {
    let t = pos as f32 / sr;

    let (decay_mult, brightness) = match part_id {
        PartId::HiHatClosed => (0.05, 1.2),
        PartId::HiHatOpen => (1.2, 1.0),
        PartId::Crash => (3.5, 0.8),
        PartId::Ride => (5.5, 0.6),
        PartId::RideBell => (4.0, 0.4),
        _ => (1.0, 1.0),
    };

    let env = SynthUtils::exp_env(t, params.noise_decay.value() * decay_mult);
    let f_base = params.freq.value();

    // 1. FM合成による「金属の光沢感」
    let mod_freq = f_base * 1.618; // 黄金比でうねりを作る
    let mod_idx = 1.5 * SynthUtils::exp_env(t, 50.0); // 打撃時に金属が強く振動する様子
    let modulation = (t * 2.0 * PI * mod_freq).sin() * mod_idx;

    // 6つの金属オシレーターを位相をずらしながら合成
    let ratios = [1.0, 1.48, 1.92, 2.41, 2.95, 3.51];
    let mut metallic = 0.0;
    for &r in ratios.iter() {
        let phase = t * 2.0 * PI * f_base * r + modulation;
        metallic += phase.sin();
    }

    // 2. 「ただのノイズ」にしないためのカラーリング
    let noise = SynthUtils::stable_noise(pos);

    // 矩形波的なザラつきとサイン波の芯をブレンド
    let metal_core = (metallic * 2.0).tanh(); // 歪ませて倍音を飽和させる
    let mut signal = metal_core * 0.4 + noise * 0.6;

    // 3. 物理的な「素材感」を出すフィルター処理
    if pos > 1 {
        let diff = signal - SynthUtils::stable_noise(pos - 1);
        signal = signal * 0.5 + diff * brightness;
    }

    // 4. ライドシンバルなら「コツッ」という粒立ちを追加
    if part_id == PartId::Ride || part_id == PartId::RideBell {
        let stick_env = SynthUtils::exp_env(t, 10.0);
        let stick_click = (t * 2.0 * PI * f_base * 4.0).sin() * stick_env;
        signal += stick_click * 0.3;
    }

    signal * env * params.noise_level.value() * 0.4 * vel
}

pub fn process_808(
    _part_id: PartId,
    params: &ElectricParams,
    pos: usize,
    vel: f32,
    sr: f32,
) -> f32 {
    let t = pos as f32 / sr;
    let env = SynthUtils::exp_env(t, params.noise_decay.value() * 0.5);
    let noise = SynthUtils::stable_noise(pos);

    (noise * env * params.noise_level.value()) * vel
}

pub fn process_909(
    _part_id: PartId,
    params: &ElectricParams,
    pos: usize,
    vel: f32,
    sr: f32,
) -> f32 {
    let t = pos as f32 / sr;
    let env = SynthUtils::exp_env(t, params.noise_decay.value());
    let noise = SynthUtils::stable_noise(pos);

    // 909はノイズの帯域が広い
    let osc = (t * 2.0 * PI * params.freq.value()).sin();

    ((noise + osc * 0.2) * env * params.noise_level.value()) * vel
}
