use egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};

use crate::params::equalizer::EqBandParams;
use crate::utils::{freq_to_norm, get_filter_gain, norm_to_freq, FilterType};

pub struct EqualizerBox;

impl EqualizerBox {
    pub fn draw(ui: &mut egui::Ui, low: &EqBandParams, mid: &EqBandParams, high: &EqBandParams) {
        let label_w = 30.0;
        let bottom_h = 20.0;
        let available_rect = ui.available_rect_before_wrap();

        let desired_size = egui::vec2(available_rect.width().max(250.0), 160.0);
        let (full_rect, _response) = ui.allocate_at_least(desired_size, egui::Sense::hover());

        // 1. 背景
        ui.painter()
            .rect_filled(full_rect, 4.0, Color32::from_black_alpha(220));
        ui.painter().rect_stroke(
            full_rect,
            4.0,
            Stroke::new(1.0, Color32::from_gray(80)),
            egui::StrokeKind::Inside,
        );

        let graph_rect = Rect::from_min_max(
            full_rect.min + egui::vec2(label_w, 10.0),
            full_rect.max - egui::vec2(10.0, bottom_h),
        );

        let painter = ui.painter().with_clip_rect(graph_rect);
        let sample_rate = 44100.0; // 表示用

        // 2. グリッド線とラベル描画
        Self::draw_grid(ui, graph_rect);

        // 3. 合成カーブの描画
        let steps = (graph_rect.width() as usize / 2).max(128);
        let points: Vec<Pos2> = (0..=steps)
            .map(|i| {
                let x_norm = i as f32 / steps as f32;
                let f = norm_to_freq(x_norm);
                let g = get_filter_gain(
                    f,
                    low.freq.value() as f32,
                    low.gain.value() as f32,
                    low.q.value() as f32,
                    FilterType::LowShelf,
                    sample_rate,
                ) + get_filter_gain(
                    f,
                    mid.freq.value() as f32,
                    mid.gain.value() as f32,
                    mid.q.value() as f32,
                    FilterType::Peaking,
                    sample_rate,
                ) + get_filter_gain(
                    f,
                    high.freq.value() as f32,
                    high.gain.value() as f32,
                    high.q.value() as f32,
                    FilterType::HighShelf,
                    sample_rate,
                );

                let y_norm = 1.0 - (g.clamp(-24.0, 24.0) + 24.0) / 48.0;
                graph_rect.min
                    + egui::vec2(x_norm * graph_rect.width(), y_norm * graph_rect.height())
            })
            .collect();

        painter.add(egui::Shape::line(
            points,
            Stroke::new(2.0, Color32::from_rgb(0, 255, 255)),
        ));

        // 4. バンド操作点
        let bands = [
            (low, Color32::from_rgb(255, 165, 0), "LOW"),
            (mid, Color32::from_rgb(0, 255, 127), "MID"),
            (high, Color32::from_rgb(180, 100, 255), "HIGH"),
        ];

        for (band, color, label) in bands {
            Self::draw_band_handle(ui, graph_rect, band, color, label);
        }
    }

    fn draw_grid(ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();
        let font = FontId::proportional(9.0);
        let stroke = Stroke::new(0.5, Color32::from_gray(50));

        // dB 軸
        for db in [-12, 0, 12] {
            let y = rect.top() + (1.0 - (db as f32 + 24.0) / 48.0) * rect.height();
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                stroke,
            );
            painter.text(
                Pos2::new(rect.left() - 5.0, y),
                Align2::RIGHT_CENTER,
                format!("{db}"),
                font.clone(),
                Color32::GRAY,
            );
        }

