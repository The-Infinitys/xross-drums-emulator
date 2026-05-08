use std::sync::Arc;
use truce::prelude::*;

mod context;
mod effects;
pub mod presets;
pub mod samples;
mod state;
mod synth;

use crate::XrossDrumsEmulatorParams;
use crate::drums::context::PartProcessingContext;
use crate::editor::editor;
use crate::events::NoteEvents;
use crate::params::PartParams;
use effects::EffectChain;
use samples::DrumsSamples;
use state::{DrumState, HiHatMode, PartState};
use synth::DrumSynth;

/// ドラムの各パーツを識別する列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartId {
    Kick,
    Snare,
    Rimshot,
    Sidestick,
    TomHigh,
    TomLow,
    TomFloor,
    HiHatClosed,
    HiHatOpen,
    HiHatPedal,
    Crash,
    Ride,
    RideBell,
}

pub struct XrossDrumsEmulator {
    samples: &'static DrumsSamples,
    params: Arc<XrossDrumsEmulatorParams>,
    events: Arc<NoteEvents>,
    state: DrumState,

    // 各パーツのエフェクトを配列や構造体で管理するとさらにスッキリしますが、
    // まずは所有権問題を解決するため個別のフィールドとして維持します。
    kick_fx: EffectChain,
    snare_drum_fx: EffectChain,
    rimshot_fx: EffectChain,
    sidestick_fx: EffectChain,
    tom_high_fx: EffectChain,
    tom_low_fx: EffectChain,
    tom_floor_fx: EffectChain,
    hihat_fx: EffectChain,
    crash_cymbal_fx: EffectChain,
    ride_cymbal_fx: EffectChain,
    ride_bell_fx: EffectChain,
    master_fx: EffectChain,

    sample_rate: f32,
}

impl XrossDrumsEmulator {
    pub fn new(params: Arc<XrossDrumsEmulatorParams>) -> Self {
        Self {
            samples: DrumsSamples::new(),
            params,
            events: Arc::new(NoteEvents::new()),
            state: DrumState::default(),
            kick_fx: EffectChain::new(),
            snare_drum_fx: EffectChain::new(),
            rimshot_fx: EffectChain::new(),
            sidestick_fx: EffectChain::new(),
            tom_high_fx: EffectChain::new(),
            tom_low_fx: EffectChain::new(),
            tom_floor_fx: EffectChain::new(),
            hihat_fx: EffectChain::new(),
            crash_cymbal_fx: EffectChain::new(),
            ride_cymbal_fx: EffectChain::new(),
            ride_bell_fx: EffectChain::new(),
            master_fx: EffectChain::new(),
            sample_rate: 44100.0,
        }
    }

    pub fn reset(&mut self, sr: f64, _bs: usize) {
        self.sample_rate = sr as f32;
        self.params.set_sample_rate(sr);
        self.params.snap_smoothers();
    }

