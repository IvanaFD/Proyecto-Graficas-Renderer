use std::collections::HashSet;

use crate::fragment::Fragment;
use crate::triangle::{triangle_barycentric, triangle_scanline};
use crate::vertex::Vertex;

pub struct Comparison {
    pub name: String,
    pub barycentric: usize,
    pub scanline: usize,
    pub only_barycentric: Vec<(i32, i32)>,
    pub only_scanline: Vec<(i32, i32)>,
    pub duplicates: usize,
}

fn positions(fragments: &[Fragment]) -> HashSet<(i32, i32)> {
    fragments
        .iter()
        .map(|f| (f.position.x as i32, f.position.y as i32))
        .collect()
}

// Compara ambos algoritmos triángulo por triángulo
pub fn compare(name: &str, triangles: &[[Vertex; 3]]) -> Comparison {
    let mut result = Comparison {
        name: name.to_string(),
        barycentric: 0,
        scanline: 0,
        only_barycentric: Vec::new(),
        only_scanline: Vec::new(),
        duplicates: 0,
    };

    for tri in triangles {
        let bary = triangle_barycentric(&tri[0], &tri[1], &tri[2]);
        let scan = triangle_scanline(&tri[0], &tri[1], &tri[2]);
        let bary_set = positions(&bary);
        let scan_set = positions(&scan);

        result.barycentric += bary.len();
        result.scanline += scan.len();
        result.duplicates += (bary.len() - bary_set.len()) + (scan.len() - scan_set.len());
        result.only_barycentric.extend(bary_set.difference(&scan_set));
        result.only_scanline.extend(scan_set.difference(&bary_set));
    }

    result
}

// Mismas posiciones con los vértices en orden inverso
pub fn same_in_reverse(tri: &[Vertex; 3]) -> bool {
    let reversed = [tri[2].clone(), tri[1].clone(), tri[0].clone()];
    let normal_b = positions(&triangle_barycentric(&tri[0], &tri[1], &tri[2]));
    let normal_s = positions(&triangle_scanline(&tri[0], &tri[1], &tri[2]));
    let reversed_b = positions(&triangle_barycentric(&reversed[0], &reversed[1], &reversed[2]));
    let reversed_s = positions(&triangle_scanline(&reversed[0], &reversed[1], &reversed[2]));
    normal_b == reversed_b && normal_s == reversed_s
}

pub fn print_table(results: &[Comparison]) {
    println!("| Caso | Baricéntrico | Scanline | Solo baricéntrico | Solo scanline | Duplicados |");
    println!("|---|---:|---:|---:|---:|---:|");
    for r in results {
        println!(
            "| {} | {} | {} | {} | {} | {} |",
            r.name,
            r.barycentric,
            r.scanline,
            r.only_barycentric.len(),
            r.only_scanline.len(),
            r.duplicates
        );
    }

    for r in results {
        for p in &r.only_barycentric {
            println!("{}: ({}, {}) solo en baricéntrico", r.name, p.0, p.1);
        }
        for p in &r.only_scanline {
            println!("{}: ({}, {}) solo en scanline", r.name, p.0, p.1);
        }
    }
}
