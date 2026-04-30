use egui::{
    epaint::{Mesh, Vertex},
    Color32, Pos2, Rect, Shape,
};

pub struct Lines;

impl Lines {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, painter: &egui::Painter, rect: Rect) {
        let h = rect.height();
        let margin = h * 0.05;
        let stroke_w = 3.0;

        self.render_set(painter, rect, margin, stroke_w, false);
        self.render_set(painter, rect, margin, stroke_w, true);
    }

    fn render_set(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        margin: f32,
        stroke_w: f32,
        flipped: bool,
    ) {
        let w = rect.width();
        let y_diff = margin * 3.0;
        let (y_base, y_target, y_mid) = if !flipped {
            (
                rect.bottom(),
                rect.bottom() - y_diff,
                rect.bottom() - margin * 2.0,
            )
        } else {
            (rect.top(), rect.top() + y_diff, rect.top() + margin * 2.0)
        };

        // 45度を維持するため、Xの移動量をYの変化量と等しくする
        let x_bend_width = y_diff;
        let bend_t = x_bend_width / w;

        // 指定されたカラーストップ (0%, 20%, 40%, 60%, 80%, 100%)
        let color_stops = [
            (0.0, Color32::from_rgb(255, 0, 0)),   // #f00
            (0.2, Color32::from_rgb(255, 255, 0)), // #ff0
            (0.4, Color32::from_rgb(0, 255, 0)),   // #0f0
            (0.6, Color32::from_rgb(0, 255, 255)), // #0ff
            (0.8, Color32::from_rgb(0, 0, 255)),   // #00f
            (1.0, Color32::from_rgb(255, 0, 255)), // #f0f
        ];

        let get_color = |x: f32, alpha: u8| {
            let t = ((x - rect.left()) / w).clamp(0.0, 1.0);
            let mut final_color = color_stops[0].1;
            for i in 0..color_stops.len() - 1 {
                let (t1, c1) = color_stops[i];
                let (t2, c2) = color_stops[i + 1];
                if t >= t1 && t <= t2 {
                    let local_t = (t - t1) / (t2 - t1);
                    final_color = self.lerp_color(c1, c2, local_t);
                    break;
                }
            }
            final_color.linear_multiply(alpha as f32 / 255.0)
        };

        // 頂点を生成: カラーストップと「曲がり角」の両方を網羅する
        let mut t_points = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        if !flipped {
            t_points.push(bend_t); // 下側の曲がり角
        } else {
            t_points.push(1.0 - bend_t); // 上側の曲がり角 (右から45度)
        }
        t_points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        t_points.dedup();

        // --- A. 折れ曲がり線 ---
        let mut points = Vec::new();
        for &t in &t_points {
            let x = rect.left() + w * t;
            let y = if !flipped {
                // 左から右へ、最初は45度で上がり、その後水平
                if t <= bend_t {
                    y_base + (y_target - y_base) * (t / bend_t)
                } else {
                    y_target
                }
            } else {
                // 右から左へ、最初は45度で下がり(or上がり)、その後水平
                let rev_t = 1.0 - t; // 右端からの距離
                if rev_t <= bend_t {
                    y_base + (y_target - y_base) * (rev_t / bend_t)
                } else {
                    y_target
                }
            };
            points.push(Pos2::new(x, y));
        }

        self.draw_mesh_strip(painter, &points, stroke_w, &get_color, 255);
        self.draw_mesh_strip(painter, &points, stroke_w * 2.0, &get_color, 60);

        // --- B. 中央の水平線 ---
        let mid_points: Vec<Pos2> = t_points
            .iter()
            .map(|&t| Pos2::new(rect.left() + w * t, y_mid))
            .collect();
        self.draw_mesh_strip(painter, &mid_points, stroke_w, &get_color, 255);
    }

    fn lerp_color(&self, c1: Color32, c2: Color32, t: f32) -> Color32 {
        Color32::from_rgb(
            (c1.r() as f32 + (c2.r() as f32 - c1.r() as f32) * t) as u8,
            (c1.g() as f32 + (c2.g() as f32 - c1.g() as f32) * t) as u8,
            (c1.b() as f32 + (c2.b() as f32 - c1.b() as f32) * t) as u8,
        )
    }

    fn draw_mesh_strip(
        &self,
        painter: &egui::Painter,
        points: &[Pos2],
        width: f32,
        color_at: &dyn Fn(f32, u8) -> Color32,
        alpha: u8,
    ) {
        if points.len() < 2 {
            return;
        }
        let mut mesh = Mesh::default();
        let h_w = width / 2.0;
        for p in points {
            let c = color_at(p.x, alpha);
            let idx = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                pos: Pos2::new(p.x, p.y - h_w),
                uv: Pos2::ZERO,
                color: c,
            });
            mesh.vertices.push(Vertex {
                pos: Pos2::new(p.x, p.y + h_w),
                uv: Pos2::ZERO,
                color: c,
            });
            if idx >= 2 {
                mesh.indices
                    .extend_from_slice(&[idx - 2, idx - 1, idx, idx - 1, idx, idx + 1]);
            }
        }
        painter.add(Shape::mesh(mesh));
    }
}
