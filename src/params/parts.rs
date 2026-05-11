use super::OffsetParams;

use super::part::PartParams;
use truce::prelude::*;
#[derive(Params, Default)]
pub struct PartsParams {
    #[nested]
    pub kick: PartParams,
    #[nested]
    pub snare: PartParams,
    #[nested]
    pub hihat: PartParams,
    #[nested]
    pub crash: PartParams,
    #[nested]
    pub ride: PartParams,
    #[nested]
    pub tom_h: PartParams,
    #[nested]
    pub tom_l: PartParams,
    #[nested]
    pub tom_f: PartParams,
}

impl PartsParams {
    pub fn new() -> Self {
        Self {
            kick: PartParams::new_kick(),
            snare: PartParams::new_snare(),
            hihat: PartParams::new_hihat(),
            crash: PartParams::new_crash(),
            ride: PartParams::new_ride(),
            tom_h: PartParams::new_tom_h(),
            tom_l: PartParams::new_tom_l(),
            tom_f: PartParams::new_tom_f(),
        }
    }
}
impl OffsetParams for PartsParams {
    const PARAM_COUNT: u32 = 8 * PartParams::PARAM_COUNT;
    fn with_id_offset(&mut self, offset: u32) {
        self.kick.with_id_offset(offset);
        self.snare.with_id_offset(offset + PartParams::PARAM_COUNT);
        self.hihat
            .with_id_offset(offset + 2 * PartParams::PARAM_COUNT);
        self.crash
            .with_id_offset(offset + 3 * PartParams::PARAM_COUNT);
        self.ride
            .with_id_offset(offset + 4 * PartParams::PARAM_COUNT);
        self.tom_h
            .with_id_offset(offset + 5 * PartParams::PARAM_COUNT);
        self.tom_l
            .with_id_offset(offset + 6 * PartParams::PARAM_COUNT);
        self.tom_f
            .with_id_offset(offset + 7 * PartParams::PARAM_COUNT);
    }
}
