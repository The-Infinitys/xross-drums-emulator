use super::OffsetParams;
use super::clipper::ClipperParams;
use super::comp::CompressorParams;
use super::equalizer::EqualizerParams;
use super::fx::FxParams;

use truce::prelude::*;

#[derive(Params, Default)]
pub struct MasterParams {
    #[nested]
    pub eq: EqualizerParams,
    #[nested]
    pub comp: CompressorParams,
    #[nested]
    pub clipper: ClipperParams,
    #[nested]
    pub fx: FxParams,
}
impl OffsetParams for MasterParams {
    fn with_id_offset(&mut self, offset: u32) {
        self.eq.with_id_offset(offset);
        self.comp
            .with_id_offset(offset + EqualizerParams::PARAM_COUNT);
        self.clipper
            .with_id_offset(offset + EqualizerParams::PARAM_COUNT + CompressorParams::PARAM_COUNT);
        self.fx.with_id_offset(
            offset
                + EqualizerParams::PARAM_COUNT
                + CompressorParams::PARAM_COUNT
                + ClipperParams::PARAM_COUNT,
        );
    }
    const PARAM_COUNT: u32 = EqualizerParams::PARAM_COUNT
        + CompressorParams::PARAM_COUNT
        + ClipperParams::PARAM_COUNT
        + FxParams::PARAM_COUNT;
}
