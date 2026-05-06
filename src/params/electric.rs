use truce::prelude::*;

#[derive(Params)]
pub struct ElectricParams {
    #[param(
        name = "Osc Freq",
        range = "log(20, 2000)",
        default = 60.0,
        unit = "Hz",
        smooth = "exp(5)"
    )]
    pub freq: FloatParam,

    #[param(
        name = "Osc Sweep",
        range = "linear(0, 1)",
        default = 0.5,
        smooth = "linear(20)"
    )]
    pub sweep: FloatParam,

    #[param(
        name = "Osc Decay",
        range = "log(10, 2000)",
        default = 100.0,
        unit = "ms",
        smooth = "exp(5)"
    )]
    pub decay: FloatParam,

    #[param(
        name = "Noise Level",
        range = "linear(0, 1)",
        default = 0.1,
        smooth = "exp(5)"
    )]
    pub noise_level: FloatParam,

    #[param(
        name = "Noise Decay",
        range = "log(10, 2000)",
        default = 50.0,
        unit = "ms",
        smooth = "exp(5)"
    )]
    pub noise_decay: FloatParam,

    // --- Mixer Levels ---
    #[param(
        name = "Heavy Kit",
        range = "linear(0, 100)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub heavy_level: FloatParam,

    #[param(
        name = "Light Kit",
        range = "linear(0, 100)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub light_level: FloatParam,

    #[param(
        name = "Medium Kit",
        range = "linear(0, 100)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub medium_level: FloatParam,

    #[param(
        name = "Electric",
        range = "linear(0, 100)",
        default = 100.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub electric_level: FloatParam,
}
