use crate::events::NoteEvents;
use egui::{self, Color32, Pos2, Rect, Stroke, Vec2, epaint::Hsva};
use std::f32::consts::{FRAC_PI_3, PI};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ZoneType {
    None,
    Segments3,
    Concentric,
}

struct PadDefinition {
    note: u8,
    key: egui::Key,
    radius: f32,
    angle_deg: f32,
    distance: f32,
    label: &'static str,
    zone_type: ZoneType,
}

pub struct DrumsPad {
    note: u8,
    key: egui::Key,
    radius: f32,
    position_config: (f32, f32),
    label: &'static str,
    last_hit_time: f64,
    last_hit_zone: Option<u8>,
    zone_type: ZoneType,
}

impl DrumsPad {
    fn new(d: PadDefinition) -> Self {
        Self {
            note: d.note,
            key: d.key,
            radius: d.radius,
            position_config: (d.angle_deg.to_radians(), d.distance),
            label: d.label,
            last_hit_time: -10.0,
            last_hit_zone: None,
            zone_type: d.zone_type,
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, center_offset: Pos2, events: &Arc<NoteEvents>) {
        let ctx = ui.ctx();
        let time: f64 = ctx.input(|i| i.time);

        let (angle, dist): (f32, f32) = self.position_config;
        let pos: Pos2 = center_offset + Vec2::new(angle.cos() * dist, angle.sin() * dist);

        let rect: Rect = Rect::from_center_size(pos, Vec2::splat(self.radius * 2.0));
        let response: egui::Response =
            ui.interact(rect, ui.id().with(self.note), egui::Sense::click());

        // キー入力を「消費」して取得（スペースキーなどがUI操作に奪われるのを防ぐ）
        let key_pressed = ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, self.key));

        // MIDI/外部入力によるフィードバックの確認
        let external_hit = self.poll_external_hit(events);

