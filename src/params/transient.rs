use super::OffsetParams;
use truce::prelude::*;

#[derive(Params)]
pub struct TransientParams {
    #[param(
        name = "Attack",
        range = "linear(-15, 15)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(10)"
    )]
    pub attack_gain: FloatParam,

    #[param(
        name = "Att Time",
        range = "log(1, 200)",
        default = 50.0,
        unit = "ms",
        smooth = "exp(5)"
    )]
    pub attack_time: FloatParam,

    #[param(
        name = "Sustain",
        range = "linear(-15, 15)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(10)"
    )]
    pub sustain_gain: FloatParam,

    #[param(
        name = "Sus Time",
        range = "log(10, 1000)",
        default = 200.0,
        unit = "ms",
        smooth = "exp(5)"
    )]
    pub sustain_time: FloatParam,

    #[param(
        name = "Detection",
        range = "linear(0, 100)",
        default = 50.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub sensitivity: FloatParam,
}
impl OffsetParams for TransientParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.attack_gain.info.id = offset;
        self.attack_time.info.id = offset + 1;
        self.sustain_gain.info.id = offset + 2;
        self.sustain_time.info.id = offset + 3;
        self.sensitivity.info.id = offset + 4;
    }
    const PARAM_COUNT: u32 = 5;
}