    pub fn process(
        &mut self,
        buffer: &mut AudioBuffer,
        events: &EventList,
        _context: &mut ProcessContext,
    ) -> ProcessStatus {
        // 1. MIDIイベントの処理
        for event in events.iter().map(|e| &e.body) {
            match event {
                EventBody::NoteOn { note, velocity, .. } => {
                    self.trigger_by_note(*note, *velocity);
                }
                EventBody::NoteOn2 { note, velocity, .. } => {
                    self.trigger_by_note(*note, *velocity as f32);
                }
                _ => {}
            }
        }

        // 内部UIトリガーの処理
        self.handle_internal_triggers();

        let sample_rate = self.sample_rate;
        let num_samples = buffer.num_samples();

        for i in 0..num_samples {
            let mut left = 0.0;
            let mut right = 0.0;

            // 所有権の競合を避けるため、self から必要な参照を先に切り出す
            let p = &self.params.parts;
            let samples = &self.samples;

            let mut run_part =
                |id: PartId, state: &mut PartState, params: &PartParams, fx: &mut EffectChain| {
                    Self::process_single_part(PartProcessingContext {
                        part_id: id,
                        state,
                        params,
                        fx,
                        samples,
                        out_l: &mut left,
                        out_r: &mut right,
                        sample_rate,
                    })
                };

            run_part(
                PartId::Kick,
                &mut self.state.kick,
                &p.kick,
                &mut self.kick_fx,
            );
            run_part(
                PartId::Snare,
                &mut self.state.snare_drum,
                &p.snare,
                &mut self.snare_drum_fx,
            );
            run_part(
                PartId::Rimshot,
                &mut self.state.rimshot,
                &p.snare,
                &mut self.rimshot_fx,
            );
            run_part(
                PartId::Sidestick,
                &mut self.state.sidestick,
                &p.snare,
                &mut self.sidestick_fx,
            );
            run_part(
                PartId::TomHigh,
                &mut self.state.tom_high,
                &p.tom_h,
                &mut self.tom_high_fx,
            );
            run_part(
                PartId::TomLow,
                &mut self.state.tom_low,
                &p.tom_l,
                &mut self.tom_low_fx,
            );
            run_part(
                PartId::TomFloor,
                &mut self.state.tom_floor,
                &p.tom_f,
                &mut self.tom_floor_fx,
            );

            let hh_id = match self.state.hihat_mode {
                HiHatMode::Closed => PartId::HiHatClosed,
                HiHatMode::Open => PartId::HiHatOpen,
                HiHatMode::Pedal => PartId::HiHatPedal,
            };
            run_part(hh_id, &mut self.state.hihat, &p.hihat, &mut self.hihat_fx);

            run_part(
                PartId::Crash,
                &mut self.state.crash_cymbal,
                &p.crash,
                &mut self.crash_cymbal_fx,
            );
            run_part(
                PartId::Ride,
                &mut self.state.ride_cymbal,
                &p.ride,
                &mut self.ride_cymbal_fx,
            );
            run_part(
                PartId::RideBell,
                &mut self.state.ride_bell,
                &p.ride,
                &mut self.ride_bell_fx,
            );

            // マスターFXを適用（簡易的なparams変換）
            let master_params = PartParams::default();
            buffer.output(0)[i] = self.master_fx.process(left, &master_params, sample_rate);
            buffer.output(1)[i] = self.master_fx.process(right, &master_params, sample_rate);

            self.state.process_advance(1);
        }

        ProcessStatus::Normal
    }

    /// 所有権問題を解決するため、selfを引数に取らず、必要なコンポーネントのみを渡す関連関数にリファクタリング
    fn process_single_part(ctx: PartProcessingContext) {
        if !ctx.state.is_playing() {
            return;
        }

        let mut combined = 0.0;

        let sample_name = match ctx.part_id {
            PartId::Kick => "bass_drum",
            PartId::Snare => "snare_drum",
            PartId::Rimshot => "rimshot",
            PartId::Sidestick => "sidestick",
            PartId::TomHigh => "tom_high",
            PartId::TomLow => "tom_low",
            PartId::TomFloor => "tom_floor",
            PartId::HiHatClosed => "hihat_closed",
            PartId::HiHatOpen => "hihat_open",
            PartId::HiHatPedal => "hihat_pedal",
            PartId::Crash => "crash_cymbal",
            PartId::Ride => "ride_cymbal",
            PartId::RideBell => "ride_bell",
        };

        for (kit_name, level_param) in [
            ("heavy", &ctx.params.electric.heavy_level),
            ("light", &ctx.params.electric.light_level),
            ("medium", &ctx.params.electric.medium_level),
        ] {
            let level = level_param.value() / 100.0;
            if level > 0.0
                && let Some(sample_data) = ctx.samples.get_sample_data(kit_name, sample_name)
                && ctx.state.current_sample < sample_data.len()
            {
                combined += sample_data[ctx.state.current_sample] * ctx.state.velocity * level;
            }
        }

        if ctx.state.current_sample != usize::MAX {
            combined += DrumSynth::process(
                ctx.part_id,
                &ctx.params.electric,
                ctx.state.current_sample,
                ctx.state.velocity,
                ctx.sample_rate,
                synth::SynthPhases {
                    modern: &mut ctx.state.phase_modern,
                    v808: &mut ctx.state.phase_808,
                    v909: &mut ctx.state.phase_909,
                },
            );
        }

        combined = ctx.fx.process(combined, ctx.params, ctx.sample_rate);
        combined *= 256.0;

        let pan = ctx.params.pan.pan.value() / 100.0;
        let left_gain = (1.0f32 - pan).clamp(0.0, 1.0);
        let right_gain = (1.0f32 + pan).clamp(0.0, 1.0);

        *ctx.out_l += combined * left_gain;
        *ctx.out_r += combined * right_gain;

        if ctx.state.current_sample > (ctx.sample_rate * 5.0) as usize {
            ctx.state.stop();
        }
    }

