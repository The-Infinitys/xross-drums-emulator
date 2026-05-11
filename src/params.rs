use truce::prelude::*;

pub mod clipper;
pub mod comp;
pub mod electric;
pub mod equalizer;
pub mod fx;
pub mod master;
pub mod pan;
pub mod part;
pub mod parts;
pub mod saturation;
pub mod transient;
use master::MasterParams;

use crate::parts::PartsParams;
#[derive(Params, Default)]
pub struct XrossDrumsEmulatorParams {
    #[nested]
    pub parts: PartsParams,
    #[nested]
    pub master: MasterParams,
}
impl XrossDrumsEmulatorParams {
    pub fn new() -> Self {
        let mut parts = PartsParams::new();
        let mut master = MasterParams::default();
        parts.with_id_offset(0);
        master.with_id_offset(PartsParams::PARAM_COUNT);
        Self { parts, master }
    }
}

trait OffsetParams: Params {
    const PARAM_COUNT: u32;
    fn with_id_offset(&mut self, offset: u32);
}
