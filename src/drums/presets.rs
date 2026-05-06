use crate::params::{PartParams, XrossDrumsEmulatorParams};

pub mod jazz;
pub mod metal;
pub mod rock;

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
    pub low: EqBandPreset,
    pub mid: EqBandPreset,
    pub high: EqBandPreset,
}

pub struct PartPreset {
    pub pan: f32,
    pub heavy: f32,
    pub light: f32,
    pub medium: f32,
    pub electric: f32,
    pub synth: SynthPreset,
    pub eq: EqPreset,
    // Dynamics
    pub comp_threshold: f32,
    pub trans_attack: f32,
    pub trans_sustain: f32,
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
    pub master_comp_threshold: f32,
    pub master_clipper_threshold: f32,
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

        params
            .master
            .comp
            .threshold
            .set_value(self.master_comp_threshold as f64);
        params
            .master
            .clipper
            .threshold
            .set_value(self.master_clipper_threshold as f64);
    }

    fn apply_part(&self, part: &PartParams, p: &PartPreset) {
        // Pan
        part.pan.pan.set_value(p.pan as f64);

        // Levels
        part.electric.heavy_level.set_value(p.heavy as f64);
        part.electric.light_level.set_value(p.light as f64);
        part.electric.medium_level.set_value(p.medium as f64);
        part.electric.electric_level.set_value(p.electric as f64);

        // Synth
        part.electric.freq.set_value(p.synth.freq as f64);
        part.electric.sweep.set_value(p.synth.sweep as f64);
        part.electric.decay.set_value(p.synth.decay as f64);
        part.electric.noise_level.set_value(p.synth.noise as f64);
        part.electric
            .noise_decay
            .set_value(p.synth.noise_decay as f64);

        // EQ
        self.set_eq_band(&part.eq.low, &p.eq.low);
        self.set_eq_band(&part.eq.mid, &p.eq.mid);
        self.set_eq_band(&part.eq.high, &p.eq.high);

        // Dynamics
        part.comp.threshold.set_value(p.comp_threshold as f64);
        part.transient.attack_gain.set_value(p.trans_attack as f64);
        part.transient
            .sustain_gain
            .set_value(p.trans_sustain as f64);
    }

    fn set_eq_band(&self, band: &crate::params::equalizer::EqBandParams, p: &EqBandPreset) {
        band.freq.set_value(p.freq as f64);
        band.gain.set_value(p.gain as f64);
        band.q.set_value(p.q as f64);
    }
}
