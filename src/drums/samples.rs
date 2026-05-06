include!(concat!(env!("OUT_DIR"), "/samples_data.rs"));

// Re-exporting for external use
pub type DrumKits = DrumKitData;
pub type DrumsSamples = DrumsSamplesData;

impl DrumsSamples {
    pub const fn new() -> &'static Self {
        &SAMPLES
    }

    pub fn get_sample_data(&self, kit: &str, name: &str) -> Option<&'static [f32]> {
        let kit_data = match kit {
            "heavy" => &self.heavy,
            "light" => &self.light,
            "medium" => &self.medium,
            _ => return None,
        };

        match name {
            "bass_drum" => Some(kit_data.bass_drum),
            "crash_cymbal" => Some(kit_data.crash_cymbal),
            "hihat_closed" => Some(kit_data.hihat_closed),
            "hihat_open" => Some(kit_data.hihat_open),
            "hihat_pedal" => Some(kit_data.hihat_pedal),
            "ride_bell" => Some(kit_data.ride_bell),
            "ride_cymbal" => Some(kit_data.ride_cymbal),
            "rimshot" => Some(kit_data.rimshot),
            "sidestick" => Some(kit_data.sidestick),
            "snare_drum" => Some(kit_data.snare_drum),
            "tom_floor" => Some(kit_data.tom_floor),
            "tom_high" => Some(kit_data.tom_high),
            "tom_low" => Some(kit_data.tom_low),
            _ => None,
        }
    }
}
