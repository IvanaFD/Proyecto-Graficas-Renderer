mod fragment;
mod framebuffer;
mod line;
mod obj;
mod primitive;
mod rasterizer;
mod shaders;
mod triangle;
mod vertex;

use raylib::prelude::*;

use framebuffer::Framebuffer;
use obj::load_obj;
use primitive::primitive_assembly;
use rasterizer::{TriangleMode, rasterize};
use shaders::{fragment_shader, vertex_shader};
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
    let triangles = primitive_assembly(&transformed_vertices);

    // 3. Rasterización
    let fragments = rasterize(&triangles, mode);

    // 4. Procesamiento de fragmentos
    for fragment in fragments {
        let color = fragment_shader(&fragment);

        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color,
        );
    }
}

fn mode_title(mode: TriangleMode) -> &'static str {
    match mode {
        TriangleMode::Wireframe => "Rellenar triangulos - [1] Contorno",
        TriangleMode::FilledBarycentric => "Rellenar triangulos - [2] Baricentricas",
        TriangleMode::FilledScanline => "Rellenar triangulos - [3] Lineas horizontales",
    }
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

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Vector3::new(0.08, 0.08, 0.12));

    // Captura sin abrir ventana
    if std::env::args().any(|arg| arg == "--captura") {
        let uniforms = Uniforms {
            model_matrix: model_matrix(rotation),
        };
        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array, TriangleMode::Wireframe);
        framebuffer.render_to_file("capturas/nave.png");
        println!("Captura guardada en capturas/nave.png");
        return;
    }

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Rellenar triangulos")
        .build();

    window.set_target_fps(60);
    framebuffer.init_texture(&mut window, &thread);

    let mut mode = TriangleMode::Wireframe;
    window.set_window_title(&thread, mode_title(mode));

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
        if let Some(new_mode) = new_mode {
            mode = new_mode;
            window.set_window_title(&thread, mode_title(mode));
        }

        let uniforms = Uniforms {
            model_matrix: model_matrix(rotation),
        };

        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array, mode);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}
