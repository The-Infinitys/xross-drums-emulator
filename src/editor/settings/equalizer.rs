use crate::params::equalizer::EqualizerParams;
use crate::utils::{FilterType, freq_to_norm, get_filter_gain, norm_to_freq};
use egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use truce::params::FloatParam;

pub struct EqualizerBox;

impl EqualizerBox {
    pub fn draw(ui: &mut egui::Ui, eq: &EqualizerParams) {
        let label_w = 35.0;
        let bottom_h = 20.0;
        let available_rect = ui.available_rect_before_wrap();
        let desired_size = egui::vec2(available_rect.width().max(300.0), 200.0);
        let (full_rect, _) = ui.allocate_at_least(desired_size, egui::Sense::hover());

        // デザイン: 背景とグラデーション
        ui.painter()
            .rect_filled(full_rect, 6.0, Color32::from_black_alpha(250));

        let graph_rect = Rect::from_min_max(
            full_rect.min + egui::vec2(label_w, 20.0),
            full_rect.max - egui::vec2(15.0, bottom_h),
        );

        let sample_rate = 44100.0;
        Self::draw_grid(ui, graph_rect);

        // 1. 特性曲線の描画
        let steps = (graph_rect.width() as usize / 2).max(128);
        let points: Vec<Pos2> = (0..=steps)
            .map(|i| {
                let x_norm = i as f32 / steps as f32;
                let f = norm_to_freq(x_norm);
                let mut g = 0.0;
                g += get_filter_gain(
                    f,
                    eq.hp.freq.value(),
                    0.0,
                    eq.hp.q.value(),
                    FilterType::HighPass,
                    sample_rate,
                );
                g += get_filter_gain(
                    f,
                    eq.low.freq.value(),
                    eq.low.gain.value(),
                    eq.low.q.value(),
                    FilterType::LowShelf,
                    sample_rate,
                );
                g += get_filter_gain(
                    f,
                    eq.mid.freq.value(),
                    eq.mid.gain.value(),
                    eq.mid.q.value(),
                    FilterType::Peaking,
                    sample_rate,
                );
                g += get_filter_gain(
                    f,
                    eq.high.freq.value(),
                    eq.high.gain.value(),
                    eq.high.q.value(),
                    FilterType::HighShelf,
                    sample_rate,
                );
                g += get_filter_gain(
                    f,
                    eq.lp.freq.value(),
                    0.0,
                    eq.lp.q.value(),
                    FilterType::LowPass,
                    sample_rate,
                );

                let y_norm = 1.0 - (g.clamp(-24.0, 24.0) + 24.0) / 48.0;
                graph_rect.min
                    + egui::vec2(x_norm * graph_rect.width(), y_norm * graph_rect.height())
            })
            .collect();
        ui.painter().add(egui::Shape::line(
            points,
            Stroke::new(1.0, Color32::from_rgb(0, 255, 255)),
        ));
        // 2. ハンドルの描画
        // IDを変えることでHP/LPも個別に垂直位置を記憶させます（Persistent Id用）
        Self::draw_band(
            ui,
            graph_rect,
            &eq.hp.freq,
            &eq.hp.gain,
            &eq.hp.q,
            Color32::from_rgb(255, 100, 100),
            "HPF",
        );
        Self::draw_band(
            ui,
            graph_rect,
            &eq.low.freq,
            &eq.low.gain,
            &eq.low.q,
            Color32::from_rgb(255, 165, 0),
            "LOW",
        );
        Self::draw_band(
            ui,
            graph_rect,
            &eq.mid.freq,
            &eq.mid.gain,
            &eq.mid.q,
            Color32::from_rgb(0, 255, 127),
            "MID",
        );
        Self::draw_band(
            ui,
            graph_rect,
            &eq.high.freq,
            &eq.high.gain,
            &eq.high.q,
            Color32::from_rgb(180, 100, 255),
            "HIGH",
        );
        Self::draw_band(
            ui,
            graph_rect,
            &eq.lp.freq,
            &eq.lp.gain,
            &eq.lp.q,
            Color32::from_rgb(100, 150, 255),
            "LPF",
        );
    }

    fn draw_grid(ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();
        let font = FontId::proportional(10.0);
        let stroke = Stroke::new(0.5, Color32::from_gray(60));

        for db in [-20, -16, -12, -8, -4, 0, 4, 8, 12, 16, 20] {
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
        for hz in [20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000] {
            let min_hz = 20.0;
            let max_hz = 20000.0;

            // 対数スケールでの位置計算 (0.0 ~ 1.0)
            let t = (f32::log10(hz as f32) - f32::log10(min_hz))
                / (f32::log10(max_hz) - f32::log10(min_hz));

            // 実際のスクリーン座標に変換
            let x = rect.left() + t * rect.width();

            // 垂直線の描画
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                stroke,
            );

            // ラベルの描画 (1000Hz以上はk表記にするとスッキリします)
            let label = if hz >= 1000 {
                format!("{}k", hz / 1000)
            } else {
                format!("{hz}")
            };

            painter.text(
                Pos2::new(x, rect.bottom() + 5.0),
                Align2::CENTER_TOP,
                label,
                font.clone(),
                Color32::GRAY,
            );
        }
    }

