use crate::consts::IMAGE_SIZE;
use crate::GraphicDemo;
use egui::{Color32, ColorImage};
use itertools::Itertools;

impl GraphicDemo {
    pub fn apply_fog(&self, zbuffor: Vec<Vec<f32>>, map: &mut ColorImage) {
        let max_x = 2.2;
        for x in 0..IMAGE_SIZE {
            for y in 0..IMAGE_SIZE {
                let fog_factor = ((zbuffor[x][y] - IMAGE_SIZE as f32 + max_x) / (max_x / 1.15))
                    .min(1.0)
                    .max(0.0);
                let fog_color = self.fog_color.iter().map(|v| v * 255.0).collect_vec();
                let color = map[(x, y)];
                map[(x, y)] = Color32::from_rgb(
                    (((fog_color[0] - (color.r() as f32)) * fog_factor) + color.r() as f32) as u8,
                    (((fog_color[1] - (color.g() as f32)) * fog_factor) + color.g() as f32) as u8,
                    (((fog_color[2] - (color.b() as f32)) * fog_factor) + color.b() as f32) as u8,
                );
            }
        }
    }
}
