use egui::{Color32, Pos2, Rect, Stroke};
use rand::prelude::*;
use std::f32::consts::PI;

pub struct Flow {
    objects: Vec<Polyhedron>,
    rng: StdRng,
}

struct Polyhedron {
    index: usize,
    x: f32,
    y: f32,
    z: f32,
    size: f32,
    rot_x: f32,
    rot_y: f32,
    v_rot_x: f32,
    v_rot_y: f32,
    current_scale: f32,
}

const CAMERA_Z: f32 = 800.0;
const MAX_Z: f32 = 3500.0;
const OBJ_COUNT: usize = 40;

const VERTICES: [[f32; 3]; 6] = [
    [0.0, 1.0, 0.0],
    [0.0, -1.0, 0.0],
    [1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, -1.0],
];

const EDGES: [[usize; 2]; 12] = [
    [0, 2],
    [0, 3],
    [0, 4],
    [0, 5],
    [1, 2],
    [1, 3],
    [1, 4],
    [1, 5],
    [2, 4],
    [4, 3],
    [3, 5],
    [5, 2],
];

impl Polyhedron {
    fn new(index: usize, w: f32, h: f32, first_time: bool, rng: &mut StdRng) -> Self {
        let mut p = Self {
            index,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            size: 0.0,
            rot_x: rng.random_range(0.0..PI),
            rot_y: rng.random_range(0.0..PI),
            v_rot_x: rng.random_range(-0.005..0.005),
            v_rot_y: rng.random_range(-0.005..0.005),
            current_scale: 0.0,
        };
        p.init(w, h, first_time, rng);
        p
    }

    fn init(&mut self, w: f32, h: f32, first_time: bool, rng: &mut StdRng) {
        let is_core = self.index % 3 == 0;
        let screen_min = w.min(h);

        if is_core {
            self.x = rng.random_range(-0.5..0.5) * w * 1.5;
            self.y = rng.random_range(-0.5..0.5) * h * 1.5;
            self.size = rng.random_range(0.04..0.12) * screen_min;
        } else {
            self.x = rng.random_range(-0.5..0.5) * w;
            self.y = rng.random_range(-0.5..0.5) * h;
            self.size = rng.random_range(0.01..0.04) * screen_min;
        }

        // first_timeなら画面全体に散らし、リスポーン時は奥(MAX_Z)から
        self.z = if first_time {
            (self.index as f32 / OBJ_COUNT as f32) * MAX_Z
        } else {
            MAX_Z
        };
    }

    fn update(&mut self, w: f32, h: f32, rng: &mut StdRng) {
        let speed_base = if self.index % 3 == 0 { 1.2 } else { 0.6 };
        self.z -= speed_base;
        self.rot_x += self.v_rot_x;
        self.rot_y += self.v_rot_y;

        // 手前に来すぎたら奥へリスポーン
        if self.z < -400.0 {
            self.init(w, h, false, rng);
        }

        self.current_scale = CAMERA_Z / (CAMERA_Z + (-CAMERA_Z + 1.0).max(self.z));
    }

    fn project(&self, vertex: [f32; 3], center: Pos2) -> Pos2 {
        let [mut x, mut y, mut z] = vertex;

        let (s_x, c_x) = self.rot_x.sin_cos();
        let ny = y * c_x - z * s_x;
        let nz = y * s_x + z * c_x;
        y = ny;
        z = nz;

        let (s_y, c_y) = self.rot_y.sin_cos();
        let nx = x * c_y + z * s_y;
        x = nx;

        Pos2::new(
            (x * self.size + self.x) * self.current_scale + center.x,
            (y * self.size + self.y) * self.current_scale + center.y,
        )
    }
}

impl Flow {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            rng: StdRng::seed_from_u64(0),
        }
    }
    pub fn draw(&mut self, painter: &egui::Painter, rect: Rect, _time: f32) {
        let center = rect.center();
        let (w, h) = (rect.width(), rect.height());

        if self.objects.is_empty() {
            for i in 0..OBJ_COUNT {
                self.objects
                    .push(Polyhedron::new(i, w, h, true, &mut self.rng));
            }
        }

        for obj in &mut self.objects {
            obj.update(w, h, &mut self.rng);

            // --- 改良版：滑らかなフェードイン・アウト ---
            // z = MAX_Z (3500) で alpha = 0
            // z = 800 (中距離) で alpha = 最大
            // z = -400 (消失点) で alpha = 0

            let alpha = if obj.z > 800.0 {
                // 奥から中距離へのフェードイン
                let t = (MAX_Z - obj.z) / (MAX_Z - 800.0);
                (t * PI * 0.5).sin() * 0.1 // サインカーブで滑らかに
            } else {
                // 中距離から手前へのフェードアウト
                // -400に近づくほど 0 になるように設計
                let t = (obj.z + 400.0) / (800.0 + 400.0);
                let t = t.clamp(0.0, 1.0);
                (t * PI * 0.5).sin() * 0.1
            };

            if alpha <= 0.001 {
                continue;
            }

            // 色相に少し変化をつけるとより綺麗です (任意)
            let color = Color32::from_rgba_premultiplied(180, 210, 255, (alpha * 255.0) as u8);

            // 線の太さも current_scale に応じて滑らかに変化
            let thickness = (0.8 * obj.current_scale).clamp(0.2, 1.5);
            let stroke = Stroke::new(thickness, color);

            for edge in &EDGES {
                let p1 = obj.project(VERTICES[edge[0]], center);
                let p2 = obj.project(VERTICES[edge[1]], center);

                // 画面外クリッピング
                if rect.contains(p1) || rect.contains(p2) {
                    painter.line_segment([p1, p2], stroke);
                }
            }
        }
    }
}
