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

// Relleno con coordenadas baricéntricas
pub fn triangle_barycentric(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex) -> Vec<Fragment> {
    Vec::new()
}

// Relleno por líneas horizontales
pub fn triangle_scanline(_v1: &Vertex, _v2: &Vertex, _v3: &Vertex) -> Vec<Fragment> {
    Vec::new()
}
