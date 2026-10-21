use raylib::math::Vector3;

#[derive(Clone, Debug)]
pub struct Vertex {
    // Posición original del vértice
    pub position: Vector3,
    pub color: Vector3,
    // Posición después del vertex shader; la usan las etapas siguientes
    pub transformed_position: Vector3,
}

impl Vertex {
    pub fn new_with_color(position: Vector3, color: Vector3) -> Self {
        Self {
            position,
            color,
            transformed_position: position,
        }
    }
}
