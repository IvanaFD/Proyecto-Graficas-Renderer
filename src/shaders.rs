use raylib::prelude::*;

use crate::Uniforms;
use crate::fragment::Fragment;
use crate::vertex::Vertex;

fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vector4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    let transformed = multiply_matrix_vector4(&uniforms.model_matrix, &position);

    let mut result = vertex.clone();
    result.transformed_position = Vector3::new(transformed.x, transformed.y, transformed.z);

    result
}

// Sombreado plano: un tono por triángulo según su normal y la luz
pub fn shade_triangle(tri: &mut [Vertex; 3]) {
    let a = tri[0].transformed_position;
    let b = tri[1].transformed_position;
    let c = tri[2].transformed_position;

    let normal = (b - a).cross(c - a).normalized();
    let light = Vector3::new(-0.4, -0.6, 1.0).normalized();

    let ambient = 0.15;
    let intensity = ambient + (1.0 - ambient) * normal.dot(light).abs();

    for vertex in tri.iter_mut() {
        vertex.color = vertex.color * intensity;
    }
}

// Passthrough: devuelve el color que el fragmento ya tiene
pub fn fragment_shader(fragment: &Fragment) -> Vector3 {
    fragment.color
}
