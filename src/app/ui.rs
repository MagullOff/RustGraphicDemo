use super::{ChosenCamera, FillingType, GraphicDemo};
use crate::consts::*;
use egui::*;
impl eframe::App for GraphicDemo {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
            chosen_camera,
            filling_type,
            light_rotation,
        } = self;
        ui.label("Camera rotation degree");
        ui.add(egui::Slider::new(light_rotation, 0f32..=359f32).text("camera rotation angle"));
        ui.separator();
        ui.label("Chose the camera");
        ui.radio_value(&mut *chosen_camera, ChosenCamera::Static, "Static");
        ui.radio_value(&mut *chosen_camera, ChosenCamera::Moving, "Moving");
        ui.radio_value(&mut *chosen_camera, ChosenCamera::Following, "Following");
        ui.separator();
        ui.label("Chose the filling type");
        ui.radio_value(&mut *filling_type, FillingType::Constant, "Constant");
        ui.radio_value(&mut *filling_type, FillingType::Gouraud, "Gouraud");
        ui.radio_value(&mut *filling_type, FillingType::Phong, "Phong");
    }
}