    fn draw_band(
        ui: &mut egui::Ui,
        rect: Rect,
        freq: &FloatParam,
        gain: &FloatParam,
        q: &FloatParam,
        color: Color32,
        label: &str,
    ) {
        let f_val = freq.value();

        // HP/LP の場合は「仮の垂直位置」を保存して動かせるようにする
        let y_id = ui.make_persistent_id(label).with("y_pos");
        let mut y_norm_custom = ui.data_mut(|d| *d.get_temp_mut_or_insert_with(y_id, || 0.5f32));

        let current_y_norm = 1.0 - (gain.value() + 24.0) / 48.0;

        let pos = Pos2::new(
            rect.left() + freq_to_norm(f_val) * rect.width(),
            rect.top() + current_y_norm * rect.height(),
        );

        let id = ui.make_persistent_id(label);
        let resp = ui.interact(
            Rect::from_center_size(pos, Vec2::splat(18.0)),
            id,
            egui::Sense::click_and_drag(),
        );

        // --- 操作ロジック ---

        // ダブルクリックでリセット
        if resp.double_clicked() {
            freq.set_value(freq.info.default_plain);
            gain.set_value(gain.info.default_plain);
            q.set_value(q.info.default_plain);
        }

        // ドラッグ操作
        if resp.hovered() {
            let delta = resp.drag_delta();
            let freq_range = 20000.0f32.ln() - 20.0f32.ln();
            let new_f = (f_val.ln() + (delta.x / rect.width()) * freq_range).exp();
            freq.set_value(new_f.clamp(20.0, 20000.0) as f64);

            let current_g = gain.value();
            let new_g = (current_g - (delta.y / rect.height()) * 48.0).clamp(-24.0, 24.0);
            gain.set_value(new_g as f64);

            // HP/LP の垂直移動
            y_norm_custom = (y_norm_custom + delta.y / rect.height()).clamp(0.1, 0.9);
            // センター付近でスナップ
            if (y_norm_custom - 0.5).abs() < 0.02 {
                y_norm_custom = 0.5;
            }
            ui.data_mut(|d| d.insert_temp(y_id, y_norm_custom));
        }

        // スクロールでQ値変更
        if resp.hovered() {
            let scroll = ui.input_mut(|i| {
                let delta = i.smooth_scroll_delta.y;
                i.smooth_scroll_delta = egui::Vec2::ZERO;
                delta
            });
            if scroll != 0.0 {
                let new_q = (q.value() + scroll / 200.0).clamp(0.1, 10.0);
                q.set_value(new_q as f64);
            }
        }

        // --- 描画 ---
        let painter = ui.painter();
        let is_active = resp.hovered() || resp.dragged();
        let display_color = if is_active { Color32::WHITE } else { color };

        // Qのガイド円
        let q_radius = ((rect.width() * 0.08) / q.value().sqrt()).clamp(8.0, rect.width() / 4.0);
        painter.with_clip_rect(rect).circle_stroke(
            pos,
            q_radius,
            Stroke::new(1.0, color.linear_multiply(0.2)),
        );

        // ハンドル外枠
        painter.circle_filled(pos, 8.0, Color32::from_black_alpha(150));
        painter.circle_stroke(pos, 6.0, Stroke::new(2.0, display_color));

        if is_active {
            let info_txt = format!("{label}\n{:.0}Hz / {:.1}dB", freq.value(), gain.value());

            // 値のテキスト描画（背景付き）
            let text_pos = pos - Vec2::new(0.0, 28.0);
            painter.text(
                text_pos,
                Align2::CENTER_BOTTOM,
                info_txt,
                FontId::proportional(12.0),
                Color32::WHITE,
            );
        }

        // 数値設定ウィンドウ（右クリックまたはクリックでトグル）
        let popup_id = id.with("popup");
        if resp.clicked() {
            ui.memory_mut(|m| m.toggle_popup(popup_id));
        }

        if ui.memory(|m| m.is_popup_open(popup_id)) {
            egui::Window::new(label)
                .id(popup_id.with("window"))
                .fixed_pos(pos + Vec2::new(15.0, 15.0))
                .collapsible(false)
                .resizable(false)
                .frame(egui::Frame::window(ui.style()).fill(Color32::from_gray(30)))
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Freq:");
                        let mut f = freq.value();
                        if ui
                            .add(
                                egui::DragValue::new(&mut f)
                                    .suffix("Hz")
                                    .range(20.0..=20000.0),
                            )
                            .changed()
                        {
                            freq.set_value(f.into());
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Gain:");
                        let mut g = gain.value();
                        if ui
                            .add(
                                egui::DragValue::new(&mut g)
                                    .suffix("dB")
                                    .range(-24.0..=24.0),
                            )
                            .changed()
                        {
                            gain.set_value(g.into());
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Q:");
                        let mut q_v = q.value();
                        if ui
                            .add(egui::DragValue::new(&mut q_v).range(0.1..=10.0))
                            .changed()
                        {
                            q.set_value(q_v.into());
                        }
                    });
                });
        }
    }
}
