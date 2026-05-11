use crate::XrossDrumsEmulatorParams;
use crate::drums::presets::{cinematic, funk, jazz, metal, rock, techno};
use egui::{self, Color32, RichText, ScrollArea, Vec2};
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
            ui.add_space(10.0);

            let preset_list = [
                (&rock::PRESET, Color32::from_rgb(255, 100, 100)),
                (&jazz::PRESET, Color32::from_rgb(100, 255, 100)),
                (&metal::PRESET, Color32::from_rgb(100, 150, 255)),
                (&funk::PRESET, Color32::from_rgb(255, 200, 50)),
                (&techno::PRESET, Color32::from_rgb(200, 100, 255)),
                (&cinematic::PRESET, Color32::from_rgb(255, 150, 50)),
            ];

            ScrollArea::vertical()
                .id_salt("preset_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    // 1カラムあたりの幅を調整し、グリッドで並べる
                    let columns = 2;
                    let spacing = 15.0;
                    let item_width = (ui.available_width() - spacing * (columns as f32 + 1.0)) / columns as f32;

                    egui::Grid::new("preset_grid")
                        .spacing(Vec2::splat(spacing))
                        .min_col_width(item_width)
                        .show(ui, |ui| {
                            for (i, (preset, color)) in preset_list.iter().enumerate() {
                                ui.group(|ui| {
                                    ui.set_width(item_width);
                                    ui.set_min_height(90.0);
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(preset.name).size(20.0).strong().color(*color));
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if ui.button(RichText::new("LOAD").strong()).clicked() {
                                                        preset.apply(&self.params);
                                                    }
                                                },
                                            );
                                        });
                                        ui.add_space(4.0);
                                        ui.label(RichText::new(preset.description).size(11.0).color(Color32::LIGHT_GRAY));
                                    });
                                });

                                if (i + 1) % columns == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                    ui.add_space(40.0);
                });
        });
    }
}
