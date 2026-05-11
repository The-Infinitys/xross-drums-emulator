use super::OffsetParams;
use truce::prelude::*;

#[derive(Params)]
pub struct FxParams {
    // Delay
    #[param(
        name = "Delay Mix",
        range = "linear(0, 100)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub delay_mix: FloatParam,

    #[param(
        name = "Delay Time",
        range = "log(10, 2000)",
        default = 300.0,
        unit = "ms",
        smooth = "exp(5)"
    )]
    pub delay_time: FloatParam,

    #[param(
        name = "Delay Feedback",
        range = "linear(0, 95)",
        default = 30.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub delay_fb: FloatParam,

    // Reverb
    #[param(
        name = "Reverb Mix",
        range = "linear(0, 100)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub reverb_mix: FloatParam,

    #[param(
        name = "Reverb Decay",
        range = "linear(0, 100)",
        default = 50.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub reverb_decay: FloatParam,
}
impl OffsetParams for FxParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.delay_mix.info.id = offset;
        self.delay_time.info.id = offset + 1;
        self.delay_fb.info.id = offset + 2;
        self.reverb_mix.info.id = offset + 3;
        self.reverb_decay.info.id = offset + 4;
    }
    const PARAM_COUNT: u32 = 5;
}
