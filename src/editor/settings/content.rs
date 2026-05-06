use super::equalizer::EqualizerBox;
use super::knob::LinearSlider;
use super::widgets::{effect_unit, labeled_knob};
use crate::params::{PartParams, XrossDrumsEmulatorParams};
use egui::{self, Color32, RichText, Vec2};

pub fn draw_part_section(ui: &mut egui::Ui, title: &str, part: &PartParams) {
    ui.vertical(|ui| {
        ui.heading(RichText::new(title).color(Color32::WHITE));
        ui.add_space(8.0);

        EqualizerBox::draw(ui, &part.eq.low, &part.eq.mid, &part.eq.high);
        ui.add_space(16.0);

        ui.horizontal_top(|ui| {
            effect_unit(ui, "Mixer / Blend", Color32::from_rgb(0, 255, 200), |ui| {
                ui.vertical(|ui| {
                    ui.set_width(180.0);
                    ui.add_space(4.0);
                    ui.add(LinearSlider::new(
                        &part.electric.heavy_level,
                        Color32::from_rgb(255, 100, 100),
                    ));
                    ui.add_space(2.0);
                    ui.add(LinearSlider::new(
                        &part.electric.light_level,
                        Color32::from_rgb(100, 255, 100),
                    ));
                    ui.add_space(2.0);
                    ui.add(LinearSlider::new(
                        &part.electric.medium_level,
                        Color32::from_rgb(100, 100, 255),
                    ));
                    ui.add_space(2.0);
                    ui.add(LinearSlider::new(
                        &part.electric.electric_level,
                        Color32::from_rgb(0, 255, 200),
                    ));
                });
            });

            effect_unit(ui, "Electric Synth", Color32::from_rgb(0, 200, 255), |ui| {
                egui::Grid::new(format!("{}_elec", title))
                    .spacing(Vec2::new(10.0, 10.0))
                    .show(ui, |ui| {
                        labeled_knob(
                            ui,
                            "Freq",
                            &part.electric.freq,
                            Color32::from_rgb(0, 255, 200),
                        );
                        labeled_knob(
                            ui,
                            "Sweep",
                            &part.electric.sweep,
                            Color32::from_rgb(0, 200, 255),
                        );
                        ui.end_row();
                        labeled_knob(
                            ui,
                            "Decay",
                            &part.electric.decay,
                            Color32::from_rgb(100, 100, 255),
                        );
                        labeled_knob(
                            ui,
                            "Noise",
                            &part.electric.noise_level,
                            Color32::from_rgb(200, 200, 255),
                        );
                    });
            });

            effect_unit(ui, "Transient", Color32::from_rgb(255, 215, 0), |ui| {
                egui::Grid::new(format!("{}_trans", title))
                    .spacing(Vec2::new(10.0, 10.0))
                    .show(ui, |ui| {
                        labeled_knob(
                            ui,
                            "Attack",
                            &part.transient.attack_gain,
                            Color32::from_rgb(255, 215, 0),
                        );
                        labeled_knob(
                            ui,
                            "Sustain",
                            &part.transient.sustain_gain,
                            Color32::from_rgb(255, 165, 0),
                        );
                        ui.end_row();
                        labeled_knob(
                            ui,
                            "Att Time",
                            &part.transient.attack_time,
                            Color32::from_rgb(200, 180, 0),
                        );
                        labeled_knob(
                            ui,
                            "Sus Time",
                            &part.transient.sustain_time,
                            Color32::from_rgb(200, 130, 0),
                        );
                        ui.end_row();
                        labeled_knob(ui, "Detection", &part.transient.sensitivity, Color32::WHITE);
                    });
            });

            effect_unit(ui, "Saturation", Color32::from_rgb(255, 100, 100), |ui| {
                labeled_knob(
                    ui,
                    "Drive",
                    &part.saturation.drive,
                    Color32::from_rgb(255, 80, 80),
                );
            });

            effect_unit(ui, "Compressor", Color32::from_rgb(100, 150, 255), |ui| {
                ui.horizontal(|ui| {
                    labeled_knob(
                        ui,
                        "Threshold",
                        &part.comp.threshold,
                        Color32::from_rgb(100, 150, 255),
                    );
                    labeled_knob(
                        ui,
                        "Ratio",
                        &part.comp.ratio,
                        Color32::from_rgb(80, 120, 220),
                    );
                });
            });
        });
    });
}

pub fn draw_master_section(ui: &mut egui::Ui, params: &XrossDrumsEmulatorParams) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("MASTER OUTPUT").color(Color32::WHITE));
        ui.add_space(8.0);

        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.set_width(300.0);
                EqualizerBox::draw(
                    ui,
                    &params.master.eq.low,
                    &params.master.eq.mid,
                    &params.master.eq.high,
                );
            });
            ui.add_space(20.0);
            effect_unit(
                ui,
                "Final Dynamics",
                Color32::from_rgb(100, 200, 255),
                |ui| {
                    ui.horizontal(|ui| {
                        labeled_knob(
                            ui,
                            "Comp Thr",
                            &params.master.comp.threshold,
                            Color32::from_rgb(100, 200, 255),
                        );
                        labeled_knob(
                            ui,
                            "Clipper",
                            &params.master.clipper.threshold,
                            Color32::from_rgb(255, 100, 100),
                        );
                    });
                },
            );
        });
    });
}
