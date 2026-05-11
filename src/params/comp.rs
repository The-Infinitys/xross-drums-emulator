use super::OffsetParams;
use truce::prelude::*;

#[derive(Params)]
pub struct CompressorParams {
    #[param(
        name = "Threshold",
        range = "linear(-60, 0)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(10)"
    )]
    pub threshold: FloatParam,

    #[param(
        name = "Ratio",
        range = "log(1, 20)",
        default = 1.0,
        smooth = "linear(20)"
    )]
    pub ratio: FloatParam,

    #[param(
        name = "Attack",
        range = "log(0.1, 500)",
        default = 10.0,
        unit = "ms",
        smooth = "exp(5)"
    )]
    pub attack: FloatParam,

    #[param(
        name = "Release",
        range = "log(10, 2000)",
        default = 100.0,
        unit = "ms",
        smooth = "exp(5)"
    )]
    pub release: FloatParam,

    #[param(
        name = "Knee",
        range = "linear(0, 24)",
        default = 0.0,
        unit = "dB",
        smooth = "linear(20)"
    )]
    pub knee: FloatParam,

    #[param(
        name = "Makeup Gain",
        range = "linear(0, 24)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub makeup: FloatParam,
}
impl OffsetParams for CompressorParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.threshold.info.id = offset;
        self.ratio.info.id = offset + 1;
        self.attack.info.id = offset + 2;
        self.release.info.id = offset + 3;
        self.knee.info.id = offset + 4;
        self.makeup.info.id = offset + 5;
    }
    const PARAM_COUNT: u32 = 6;
}
