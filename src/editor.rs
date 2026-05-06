use crate::params::XrossDrumsEmulatorParams;
use std::sync::Arc;
mod background;
mod knobs;
use background::Background;
use egui::Frame;
use truce_egui::EguiEditor;
pub fn editor(_params: Arc<XrossDrumsEmulatorParams>) -> EguiEditor {
    let width = 840;
    let height = 600;
    let mut bg = Background::new();
    EguiEditor::new((width, height), move |egui_ctx, _state| {
        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(egui_ctx, |ui| {
                bg.draw(ui);
            });
    })
}
