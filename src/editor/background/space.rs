use egui::{Color32, Pos2, Rect};
use rand::prelude::*;
use rand::rngs::StdRng;
use std::f32::consts::TAU;

pub struct Space {
    stars: Vec<Star>,
}

struct Star {
    index: usize,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
    base_color: Color32,
    size: f32,
}

// --- 設定値 (JS版と同期) ---
const STAR_COUNT: usize = 5000;
const SPEED: f32 = 0.005;
const INFINITY_SCALE: f32 = 380.0;
const PERSPECTIVE: f32 = 500.0;

impl Star {
    fn new(index: usize, rng: &mut StdRng) -> Self {
        // 太さを極限まで絞るための分布 (Math.pow(rand, 4))
        let dist_factor: f32 = rng.random_range(0.0..1.0f32).powi(4);
        let angle: f32 = rng.random_range(0.0..TAU);

        let colors = [
            Color32::from_rgb(140, 190, 255), // 青白
            Color32::from_rgb(210, 160, 255), // 薄紫
            Color32::from_rgb(255, 255, 255), // 白
        ];

        Self {
            index,
            offset_x: angle.cos() * dist_factor * 30.0,
            offset_y: angle.sin() * dist_factor * 30.0,
            offset_z: (rng.random_range(0.0..1.0f32) - 0.5) * 30.0,
            base_color: colors[index % colors.len()],
            size: rng.random_range(0.3..1.0),
        }
    }

    fn draw(&self, painter: &egui::Painter, center: Pos2, time: f32, rect: Rect) {
        // --- 途切れをなくすための位相計算 ---
        let segment = (self.index as f32 / STAR_COUNT as f32) * TAU;
        let t = segment + time;

        // レムニスケート軌道
        let denom = 1.0 + t.sin().powi(2);
        let base_x = (INFINITY_SCALE * t.cos()) / denom;
        let base_y = (INFINITY_SCALE * t.sin() * t.cos()) / denom;
        let base_z = (t * 2.0).sin() * 150.0;

        let x = base_x + self.offset_x;
        let y = base_y + self.offset_y;
        let z = base_z + self.offset_z + 300.0;

        // 3D投影
        let scale = PERSPECTIVE / (PERSPECTIVE + z);
        let px = x * scale + center.x;
        let py = y * scale + center.y;

        // 画面外ならスキップ
        if !rect.contains(Pos2::new(px, py)) {
            return;
        }

        // 距離によるフェードアウト
        let alpha_factor = (1.0 - z / 1300.0).max(0.0);
        if alpha_factor <= 0.0 {
            return;
        }

        let color = Color32::from_rgba_premultiplied(
            (self.base_color.r() as f32 * alpha_factor) as u8,
            (self.base_color.g() as f32 * alpha_factor) as u8,
            (self.base_color.b() as f32 * alpha_factor) as u8,
            (alpha_factor * 0.7 * 255.0) as u8,
        );

        let r = self.size * scale * 1.5;

        // 5000個あるため、最も軽量な描画メソッドを使用
        painter.circle_filled(Pos2::new(px, py), r, color);
    }
}

impl Space {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(42);
        let mut stars = Vec::with_capacity(STAR_COUNT);

        for i in 0..STAR_COUNT {
            stars.push(Star::new(i, &mut rng));
        }

        Self { stars }
    }

    pub fn draw(&mut self, painter: &egui::Painter, rect: Rect, time: f32) {
        let center = rect.center();
        let current_time = time * SPEED * 60.0; // JSのスピード感に合わせる調整

        // 5000個の星を順次描画
        // Zソートは描画負荷の関係で省略していますが、加算合成(premultiplied)なので
        // 重なり順がなくても不自然にはなりにくいです。
        for star in &self.stars {
            star.draw(painter, center, current_time, rect);
        }
    }
}
