use raylib::prelude::*;

use crate::vertex::Vertex;

pub struct TestCase {
    pub name: &'static str,
    pub positions: [Vector3; 3],
    pub color: Vector3,
}

fn case(name: &'static str, p: [(f32, f32); 3], color: (f32, f32, f32)) -> TestCase {
    TestCase {
        name,
        positions: p.map(|(x, y)| Vector3::new(x, y, 0.0)),
        color: Vector3::new(color.0, color.1, color.2),
    }
}

// Triángulos de prueba en coordenadas de pantalla (800 x 600)
pub fn test_cases() -> Vec<TestCase> {
    vec![
        // 1. Pequeño y grande
        case("pequeño", [(30.0, 30.0), (45.0, 30.0), (37.0, 42.0)], (1.0, 0.85, 0.3)),
        case("grande", [(70.0, 20.0), (300.0, 50.0), (150.0, 210.0)], (1.0, 0.85, 0.3)),
        // 2. Borde horizontal y borde vertical
        case("borde horizontal", [(340.0, 40.0), (500.0, 40.0), (420.0, 190.0)], (0.4, 0.8, 1.0)),
        case("borde vertical", [(560.0, 30.0), (560.0, 200.0), (720.0, 115.0)], (0.4, 0.8, 1.0)),
        // 3. Mismos vértices en orden normal e inverso (el inverso se muestra desplazado)
        case("orden normal", [(60.0, 250.0), (250.0, 280.0), (120.0, 420.0)], (0.5, 1.0, 0.5)),
        case("orden inverso", [(370.0, 420.0), (500.0, 280.0), (310.0, 250.0)], (0.5, 1.0, 0.5)),
        // 4. Parcialmente fuera del framebuffer
        case("parcialmente fuera", [(620.0, 300.0), (900.0, 420.0), (680.0, 700.0)], (1.0, 0.5, 0.8)),
        // 5. Tres vértices sobre una misma línea
        case("colineal", [(80.0, 470.0), (230.0, 520.0), (380.0, 570.0)], (1.0, 0.4, 0.4)),
    ]
}

pub fn test_vertex_array() -> Vec<Vertex> {
    test_cases()
        .iter()
        .flat_map(|case| {
            case.positions
                .iter()
                .map(|&position| Vertex::new_with_color(position, case.color))
                .collect::<Vec<_>>()
        })
        .collect()
}
