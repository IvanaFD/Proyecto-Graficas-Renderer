use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn triangle(framebuffer: &mut Framebuffer, a: Vector2, b: Vector2, c: Vector2) {
    line(framebuffer, a, b);
    line(framebuffer, b, c);
    line(framebuffer, c, a);
}
