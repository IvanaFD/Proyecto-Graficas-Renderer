use raylib::math::Vector3;

use crate::fragment::Fragment;
use crate::line::line;
use crate::vertex::Vertex;

// Contorno
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));

    fragments
}

// Área con signo (x2) del triángulo a, b, p
fn edge_function(a: Vector3, b: Vector3, p: Vector3) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

// Regla top-left: arista izquierda o arista horizontal superior
fn is_top_left(a: Vector3, b: Vector3) -> bool {
    let normal_x = -(b.y - a.y);
    let normal_y = b.x - a.x;
    normal_x > 0.0 || (normal_x == 0.0 && normal_y > 0.0)
}

fn is_inside(w: f32, top_left: bool) -> bool {
    w > 0.0 || (w == 0.0 && top_left)
}

// Relleno con coordenadas baricéntricas
pub fn triangle_barycentric(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let a = v1.transformed_position;
    let mut b = v2.transformed_position;
    let mut c = v3.transformed_position;

    let mut area = edge_function(a, b, c);

    // Caso degenerado
    if area == 0.0 {
        return fragments;
    }

    // Orden inverso
    if area < 0.0 {
        std::mem::swap(&mut b, &mut c);
        area = -area;
    }

    let top_left_bc = is_top_left(b, c);
    let top_left_ca = is_top_left(c, a);
    let top_left_ab = is_top_left(a, b);

    // Bounding box
    let min_x = a.x.min(b.x).min(c.x).floor() as i32;
    let min_y = a.y.min(b.y).min(c.y).floor() as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil() as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil() as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            // Centro del píxel
            let p = Vector3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            let w1 = edge_function(b, c, p);
            let w2 = edge_function(c, a, p);
            let w3 = edge_function(a, b, p);

            if is_inside(w1, top_left_bc) && is_inside(w2, top_left_ca) && is_inside(w3, top_left_ab) {
                // Coordenadas baricéntricas
                let l1 = w1 / area;
                let l2 = w2 / area;
                let l3 = w3 / area;
                let depth = l1 * a.z + l2 * b.z + l3 * c.z;

                fragments.push(Fragment::new(x as f32, y as f32, v1.color, depth));
            }
        }
    }

    fragments
}

// Relleno por líneas horizontales
pub fn triangle_scanline(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex) -> Vec<Fragment> {
    Vec::new()
}
