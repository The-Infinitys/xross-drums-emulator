use super::OffsetParams;
use truce::prelude::*;

#[derive(Params)]
pub struct SaturationParams {
    #[param(
        name = "Drive",
        range = "linear(0, 40)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub drive: FloatParam,

    #[param(name = "Sat Type")]
    pub sat_type: EnumParam<SaturationType>,

    #[param(
        name = "Sat HPF",
        range = "log(20, 2000)",
        default = 20.0,
        unit = "Hz",
        smooth = "exp(5)"
    )]
    pub input_high_pass: FloatParam,

    #[param(
        name = "Sat LPF",
        range = "log(1000, 20000)",
        default = 20000.0,
        unit = "Hz",
        smooth = "exp(5)"
    )]
    pub input_low_pass: FloatParam,

    #[param(
        name = "Dry/Wet",
        range = "linear(0, 100)",
        default = 100.0,
        unit = "%",
        smooth = "linear(20)"
    )]
    pub mix: FloatParam,

    #[param(
        name = "Out Gain",
        range = "linear(-24, 24)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub output_gain: FloatParam,
}

#[derive(ParamEnum, Debug)]
pub enum SaturationType {
    #[name = "Soft S-Curve"]
    Soft,
    #[name = "Hard Clip"]
    Hard,
    #[name = "Tape Simulation"]
    Tape,
    #[name = "Tube Warmth"]
    Tube,
}
impl OffsetParams for SaturationParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.drive.info.id = offset;
        self.sat_type.info.id = offset + 1;
        self.input_high_pass.info.id = offset + 2;
        self.input_low_pass.info.id = offset + 3;
        self.mix.info.id = offset + 4;
        self.output_gain.info.id = offset + 5;
    }
    const PARAM_COUNT: u32 = 6;
}
