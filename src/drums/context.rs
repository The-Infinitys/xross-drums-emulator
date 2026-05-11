use crate::drums::{DrumsSamples, EffectChain, PartId, PartState};
use crate::params::part::PartParams;

pub struct PartProcessingContext<'a> {
    pub part_id: PartId,
    pub state: &'a mut PartState,
    pub params: &'a PartParams,
    pub fx: &'a mut EffectChain,
    pub samples: &'a DrumsSamples,
    pub out_l: &'a mut f32,
    pub out_r: &'a mut f32,
    pub sample_rate: f32,
}
