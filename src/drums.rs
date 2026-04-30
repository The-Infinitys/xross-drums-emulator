use std::sync::Arc;
use truce::prelude::*;

use crate::XrossDrumsEmulatorParams;
pub struct XrossDrumsEmulator {
    params: Arc<XrossDrumsEmulatorParams>,
}

impl XrossDrumsEmulator {
    pub fn new(params: Arc<XrossDrumsEmulatorParams>) -> Self {
        Self { params }
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
        for i in 0..buffer.num_samples() {
            let gain = db_to_linear(self.params.gain.smoothed_next() as f64) as f32;
            for ch in 0..buffer.channels() {
                let (inp, out) = buffer.io(ch);
                out[i] = inp[i] * gain;
            }
        }
        ProcessStatus::Normal
    }
}
