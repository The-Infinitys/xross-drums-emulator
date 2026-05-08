pub mod kick;
pub mod metallic;
pub mod snare;
pub mod tom;
pub mod utils;

use crate::drums::PartId;
use crate::params::electric::ElectricParams;
pub struct DrumSynth;

pub struct SynthPhases<'a> {
    pub modern: &'a mut f32,
    pub v808: &'a mut f32,
    pub v909: &'a mut f32,
}

impl DrumSynth {
    pub fn process(
        part_id: PartId,
        params: &ElectricParams,
        sample_pos: usize,
        velocity: f32,
        sample_rate: f32,
        phases: SynthPhases,
    ) -> f32 {
        let mut output = 0.0;

        // Modern
        let modern_lvl = params.synth_modern.value() / 100.0;
        if modern_lvl > 0.0 {
            output += match part_id {
                PartId::Kick => {
                    kick::process(params, sample_pos, velocity, sample_rate, phases.modern)
                }
                PartId::Snare | PartId::Rimshot | PartId::Sidestick => snare::process(
                    part_id,
                    params,
                    sample_pos,
                    velocity,
                    sample_rate,
                    phases.modern,
                ),
                PartId::TomHigh | PartId::TomLow | PartId::TomFloor => tom::process(
                    part_id,
                    params,
                    sample_pos,
                    velocity,
                    sample_rate,
                    phases.modern,
                ),
                PartId::HiHatClosed
                | PartId::HiHatOpen
                | PartId::HiHatPedal
                | PartId::Crash
                | PartId::Ride
                | PartId::RideBell => {
                    metallic::process_cymbal(part_id, params, sample_pos, velocity, sample_rate)
                }
            } * modern_lvl;
        }

        // 808
        let v808_lvl = params.synth_808.value() / 100.0;
        if v808_lvl > 0.0 {
            output += match part_id {
                PartId::Kick => {
                    kick::process_808(params, sample_pos, velocity, sample_rate, phases.v808)
                }
                PartId::Snare | PartId::Rimshot | PartId::Sidestick => snare::process_808(
                    part_id,
                    params,
                    sample_pos,
                    velocity,
                    sample_rate,
                    phases.v808,
                ),
                PartId::TomHigh | PartId::TomLow | PartId::TomFloor => tom::process_808(
                    part_id,
                    params,
                    sample_pos,
                    velocity,
                    sample_rate,
                    phases.v808,
                ),
                PartId::HiHatClosed
                | PartId::HiHatOpen
                | PartId::HiHatPedal
                | PartId::Crash
                | PartId::Ride
                | PartId::RideBell => {
                    metallic::process_808(part_id, params, sample_pos, velocity, sample_rate)
                }
            } * v808_lvl;
        }

        // 909
        let v909_lvl = params.synth_909.value() / 100.0;
        if v909_lvl > 0.0 {
            output += match part_id {
                PartId::Kick => {
                    kick::process_909(params, sample_pos, velocity, sample_rate, phases.v909)
                }
                PartId::Snare | PartId::Rimshot | PartId::Sidestick => snare::process_909(
                    part_id,
                    params,
                    sample_pos,
                    velocity,
                    sample_rate,
                    phases.v909,
                ),
                PartId::TomHigh | PartId::TomLow | PartId::TomFloor => tom::process_909(
                    part_id,
                    params,
                    sample_pos,
                    velocity,
                    sample_rate,
                    phases.v909,
                ),
                PartId::HiHatClosed
                | PartId::HiHatOpen
                | PartId::HiHatPedal
                | PartId::Crash
                | PartId::Ride
                | PartId::RideBell => {
                    metallic::process_909(part_id, params, sample_pos, velocity, sample_rate)
                }
            } * v909_lvl;
        }

        output
    }
}
