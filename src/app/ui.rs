use super::{GraphicDemo, LightParameters, ShadingType};
use crate::camera::CameraKind;
use crate::consts::*;
use crate::traits::movable::Movable;
use egui::*;
impl eframe::App for GraphicDemo {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.animation {
                use chrono::Timelike;
                let time = chrono::Local::now().time();
                let sec_since_midnight =
                    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64);
                for shape in &mut self.shapes {
                    shape.update(sec_since_midnight as f32);
                }
                self.camera.update(sec_since_midnight as f32);
                self.lights
                    .iter_mut()
                    .for_each(|l| l.update(sec_since_midnight as f32));
                ui.ctx().request_repaint();
            }
            let bitmap = self.paint();
            let window_size = ui.available_size();
            let mut img_ui = ui.child_ui(
                Rect {
                    min: pos2(
                        window_size.x / 2.0 - IMAGE_SIZE as f32 / 2.0,
                        window_size.y / 2.0 - IMAGE_SIZE as f32 / 2.0,
                    ),
                    max: pos2(
                        window_size.x / 2.0 + IMAGE_SIZE as f32 / 2.0,
                        window_size.y / 2.0 + IMAGE_SIZE as f32 / 2.0,
                    ),
                },
                egui::Layout::left_to_right(egui::Align::RIGHT),
            );

            let texture = &img_ui
                .ctx()
                .load_texture("sphere", bitmap, egui::TextureFilter::Linear);
            img_ui.image(texture, texture.size_vec2());
            Frame::popup(ui.style())
                .stroke(Stroke::none())
                .show(ui, |ui| {
                    ui.set_max_width(270.0);
                    CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
                })
        });
    }
}

impl GraphicDemo {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            shading_type,
            fov,
            light_parameters,
            animation,
            camera,
            fog,
            fog_color,
            light_force,
            ..
        } = self;
        let LightParameters { m, kd, ks } = light_parameters;
        ui.checkbox(&mut *animation, "Enable animation");
        ui.label("Camera rotation degree");
        ui.add(egui::Slider::new(fov, 30f32..=120f32).text("field of view"));
        ui.separator();
        ui.label("Chose the camera");
        ui.radio_value(&mut camera.kind, CameraKind::Static, "Static");
        ui.radio_value(&mut camera.kind, CameraKind::Moving, "Moving");
        ui.radio_value(&mut camera.kind, CameraKind::Following, "Following");
        ui.separator();
        ui.label("Chose the filling type");
        ui.radio_value(&mut *shading_type, ShadingType::Constant, "Constant");
        ui.radio_value(&mut *shading_type, ShadingType::Gouraud, "Gouraud");
        ui.radio_value(&mut *shading_type, ShadingType::Phong, "Phong");
        ui.separator();
        ui.label("Light parameters");
        ui.add(egui::Slider::new(m, 1f32..=MAX_M).text("m"));
        ui.add(egui::Slider::new(kd, 0.001..=MAX_KD).text("kd"));
        ui.add(egui::Slider::new(ks, 0.001..=MAX_KS).text("ks"));
        ui.separator();
        ui.label("Fog parameters");
        ui.checkbox(&mut *fog, "Enable fog");
        ui.color_edit_button_rgb(&mut *fog_color);
        ui.separator();
        ui.label("Night and day");
        ui.add(egui::Slider::new(light_force, 0.001..=1.0).text("Light force"));
        ui.separator();
        ui.label("moving options");
        ui.checkbox(&mut self.shapes[4].vibration, "Enable vibration");
        ui.add(
            egui::Slider::new(&mut self.lights[2].rotation_angle, -1.0..=1.0)
                .text("Light rotation"),
        );

        let current_color = Color32::WHITE;
        self.lights[0].set_color(Color32::from_rgb(
            (current_color.r() as f32 * self.light_force) as u8,
            (current_color.g() as f32 * self.light_force) as u8,
            (current_color.b() as f32 * self.light_force) as u8,
        ));
        self.lights[1].set_color(Color32::from_rgb(
            (current_color.r() as f32 * self.light_force) as u8,
            (current_color.g() as f32 * self.light_force) as u8,
            (current_color.b() as f32 * self.light_force) as u8,
        ));
    }
}
