use super::OffsetParams;
use truce::prelude::*;

#[derive(Params)]
pub struct ElectricParams {
    #[param(
        name = "Osc Freq",
        range = "log(20, 15000)",
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

    // --- Synth Variations ---
    #[param(
        name = "Synth Modern",
        range = "linear(0, 100)",
        default = 100.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub synth_modern: FloatParam,

    #[param(
        name = "Synth 808",
        range = "linear(0, 100)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub synth_808: FloatParam,

    #[param(
        name = "Synth 909",
        range = "linear(0, 100)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub synth_909: FloatParam,

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
}
impl OffsetParams for ElectricParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.decay.info.id = offset;
        self.freq.info.id = offset + 1;
        self.noise_decay.info.id = offset + 2;
        self.noise_level.info.id = offset + 3;
        self.sweep.info.id = offset + 4;
        self.synth_modern.info.id = offset + 5;
        self.synth_808.info.id = offset + 6;
        self.synth_909.info.id = offset + 7;
        self.heavy_level.info.id = offset + 8;
        self.light_level.info.id = offset + 9;
        self.medium_level.info.id = offset + 10;
    }
    const PARAM_COUNT: u32 = 11;
}
