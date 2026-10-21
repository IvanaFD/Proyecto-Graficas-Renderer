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
use rasterizer::rasterize;
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

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // 1. Procesamiento de vértices
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // 2. Ensamblaje de primitivas
    let triangles = primitive_assembly(&transformed_vertices);

    // 3. Rasterización
    let fragments = rasterize(&triangles);

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

    // Centrar el modelo en el origen y ajustarlo al 80% de la ventana
    let (center, size) = obj.bounds();
    let scale = 0.8 * (WIDTH as f32 / size.x).min(HEIGHT as f32 / size.y);
    let translation = Vector3::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 0.0);
    let rotation = Vector3::new(0.0, 0.0, 0.0);

    let uniforms = Uniforms {
        model_matrix: Matrix::translate(-center.x, -center.y, -center.z)
            * create_model_matrix(translation, scale, rotation),
    };

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Vector3::new(0.08, 0.08, 0.12));

    // Captura sin abrir ventana
    if std::env::args().any(|arg| arg == "--captura") {
        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array);
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

    while !window.window_should_close() {
        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}
