use std::hint::black_box;
use std::time::{Duration, Instant};

use crate::rasterizer::{TriangleMode, rasterize};
use crate::vertex::Vertex;

const RUNS: u32 = 100;
const REPETITIONS: usize = 15;

fn median(mut times: Vec<Duration>) -> Duration {
    times.sort();
    times[times.len() / 2]
}

fn mode_name(mode: TriangleMode) -> &'static str {
    match mode {
        TriangleMode::Wireframe => "Contorno",
        TriangleMode::FilledBarycentric => "Baricéntrico",
        TriangleMode::FilledScanline => "Scanline",
    }
}

fn millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

// Mide solo la generación de fragmentos
pub fn run(name: &str, triangles: &[[Vertex; 3]]) {
    println!("\n### {name} ({} triángulos)\n", triangles.len());
    println!("| Algoritmo | Fragmentos | Mediana por {RUNS} ejecuciones | Por ejecución |");
    println!("|---|---:|---:|---:|");

    let mut medians = Vec::new();

    for mode in [
        TriangleMode::Wireframe,
        TriangleMode::FilledBarycentric,
        TriangleMode::FilledScanline,
    ] {
        let fragment_count = rasterize(triangles, mode).len();

        let mut times = Vec::with_capacity(REPETITIONS);
        for _ in 0..REPETITIONS {
            let start = Instant::now();

            for _ in 0..RUNS {
                let fragments = rasterize(black_box(triangles), mode);
                black_box(fragments);
            }

            times.push(start.elapsed());
        }

        let median = median(times);
        medians.push(median);

        println!(
            "| {} | {} | {:.2} ms | {:.3} ms |",
            mode_name(mode),
            fragment_count,
            millis(median),
            millis(median) / RUNS as f64
        );
    }

    let speedup = medians[1].as_secs_f64() / medians[2].as_secs_f64();
    println!("\nScanline es {speedup:.2} veces más rápido que baricéntrico.");
}
