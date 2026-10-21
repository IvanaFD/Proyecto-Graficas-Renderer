use crate::fragment::Fragment;
use crate::vertex::Vertex;

// Algoritmo de Bresenham
pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let x0 = a.transformed_position.x.round() as i32;
    let y0 = a.transformed_position.y.round() as i32;
    let x1 = b.transformed_position.x.round() as i32;
    let y1 = b.transformed_position.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        fragments.push(Fragment::new(x as f32, y as f32, a.color, a.transformed_position.z));

        if x == x1 && y == y1 {
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

    fragments
}
