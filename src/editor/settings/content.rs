use crate::clipper::{ClipperParams, OversamplingMode};
use crate::comp::CompressorParams;
use crate::editor::settings::equalizer::EqualizerBox;
use crate::electric::ElectricParams;
use crate::equalizer::EqualizerParams;
use crate::saturation::{SaturationParams, SaturationType};
use crate::transient::TransientParams;
use crate::{XrossDrumsEmulatorParams, part::PartParams};

use super::knob::LinearSlider;
use super::widgets::{effect_unit, labeled_knob};
use egui::{self, Color32, RichText, Vec2};

/// パーツセクション全体の描画
pub fn draw_part_section(ui: &mut egui::Ui, title: &str, part: &PartParams) {
    ui.vertical(|ui| {
        ui.heading(RichText::new(title).color(Color32::WHITE));
        ui.add_space(8.0);

        // 1. EQ Section
        draw_equalizer_full_unit(ui, title, &part.eq);
        ui.add_space(16.0);

        // 2. Row 1
        ui.horizontal_wrapped(|ui| {
            draw_mixer_unit(ui, part);
            draw_synth_unit(ui, title, &part.electric);
            draw_delay_unit(ui, title, &part.fx);
            draw_reverb_unit(ui, title, &part.fx);
            draw_transient_unit(ui, title, &part.transient);
        });
        ui.add_space(8.0);
        // 3. Row 2
        ui.horizontal_wrapped(|ui| {
            draw_saturation_unit(ui, title, &part.saturation);
            draw_compressor_unit(ui, title, &part.comp);
        });
    });
}

/// マスターセクションの描画
pub fn draw_master_section(ui: &mut egui::Ui, params: &XrossDrumsEmulatorParams) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("MASTER OUTPUT").color(Color32::WHITE));
        ui.add_space(8.0);

        // Row 1
        ui.horizontal_wrapped(|ui| {
            draw_equalizer_full_unit(ui, "master_eq", &params.master.eq);
            draw_delay_unit(ui, "master", &params.master.fx);
            draw_reverb_unit(ui, "master", &params.master.fx);
        });
        ui.add_space(8.0);
        // Row 2
        ui.horizontal_wrapped(|ui| {
            effect_unit(
                ui,
                "Master Dynamics",
                Color32::from_rgb(255, 150, 50),
                |ui| {
                    ui.horizontal_top(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Compressor");
                            draw_compressor_grid(ui, "master_comp", &params.master.comp);
                        });
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            ui.label("Clipper");
                            draw_clipper_grid(ui, "master_clip", &params.master.clipper);
                        });
                    });
                },
            );
        });
    });
}

// --- Enum セレクターのユーティリティ ---

fn draw_enum_buttons<T: Copy + PartialEq + std::fmt::Debug>(
    ui: &mut egui::Ui,
    current_val: T,
    options: &[(T, &str)],
    on_change: impl FnOnce(T),
) {
    ui.horizontal(|ui| {
        let mut next_val = None;
        for (option, label) in options {
            let is_selected = current_val == *option;
            let btn = egui::Button::new(RichText::new(*label).size(10.0).color(if is_selected {
                Color32::WHITE
            } else {
                Color32::GRAY
            }))
            .fill(if is_selected {
                Color32::from_rgb(60, 60, 100)
            } else {
                Color32::from_rgb(30, 30, 30)
            })
            .min_size(Vec2::new(40.0, 18.0));

            if ui.add(btn).clicked() {
                next_val = Some(*option);
            }
        }
        if let Some(val) = next_val {
            on_change(val);
        }
    });
}
fn draw_mixer_unit(ui: &mut egui::Ui, part: &PartParams) {
    let params = &part.electric;
    effect_unit(ui, "Mixer / Blend", Color32::from_rgb(0, 255, 200), |ui| {
        ui.vertical(|ui| {
            ui.set_width(180.0);
            ui.label("Kit Levels");
            ui.add(LinearSlider::new(
                &params.heavy_level,
                Color32::from_rgb(255, 100, 100),
            ));
            ui.add(LinearSlider::new(
                &params.light_level,
                Color32::from_rgb(100, 255, 100),
            ));
            ui.add(LinearSlider::new(
                &params.medium_level,
                Color32::from_rgb(100, 100, 255),
            ));
            ui.add_space(8.0);
            ui.label("Synth Models");
            ui.add(LinearSlider::new(
                &params.synth_modern,
                Color32::from_rgb(0, 200, 255),
            ));
            ui.add(LinearSlider::new(
                &params.synth_808,
                Color32::from_rgb(255, 100, 100),
            ));
            ui.add(LinearSlider::new(
                &params.synth_909,
                Color32::from_rgb(100, 255, 100),
            ));
            ui.add_space(8.0);
            labeled_knob(ui, "Pan", &part.pan.pan, Color32::from_rgb(255, 150, 50));
        });
    });
}

