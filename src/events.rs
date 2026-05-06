use std::sync::atomic::{AtomicU8, Ordering};

/// 1つのドラムパーツのトリガー状態
pub struct DrumTrigger {
    /// 0: トリガーなし, 1-127: Velocity
    pub velocity: AtomicU8,
}

impl DrumTrigger {
    pub fn new() -> Self {
        Self {
            velocity: AtomicU8::new(0),
        }
    }

    /// UIから叩く
    pub fn trigger(&self, velocity: u8) {
        self.velocity.store(velocity, Ordering::Release);
    }

    /// プロセッサで値を取り出し、0に戻す
    pub fn consume(&self) -> u8 {
        self.velocity.swap(0, Ordering::Acquire)
    }
}

/// 全ドラムパーツの共有キュー
pub struct NoteEvents {
    pub bass_drum: DrumTrigger,
    pub crash_cymbal: DrumTrigger,
    pub hihat_closed: DrumTrigger,
    pub hihat_open: DrumTrigger,
    pub hihat_pedal: DrumTrigger,
    pub ride_bell: DrumTrigger,
    pub ride_cymbal: DrumTrigger,
    pub rimshot: DrumTrigger,
    pub sidestick: DrumTrigger,
    pub snare_drum: DrumTrigger,
    pub tom_floor: DrumTrigger,
    pub tom_high: DrumTrigger,
    pub tom_low: DrumTrigger,
}

impl NoteEvents {
    pub fn new() -> Self {
        Self {
            bass_drum: DrumTrigger::new(),
            crash_cymbal: DrumTrigger::new(),
            hihat_closed: DrumTrigger::new(),
            hihat_open: DrumTrigger::new(),
            hihat_pedal: DrumTrigger::new(),
            ride_bell: DrumTrigger::new(),
            ride_cymbal: DrumTrigger::new(),
            rimshot: DrumTrigger::new(),
            sidestick: DrumTrigger::new(),
            snare_drum: DrumTrigger::new(),
            tom_floor: DrumTrigger::new(),
            tom_high: DrumTrigger::new(),
            tom_low: DrumTrigger::new(),
        }
    }

    /// MIDIノート番号から対応するトリガーへ書き込むユーティリティ
    pub fn trigger_by_note(&self, note: u8, velocity: u8) {
        match note {
            36 => self.bass_drum.trigger(velocity),
            38 => self.snare_drum.trigger(velocity),
            40 => self.rimshot.trigger(velocity),
            37 => self.sidestick.trigger(velocity),
            48 => self.tom_high.trigger(velocity),
            45 => self.tom_low.trigger(velocity),
            41 => self.tom_floor.trigger(velocity),
            42 => self.hihat_closed.trigger(velocity),
            46 => self.hihat_open.trigger(velocity),
            44 => self.hihat_pedal.trigger(velocity),
            49 => self.crash_cymbal.trigger(velocity),
            51 => self.ride_cymbal.trigger(velocity),
            53 => self.ride_bell.trigger(velocity),
            _ => {}
        }
    }
}
