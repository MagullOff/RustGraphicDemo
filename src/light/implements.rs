use super::Light;
use egui::Color32;
use nalgebra::Point3;
impl Light {
    pub fn get_position(&self) -> Point3<f32> {
        self.position
    }
    pub fn set_position(&mut self, position: Point3<f32>) -> &mut Self {
        self.position = position;
        self
    }
    pub fn set_target(&mut self, target: Point3<f32>) -> &mut Self {
        self.direction = Some(target - self.position);
        self
    }
    pub fn delete_target(&mut self) -> &mut Self {
        self.direction = None;
        self
    }
    pub fn get_color(&self) -> Color32 {
        self.color
    }
    pub fn set_color(&mut self, color: Color32) -> &mut Self {
        self.color = color;
        self
    }
}
