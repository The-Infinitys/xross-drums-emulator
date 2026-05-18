use truce::prelude::*;

use crate::XrossDrumsEmulator;

impl PluginLogic for XrossDrumsEmulator {
    fn reset(&mut self, sr: f64, bs: usize) {
        self.reset(sr, bs)
    }

    fn process(
        &mut self,
        buffer: &mut AudioBuffer,
        events: &EventList,
        context: &mut ProcessContext,
    ) -> ProcessStatus {
        self.process(buffer, events, context)
    }
    fn custom_editor(&self) -> Option<Box<dyn Editor>> {
        Some(self.editor())
    }
    fn bus_layouts() -> Vec<BusLayout> {
        vec![BusLayout::new().with_output("Main", ChannelConfig::Stereo)]
    }
}
