use crate::params::saturation::SaturationType;
use crate::presets::KitBlend;

use super::{
    CompPreset, DrumPreset, EqBandPreset, EqPreset, FxPreset, PartPreset, SaturationPreset,
    SynthPreset, TransientPreset,
};

const DEFAULT_COMP: CompPreset = CompPreset {
    threshold: 0.0,
    ratio: 1.0,
    attack: 10.0,
    release: 100.0,
    knee: 0.0,
    makeup: 0.0,
};

const DEFAULT_SAT: SaturationPreset = SaturationPreset {
    drive: 0.0,
    sat_type: SaturationType::Soft,
    hpf: 20.0,
    lpf: 20000.0,
    mix: 100.0,
    out_gain: 0.0,
};

const DEFAULT_FX: FxPreset = FxPreset {
    delay_mix: 0.0,
    delay_time: 300.0,
    delay_fb: 30.0,
    reverb_mix: 0.0,
    reverb_decay: 50.0,
};

pub const PRESET: DrumPreset = DrumPreset {
    name: "JAZZ",
    description: "High-fidelity organic sound. Focus on resonance, stick definition, and wide dynamics.",
    kick: PartPreset {
        pan: 0.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 58.0,
            sweep: 0.1,
            decay: 180.0,
            noise: 0.05,
            noise_decay: 10.0,
        },
        eq: EqPreset {
            hp: 20.0,
            low: EqBandPreset {
                freq: 100.0,
                gain: 2.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 400.0,
                gain: -3.0,
                q: 0.5,
            },
            high: EqBandPreset {
                freq: 3000.0,
                gain: 1.0,
                q: 0.7,
            },
            lp: 15000.0,
        },
        comp: DEFAULT_COMP,
        transient: TransientPreset {
            attack: 2.0,
            attack_time: 20.0,
            sustain: 2.0,
            sustain_time: 500.0,
            sensitivity: 40.0,
        },
        saturation: DEFAULT_SAT,
        fx: FxPreset {
            reverb_mix: 10.0,
            reverb_decay: 60.0,
            ..DEFAULT_FX
        },
    },
    snare: PartPreset {
        pan: 10.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 240.0,
            sweep: 0.05,
            decay: 100.0,
            noise: 0.3,
            noise_decay: 150.0,
        },
        eq: EqPreset {
            hp: 100.0,
            low: EqBandPreset {
                freq: 250.0,
                gain: 1.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 1200.0,
                gain: 3.0,
                q: 0.6,
            },
            high: EqBandPreset {
                freq: 5000.0,
                gain: 4.0,
                q: 0.7,
            },
            lp: 18000.0,
        },
        comp: CompPreset {
            threshold: -5.0,
            ratio: 1.5,
            attack: 20.0,
            release: 100.0,
            ..DEFAULT_COMP
        },
        transient: TransientPreset {
            attack: 3.0,
            attack_time: 25.0,
            sustain: 3.0,
            sustain_time: 300.0,
            sensitivity: 50.0,
        },
        saturation: DEFAULT_SAT,
        fx: FxPreset {
            reverb_mix: 15.0,
            reverb_decay: 50.0,
            ..DEFAULT_FX
        },
    },
    hihat: PartPreset {
        pan: -30.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 8500.0,
            sweep: 0.0,
            decay: 60.0,
            noise: 0.5,
            noise_decay: 60.0,
        },
        eq: EqPreset {
            hp: 600.0,
            low: EqBandPreset {
                freq: 600.0,
                gain: -12.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 5000.0,
                gain: 2.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 12000.0,
                gain: 4.0,
                q: 0.7,
            },
            lp: 20000.0,
        },
        comp: DEFAULT_COMP,
        transient: TransientPreset {
            attack: 2.0,
            attack_time: 10.0,
            sustain: 1.0,
            sustain_time: 100.0,
            sensitivity: 60.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    tom_h: PartPreset {
        pan: -20.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 210.0,
            sweep: 0.1,
            decay: 400.0,
            noise: 0.05,
            noise_decay: 10.0,
        },
        eq: EqPreset {
            hp: 100.0,
            low: EqBandPreset {
                freq: 250.0,
                gain: 2.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 1000.0,
                gain: 2.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 5000.0,
                gain: 3.0,
                q: 0.7,
            },
            lp: 12000.0,
        },
        comp: DEFAULT_COMP,
        transient: TransientPreset {
            attack: 1.0,
            attack_time: 30.0,
            sustain: 5.0,
            sustain_time: 600.0,
            sensitivity: 40.0,
        },
        saturation: DEFAULT_SAT,
        fx: FxPreset {
            reverb_mix: 10.0,
            reverb_decay: 60.0,
            ..DEFAULT_FX
        },
    },
    tom_l: PartPreset {
        pan: 0.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 150.0,
            sweep: 0.1,
            decay: 450.0,
            noise: 0.05,
            noise_decay: 10.0,
        },
        eq: EqPreset {
            hp: 80.0,
            low: EqBandPreset {
                freq: 180.0,
                gain: 2.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 800.0,
                gain: 2.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 4000.0,
                gain: 3.0,
                q: 0.7,
            },
            lp: 10000.0,
        },
        comp: DEFAULT_COMP,
        transient: TransientPreset {
            attack: 1.0,
            attack_time: 30.0,
            sustain: 5.0,
            sustain_time: 600.0,
            sensitivity: 40.0,
        },
        saturation: DEFAULT_SAT,
        fx: FxPreset {
            reverb_mix: 10.0,
            reverb_decay: 60.0,
            ..DEFAULT_FX
        },
    },
    tom_f: PartPreset {
        pan: 20.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 95.0,
            sweep: 0.1,
            decay: 500.0,
            noise: 0.05,
            noise_decay: 10.0,
        },
        eq: EqPreset {
            hp: 60.0,
            low: EqBandPreset {
                freq: 130.0,
                gain: 3.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 600.0,
                gain: 2.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 3000.0,
                gain: 3.0,
                q: 0.7,
            },
            lp: 8000.0,
        },
        comp: DEFAULT_COMP,
        transient: TransientPreset {
            attack: 1.0,
            attack_time: 30.0,
            sustain: 5.0,
            sustain_time: 800.0,
            sensitivity: 40.0,
        },
        saturation: DEFAULT_SAT,
        fx: FxPreset {
            reverb_mix: 10.0,
            reverb_decay: 60.0,
            ..DEFAULT_FX
        },
    },
    crash: PartPreset {
        pan: -40.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 5500.0,
            sweep: 0.0,
            decay: 1500.0,
            noise: 0.6,
            noise_decay: 1200.0,
        },
        eq: EqPreset {
            hp: 400.0,
            low: EqBandPreset {
                freq: 400.0,
                gain: -15.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 4000.0,
                gain: 1.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 12000.0,
                gain: 4.0,
                q: 0.7,
            },
            lp: 20000.0,
        },
        comp: DEFAULT_COMP,
        transient: TransientPreset {
            attack: 2.0,
            attack_time: 20.0,
            sustain: 2.0,
            sustain_time: 1000.0,
            sensitivity: 60.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    ride: PartPreset {
        pan: 40.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 0.0,
            medium: 100.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 4500.0,
            sweep: 0.0,
            decay: 2500.0,
            noise: 0.4,
            noise_decay: 2000.0,
        },
        eq: EqPreset {
            hp: 500.0,
            low: EqBandPreset {
                freq: 500.0,
                gain: -12.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 3000.0,
                gain: 4.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 10000.0,
                gain: 6.0,
                q: 0.7,
            },
            lp: 18000.0,
        },
        comp: DEFAULT_COMP,
        transient: TransientPreset {
            attack: 4.0,
            attack_time: 15.0,
            sustain: 2.0,
            sustain_time: 1200.0,
            sensitivity: 70.0,
        },
        saturation: DEFAULT_SAT,
        fx: FxPreset {
            reverb_mix: 5.0,
            reverb_decay: 40.0,
            ..DEFAULT_FX
        },
    },
    master_eq: EqPreset {
        hp: 20.0,
        low: EqBandPreset {
            freq: 100.0,
            gain: 1.0,
            q: 0.7,
        },
        mid: EqBandPreset {
            freq: 1000.0,
            gain: 0.0,
            q: 0.7,
        },
        high: EqBandPreset {
            freq: 10000.0,
            gain: 1.0,
            q: 0.7,
        },
        lp: 20000.0,
    },
    master_comp: CompPreset {
        threshold: -6.0,
        ratio: 1.5,
        attack: 100.0,
        release: 300.0,
        knee: 12.0,
        makeup: 1.0,
    },
    master_clipper_threshold: -0.5,
    master_fx: FxPreset {
        reverb_mix: 5.0,
        reverb_decay: 40.0,
        ..DEFAULT_FX
    },
};
