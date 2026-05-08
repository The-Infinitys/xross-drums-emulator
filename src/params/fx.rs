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
