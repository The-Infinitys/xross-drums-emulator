use std::sync::Arc;
use truce::prelude::*;

pub mod clipper;
pub mod comp;
pub mod electric;
pub mod equalizer;
pub mod saturation;
pub mod transient;

use clipper::ClipperParams;
use comp::CompressorParams;
use electric::ElectricParams;
use equalizer::EqualizerParams;
use saturation::SaturationParams;
use transient::TransientParams;

#[derive(Params, Default)]
pub struct XrossDrumsEmulatorParams {
    #[nested]
    pub parts: PartsParams,
    #[nested]
    pub master: MasterParams,
}
impl XrossDrumsEmulatorParams {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Params, Default)]
pub struct PartsParams {
    #[nested]
    pub kick: PartParams,
    #[nested]
    pub snare: PartParams,
    #[nested]
    pub hihat: PartParams,
    #[nested]
    pub crash: PartParams,
    #[nested]
    pub ride: PartParams,
    #[nested]
    pub tom_h: PartParams,
    #[nested]
    pub tom_l: PartParams,
    #[nested]
    pub tom_f: PartParams,
}

#[derive(Params, Default)]
pub struct PartParams {
    #[nested]
    pub electric: ElectricParams,
    #[nested]
    pub eq: EqualizerParams,
    #[nested]
    pub comp: CompressorParams,
    #[nested]
    pub transient: TransientParams,
    #[nested]
    pub saturation: SaturationParams,
}

#[derive(Params, Default)]
pub struct MasterParams {
    #[nested]
    pub eq: EqualizerParams,
    #[nested]
    pub comp: CompressorParams,
    #[nested]
    pub clipper: ClipperParams,
}
