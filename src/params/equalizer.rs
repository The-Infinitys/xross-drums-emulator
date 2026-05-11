use super::OffsetParams;
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

/// 3. メイン構造体
#[derive(Params)]
pub struct EqualizerParams {
    // 300-309: HPF
    #[nested]
    pub hp: EqBandParams,

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
    pub lp: EqBandParams,
}
impl EqualizerParams {
    pub fn new() -> Self {
        let mut hp = EqBandParams::new();
        hp.freq.info.default_plain = 20.0;
        hp.freq.set_value(20.0);
        hp.q.info.default_plain = 0.707;
        hp.q.set_value(0.707);
        hp.gain.info.default_plain = -12.0;
        hp.gain.set_value(-12.0);
        let mut low = EqBandParams::new();
        low.freq.info.default_plain = 100.0;
        low.freq.set_value(100.0);
        low.gain.info.default_plain = 0.0;
        low.gain.set_value(0.0);
        low.q.info.default_plain = 0.707;
        low.q.set_value(0.707);

        let mut mid = EqBandParams::new();
        mid.freq.info.default_plain = 1000.0;
        mid.freq.set_value(1000.0);
        mid.gain.info.default_plain = 0.0;
        mid.gain.set_value(0.0);
        mid.q.info.default_plain = 1.0;
        mid.q.set_value(1.0);

        let mut high = EqBandParams::new();
        high.freq.info.default_plain = 5000.0;
        high.freq.set_value(5000.0);
        high.gain.info.default_plain = 0.0;
        high.gain.set_value(0.0);
        high.q.info.default_plain = 0.707;
        high.q.set_value(0.707);

        let mut lp = EqBandParams::new();
        lp.freq.info.default_plain = 20000.0;
        lp.freq.set_value(20000.0);
        lp.q.info.default_plain = 0.707;
        lp.q.set_value(0.707);
        lp.gain.info.default_plain = -12.0;
        lp.gain.set_value(-12.0);
        Self {
            hp,
            low,
            mid,
            high,
            lp,
        }
    }
}
impl Default for EqualizerParams {
    fn default() -> Self {
        Self::new()
    }
}
impl OffsetParams for EqualizerParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.hp.with_id_offset(offset);
        self.low.with_id_offset(offset + EqBandParams::PARAM_COUNT);
        self.mid
            .with_id_offset(offset + 2 * EqBandParams::PARAM_COUNT);
        self.high
            .with_id_offset(offset + 3 * EqBandParams::PARAM_COUNT);
        self.lp
            .with_id_offset(offset + 4 * EqBandParams::PARAM_COUNT);
    }
    const PARAM_COUNT: u32 = 5 * EqBandParams::PARAM_COUNT;
}
impl OffsetParams for EqBandParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.freq.info.id = offset;
        self.gain.info.id = offset + 1;
        self.q.info.id = offset + 2;
    }
    const PARAM_COUNT: u32 = 3;
}
