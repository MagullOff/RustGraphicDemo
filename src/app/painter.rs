use super::GraphicDemo;
use crate::consts::*;
use egui::*;
impl GraphicDemo {
    pub fn paint(&mut self) -> egui::ColorImage {
        let mut map = ColorImage::new(
            [(IMAGE_SIZE + 1) as usize, (IMAGE_SIZE + 1) as usize],
            Color32::TRANSPARENT,
        );
        for x in 100..200 {
            for y in 100..200 {
                map[(x, y)] = Color32::RED;
            }
        }
        map
    }
}
