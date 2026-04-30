use truce::prelude::*;
#[derive(Params)]
pub struct XrossDrumsEmulatorParams {
    #[param(
        name = "Gain",
        range = "linear(-60, 6)",
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub gain: FloatParam,
}

pub use XrossDrumsEmulatorParamsParamId as P;
