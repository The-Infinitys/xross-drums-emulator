include!(concat!(env!("OUT_DIR"), "/samples_data.rs"));

use crate::drums::PartId; // PartIdの定義場所に合わせてインポートパスを調整してください

// Re-exporting for external use
pub type DrumKits = DrumKitData;
pub type DrumsSamples = DrumsSamplesData;

/// ドラムキットの種類を識別する列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitId {
    Heavy,
    Light,
    Medium,
}

impl DrumsSamples {
    pub const fn new() -> &'static Self {
        &SAMPLES
    }

    // 引数を &str から KitId と PartId に変更
    pub fn get_sample_data_enum(&self, kit: KitId, part: PartId) -> &'static [f32] {
        let kit_data = match kit {
            KitId::Heavy => &self.heavy,
            KitId::Light => &self.light,
            KitId::Medium => &self.medium,
        };

        match part {
            PartId::Kick => kit_data.bass_drum,
            PartId::Crash => kit_data.crash_cymbal,
            PartId::HiHatClosed => kit_data.hihat_closed,
            PartId::HiHatOpen => kit_data.hihat_open,
            PartId::HiHatPedal => kit_data.hihat_pedal,
            PartId::RideBell => kit_data.ride_bell,
            PartId::Ride => kit_data.ride_cymbal,
            PartId::Rimshot => kit_data.rimshot,
            PartId::Sidestick => kit_data.sidestick,
            PartId::Snare => kit_data.snare_drum,
            PartId::TomFloor => kit_data.tom_floor,
            PartId::TomHigh => kit_data.tom_high,
            PartId::TomLow => kit_data.tom_low,
        }
    }
}
