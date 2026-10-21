use std::fs;

use raylib::prelude::*;

pub struct Obj {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<usize>,
}

pub fn load_obj(path: &str) -> Result<Obj, String> {
    // 1. Abrir y leer el archivo
    let content = fs::read_to_string(path).map_err(|e| format!("No se pudo leer {path}: {e}"))?;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 2. Recorrer sus líneas
    for line in content.lines() {
        let mut parts = line.split_whitespace();

        match parts.next() {
            // Vértice: v x y z
            Some("v") => {
                let values: Vec<f32> = parts
                    .take(3)
                    .map(|value| value.parse::<f32>().map_err(|e| format!("Vértice inválido '{line}': {e}")))
                    .collect::<Result<_, _>>()?;

                if values.len() != 3 {
                    return Err(format!("Vértice incompleto: '{line}'"));
                }
                vertices.push(Vector3::new(values[0], values[1], values[2]));
            }
            // Cara: f 1 2 3  o  f 1/1/1 2/2/2 3/3/3
            Some("f") => {
                let face: Vec<usize> = parts
                    .map(|group| {
                        let first = group.split('/').next().unwrap_or("");
                        first
                            .parse::<usize>()
                            .map(|index| index - 1) // OBJ empieza en 1, Vec en 0
                            .map_err(|e| format!("Índice inválido '{group}': {e}"))
                    })
                    .collect::<Result<_, _>>()?;

                if face.len() != 3 {
                    return Err(format!("La cara no es un triángulo: '{line}'"));
                }
                indices.extend(face);
            }
            // Cualquier otra línea se ignora
            _ => {}
        }
    }

    Ok(Obj { vertices, indices })
}
