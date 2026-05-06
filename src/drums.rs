use std::sync::Arc;
use truce::prelude::*;
mod samples;
use crate::editor::editor;
use crate::XrossDrumsEmulatorParams;
use samples::DrumsSamples;
pub struct XrossDrumsEmulator {
    samples: Arc<&'static DrumsSamples>,
    params: Arc<XrossDrumsEmulatorParams>,
}

impl XrossDrumsEmulator {
    pub fn new(params: Arc<XrossDrumsEmulatorParams>) -> Self {
        let samples = Arc::new(DrumsSamples::new());
        Self { samples, params }
    }
}

impl XrossDrumsEmulator {
    pub fn reset(&mut self, sr: f64, _bs: usize) {
        self.params.set_sample_rate(sr);
        self.params.snap_smoothers();
    }

    pub fn process(
        &mut self,
        buffer: &mut AudioBuffer,
        _events: &EventList,
        _context: &mut ProcessContext,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
    pub fn params(&self) -> Arc<XrossDrumsEmulatorParams> {
        self.params.clone()
    }
    pub fn editor(&self) -> Box<dyn Editor> {
        Box::new(editor(self.params()))
    }
}
