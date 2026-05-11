use super::OffsetParams;
use super::comp::CompressorParams;
use super::electric::ElectricParams;
use super::equalizer::EqualizerParams;
use super::fx::FxParams;
use super::pan::PanParam;
use super::saturation::SaturationParams;
use super::transient::TransientParams;
use truce::prelude::*;

impl PartParams {
    pub fn new_kick() -> Self {
        let mut p = Self::default();
        let val = 55.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_snare() -> Self {
        let mut p = Self::default();
        let val = 220.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_hihat() -> Self {
        let mut p = Self::default();
        let val = 8000.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_crash() -> Self {
        let mut p = Self::default();
        let val = 6000.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_ride() -> Self {
        let mut p = Self::default();
        let val = 4500.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_tom_h() -> Self {
        let mut p = Self::default();
        let val = 180.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_tom_l() -> Self {
        let mut p = Self::default();
        let val = 130.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
    pub fn new_tom_f() -> Self {
        let mut p = Self::default();
        let val = 90.0;
        p.electric.freq.set_value(val);
        p.electric.freq.info.default_plain = val;
        p
    }
}
#[derive(Params, Default)]
pub struct PartParams {
    #[nested]
    pub pan: PanParam,
    #[nested]
    pub electric: ElectricParams,
    #[nested]
    pub eq: EqualizerParams,
    #[nested]
    pub comp: CompressorParams,
    #[nested]
    pub transient: TransientParams,
    #[nested]
    pub saturation: SaturationParams,
    #[nested]
    pub fx: FxParams,
}
impl OffsetParams for PartParams {
    const PARAM_COUNT: u32 = PanParam::PARAM_COUNT
        + ElectricParams::PARAM_COUNT
        + EqualizerParams::PARAM_COUNT
        + CompressorParams::PARAM_COUNT
        + TransientParams::PARAM_COUNT
        + SaturationParams::PARAM_COUNT
        + FxParams::PARAM_COUNT;
    fn with_id_offset(&mut self, offset: u32) {
        self.pan.with_id_offset(offset);
        self.electric.with_id_offset(offset + PanParam::PARAM_COUNT);
        self.eq
            .with_id_offset(offset + PanParam::PARAM_COUNT + ElectricParams::PARAM_COUNT);
        self.comp.with_id_offset(
            offset
                + PanParam::PARAM_COUNT
                + ElectricParams::PARAM_COUNT
                + EqualizerParams::PARAM_COUNT,
        );
        self.transient.with_id_offset(
            offset
                + PanParam::PARAM_COUNT
                + ElectricParams::PARAM_COUNT
                + EqualizerParams::PARAM_COUNT
                + CompressorParams::PARAM_COUNT,
        );
        self.saturation.with_id_offset(
            offset
                + PanParam::PARAM_COUNT
                + ElectricParams::PARAM_COUNT
                + EqualizerParams::PARAM_COUNT
                + CompressorParams::PARAM_COUNT
                + TransientParams::PARAM_COUNT,
        );
        self.fx.with_id_offset(
            offset
                + PanParam::PARAM_COUNT
                + ElectricParams::PARAM_COUNT
                + EqualizerParams::PARAM_COUNT
                + CompressorParams::PARAM_COUNT
                + TransientParams::PARAM_COUNT
                + SaturationParams::PARAM_COUNT,
        );
    }
}
