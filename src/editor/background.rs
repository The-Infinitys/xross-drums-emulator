pub mod flow;
pub mod gradient;
pub mod lines;
pub mod space;

pub struct Background {
    time: f32,
    gradient: gradient::Gradient,
    flow: flow::Flow,
    space: space::Space,
    lines: lines::Lines,
}

impl Background {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            gradient: gradient::Gradient::new(),
            flow: flow::Flow::new(),
            space: space::Space::new(),
            lines: lines::Lines::new(),
        }
    }

    pub fn draw(&mut self, ui: &egui::Ui) {
        let rect = ui.max_rect();
        let painter = ui.painter();

        // 1フレームあたりの進展量
        self.time += 0.01;

        // 背面から順に描画
        self.gradient.draw(painter, rect, self.time);
        self.flow.draw(painter, rect, self.time);
        self.space.draw(painter, rect, self.time);
        self.lines.draw(painter, rect);
        let overlay_opacity = 0.6;
        painter.rect_filled(
            rect,
            0.0,
            egui::Color32::from_black_alpha((255.0 * overlay_opacity) as u8),
        );
    }
}
