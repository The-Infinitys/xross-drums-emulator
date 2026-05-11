use truce::prelude::*;

use super::OffsetParams;

#[derive(Params)]
pub struct ClipperParams {
    #[param(
        name = "In Gain",
        range = "linear(0, 24)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub input_gain: FloatParam,

    #[param(
        name = "Ceiling",
        range = "linear(-12, 0)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub threshold: FloatParam,

    #[param(
        name = "Softness",
        range = "linear(0, 100)",
        default = 20.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub softness: FloatParam,

    #[param(name = "Oversampling")]
    pub oversampling: EnumParam<OversamplingMode>,
}
impl OffsetParams for ClipperParams {
    const PARAM_COUNT: u32 = 4;
    fn with_id_offset(&mut self, offset: u32) {
        self.input_gain.info.id = offset;
        self.threshold.info.id = offset + 1;
        self.softness.info.id = offset + 2;
        self.oversampling.info.id = offset + 3;
    }
}

#[derive(ParamEnum, Debug)]
pub enum OversamplingMode {
    #[name = "Off"]
    None,
    #[name = "2x"]
    TwoTimes,
    #[name = "4x"]
    FourTimes,
}
