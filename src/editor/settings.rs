use egui::{self, Color32, RichText, ScrollArea, Vec2};
use std::sync::{Arc, Mutex};

mod content;
mod equalizer;
mod knob;
mod tabs;
mod widgets;

use tabs::{DRUM_TABS, SelectedTab};

use crate::XrossDrumsEmulatorParams;

pub struct Settings {
    params: Arc<XrossDrumsEmulatorParams>,
    selected_tab: Mutex<SelectedTab>,
}

impl Settings {
    pub fn new(params: Arc<XrossDrumsEmulatorParams>) -> Self {
        Self {
            params,
            selected_tab: Mutex::new(SelectedTab::Kick),
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        let total_height = ui.available_height();
        let mut current = self.selected_tab.lock().unwrap();

        ui.horizontal_top(|ui| {
            // --- 左側: サイドバー ---
            ui.vertical(|ui| {
                ui.set_width(100.0);
                ui.set_min_height(total_height);

                ui.add_space(10.0);
                ui.label(RichText::new("PARTS").strong().color(Color32::GRAY));
                ui.separator();

                for (name, tab) in DRUM_TABS {
                    if ui.selectable_label(*current == tab, name).clicked() {
                        *current = tab;
                    }
                }

                ui.add_space(10.0);
                ui.label(RichText::new("GLOBAL").strong().color(Color32::GRAY));
                ui.separator();

                if ui
                    .selectable_label(*current == SelectedTab::Master, "MASTER")
                    .clicked()
                {
                    *current = SelectedTab::Master;
                }

                // 領域を下に伸ばすためのスペース埋め
                let spacer_height = ui.available_height();
                ui.allocate_space(Vec2::new(100.0, spacer_height));
            });

            ui.separator();

            // --- 右側: メイン ---
            ui.vertical(|ui| {
                ui.set_min_height(total_height);
                ScrollArea::vertical()
                    .id_salt("main_settings_scroll")
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        match *current {
                            SelectedTab::Kick => {
                                content::draw_part_section(ui, "KICK", &self.params.parts.kick)
                            }
                            SelectedTab::Snare => {
                                content::draw_part_section(ui, "SNARE", &self.params.parts.snare)
                            }
                            SelectedTab::HiHat => {
                                content::draw_part_section(ui, "HI-HAT", &self.params.parts.hihat)
                            }
                            SelectedTab::Crash => {
                                content::draw_part_section(ui, "CRASH", &self.params.parts.crash)
                            }
                            SelectedTab::Ride => {
                                content::draw_part_section(ui, "RIDE", &self.params.parts.ride)
                            }
                            SelectedTab::TomH => {
                                content::draw_part_section(ui, "TOM HIGH", &self.params.parts.tom_h)
                            }
                            SelectedTab::TomL => {
                                content::draw_part_section(ui, "TOM LOW", &self.params.parts.tom_l)
                            }
                            SelectedTab::Floor => content::draw_part_section(
                                ui,
                                "FLOOR TOM",
                                &self.params.parts.tom_f,
                            ),
                            SelectedTab::Master => content::draw_master_section(ui, &self.params),
                        }
                        ui.add_space(40.0);
                    });
            });
        });
    }
}
