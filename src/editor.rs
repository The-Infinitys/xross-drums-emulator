use crate::events::NoteEvents;
use crate::params::XrossDrumsEmulatorParams;
use std::sync::Arc;

mod background;
mod pad;
mod presets;
mod settings;
mod tabs;

use background::Background;
use egui::{Color32, Frame, RichText};
use pad::PadGrid;
use presets::PresetsUI;
use settings::Settings;
use tabs::{AppTab, Tabs};
use truce_egui::EguiEditor;

struct EditorState {
    background: Background,
    tabs: Tabs,
    pad_grid: PadGrid,
    settings: Settings,
    presets: PresetsUI,
}

pub fn editor(params: Arc<XrossDrumsEmulatorParams>, events: Arc<NoteEvents>) -> EguiEditor {
    let width = 1024;
    let height = 640;

    let mut state = EditorState {
        background: Background::new(),
        tabs: Tabs::new(),
        pad_grid: PadGrid::new(events),
        settings: Settings::new(params.clone()),
        presets: PresetsUI::new(params),
    };

    EguiEditor::new((width, height), move |egui_ctx, _truce_state| {
        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(Color32::BLACK))
            .show(egui_ctx, |ui| {
                // --- 最背面: 背景アニメーション ---
                state.background.draw(ui);

                // --- 中面: メインUIレイアウト ---
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    // タイトル表示
                    ui.heading(
                        RichText::new("XROSS DRUMS")
                            .size(40.0)
                            .strong()
                            .color(Color32::WHITE),
                    );

                    ui.add_space(10.0);

                    // タブ切り替えボタンの描画
                    state.tabs.draw(ui);

                    ui.add_space(30.0);

                    // --- タブに応じたコンテンツの表示 ---
                    match state.tabs.current_tab {
                        AppTab::Pad => {
                            // ドラムパッド画面（円形配置 & キーボード入力）
                            state.pad_grid.draw(ui);
                        }
                        AppTab::Settings => {
                            state.settings.draw(ui);
                        }
                        AppTab::Presets => {
                            state.presets.draw(ui);
                        }
                    }
                });

                // 常にアニメーション（背景やパッドの発光）を動かすために再描画
                egui_ctx.request_repaint();
            });
    })
}
