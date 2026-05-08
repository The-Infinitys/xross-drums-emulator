use truce::prelude::*;

pub mod clipper;
pub mod comp;
pub mod electric;
pub mod equalizer;
pub mod fx;
pub mod saturation;
pub mod transient;

use clipper::ClipperParams;
use comp::CompressorParams;
use electric::ElectricParams;
use equalizer::EqualizerParams;
use fx::FxParams;
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
        Self {
            parts: PartsParams::new(),
            master: MasterParams::default(),
        }
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

impl PartsParams {
    pub fn new() -> Self {
        Self {
            kick: PartParams::new_kick(),
            snare: PartParams::new_snare(),
            hihat: PartParams::new_hihat(),
            ..Default::default()
        }
    }
}

impl PartParams {
    pub fn new_kick() -> Self {
        let mut p = Self::default();
        let val = 60.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_snare() -> Self {
        let mut p = Self::default();
        let val = 200.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_hihat() -> Self {
        let mut p = Self::default();
        let val = 8000.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_crash() -> Self {
        Self::default()
    }
    pub fn new_ride() -> Self {
        Self::default()
    }
    pub fn new_tom() -> Self {
        Self::default()
    }
}
#[derive(Params, Default)]
pub struct PartParams {
    pub pan: PanParam,
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
    #[nested]
    pub fx: FxParams,
}

#[derive(Params, Default)]
pub struct MasterParams {
    #[nested]
    pub eq: EqualizerParams,
    #[nested]
    pub comp: CompressorParams,
    #[nested]
    pub clipper: ClipperParams,
    #[nested]
    pub fx: FxParams,
}

#[derive(Params)]
pub struct PanParam {
    #[param(
        name = "Pan",
        range = "linear(-100.0, 100.0)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20.0)"
    )]
    pub pan: FloatParam,
}