fn draw_delay_unit(ui: &mut egui::Ui, id_prefix: &str, params: &crate::params::fx::FxParams) {
    effect_unit(ui, "Delay", Color32::from_rgb(255, 200, 100), |ui| {
        egui::Grid::new(format!("{}_dly", id_prefix)).show(ui, |ui| {
            labeled_knob(
                ui,
                "Mix",
                &params.delay_mix,
                Color32::from_rgb(255, 200, 100),
            );
            labeled_knob(
                ui,
                "Time",
                &params.delay_time,
                Color32::from_rgb(255, 200, 100),
            );
            labeled_knob(ui, "FB", &params.delay_fb, Color32::from_rgb(255, 200, 100));
        });
    });
}

fn draw_reverb_unit(ui: &mut egui::Ui, id_prefix: &str, params: &crate::params::fx::FxParams) {
    effect_unit(ui, "Reverb", Color32::from_rgb(200, 200, 255), |ui| {
        egui::Grid::new(format!("{}_rev", id_prefix)).show(ui, |ui| {
            labeled_knob(
                ui,
                "Mix",
                &params.reverb_mix,
                Color32::from_rgb(200, 200, 255),
            );
            labeled_knob(
                ui,
                "Decay",
                &params.reverb_decay,
                Color32::from_rgb(200, 200, 255),
            );
        });
    });
}

fn draw_synth_unit(ui: &mut egui::Ui, id_prefix: &str, params: &ElectricParams) {
    effect_unit(ui, "Electric Synth", Color32::from_rgb(0, 200, 255), |ui| {
        egui::Grid::new(format!("{}_elec", id_prefix))
            .spacing(Vec2::new(8.0, 8.0))
            .show(ui, |ui| {
                labeled_knob(ui, "Freq", &params.freq, Color32::from_rgb(0, 255, 200));
                labeled_knob(ui, "Sweep", &params.sweep, Color32::from_rgb(0, 200, 255));
                ui.end_row();
                labeled_knob(
                    ui,
                    "Osc Dec",
                    &params.decay,
                    Color32::from_rgb(100, 100, 255),
                );
                labeled_knob(
                    ui,
                    "Noi Dec",
                    &params.noise_decay,
                    Color32::from_rgb(150, 150, 255),
                );
                ui.end_row();
                labeled_knob(
                    ui,
                    "Noi Lvl",
                    &params.noise_level,
                    Color32::from_rgb(200, 200, 255),
                );
            });
    });
}

fn draw_transient_unit(ui: &mut egui::Ui, id_prefix: &str, params: &TransientParams) {
    effect_unit(ui, "Transient", Color32::from_rgb(255, 215, 0), |ui| {
        egui::Grid::new(format!("{}_trans", id_prefix))
            .spacing(Vec2::new(8.0, 8.0))
            .show(ui, |ui| {
                labeled_knob(
                    ui,
                    "Attack",
                    &params.attack_gain,
                    Color32::from_rgb(255, 215, 0),
                );
                labeled_knob(
                    ui,
                    "Sustain",
                    &params.sustain_gain,
                    Color32::from_rgb(255, 165, 0),
                );
                ui.end_row();
                labeled_knob(
                    ui,
                    "Att Time",
                    &params.attack_time,
                    Color32::from_rgb(200, 180, 0),
                );
                labeled_knob(
                    ui,
                    "Sus Time",
                    &params.sustain_time,
                    Color32::from_rgb(200, 130, 0),
                );
                ui.end_row();
                labeled_knob(ui, "Detect", &params.sensitivity, Color32::WHITE);
            });
    });
}

