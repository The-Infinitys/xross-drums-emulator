include!(concat!(env!("OUT_DIR"), "/samples_data.rs"));

// Re-exporting for external use
pub type DrumKits = DrumKitData;
pub type DrumsSamples = DrumsSamplesData;

impl DrumsSamples {
    pub const fn new() -> &'static Self {
        &SAMPLES
    }
}
