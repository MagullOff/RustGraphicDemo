use super::{FillingType, GraphicDemo, LightParameters};
use crate::camera::CameraKind;
use crate::consts::*;
use crate::traits::movable::Movable;
use egui::*;
use nalgebra::{Matrix4, Point3, Vector3};
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
        use crate::shape::*;
        use crate::utils::file_load::load_polygons;
        println!(
            "{:?}",
            Shape::new(
                load_polygons("assets/cube.obj"),
                [0, 0, 0],
                ShapeMovementType::Orbital,
            ),
        );
        Default::default()
    }
    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            filling_type,
            light_rotation,
            light_parameters,
            animation,
            camera,
            ..
        } = self;
        let LightParameters { m, kd, ks } = light_parameters;
        ui.checkbox(&mut *animation, "Enable animation");
        ui.label("Camera rotation degree");
        ui.add(egui::Slider::new(light_rotation, 0f32..=360f32).text("camera rotation angle"));
        ui.separator();
        ui.label("Chose the camera");
        ui.radio_value(&mut camera.kind, CameraKind::Static, "Static");
        ui.radio_value(&mut camera.kind, CameraKind::Moving, "Moving");
        ui.radio_value(&mut camera.kind, CameraKind::Following, "Following");
        ui.separator();
        ui.label("Chose the filling type");
        ui.radio_value(&mut *filling_type, FillingType::Constant, "Constant");
        ui.radio_value(&mut *filling_type, FillingType::Gouraud, "Gouraud");
        ui.radio_value(&mut *filling_type, FillingType::Phong, "Phong");
        ui.separator();
        ui.label("Light parameters");
        ui.add(egui::Slider::new(m, 1f32..=MAX_M).text("m"));
        ui.add(egui::Slider::new(kd, 0.001..=MAX_KD).text("kd"));
        ui.add(egui::Slider::new(ks, 0.001..=MAX_KS).text("ks"));
        ui.label("Light parameters");
        ui.add(egui::Slider::new(&mut self.camera.position[0], -10..=10).text("x"));
        ui.add(egui::Slider::new(&mut self.camera.position[1], -10..=10).text("y"));
        ui.add(egui::Slider::new(&mut self.camera.position[2], -100..=100).text("z"));
        self.camera.matrix = Matrix4::look_at_rh(
            &Point3::new(
                self.camera.position[0] as f32,
                self.camera.position[1] as f32,
                self.camera.position[2] as f32,
            ),
            &Point3::new(
                STATIC_CAMERA_TARGET[0] as f32,
                STATIC_CAMERA_TARGET[1] as f32,
                STATIC_CAMERA_TARGET[2] as f32,
            ),
            &Vector3::from_vec(UP_VECTOR.map(|a| a as f32).to_vec()),
        )
    }
}