        // 周波数軸
        for f in [100.0, 1000.0, 10000.0] {
            let x = rect.left() + freq_to_norm(f) * rect.width();
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                stroke,
            );
            let txt = if f >= 1000.0 {
                format!("{:.0}k", f / 1000.0)
            } else {
                f.to_string()
            };
            painter.text(
                Pos2::new(x, rect.bottom() + 10.0),
                Align2::CENTER_CENTER,
                txt,
                font.clone(),
                Color32::GRAY,
            );
        }
    }

    fn draw_band_handle(
        ui: &mut egui::Ui,
        rect: Rect,
        band: &EqBandParams,
        color: Color32,
        label: &str,
    ) {
        let pos = Pos2::new(
            rect.left() + freq_to_norm(band.freq.value() as f32) * rect.width(),
            rect.top() + (1.0 - (band.gain.value() as f32 + 24.0) / 48.0) * rect.height(),
        );

        let id = ui.make_persistent_id(label);
        let resp = ui.interact(
            Rect::from_center_size(pos, Vec2::splat(14.0)),
            id,
            egui::Sense::click_and_drag(),
        );

        // ダブルクリックでリセット
        if resp.double_clicked() {
            band.freq.set_value(band.freq.info.default_plain);
            band.gain.set_value(band.gain.info.default_plain);
            band.q.set_value(band.q.info.default_plain);
        }

        // ドラッグ操作
        if resp.dragged() {
            let delta = resp.drag_delta();
            let freq_range = 20000.0f32.ln() - 20.0f32.ln();
            let new_f =
                ((band.freq.value() as f32).ln() + (delta.x / rect.width()) * freq_range).exp();
            let new_g =
                (band.gain.value() as f32 - (delta.y / rect.height()) * 48.0).clamp(-24.0, 24.0);

            band.freq.set_value(new_f.clamp(20.0, 20000.0) as f64);
            band.gain.set_value(new_g as f64);
        }

        // スクロールでQ値を変更
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let new_q = (band.q.value() as f32 + scroll / 200.0).clamp(0.1, 10.0);
                band.q.set_value(new_q as f64);
            }
        }

        // --- ポップアップ編集 ---
        let popup_id = id.with("popup");
        if resp.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }

        if ui.memory(|mem| mem.is_popup_open(popup_id)) {
            egui::Window::new(format!("Edit {}", label))
                .id(popup_id.with("window"))
                .fixed_pos(pos + Vec2::new(10.0, 10.0))
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.set_min_width(100.0);

                    let mut f = band.freq.value();
                    if ui
                        .add(
                            egui::DragValue::new(&mut f)
                                .range(20.0..=20000.0)
                                .suffix(" Hz")
                                .speed(10.0),
                        )
                        .changed()
                    {
                        band.freq.set_value(f as f64);
                    }

                    let mut g = band.gain.value();
                    if ui
                        .add(
                            egui::DragValue::new(&mut g)
                                .range(-24.0..=24.0)
                                .suffix(" dB")
                                .speed(0.1),
                        )
                        .changed()
                    {
                        band.gain.set_value(g as f64);
                    }

                    let mut q = band.q.value();
                    if ui
                        .add(
                            egui::DragValue::new(&mut q)
                                .range(0.1..=10.0)
                                .prefix("Q: ")
                                .speed(0.01),
                        )
                        .changed()
                    {
                        band.q.set_value(q as f64);
                    }

                    if ui.button("Close").clicked() {
                        ui.memory_mut(|mem| mem.close_popup());
                    }
                });
        }

        // 描画処理
        let painter = ui.painter();
        let is_active =
            resp.hovered() || resp.dragged() || ui.memory(|mem| mem.is_popup_open(popup_id));

        // Qガイド円（クリップあり）
        let q_painter = painter.with_clip_rect(rect);
        let q_radius =
            ((rect.width() * 0.08) / (band.q.value() as f32).sqrt()).clamp(5.0, rect.width() / 3.0);
        q_painter.circle_stroke(pos, q_radius, Stroke::new(1.0, color.linear_multiply(0.2)));

        // ハンドル
        painter.circle_filled(pos, 6.0, if is_active { Color32::WHITE } else { color });
        if is_active {
            painter.circle_stroke(pos, 9.0, Stroke::new(1.5, color));

            let label_offset = if pos.y - rect.top() < 40.0 {
                Vec2::new(0.0, 35.0)
            } else {
                Vec2::new(0.0, -35.0)
            };
            let align = if pos.y - rect.top() < 40.0 {
                Align2::CENTER_TOP
            } else {
                Align2::CENTER_BOTTOM
            };

            let info = format!(
                "{}\n{:.0}Hz\n{:.1}dB",
                label,
                band.freq.value(),
                band.gain.value()
            );
            painter.text(
                pos + label_offset,
                align,
                info,
                FontId::proportional(11.0),
                Color32::WHITE,
            );
        }
    }
}
