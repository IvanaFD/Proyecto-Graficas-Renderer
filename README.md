# Renderer de software en Rust

Renderer en Rust que carga un modelo `.obj` con un cargador propio y lo dibuja píxel por píxel en un framebuffer. Raylib solo se usa para abrir la ventana y mostrar el framebuffer; no se usan sus funciones para cargar ni dibujar modelos 3D.

- **Laboratorio 4 — Cargar un modelo OBJ** (rama `lab4-cargar-modelo`): dibuja los triángulos de la nave como contorno.
- **Laboratorio 5 — Rellenar triángulos** (rama `lab5-rellenar-triangulos`): rellena esos triángulos con dos algoritmos y los compara. Es lo que describe este README.

Modelo: [`assets/nave.obj`](assets/nave.obj), una nave hecha en Blender y exportada con caras trianguladas (401 vértices, 802 triángulos).

## Ejecutar

```bash
cargo run --release
```

| Tecla | Acción |
|---|---|
| `1` | Contorno |
| `2` | Relleno con coordenadas baricéntricas |
| `3` | Relleno por líneas horizontales (scanline) |
| Flechas | Rotar la nave |
| `R` | Volver a la vista de frente |
| `TAB` | Cambiar entre la nave y la escena de pruebas |

Otros modos, ninguno abre la ventana:

```bash
cargo run --release -- --benchmark-triangles   # mediciones
cargo run --release -- --comparar              # comparación de posiciones
cargo run --release -- --captura               # regenera las imágenes de capturas/
```

## Modos de dibujo

```rust
#[derive(Clone, Copy)]
pub enum TriangleMode {
    Wireframe,
    FilledBarycentric,
    FilledScanline,
}
```

`rasterize` (`src/rasterizer.rs`) recibe los triángulos y el modo, y devuelve un `Vec<Fragment>`. Cada modo es una función aparte en `src/triangle.rs` que recibe los tres vértices:

- **`triangle`** (contorno): tres llamadas a `line` (Bresenham).
- **`triangle_barycentric`**: recorre el rectángulo que envuelve al triángulo y evalúa el centro de cada píxel `(x + 0.5, y + 0.5)` con las tres funciones de arista. Si el área es 0, el triángulo es degenerado y no genera fragmentos. Si el área es negativa, intercambia dos vértices, así que el orden de los vértices no importa.
- **`triangle_scanline`**: ordena los vértices por `y` y, para cada fila cuyo centro cae dentro del triángulo, calcula dónde cortan las aristas y llena el tramo entre ambos cortes.

En los modos de relleno cada triángulo recibe un tono según su normal y una luz fija (sombreado plano), y un z-buffer decide qué cara queda delante. Esto se aplica después de generar los fragmentos, así que no cambia las posiciones ni las mediciones.

## Imágenes de cada modo

| Contorno | Baricéntrico | Scanline |
|---|---|---|
| ![Nave contorno](capturas/nave_contorno.png) | ![Nave baricéntrico](capturas/nave_baricentrico.png) | ![Nave scanline](capturas/nave_scanline.png) |
| ![Pruebas contorno](capturas/pruebas_contorno.png) | ![Pruebas baricéntrico](capturas/pruebas_baricentrico.png) | ![Pruebas scanline](capturas/pruebas_scanline.png) |

Arriba, la nave vista desde arriba. Abajo, la escena de pruebas (`src/test_scene.rs`), con los casos que pide la tarea:

1. Pequeño (amarillo, esquina) y grande (amarillo).
2. Borde horizontal y borde vertical (azules).
3. Los mismos tres vértices en orden normal y en orden inverso (verdes).
4. Parcialmente fuera del framebuffer (rosado, abajo a la derecha).
5. Tres vértices sobre una misma línea: en contorno se ve como una línea; en los rellenos no genera nada porque su área es 0.

## Comparación de posiciones

`--comparar` genera los fragmentos de ambos algoritmos para cada triángulo y compara los conjuntos de posiciones `(x, y)`, no la imagen.

| Caso | Baricéntrico | Scanline | Solo baricéntrico | Solo scanline | Duplicados |
|---|---:|---:|---:|---:|---:|
| pequeño | 90 | 90 | 0 | 0 | 0 |
| grande | 20645 | 20645 | 0 | 0 | 0 |
| borde horizontal | 12000 | 12000 | 0 | 0 | 0 |
| borde vertical | 13600 | 13600 | 0 | 0 | 0 |
| orden normal | 15245 | 15245 | 0 | 0 | 0 |
| orden inverso | 15245 | 15245 | 0 | 0 | 0 |
| parcialmente fuera | 52380 | 52380 | 0 | 0 | 0 |
| colineal | 0 | 0 | 0 | 0 | 0 |
| nave (frente) | 39486 | 39486 | 0 | 0 | 0 |
| nave (arriba) | 175692 | 175692 | 1 | 1 | 0 |

Además, con los vértices en orden inverso ambos algoritmos generan exactamente las mismas posiciones en todos los casos.

### Regla de cada implementación

Los dos usan la misma idea: un píxel se pinta si su **centro** está dentro del triángulo. Cuando el centro cae justo sobre una arista, aplican esta regla para que un píxel compartido por dos triángulos se pinte una sola vez:

