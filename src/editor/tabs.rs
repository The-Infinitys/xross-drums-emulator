// tabs.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTab {
    Pad,
    Settings,
    Presets,
}

pub struct Tabs {
    pub current_tab: AppTab,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            current_tab: AppTab::Pad,
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, AppTab::Pad, "DRUM PAD");
            ui.selectable_value(&mut self.current_tab, AppTab::Settings, "SETTINGS");
            ui.selectable_value(&mut self.current_tab, AppTab::Presets, "PRESETS");
        });
    }
}
