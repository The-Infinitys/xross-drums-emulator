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
    let freq = params.freq.value();

    match name {
        "sidestick" => {
            // サイドスティック: 鋭く「カキーン」と響く設定
            let env = SynthUtils::exp_env(t, 20.0); // わずかに長くして余韻を出す

            // 1. FM合成による金属的な芯 (Modulator -> Carrier)
            let mod_freq = freq * 14.5;
            let mod_index = 2.5 * SynthUtils::exp_env(t, 5.0); // アタック時だけ強く変調
            let modulation = (t * 2.0 * PI * mod_freq).sin() * mod_index;

            let carrier_freq = freq * 8.2; // かなり高めの基本ピッチ
            let mut metal = (t * 2.0 * PI * carrier_freq + modulation).sin();

            // 2. 金属の「鳴り」を補強する非整数倍音
            let res_ratios = [1.0, 1.618, 2.32]; // 黄金比などを用いて不自然な共鳴を作る
            for &r in res_ratios.iter() {
                metal += (t * 2.0 * PI * carrier_freq * r).sin() * 0.3;
            }

            // 3. 瞬間的な「コンッ」という衝撃音（クリック）
            let click_env = SynthUtils::exp_env(t, 2.0);
            let click = (SynthUtils::stable_noise(pos).signum()) * click_env * 0.5;

            // 全体をtanhでサチュレート（歪ませて）倍音を飽和させる
            ((metal * 0.6 + click) * 1.8).tanh() * env * vel
        }
        _ => {
            // 通常のスネア & リムショット
            let decay = params.decay.value();
            let body_env = SynthUtils::exp_env(t, decay * 0.4);

            // 1. ボディトーン (2つのサイン波の干渉でスネアの胴鳴りを再現)
            *phase += 2.0 * PI * freq / sr;
            let body = (phase.sin() + (*phase * 1.6).sin() * 0.3) * body_env;

            // 2. スナッピー (White Noise + High Pass Filter)
            let noise_decay = params.noise_decay.value();
            let snappy_env = SynthUtils::exp_env(t, noise_decay);
            let mut snappy = SynthUtils::stable_noise(pos);
            if pos > 0 {
                snappy = SynthUtils::hp_filter(snappy, SynthUtils::stable_noise(pos - 1), 0.7);
            }
            snappy *= snappy_env * params.noise_level.value();

            // 3. リムショットならアタックに鋭いパルスを追加
            let rim_attack = if name == "rimshot" {
                let r_env = SynthUtils::exp_env(t, 5.0);
                (t * 2.0 * PI * freq * 12.0).sin() * r_env * 0.8
            } else {
                0.0
            };

            (body * 0.6 + snappy + rim_attack) * vel
        }
    }
}
