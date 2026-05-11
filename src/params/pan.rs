use super::OffsetParams;
use truce::prelude::*;

#[derive(Params)]
pub struct PanParam {
    #[param(
        name = "Pan",
        range = "linear(-100.0, 100.0)",
        default = 0.0,
        unit = "%",
        smooth = "linear(20.0)"
    )]
    pub pan: FloatParam,
}
impl OffsetParams for PanParam {
    fn with_id_offset(&mut self, offset: u32) {
        self.pan.info.id = offset;
    }
    const PARAM_COUNT: u32 = 1;
}