fn draw_saturation_unit(ui: &mut egui::Ui, id_prefix: &str, params: &SaturationParams) {
    effect_unit(ui, "Saturation", Color32::from_rgb(255, 100, 100), |ui| {
        ui.vertical(|ui| {
            ui.label("Mode");
            draw_enum_buttons(
                ui,
                params.sat_type.value(),
                &[
                    (SaturationType::Soft, "Soft"),
                    (SaturationType::Hard, "Hard"),
                    (SaturationType::Tape, "Tape"),
                    (SaturationType::Tube, "Tube"),
                ],
                |val| params.sat_type.set_value(val),
            );

            ui.add_space(4.0);
            egui::Grid::new(format!("{}_sat_grid", id_prefix)).show(ui, |ui| {
                labeled_knob(ui, "Drive", &params.drive, Color32::from_rgb(255, 80, 80));
                labeled_knob(ui, "Mix", &params.mix, Color32::from_rgb(200, 200, 200));
                ui.end_row();
                labeled_knob(
                    ui,
                    "HPF",
                    &params.input_high_pass,
                    Color32::from_rgb(255, 150, 100),
                );
                labeled_knob(
                    ui,
                    "LPF",
                    &params.input_low_pass,
                    Color32::from_rgb(100, 150, 255),
                );
                ui.end_row();
                labeled_knob(ui, "Output", &params.output_gain, Color32::GRAY);
            });
        });
    });
}

fn draw_compressor_unit(ui: &mut egui::Ui, id_prefix: &str, params: &CompressorParams) {
    effect_unit(ui, "Compressor", Color32::from_rgb(100, 150, 255), |ui| {
        draw_compressor_grid(ui, id_prefix, params);
    });
}

fn draw_compressor_grid(ui: &mut egui::Ui, id_prefix: &str, params: &CompressorParams) {
    egui::Grid::new(format!("{}_comp_grid", id_prefix)).show(ui, |ui| {
        labeled_knob(
            ui,
            "Thresh",
            &params.threshold,
            Color32::from_rgb(100, 150, 255),
        );
        labeled_knob(ui, "Ratio", &params.ratio, Color32::from_rgb(80, 120, 220));
        ui.end_row();
        labeled_knob(
            ui,
            "Attack",
            &params.attack,
            Color32::from_rgb(150, 200, 255),
        );
        labeled_knob(
            ui,
            "Release",
            &params.release,
            Color32::from_rgb(150, 150, 255),
        );
        ui.end_row();
        labeled_knob(ui, "Knee", &params.knee, Color32::LIGHT_BLUE);
        labeled_knob(
            ui,
            "Makeup",
            &params.makeup,
            Color32::from_rgb(255, 200, 100),
        );
    });
}

fn draw_equalizer_full_unit(ui: &mut egui::Ui, title: &str, eq: &EqualizerParams) {
    effect_unit(
        ui,
        format!("Equalizer ({})", title).as_str(),
        Color32::from_rgb(200, 200, 200),
        |ui| {
            EqualizerBox::draw(ui, eq);
        },
    );
}

fn draw_clipper_grid(ui: &mut egui::Ui, id_prefix: &str, params: &ClipperParams) {
    ui.vertical(|ui| {
        ui.label("Oversampling");
        draw_enum_buttons(
            ui,
            params.oversampling.value(),
            &[
                (OversamplingMode::None, "Off"),
                (OversamplingMode::TwoTimes, "2x"),
                (OversamplingMode::FourTimes, "4x"),
            ],
            |val| params.oversampling.set_value(val),
        );

        ui.add_space(4.0);
        egui::Grid::new(format!("{}_clip_grid", id_prefix)).show(ui, |ui| {
            labeled_knob(
                ui,
                "In Gain",
                &params.input_gain,
                Color32::from_rgb(255, 100, 100),
            );
            labeled_knob(
                ui,
                "Ceiling",
                &params.threshold,
                Color32::from_rgb(255, 200, 100),
            );
            ui.end_row();
            labeled_knob(
                ui,
                "Soft",
                &params.softness,
                Color32::from_rgb(150, 255, 150),
            );
        });
    });
}
