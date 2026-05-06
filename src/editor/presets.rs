use crate::drums::presets::{jazz, metal, rock};
use crate::XrossDrumsEmulatorParams;
use egui::{self, Color32, RichText};
use std::sync::Arc;

pub struct PresetsUI {
    params: Arc<XrossDrumsEmulatorParams>,
}

impl PresetsUI {
    pub fn new(params: Arc<XrossDrumsEmulatorParams>) -> Self {
        Self { params }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading(RichText::new("FACTORY PRESETS").color(Color32::GRAY));
            ui.add_space(30.0);

            let preset_list = [
                (&rock::PRESET, Color32::from_rgb(255, 100, 100)),
                (&jazz::PRESET, Color32::from_rgb(100, 255, 100)),
                (&metal::PRESET, Color32::from_rgb(100, 150, 255)),
            ];

            for (preset, color) in preset_list {
                ui.group(|ui| {
                    ui.set_width(400.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(preset.name).size(24.0).strong().color(color));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button(RichText::new("LOAD").strong()).clicked() {
                                        preset.apply(&self.params);
                                    }
                                },
                            );
                        });
                        ui.label(preset.description);
                        ui.add_space(5.0);
                    });
                });
                ui.add_space(15.0);
            }
        });
    }
}
