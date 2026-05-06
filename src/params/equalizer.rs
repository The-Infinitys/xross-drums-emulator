use truce::prelude::*;

/// 1. ピーキング/シェルビング用バンド（Gainあり）
#[derive(Params)]
pub struct EqBandParams {
    #[param(
        name = "Gain",
        range = "linear(-18, 18)",
        default = 0.0,
        unit = "dB",
        smooth = "exp(10)"
    )]
    pub gain: FloatParam,

    #[param(
        name = "Freq",
        range = "log(20, 20000)",
        default = 1000.0,
        unit = "Hz",
        smooth = "exp(5)"
    )]
    pub freq: FloatParam,

    #[param(
        name = "Q",
        range = "log(0.1, 10)",
        default = 0.707,
        smooth = "linear(20)"
    )]
    pub q: FloatParam,
}

/// 2. ハイパス/ローパス専用バンド（Gainなし）
#[derive(Params)]
pub struct FilterCutoffParams {
    #[param(
        name = "Freq",
        range = "log(20, 20000)",
        default = 20.0,
        unit = "Hz",
        smooth = "exp(5)"
    )]
    pub freq: FloatParam,

    #[param(
        name = "Q",
        range = "linear(0.1, 2.0)",
        default = 0.707,
        smooth = "linear(20)"
    )]
    pub q: FloatParam,
}

/// 3. メイン構造体
#[derive(Params)]
pub struct EqualizerParams {
    // 300-309: HPF
    #[nested]
    pub hp: FilterCutoffParams,

    // 310-319: Low Band
    #[nested]
    pub low: EqBandParams,

    // 320-329: Mid Band
    #[nested]
    pub mid: EqBandParams,

    // 330-339: High Band
    #[nested]
    pub high: EqBandParams,

    // 340-349: LPF
    #[nested]
    pub lp: FilterCutoffParams,
}

impl EqualizerParams {
    pub fn new() -> Self {
        Self {
            hp: FilterCutoffParams::new(),
            low: EqBandParams::new(),
            mid: EqBandParams::new(),
            high: EqBandParams::new(),
            lp: FilterCutoffParams::new(),
        }
    }
}
impl Default for EqualizerParams {
    fn default() -> Self {
        Self::new()
    }
}
