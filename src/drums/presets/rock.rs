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
    name: "ROCK",
    description: "Punchy and polished. Enhanced dynamics with clear snap and solid weight.",
    kick: PartPreset {
        pan: 0.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 55.0,
            sweep: 0.7,
            decay: 180.0,
            noise: 0.2,
            noise_decay: 25.0,
        },
        eq: EqPreset {
            hp: 30.0,
            low: EqBandPreset {
                freq: 80.0,
                gain: 5.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 400.0,
                gain: -10.0,
                q: 0.5,
            },
            high: EqBandPreset {
                freq: 3000.0,
                gain: 6.0,
                q: 0.7,
            },
            lp: 12000.0,
        },
        comp: CompPreset {
            threshold: -14.0,
            ratio: 4.0,
            attack: 5.0,
            release: 150.0,
            knee: 6.0,
            makeup: 2.0,
        },
        transient: TransientPreset {
            attack: 8.0,
            attack_time: 40.0,
            sustain: -4.0,
            sustain_time: 150.0,
            sensitivity: 60.0,
        },
        saturation: SaturationPreset {
            drive: 3.0,
            sat_type: SaturationType::Tape,
            hpf: 40.0,
            lpf: 8000.0,
            mix: 50.0,
            out_gain: 0.0,
        },
        fx: DEFAULT_FX,
    },
    snare: PartPreset {
        pan: 5.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 210.0,
            sweep: 0.2,
            decay: 120.0,
            noise: 0.6,
            noise_decay: 200.0,
        },
        eq: EqPreset {
            hp: 150.0,
            low: EqBandPreset {
                freq: 220.0,
                gain: 3.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 800.0,
                gain: -4.0,
                q: 0.5,
            },
            high: EqBandPreset {
                freq: 5000.0,
                gain: 7.0,
                q: 0.7,
            },
            lp: 18000.0,
        },
        comp: CompPreset {
            threshold: -16.0,
            ratio: 3.0,
            attack: 10.0,
            release: 120.0,
            knee: 4.0,
            makeup: 3.0,
        },
        transient: TransientPreset {
            attack: 7.0,
            attack_time: 30.0,
            sustain: 3.0,
            sustain_time: 200.0,
            sensitivity: 70.0,
        },
        saturation: SaturationPreset {
            drive: 4.0,
            sat_type: SaturationType::Tube,
            hpf: 100.0,
            lpf: 12000.0,
            mix: 30.0,
            out_gain: 0.0,
        },
        fx: DEFAULT_FX,
    },
    hihat: PartPreset {
        pan: -40.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 8000.0,
            sweep: 0.1,
            decay: 50.0,
            noise: 0.7,
            noise_decay: 50.0,
        },
        eq: EqPreset {
            hp: 400.0,
            low: EqBandPreset {
                freq: 400.0,
                gain: -18.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 6000.0,
                gain: 2.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 12000.0,
                gain: 5.0,
                q: 0.7,
            },
            lp: 20000.0,
        },
        comp: CompPreset {
            threshold: -8.0,
            ratio: 2.0,
            attack: 1.0,
            release: 50.0,
            ..DEFAULT_COMP
        },
        transient: TransientPreset {
            attack: 3.0,
            attack_time: 10.0,
            sustain: 0.0,
            sustain_time: 50.0,
            sensitivity: 80.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    tom_h: PartPreset {
        pan: -30.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 190.0,
            sweep: 0.4,
            decay: 300.0,
            noise: 0.1,
            noise_decay: 20.0,
        },
        eq: EqPreset {
            hp: 100.0,
            low: EqBandPreset {
                freq: 200.0,
                gain: 4.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 600.0,
                gain: -6.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 4000.0,
                gain: 4.0,
                q: 0.7,
            },
            lp: 12000.0,
        },
        comp: CompPreset {
            threshold: -12.0,
            ratio: 3.0,
            attack: 15.0,
            release: 200.0,
            ..DEFAULT_COMP
        },
        transient: TransientPreset {
            attack: 5.0,
            attack_time: 40.0,
            sustain: 2.0,
            sustain_time: 300.0,
            sensitivity: 60.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    tom_l: PartPreset {
        pan: 0.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 140.0,
            sweep: 0.4,
            decay: 350.0,
            noise: 0.1,
            noise_decay: 20.0,
        },
        eq: EqPreset {
            hp: 80.0,
            low: EqBandPreset {
                freq: 150.0,
                gain: 4.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 500.0,
                gain: -6.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 3500.0,
                gain: 4.0,
                q: 0.7,
            },
            lp: 10000.0,
        },
        comp: CompPreset {
            threshold: -12.0,
            ratio: 3.0,
            attack: 15.0,
            release: 200.0,
            ..DEFAULT_COMP
        },
        transient: TransientPreset {
            attack: 5.0,
            attack_time: 40.0,
            sustain: 2.0,
            sustain_time: 300.0,
            sensitivity: 60.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    tom_f: PartPreset {
        pan: 30.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 90.0,
            sweep: 0.5,
            decay: 450.0,
            noise: 0.1,
            noise_decay: 20.0,
        },
        eq: EqPreset {
            hp: 60.0,
            low: EqBandPreset {
                freq: 100.0,
                gain: 5.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 400.0,
                gain: -8.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 3000.0,
                gain: 4.0,
                q: 0.7,
            },
            lp: 8000.0,
        },
        comp: CompPreset {
            threshold: -12.0,
            ratio: 3.0,
            attack: 15.0,
            release: 200.0,
            ..DEFAULT_COMP
        },
        transient: TransientPreset {
            attack: 5.0,
            attack_time: 40.0,
            sustain: 2.0,
            sustain_time: 300.0,
            sensitivity: 60.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    crash: PartPreset {
        pan: -50.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 5500.0,
            sweep: 0.0,
            decay: 1500.0,
            noise: 0.8,
            noise_decay: 1200.0,
        },
        eq: EqPreset {
            hp: 300.0,
            low: EqBandPreset {
                freq: 300.0,
                gain: -20.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 5000.0,
                gain: 2.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 10000.0,
                gain: 4.0,
                q: 0.7,
            },
            lp: 20000.0,
        },
        comp: CompPreset {
            threshold: -10.0,
            ratio: 2.0,
            attack: 1.0,
            release: 50.0,
            ..DEFAULT_COMP
        },
        transient: TransientPreset {
            attack: 4.0,
            attack_time: 10.0,
            sustain: 0.0,
            sustain_time: 500.0,
            sensitivity: 80.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    ride: PartPreset {
        pan: 50.0,
        kit_blend: KitBlend {
            heavy: 0.0,
            light: 100.0,
            medium: 0.0,
            modern_synth: 0.0,
            synth_808: 0.0,
            synth_909: 0.0,
        },
        synth: SynthPreset {
            freq: 4500.0,
            sweep: 0.0,
            decay: 2500.0,
            noise: 0.5,
            noise_decay: 2000.0,
        },
        eq: EqPreset {
            hp: 400.0,
            low: EqBandPreset {
                freq: 400.0,
                gain: -15.0,
                q: 0.7,
            },
            mid: EqBandPreset {
                freq: 3500.0,
                gain: 1.0,
                q: 0.7,
            },
            high: EqBandPreset {
                freq: 8000.0,
                gain: 5.0,
                q: 0.7,
            },
            lp: 18000.0,
        },
        comp: CompPreset {
            threshold: -6.0,
            ratio: 1.5,
            attack: 2.0,
            release: 100.0,
            ..DEFAULT_COMP
        },
        transient: TransientPreset {
            attack: 3.0,
            attack_time: 15.0,
            sustain: 0.0,
            sustain_time: 800.0,
            sensitivity: 80.0,
        },
        saturation: DEFAULT_SAT,
        fx: DEFAULT_FX,
    },
    master_eq: EqPreset {
        hp: 25.0,
        low: EqBandPreset {
            freq: 100.0,
            gain: 2.0,
            q: 0.7,
        },
        mid: EqBandPreset {
            freq: 1000.0,
            gain: -1.0,
            q: 0.5,
        },
        high: EqBandPreset {
            freq: 8000.0,
            gain: 2.0,
            q: 0.7,
        },
        lp: 18000.0,
    },
    master_comp: CompPreset {
        threshold: -18.0,
        ratio: 2.0,
        attack: 30.0,
        release: 200.0,
        knee: 6.0,
        makeup: 3.0,
    },
    master_clipper_threshold: -0.2,
    master_fx: DEFAULT_FX,
};
