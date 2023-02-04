use egui::{Color32, ColorImage};
use nalgebra::Point3;

pub fn draw_bresenham_line(map: &mut ColorImage, x: Point3<f32>, y: Point3<f32>, color: Color32) {
    let mut x0: f32 = x.x.floor();
    let mut x1: f32 = y.x.floor();
    let mut y0: f32 = x.y.floor();
    let mut y1: f32 = y.y.floor();

    let steep = (x0 - x1).abs() < (y0 - y1).abs();

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2.0;
    let mut error2 = 0.0;
    let mut y = y0;

    let mut x = x0;
    while x <= x1 {
        if steep {
            map[(y as usize, x as usize)] = color;
        } else {
            map[(x as usize, y as usize)] = color;
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1.0 } else { -1.0 };
            error2 -= dx * 2.0;
        }
        x += 1.0;
    }
}
