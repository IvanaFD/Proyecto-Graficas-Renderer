mod benchmark;
mod compare;
mod fragment;
mod framebuffer;
mod line;
mod obj;
mod primitive;
mod rasterizer;
mod shaders;
mod test_scene;
mod triangle;
mod vertex;

use raylib::prelude::*;

use framebuffer::Framebuffer;
use obj::load_obj;
use primitive::primitive_assembly;
use rasterizer::{TriangleMode, rasterize};
use shaders::{fragment_shader, shade_triangle, vertex_shader};
use test_scene::{test_cases, test_vertex_array};
use vertex::Vertex;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

// Rotación, luego escala (y invertida) y luego traslación
fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    Matrix::rotate_xyz(rotation)
        * Matrix::scale(scale, -scale, scale)
        * Matrix::translate(translation.x, translation.y, translation.z)
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    mode: TriangleMode,
) {
    // 1. Procesamiento de vértices
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // 2. Ensamblaje de primitivas
    let mut triangles = primitive_assembly(&transformed_vertices);

    // Sombreado plano en los modos de relleno
    let filled = !matches!(mode, TriangleMode::Wireframe);
    if filled {
        for tri in triangles.iter_mut() {
            shade_triangle(tri);
        }
    }

    // 3. Rasterización
    let fragments = rasterize(&triangles, mode);

    // 4. Procesamiento de fragmentos
    for fragment in fragments {
        let color = fragment_shader(&fragment);
        let x = fragment.position.x as i32;
        let y = fragment.position.y as i32;

        if filled {
            framebuffer.point_with_depth(x, y, color, fragment.depth);
        } else {
            framebuffer.point(x, y, color);
        }
    }
}

fn transform(vertex_array: &[Vertex], uniforms: &Uniforms) -> Vec<[Vertex; 3]> {
    let transformed: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();
    primitive_assembly(&transformed)
}

fn run_comparison(vertex_array: &[Vertex], model_matrix: impl Fn(Vector3) -> Matrix) {
    let mut results = Vec::new();

    // Casos de prueba
    for case in test_cases() {
        let tri = case.positions.map(|p| Vertex::new_with_color(p, case.color));
        results.push(compare::compare(case.name, &[tri]));
    }

    // Nave de frente y desde arriba
    for (name, rotation) in [
        ("nave (frente)", Vector3::new(0.0, 0.0, 0.0)),
        ("nave (arriba)", Vector3::new(-std::f32::consts::FRAC_PI_2, 0.0, 0.0)),
    ] {
        let uniforms = Uniforms {
            model_matrix: model_matrix(rotation),
        };
        results.push(compare::compare(name, &transform(vertex_array, &uniforms)));
    }

    compare::print_table(&results);

    // Orden inverso
    let all_same = test_cases().iter().all(|case| {
        let tri = case.positions.map(|p| Vertex::new_with_color(p, case.color));
        compare::same_in_reverse(&tri)
    });
    println!("Orden inverso genera las mismas posiciones en todos los casos: {all_same}");
}

fn window_title(mode: TriangleMode, show_tests: bool) -> String {
    let mode_name = match mode {
        TriangleMode::Wireframe => "[1] Contorno",
        TriangleMode::FilledBarycentric => "[2] Baricentricas",
        TriangleMode::FilledScanline => "[3] Lineas horizontales",
    };
    let scene_name = if show_tests { "Pruebas" } else { "Nave" };
    format!("Rellenar triangulos - {mode_name} - [TAB] {scene_name}")
}

