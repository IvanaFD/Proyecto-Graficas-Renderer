mod framebuffer;
mod line;
mod obj;
mod triangle;

use raylib::prelude::*;

use framebuffer::Framebuffer;
use obj::{Obj, load_obj};
use triangle::triangle;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// Convierte x, y del modelo a coordenadas de pantalla (escalado y centrado)
fn to_screen(obj: &Obj) -> Vec<Vector2> {
    let mut min = Vector2::new(f32::MAX, f32::MAX);
    let mut max = Vector2::new(f32::MIN, f32::MIN);
    for v in &obj.vertices {
        min.x = min.x.min(v.x);
        min.y = min.y.min(v.y);
        max.x = max.x.max(v.x);
        max.y = max.y.max(v.y);
    }

    let size = Vector2::new(max.x - min.x, max.y - min.y);
    let center = Vector2::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0);

    // 80% de la ventana
    let scale = 0.8 * (WIDTH as f32 / size.x).min(HEIGHT as f32 / size.y);

    obj.vertices
        .iter()
        .map(|v| {
            Vector2::new(
                WIDTH as f32 / 2.0 + (v.x - center.x) * scale,
                HEIGHT as f32 / 2.0 - (v.y - center.y) * scale, // y hacia abajo en pantalla
            )
        })
        .collect()
}

fn draw_model(framebuffer: &mut Framebuffer, obj: &Obj) {
    let points = to_screen(obj);

    for face in obj.indices.chunks_exact(3) {
        // 1. Índices del triángulo
        let (i1, i2, i3) = (face[0], face[1], face[2]);

        // 2. Vértices correspondientes
        let (a, b, c) = (points[i1], points[i2], points[i3]);

        // 3. Dibujar
        triangle(framebuffer, a, b, c);
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

    println!(
        "Modelo cargado: {} vértices, {} índices, {} triángulos",
        obj.vertices.len(),
        obj.indices.len(),
        obj.indices.len() / 3
    );

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Color::new(20, 20, 30, 255));
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.clear();
    draw_model(&mut framebuffer, &obj);

    // Captura sin abrir ventana
    if std::env::args().any(|arg| arg == "--captura") {
        framebuffer.render_to_file("capturas/nave.png");
        println!("Captura guardada en capturas/nave.png");
        return;
    }

    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Lab 4 - Modelo OBJ")
        .build();

    window.set_target_fps(60);
    framebuffer.init_texture(&mut window, &thread);

    while !window.window_should_close() {
        framebuffer.swap_buffers(&mut window, &thread);
    }
}
