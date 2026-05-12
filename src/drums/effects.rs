pub mod clipper;
pub mod compressor;
pub mod delay;
pub mod equalizer;
pub mod reverb;
pub mod saturation;
pub mod transient;
pub mod utils;

use crate::params::master::MasterParams;
use crate::params::part::PartParams;

pub use clipper::Clipper;
pub use compressor::Compressor;
pub use delay::DelayEffect;
pub use equalizer::Equalizer;
pub use reverb::ReverbEffect;
pub use saturation::Saturation;
pub use transient::TransientShaper;

pub struct EffectChain {
    pub saturation: Saturation,
    pub transient: TransientShaper,
    pub eq: Equalizer,
    pub comp: Compressor,
    pub delay: DelayEffect,
    pub reverb: ReverbEffect,
    pub clipper: Clipper,
}

impl EffectChain {
    pub fn new() -> Self {
        Self {
            saturation: Saturation::new(),
            transient: TransientShaper::new(),
            eq: Equalizer::new(),
            comp: Compressor::new(),
            delay: DelayEffect::new(192000 * 2), // 2 seconds at 192k
            reverb: ReverbEffect::new(),
            clipper: Clipper::new(),
        }
    }

    pub fn process_part(&mut self, input: f32, params: &PartParams, sample_rate: f32) -> f32 {
        let mut x = input;
        x = self.saturation.process(x, &params.saturation, sample_rate);
        x = self.transient.process(x, &params.transient, sample_rate);
        x = self.eq.process(x, &params.eq, sample_rate);
        x = self.comp.process(x, &params.comp, sample_rate);
        x = self.delay.process(x, &params.fx, sample_rate);
        x = self.reverb.process(x, &params.fx, sample_rate);
        x
    }

    pub fn process_master(&mut self, input: f32, params: &MasterParams, sample_rate: f32) -> f32 {
        let mut x = input;
        x = self.eq.process(x, &params.eq, sample_rate);
        x = self.comp.process(x, &params.comp, sample_rate);
        x = self.clipper.process(x, &params.clipper, sample_rate);
        x = self.delay.process(x, &params.fx, sample_rate);
        x = self.reverb.process(x, &params.fx, sample_rate);
        x
    }
}
