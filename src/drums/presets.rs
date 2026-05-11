use crate::params::saturation::SaturationType;
use crate::params::{XrossDrumsEmulatorParams, part::PartParams};

pub mod cinematic;
pub mod funk;
pub mod jazz;
pub mod metal;
pub mod rock;
pub mod techno;

pub struct SynthPreset {
    pub freq: f32,
    pub sweep: f32,
    pub decay: f32,
    pub noise: f32,
    pub noise_decay: f32,
}

pub struct EqBandPreset {
    pub freq: f32,
    pub gain: f32,
    pub q: f32,
}

pub struct EqPreset {
    pub hp: f32,
    pub low: EqBandPreset,
    pub mid: EqBandPreset,
    pub high: EqBandPreset,
    pub lp: f32,
}

pub struct CompPreset {
    pub threshold: f32,
    pub ratio: f32,
    pub attack: f32,
    pub release: f32,
    pub knee: f32,
    pub makeup: f32,
}

pub struct TransientPreset {
    pub attack: f32,
    pub attack_time: f32,
    pub sustain: f32,
    pub sustain_time: f32,
    pub sensitivity: f32,
}

pub struct SaturationPreset {
    pub drive: f32,
    pub sat_type: SaturationType,
    pub hpf: f32,
    pub lpf: f32,
    pub mix: f32,
    pub out_gain: f32,
}

pub struct FxPreset {
    pub delay_mix: f32,
    pub delay_time: f32,
    pub delay_fb: f32,
    pub reverb_mix: f32,
    pub reverb_decay: f32,
}

pub struct PartPreset {
    pub pan: f32,
    pub kit_blend: KitBlend,
    pub synth: SynthPreset,
    pub eq: EqPreset,
    pub comp: CompPreset,
    pub transient: TransientPreset,
    pub saturation: SaturationPreset,
    pub fx: FxPreset,
}

pub struct KitBlend {
    pub heavy: f32,
    pub light: f32,
    pub medium: f32,
    pub modern_synth: f32,
    pub synth_808: f32,
    pub synth_909: f32,
}

pub struct DrumPreset {
    pub name: &'static str,
    pub description: &'static str,
    pub kick: PartPreset,
    pub snare: PartPreset,
    pub hihat: PartPreset,
    pub tom_h: PartPreset,
    pub tom_l: PartPreset,
    pub tom_f: PartPreset,
    pub crash: PartPreset,
    pub ride: PartPreset,
    pub master_eq: EqPreset,
    pub master_comp: CompPreset,
    pub master_clipper_threshold: f32,
    pub master_fx: FxPreset,
}

impl DrumPreset {
    pub fn apply(&self, params: &XrossDrumsEmulatorParams) {
        self.apply_part(&params.parts.kick, &self.kick);
        self.apply_part(&params.parts.snare, &self.snare);
        self.apply_part(&params.parts.hihat, &self.hihat);
        self.apply_part(&params.parts.tom_h, &self.tom_h);
        self.apply_part(&params.parts.tom_l, &self.tom_l);
        self.apply_part(&params.parts.tom_f, &self.tom_f);
        self.apply_part(&params.parts.crash, &self.crash);
        self.apply_part(&params.parts.ride, &self.ride);

        // Master EQ
        self.set_eq_full(&params.master.eq, &self.master_eq);

        // Master Comp
        self.set_comp(&params.master.comp, &self.master_comp);

        // Master Clipper
        params
            .master
            .clipper
            .threshold
            .set_value(self.master_clipper_threshold as f64);

        // Master FX
        self.set_fx(&params.master.fx, &self.master_fx);
    }

    fn apply_part(&self, part: &PartParams, p: &PartPreset) {
        // Pan
        part.pan.pan.set_value(p.pan as f64);

        // Levels
        part.electric
            .heavy_level
            .set_value(p.kit_blend.heavy as f64);
        part.electric
            .light_level
            .set_value(p.kit_blend.light as f64);
        part.electric
            .medium_level
            .set_value(p.kit_blend.medium as f64);
        part.electric
            .synth_modern
            .set_value(p.kit_blend.modern_synth as f64);
        part.electric
            .synth_808
            .set_value(p.kit_blend.synth_808 as f64);
        part.electric
            .synth_909
            .set_value(p.kit_blend.synth_909 as f64);

        // Synth
        part.electric.freq.set_value(p.synth.freq as f64);
        part.electric.sweep.set_value(p.synth.sweep as f64);
        part.electric.decay.set_value(p.synth.decay as f64);
        part.electric.noise_level.set_value(p.synth.noise as f64);
        part.electric
            .noise_decay
            .set_value(p.synth.noise_decay as f64);

        // EQ
        self.set_eq_full(&part.eq, &p.eq);

        // Dynamics
        self.set_comp(&part.comp, &p.comp);

        // Transient
        part.transient
            .attack_gain
            .set_value(p.transient.attack as f64);
        part.transient
            .attack_time
            .set_value(p.transient.attack_time as f64);
        part.transient
            .sustain_gain
            .set_value(p.transient.sustain as f64);
        part.transient
            .sustain_time
            .set_value(p.transient.sustain_time as f64);
        part.transient
            .sensitivity
            .set_value(p.transient.sensitivity as f64);

        // Saturation
        part.saturation.drive.set_value(p.saturation.drive as f64);
        part.saturation.sat_type.set_value(p.saturation.sat_type);
        part.saturation
            .input_high_pass
            .set_value(p.saturation.hpf as f64);
        part.saturation
            .input_low_pass
            .set_value(p.saturation.lpf as f64);
        part.saturation.mix.set_value(p.saturation.mix as f64);
        part.saturation
            .output_gain
            .set_value(p.saturation.out_gain as f64);

        // FX
        self.set_fx(&part.fx, &p.fx);
    }

    fn set_eq_full(&self, eq: &crate::params::equalizer::EqualizerParams, p: &EqPreset) {
        eq.hp.freq.set_value(p.hp as f64);
        self.set_eq_band(&eq.low, &p.low);
        self.set_eq_band(&eq.mid, &p.mid);
        self.set_eq_band(&eq.high, &p.high);
        eq.lp.freq.set_value(p.lp as f64);
    }

    fn set_eq_band(&self, band: &crate::params::equalizer::EqBandParams, p: &EqBandPreset) {
        band.freq.set_value(p.freq as f64);
        band.gain.set_value(p.gain as f64);
        band.q.set_value(p.q as f64);
    }

    fn set_comp(&self, comp: &crate::params::comp::CompressorParams, p: &CompPreset) {
        comp.threshold.set_value(p.threshold as f64);
        comp.ratio.set_value(p.ratio as f64);
        comp.attack.set_value(p.attack as f64);
        comp.release.set_value(p.release as f64);
        comp.knee.set_value(p.knee as f64);
        comp.makeup.set_value(p.makeup as f64);
    }

    fn set_fx(&self, fx: &crate::params::fx::FxParams, p: &FxPreset) {
        fx.delay_mix.set_value(p.delay_mix as f64);
        fx.delay_time.set_value(p.delay_time as f64);
        fx.delay_fb.set_value(p.delay_fb as f64);
        fx.reverb_mix.set_value(p.reverb_mix as f64);
        fx.reverb_decay.set_value(p.reverb_decay as f64);
    }
}