    fn trigger_by_note(&mut self, note: u8, velocity: f32) {
        // velocityが0-127(u8)か0.0-1.0(f32)かによって正規化が必要な場合があります
        // ここでは受け取った値をそのまま渡します
        match note {
            36 => self.state.kick.trigger(velocity),
            38 => self.state.snare_drum.trigger(velocity),
            40 => self.state.rimshot.trigger(velocity),
            37 => self.state.sidestick.trigger(velocity),
            48 => self.state.tom_high.trigger(velocity),
            45 => self.state.tom_low.trigger(velocity),
            41 => self.state.tom_floor.trigger(velocity),
            42 => {
                self.state.hihat.trigger(velocity);
                self.state.hihat_mode = HiHatMode::Closed;
            }
            46 => {
                self.state.hihat.trigger(velocity);
                self.state.hihat_mode = HiHatMode::Open;
            }
            44 => {
                self.state.hihat.trigger(velocity);
                self.state.hihat_mode = HiHatMode::Pedal;
            }
            49 => self.state.crash_cymbal.trigger(velocity),
            51 => self.state.ride_cymbal.trigger(velocity),
            53 => self.state.ride_bell.trigger(velocity),
            _ => {}
        }
    }

    fn handle_internal_triggers(&mut self) {
        // u8 (0-127) を f32 (0.0-1.0) に変換してトリガー
        let normalize = |v: u8| v as f32 / 127.0;

        if let Some(v) = self.consume_if_any(&self.events.bass_drum) {
            self.state.kick.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.snare_drum) {
            self.state.snare_drum.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.rimshot) {
            self.state.rimshot.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.sidestick) {
            self.state.sidestick.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.tom_high) {
            self.state.tom_high.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.tom_low) {
            self.state.tom_low.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.tom_floor) {
            self.state.tom_floor.trigger(normalize(v));
        }

        if let Some(v) = self.consume_if_any(&self.events.hihat_closed) {
            self.state.hihat.trigger(normalize(v));
            self.state.hihat_mode = HiHatMode::Closed;
        }
        if let Some(v) = self.consume_if_any(&self.events.hihat_open) {
            self.state.hihat.trigger(normalize(v));
            self.state.hihat_mode = HiHatMode::Open;
        }
        if let Some(v) = self.consume_if_any(&self.events.hihat_pedal) {
            self.state.hihat.trigger(normalize(v));
            self.state.hihat_mode = HiHatMode::Pedal;
        }

        if let Some(v) = self.consume_if_any(&self.events.crash_cymbal) {
            self.state.crash_cymbal.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.ride_cymbal) {
            self.state.ride_cymbal.trigger(normalize(v));
        }
        if let Some(v) = self.consume_if_any(&self.events.ride_bell) {
            self.state.ride_bell.trigger(normalize(v));
        }
    }

    fn consume_if_any(&self, trigger: &crate::events::DrumTrigger) -> Option<u8> {
        let v = trigger.consume();
        if v > 0 { Some(v) } else { None }
    }

    pub fn params(&self) -> Arc<XrossDrumsEmulatorParams> {
        self.params.clone()
    }
    pub fn editor(&self) -> Box<dyn Editor> {
        Box::new(editor(self.params(), self.events.clone()))
    }
}
