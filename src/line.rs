use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

// Algoritmo de Bresenham
pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2) {
    let x1 = start.x.round() as i32;
    let y1 = start.y.round() as i32;

    let x2 = end.x.round() as i32;
    let y2 = end.y.round() as i32;

    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        framebuffer.point(Vector2::new(x as f32, y as f32));

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
