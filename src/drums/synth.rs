pub mod kick;
pub mod metallic;
pub mod snare;
pub mod tom;
pub mod utils;

use crate::params::electric::ElectricParams;
pub struct DrumSynth;

impl DrumSynth {
    pub fn process(
        sample_name: &str,
        params: &ElectricParams,
        sample_pos: usize,
        velocity: f32,
        sample_rate: f32,
        phase: &mut f32,
    ) -> f32 {
        match sample_name {
            "bass_drum" => kick::process(params, sample_pos, velocity, sample_rate, phase),
            "snare_drum" | "rimshot" | "sidestick" => snare::process(
                sample_name,
                params,
                sample_pos,
                velocity,
                sample_rate,
                phase,
            ),
            "tom_high" | "tom_low" | "tom_floor" => tom::process(
                sample_name,
                params,
                sample_pos,
                velocity,
                sample_rate,
                phase,
            ),
            "hihat_closed" | "hihat_open" | "hihat_pedal" | "crash_cymbal" | "ride_cymbal"
            | "ride_bell" => {
                metallic::process_cymbal(sample_name, params, sample_pos, velocity, sample_rate)
            }
            _ => 0.0,
        }
    }
}
