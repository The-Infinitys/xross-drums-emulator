/// 個別のドラムパーツの再生状態
pub struct PartState {
    /// 現在のサンプル位置。
    /// usize::MAX を「停止/アイドル状態」として扱います。
    pub current_sample: usize,
    /// トリガー時のベロシティ (0.0 - 1.0)
    pub velocity: f32,
    /// シンセ用のフェーズ
    pub phase_modern: f32,
    pub phase_808: f32,
    pub phase_909: f32,
}

impl Default for PartState {
    fn default() -> Self {
        Self::new()
    }
}

impl PartState {
    pub fn new() -> Self {
        Self {
            current_sample: usize::MAX,
            velocity: 0.0,
            phase_modern: 0.0,
            phase_808: 0.0,
            phase_909: 0.0,
        }
    }

    pub fn trigger(&mut self, velocity: f32) {
        self.current_sample = 0;
        self.velocity = velocity.clamp(0.0, 1.0);
        self.phase_modern = 0.0;
        self.phase_808 = 0.0;
        self.phase_909 = 0.0;
    }

    pub fn stop(&mut self) {
        self.current_sample = usize::MAX;
    }

    pub fn advance(&mut self, amount: usize) {
        if self.current_sample != usize::MAX {
            self.current_sample = self.current_sample.saturating_add(amount);
        }
    }

    pub fn is_playing(&self) -> bool {
        self.current_sample != usize::MAX
    }
}

/// 全体のドラム状態
#[derive(Default)]
pub struct DrumState {
    pub kick: PartState,
    pub snare_drum: PartState,
    pub rimshot: PartState,
    pub sidestick: PartState,
    pub tom_high: PartState,
    pub tom_low: PartState,
    pub tom_floor: PartState,
    pub hihat: PartState, // Closed, Open, Pedal share one voice for choking
    pub crash_cymbal: PartState,
    pub ride_cymbal: PartState,
    pub ride_bell: PartState,

    // Hi-hat state to know which sample to play
    pub hihat_mode: HiHatMode,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum HiHatMode {
    #[default]
    Closed,
    Open,
    Pedal,
}

impl DrumState {
    pub fn process_advance(&mut self, buffer_size: usize) {
        self.kick.advance(buffer_size);
        self.snare_drum.advance(buffer_size);
        self.rimshot.advance(buffer_size);
        self.sidestick.advance(buffer_size);
        self.tom_high.advance(buffer_size);
        self.tom_low.advance(buffer_size);
        self.tom_floor.advance(buffer_size);
        self.hihat.advance(buffer_size);
        self.crash_cymbal.advance(buffer_size);
        self.ride_cymbal.advance(buffer_size);
        self.ride_bell.advance(buffer_size);
    }
}