- **Baricéntrico: regla arriba-izquierda.** Un centro que cae justo sobre una arista cuenta como dentro solo si esa arista es superior (horizontal, arriba) o izquierda. Por eso un borde horizontal superior se incluye y uno inferior no.
- **Scanline: intervalos medio abiertos.** Las filas van de `[y_min, y_max)` y los tramos de `[x_izq, x_der)`. Un centro exactamente en el borde izquierdo o superior se incluye, y uno en el borde derecho o inferior no. Los vértices que caen justo en una fila los cubre la misma regla, y una arista horizontal no genera ningún corte.

Las dos reglas dan el mismo resultado en todos los casos de prueba. La única diferencia es **un píxel** de la nave vista desde arriba: el (574, 354). Su centro (574.5, 354.5) cae prácticamente sobre la arista que comparten los triángulos 130 y 531.

- El baricéntrico obtiene ±0.00055 en la función de arista, no exactamente 0, por el redondeo de `f32`, y asigna el píxel al triángulo 531.
- El scanline calcula el corte en exactamente x = 574.5 y, por el intervalo `[x_izq, x_der)`, asigna el píxel al triángulo 130.

En ambos el píxel se pinta una sola vez: no hay huecos ni duplicados, el total de fragmentos es el mismo y la imagen final es idéntica.

## Mediciones

`cargo run --release -- --benchmark-triangles` (i7-13620H). Se mide solo la generación de fragmentos con `Instant` y `black_box`: 100 ejecuciones seguidas, repetidas 15 veces, y se reporta la mediana.

### Casos de prueba (8 triángulos)

| Algoritmo | Fragmentos | Mediana por 100 ejecuciones | Por ejecución |
|---|---:|---:|---:|
| Contorno | 4153 | 2.93 ms | 0.029 ms |
| Baricéntrico | 129205 | 264.53 ms | 2.645 ms |
| Scanline | 129205 | 217.80 ms | 2.178 ms |

Scanline es **1.21 veces** más rápido que baricéntrico.

### Nave desde arriba (802 triángulos)

| Algoritmo | Fragmentos | Mediana por 100 ejecuciones | Por ejecución |
|---|---:|---:|---:|
| Contorno | 92630 | 73.80 ms | 0.738 ms |
| Baricéntrico | 175692 | 228.79 ms | 2.288 ms |
| Scanline | 175692 | 99.25 ms | 0.992 ms |

Scanline es **2.31 veces** más rápido que baricéntrico.

Ambos rellenos generan los mismos fragmentos, así que la diferencia no viene de hacer menos trabajo útil. En la escena de pruebas los triángulos son grandes y anchos: su rectángulo envolvente no es mucho mayor que el triángulo y la diferencia es pequeña. La nave tiene muchos triángulos delgados y en diagonal, y ahí el baricéntrico evalúa muchos píxeles del rectángulo que quedan fuera. El scanline solo recorre los que están dentro.

## Perfil

![Flamegraph](docs/flamegraph.svg)

[`docs/flamegraph.svg`](docs/flamegraph.svg) se puede abrir en el navegador; es interactivo. Se generó con:

```bash
CARGO_TARGET_DIR=target/perfil RUSTFLAGS="-C force-frame-pointers=yes" \
  cargo flamegraph --no-inline -o docs/flamegraph.svg --cmd "record -F 997 -g" -- --benchmark-triangles
```

`triangle_barycentric` y `triangle_scanline` llevan `#[inline(never)]` para que aparezcan como bloques propios en el perfil y no se mezclen dentro de `rasterize`. Se usan *frame pointers* porque, con la configuración por defecto, el perfil atribuía mal las muestras: `line` salía con 35 % y `triangle_scanline` no aparecía. El resultado se comprobó con un perfil plano de `perf record`.

| Dónde | % de muestras |
|---|---:|
| `triangle_barycentric` | 42.7 |
| ↳ código propio: recorrer el rectángulo, centro del píxel, funciones de arista, crear fragmentos | 38.2 |
| ↳ crecimiento de su `Vec` | 3.7 |
| `triangle_scanline` | 18.2 |
| ↳ código propio: cortes por fila, recorrer el tramo, crear fragmentos | 13.1 |
| ↳ crecimiento de su `Vec` | 4.1 |
| `line` (contorno) | 6.3 |
| `rasterize`: juntar los fragmentos de cada triángulo en un solo `Vec` (`extend`) | 16.3 |
| Kernel: fallos de página y liberar memoria | 13.9 |
| Otros | 2.6 |

**¿Cálculos geométricos, creación de fragmentos o crecimiento del `Vec`?**

- **La memoria pesa tanto como la geometría.** Sumando el crecimiento de cada `Vec`, el `extend` de `rasterize` y los fallos de página, cerca del **42 %** de las muestras es manejo de memoria, y ambos algoritmos pagan ese costo por igual.
- **El código propio del baricéntrico pesa unas 3 veces más que el del scanline** (38 % frente a 13 %) generando los mismos fragmentos. Eso confirma que el costo extra es evaluar los píxeles del rectángulo que quedan fuera del triángulo.
- Dentro del baricéntrico, aproximadamente: calcular el centro del píxel ~19 %, las funciones de arista ~16 % y guardar el fragmento (`Vec::push`) ~18 %.
- Una mejora posible sería reservar memoria con `Vec::with_capacity` o escribir todos los fragmentos en un solo `Vec`, en lugar de crear uno por triángulo y luego unirlos.
