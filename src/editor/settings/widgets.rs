use crate::editor::settings::knob::SingleKnob;
use egui::{self, Color32, RichText};
use truce::params::FloatParam;

pub fn effect_unit<F>(ui: &mut egui::Ui, label: &str, color: Color32, add_contents: F)
where
    F: FnOnce(&mut egui::Ui),
{
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(label).color(color).strong());
            ui.add_space(4.0);
            add_contents(ui);
        });
    });
}

pub fn labeled_knob(ui: &mut egui::Ui, label: &str, param: &FloatParam, color: Color32) {
    ui.vertical_centered(|ui| {
        ui.set_width(50.0);
        ui.add(SingleKnob::new(param, color));
        ui.label(RichText::new(label).size(10.0));
    });
}
