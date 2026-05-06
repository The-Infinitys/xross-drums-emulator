use egui::{
    Color32, Pos2, Rect,
    epaint::{Mesh, Shape, Vertex},
};
use std::f32::consts::TAU;

pub struct Gradient;

impl Gradient {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, painter: &egui::Painter, rect: Rect, time: f32) {
        // 1. ベース背景（JS: #010102）
        painter.rect_filled(rect, 0.0, Color32::from_rgb(1, 1, 2));

        let w = rect.width();
        let center = rect.center();

        // JS版の設定値
        const ORB_COUNT: usize = 12;
        const SPEED_FACTOR: f32 = 0.15;
        let t_flow = time * SPEED_FACTOR;

        for i in 0..ORB_COUNT {
            // --- ♾ 軌道計算 (JSと完全同期) ---
            let segment = (i as f32 / ORB_COUNT as f32) * TAU;
            let t = segment + t_flow;

            let infinity_scale = w * 0.35;
            let denom = 1.0 + t.sin().powi(2);
            let x = center.x + (infinity_scale * t.cos()) / denom;
            let y = center.y + (infinity_scale * t.sin() * t.cos()) / denom;

            // --- 色相の設定 (JSの V: 0.3, S: 1.0) ---
            let hue = (i as f32 / ORB_COUNT as f32 + t_flow * 0.05) % 1.0;
            let base_rgb = self.hsv_to_rgb(hue, 1.0, 0.3);

            // --- メッシュによる「極限ぼかし」描画 ---
            // 半径を大きくし、中心から外側へアルファを非線形に落とすことでエッジを消す
            let glow_radius = w * 0.5;
            self.draw_soft_glow_mesh(painter, Pos2::new(x, y), glow_radius, base_rgb);
        }
    }

    /// 滑らかな発光感を作るメッシュ描画
    fn draw_soft_glow_mesh(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        color: Color32,
    ) {
        let mut mesh = Mesh::default();
        let n_points = 32; // 円の分割数

        // JS版の stop 0 (0.5) を再現
        // 加算合成（lighter）っぽく見せるため premultiplied alpha を使用
        let center_color = Color32::from_rgba_premultiplied(
            (color.r() as f32 * 0.5) as u8,
            (color.g() as f32 * 0.5) as u8,
            (color.b() as f32 * 0.5) as u8,
            128, // アルファ 0.5
        );

        let center_idx = mesh.vertices.len() as u32;
        mesh.vertices.push(Vertex {
            pos: center,
            uv: Pos2::ZERO,
            color: center_color,
        });

        for k in 0..n_points {
            let angle = k as f32 * TAU / n_points as f32;
            let unit_vec = egui::vec2(angle.cos(), angle.sin());

            // 外周に向かって透明にする
            // ※ eguiの頂点カラー補間は線形なので、
            // 輪郭が気になる場合はここに中間地点の頂点を挟むとより滑らかになります
            mesh.vertices.push(Vertex {
                pos: center + unit_vec * radius,
                uv: Pos2::ZERO,
                color: Color32::TRANSPARENT,
            });

            mesh.indices.push(center_idx);
            mesh.indices.push(center_idx + 1 + k);
            mesh.indices.push(center_idx + 1 + (k + 1) % n_points);
        }

        painter.add(Shape::mesh(mesh));
    }

    fn hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> Color32 {
        let f = |n: f32| {
            let k = (n + h * 6.0) % 6.0;
            v - v * s * 0.0f32.max(1.0f32.min(k.min(4.0 - k)))
        };
        Color32::from_rgb(
            (f(5.0) * 255.0) as u8,
            (f(3.0) * 255.0) as u8,
            (f(1.0) * 255.0) as u8,
        )
    }
}
