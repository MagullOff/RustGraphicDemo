use super::GraphicDemo;
use crate::consts::*;
use egui::*;
impl GraphicDemo {
    pub fn paint(&mut self) -> egui::ColorImage {
        let mut map = ColorImage::new(
            [(IMAGE_SIZE + 1) as usize, (IMAGE_SIZE + 1) as usize],
            Color32::TRANSPARENT,
        );
        for x in 0..IMAGE_SIZE {
            [(x, 0), (0, x), (IMAGE_SIZE, x), (x, IMAGE_SIZE)]
                .into_iter()
                .for_each(|index| map[index] = Color32::WHITE);
        }
        map
    }
}