        if response.clicked() || key_pressed || external_hit.is_some() {
            self.last_hit_time = time;
            let mut target_note: u8 = self.note;
            let mut velocity: u8 = 100;
            let mut needs_trigger = true;

            if let Some((v, zone)) = external_hit {
                velocity = v;
                needs_trigger = false;
                // MIDIからの場合は既にトリガーされているので、視覚的な更新のみ
                if self.zone_type != ZoneType::None {
                    self.last_hit_zone = zone;
                }
            } else if response.clicked() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let local_vec: Vec2 = pointer_pos - pos;
                    let dist_sq_ratio: f32 = (local_vec.length() / self.radius).powi(2);
                    velocity = (127.0 - dist_sq_ratio * 80.0).clamp(40.0, 127.0) as u8;

                    match self.zone_type {
                        ZoneType::Segments3 => {
                            let hit_angle: f32 = local_vec.angle();
                            let zone: u8 = if hit_angle < -FRAC_PI_3 {
                                0
                            } else if hit_angle < FRAC_PI_3 {
                                1
                            } else {
                                2
                            };
                            self.last_hit_zone = Some(zone);
                            target_note = self.get_zone_note(zone);
                        }
                        ZoneType::Concentric => {
                            let dist: f32 = local_vec.length();
                            let zone: u8 = if dist < self.radius * 0.4 { 1 } else { 0 };
                            self.last_hit_zone = Some(zone);
                            target_note = self.get_zone_note(zone);
                        }
                        ZoneType::None => {
                            self.last_hit_zone = None;
                        }
                    }
                }
            } else if key_pressed {
                self.last_hit_zone = if self.zone_type != ZoneType::None {
                    Some(1)
                } else {
                    None
                };
            }

            if needs_trigger {
                events.trigger_by_note(target_note, velocity);
            }
        }

        let delta: f32 = (time - self.last_hit_time) as f32;
        let flash: f32 = (1.0 - delta * 4.0).max(0.0);
        let hue: f32 = ((pos.x * 0.0005) + (pos.y * 0.0005)).fract().abs();
        let base_color: Color32 = Color32::from(Hsva::new(hue, 0.7, 0.8, 1.0));
        let glow_color: Color32 = base_color.lerp_to_gamma(Color32::WHITE, flash);
        let painter = ui.painter();

        // 1. グロー
        painter.circle_filled(
            pos,
            self.radius * 1.15,
            base_color.gamma_multiply(0.15 * (1.0 + flash)),
        );

        // 2. ゾーン発光
        if self.zone_type != ZoneType::None
            && flash > 0.0
            && let Some(zone) = self.last_hit_zone
        {
            match self.zone_type {
                ZoneType::Segments3 => {
                    let s_ang: f32 = match zone {
                        0 => -PI,
                        1 => -FRAC_PI_3,
                        _ => FRAC_PI_3,
                    };
                    let e_ang: f32 = match zone {
                        0 => -FRAC_PI_3,
                        1 => FRAC_PI_3,
                        _ => PI,
                    };
                    let mut points = vec![pos];
                    for i in 0..=10 {
                        let a = s_ang + (e_ang - s_ang) * (i as f32 / 10.0);
                        points.push(pos + Vec2::new(a.cos(), a.sin()) * self.radius);
                    }
                    painter.add(egui::Shape::convex_polygon(
                        points,
                        Color32::WHITE.gamma_multiply(flash * 0.4),
                        Stroke::NONE,
                    ));
                }
                ZoneType::Concentric => {
                    if zone == 1 {
                        // ベル部分の発光
                        painter.circle_filled(
                            pos,
                            self.radius * 0.4,
                            Color32::WHITE.gamma_multiply(flash * 0.6),
                        );
                    } else {
                        // 本体部分の発光（ドーナツ状だが簡易的に円のストロークで表現）
                        painter.circle_stroke(
                            pos,
                            self.radius * 0.7,
                            Stroke::new(
                                self.radius * 0.6,
                                Color32::WHITE.gamma_multiply(flash * 0.3),
                            ),
                        );
                    }
                }
                _ => {}
            }
        }

        // 3. パッド本体
        painter.circle_filled(
            pos,
            self.radius,
            Color32::from_rgba_premultiplied(15, 15, 15, 210),
        );

        // ライドベルの円ライン描画
        if self.zone_type == ZoneType::Concentric {
            painter.circle_stroke(
                pos,
                self.radius * 0.4,
                Stroke::new(1.0, Color32::WHITE.gamma_multiply(0.2)),
            );
        }

        // 4. 外周
        painter.circle_stroke(pos, self.radius, Stroke::new(1.0 + flash * 2.0, glow_color));
        if flash > 0.0 {
            painter.circle_stroke(
                pos,
                self.radius + 1.0,
                Stroke::new(flash * 4.0, Color32::WHITE.gamma_multiply(flash)),
            );
        }

        // 5. 分割線の描画
        if self.zone_type == ZoneType::Segments3 {
            let split_angles: [f32; 3] = [-FRAC_PI_3, FRAC_PI_3, PI];
            for &a in &split_angles {
                painter.line_segment(
                    [pos, pos + Vec2::new(a.cos(), a.sin()) * self.radius],
                    Stroke::new(1.0, Color32::WHITE.gamma_multiply(0.2)),
                );
            }
        }

        let font_id = egui::FontId::proportional(self.radius * 0.25);
        let key_text = format!("{:?}", self.key).to_uppercase();

        // 楽器名
        painter.text(
            pos - Vec2::new(0.0, self.radius * 0.1),
            egui::Align2::CENTER_CENTER,
            self.label,
            font_id.clone(),
            if flash > 0.3 {
                Color32::WHITE
            } else {
                Color32::LIGHT_GRAY
            },
        );

        // 対応キー
        painter.text(
            pos + Vec2::new(0.0, self.radius * 0.35),
            egui::Align2::CENTER_CENTER,
            format!("[ {} ]", key_text),
            egui::FontId::proportional(self.radius * 0.2),
            base_color.gamma_multiply(0.8),
        );
        if flash > 0.0 {
            let ripple_radius = self.radius + (1.0 - flash) * self.radius * 0.5;
            let ripple_alpha = flash * 0.5;
            painter.circle_stroke(
                pos,
                ripple_radius,
                Stroke::new(1.0, glow_color.gamma_multiply(ripple_alpha)),
            );
        }

        // 減衰が終わったらゾーン情報をリセット
        if flash <= 0.0 {
            self.last_hit_zone = None;
        }
    }

    fn get_zone_note(&self, zone: u8) -> u8 {
        match self.note {
            38 => [37, 38, 40][zone as usize], // Snare
            42 => [44, 42, 46][zone as usize], // Hi-Hat
            51 => [51, 53][zone as usize],     // Ride: 0=Cymbal, 1=Bell
            n => n,
        }
    }

    fn poll_external_hit(&self, events: &NoteEvents) -> Option<(u8, Option<u8>)> {
        match self.note {
            36 => {
                let v = events.bass_drum.consume_visual();
                if v > 0 { Some((v, None)) } else { None }
            }
            38 => {
                let v_snare = events.snare_drum.consume_visual();
                let v_rim = events.rimshot.consume_visual();
                let v_side = events.sidestick.consume_visual();
                if v_snare > 0 {
                    Some((v_snare, Some(1)))
                } else if v_rim > 0 {
                    Some((v_rim, Some(2)))
                } else if v_side > 0 {
                    Some((v_side, Some(0)))
                } else {
                    None
                }
            }
            41 => {
                let v = events.tom_floor.consume_visual();
                if v > 0 { Some((v, None)) } else { None }
            }
            42 => {
                let v_closed = events.hihat_closed.consume_visual();
                let v_open = events.hihat_open.consume_visual();
                let v_pedal = events.hihat_pedal.consume_visual();
                if v_closed > 0 {
                    Some((v_closed, Some(1)))
                } else if v_open > 0 {
                    Some((v_open, Some(2)))
                } else if v_pedal > 0 {
                    Some((v_pedal, Some(0)))
                } else {
                    None
                }
            }
            48 => {
                let v = events.tom_high.consume_visual();
                if v > 0 { Some((v, None)) } else { None }
            }
            45 => {
                let v = events.tom_low.consume_visual();
                if v > 0 { Some((v, None)) } else { None }
            }
            49 => {
                let v = events.crash_cymbal.consume_visual();
                if v > 0 { Some((v, None)) } else { None }
            }
            51 => {
                let v_cymbal = events.ride_cymbal.consume_visual();
                let v_bell = events.ride_bell.consume_visual();
                if v_cymbal > 0 {
                    Some((v_cymbal, Some(0)))
                } else if v_bell > 0 {
                    Some((v_bell, Some(1)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub struct PadGrid {
    pub pads: Vec<DrumsPad>,
    events: Arc<NoteEvents>,
}

impl PadGrid {
    pub fn new(events: Arc<NoteEvents>) -> Self {
        let mut pads = Vec::new();
        let defs = vec![
            PadDefinition {
                note: 36,
                key: egui::Key::B,
                radius: 95.0,
                angle_deg: 90.0,
                distance: 100.0,
                label: "KICK",
                zone_type: ZoneType::None,
            },
            PadDefinition {
                note: 38,
                key: egui::Key::D,
                radius: 68.0,
                angle_deg: 165.0,
                distance: 205.0,
                label: "SNARE",
                zone_type: ZoneType::Segments3,
            },
            PadDefinition {
                note: 41,
                key: egui::Key::F,
                radius: 78.0,
                angle_deg: 15.0,
                distance: 225.0,
                label: "FLOOR",
                zone_type: ZoneType::None,
            },
            PadDefinition {
                note: 42,
                key: egui::Key::J,
                radius: 52.0,
                angle_deg: 190.0,
                distance: 290.0,
                label: "HI-HAT",
                zone_type: ZoneType::Segments3,
            },
            PadDefinition {
                note: 48,
                key: egui::Key::T,
                radius: 55.0,
                angle_deg: -125.0,
                distance: 110.0,
                label: "HI TOM",
                zone_type: ZoneType::None,
            },
            PadDefinition {
                note: 45,
                key: egui::Key::G,
                radius: 55.0,
                angle_deg: -55.0,
                distance: 110.0,
                label: "LOW TOM",
                zone_type: ZoneType::None,
            },
            PadDefinition {
                note: 49,
                key: egui::Key::Q,
                radius: 65.0,
                angle_deg: -135.0,
                distance: 300.0,
                label: "CRASH",
                zone_type: ZoneType::None,
            },
            PadDefinition {
                note: 51,
                key: egui::Key::I,
                radius: 82.0,
                angle_deg: -35.0,
                distance: 300.0,
                label: "RIDE",
                zone_type: ZoneType::Concentric,
            },
        ];

        for d in defs {
            pads.push(DrumsPad::new(d));
        }
        Self { pads, events }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let center_pos: Pos2 = ui.max_rect().center() + Vec2::new(0.0, 70.0);
        for pad in self.pads.iter_mut() {
            pad.draw(ui, center_pos, &self.events);
        }
    }
}