fn main() {
    let obj = match load_obj("assets/nave.obj") {
        Ok(obj) => obj,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    let vertex_array = obj.get_vertex_array(Vector3::new(1.0, 1.0, 1.0));
    println!(
        "Modelo cargado: {} vértices, {} triángulos",
        obj.vertices.len(),
        vertex_array.len() / 3
    );

    // Centrar en el origen; la escala usa el radio para que quepa en cualquier ángulo
    let (center, _) = obj.bounds();
    let radius = obj.radius(center);
    let scale = 0.9 * (WIDTH.min(HEIGHT) as f32 / 2.0) / radius;
    let translation = Vector3::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);

    let model_matrix = |rotation: Vector3| {
        Matrix::translate(-center.x, -center.y, -center.z)
            * create_model_matrix(translation, scale, rotation)
    };

    // Escena de pruebas: ya en coordenadas de pantalla
    let test_vertices = test_vertex_array();
    let test_uniforms = Uniforms {
        model_matrix: Matrix::identity(),
    };

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Vector3::new(0.08, 0.08, 0.12));

    if std::env::args().any(|arg| arg == "--benchmark-triangles") {
        benchmark::run("Casos de prueba", &transform(&test_vertices, &test_uniforms));
        let uniforms = Uniforms {
            model_matrix: model_matrix(Vector3::new(-std::f32::consts::FRAC_PI_2, 0.0, 0.0)),
        };
        benchmark::run("Nave (desde arriba)", &transform(&vertex_array, &uniforms));
        return;
    }

    if std::env::args().any(|arg| arg == "--comparar") {
        run_comparison(&vertex_array, model_matrix);
        return;
    }

    // Capturas sin abrir ventana
    if std::env::args().any(|arg| arg == "--captura") {
        let uniforms = Uniforms {
            model_matrix: model_matrix(rotation),
        };
        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array, TriangleMode::Wireframe);
        framebuffer.render_to_file("capturas/nave.png");

        let top_uniforms = Uniforms {
            model_matrix: model_matrix(Vector3::new(-std::f32::consts::FRAC_PI_2, 0.0, 0.0)),
        };
        for (mode, name) in [
            (TriangleMode::Wireframe, "contorno"),
            (TriangleMode::FilledBarycentric, "baricentrico"),
            (TriangleMode::FilledScanline, "scanline"),
        ] {
            framebuffer.clear();
            render(&mut framebuffer, &top_uniforms, &vertex_array, mode);
            framebuffer.render_to_file(&format!("capturas/nave_{name}.png"));

            framebuffer.clear();
            render(&mut framebuffer, &test_uniforms, &test_vertices, mode);
            framebuffer.render_to_file(&format!("capturas/pruebas_{name}.png"));
        }
        println!("Capturas guardadas en capturas/");
        return;
    }

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Rellenar triangulos")
        .build();

    window.set_target_fps(60);
    framebuffer.init_texture(&mut window, &thread);

    let mut mode = TriangleMode::Wireframe;
    let mut show_tests = false;
    window.set_window_title(&thread, &window_title(mode, show_tests));

    while !window.window_should_close() {
        // Rotación con flechas, R reinicia
        let step = 1.5 * window.get_frame_time();
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            rotation.y -= step;
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            rotation.y += step;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            rotation.x -= step;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            rotation.x += step;
        }
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            rotation = Vector3::new(0.0, 0.0, 0.0);
        }

        // Modo con 1, 2 y 3
        let new_mode = if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            Some(TriangleMode::Wireframe)
        } else if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            Some(TriangleMode::FilledBarycentric)
        } else if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            Some(TriangleMode::FilledScanline)
        } else {
            None
        };
        // Escena con TAB
        let toggle_scene = window.is_key_pressed(KeyboardKey::KEY_TAB);
        if toggle_scene {
            show_tests = !show_tests;
        }
        if let Some(new_mode) = new_mode {
            mode = new_mode;
        }
        if toggle_scene || new_mode.is_some() {
            window.set_window_title(&thread, &window_title(mode, show_tests));
        }

        framebuffer.clear();
        if show_tests {
            render(&mut framebuffer, &test_uniforms, &test_vertices, mode);
        } else {
            let uniforms = Uniforms {
                model_matrix: model_matrix(rotation),
            };
            render(&mut framebuffer, &uniforms, &vertex_array, mode);
        }
        framebuffer.swap_buffers(&mut window, &thread);
    }
}
