use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    background_color: Color,
    current_color: Color,
    // RGBA, 4 bytes por píxel
    color_buffer: Vec<u8>,
    texture: Option<Texture2D>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let mut framebuffer = Framebuffer {
            width,
            height,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
            color_buffer: vec![0; (width * height * 4) as usize],
            texture: None,
        };
        framebuffer.clear();
        framebuffer
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        let c = self.background_color;
        for pixel in self.color_buffer.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[c.r, c.g, c.b, c.a]);
        }
    }

    pub fn point(&mut self, position: Vector2) {
        let x = position.x as i32;
        let y = position.y as i32;
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            let c = self.current_color;
            self.color_buffer[index..index + 4].copy_from_slice(&[c.r, c.g, c.b, c.a]);
        }
    }

    pub fn init_texture(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        let image = Image::gen_image_color(self.width, self.height, Color::BLACK);
        self.texture = window.load_texture_from_image(thread, &image).ok();
    }

    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        let Some(texture) = self.texture.as_mut() else {
            return;
        };
        let _ = texture.update_texture(&self.color_buffer);

        let screen_width = window.get_screen_width() as f32;
        let screen_height = window.get_screen_height() as f32;

        let mut render = window.begin_drawing(thread);
        render.draw_texture_pro(
            &*texture,
            Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32),
            Rectangle::new(0.0, 0.0, screen_width, screen_height),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
    }

    pub fn render_to_file(&self, file_path: &str) {
        let mut image = Image::gen_image_color(self.width, self.height, Color::BLACK);
        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 4) as usize;
                let p = &self.color_buffer[index..index + 4];
                image.draw_pixel(x, y, Color::new(p[0], p[1], p[2], p[3]));
            }
        }
        image.export_image(file_path);
    }
}
