use crate::WindowCanvas;
use sdl2::rect::{Rect, Point};
use sdl2::render::Texture;

pub struct GameObject<'a> {
    texture: Texture<'a>,
    position: Point,
    rect: Rect,
}

impl GameObject<'_> {
    pub fn new(texture: Texture, position: Point, rect: Rect) -> GameObject {
        GameObject { texture, position, rect }
    }
    pub fn draw_onto_canvas(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;

        let screen_position = self.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            self.rect.width(),
            self.rect.height(),
        );
        canvas.copy(&self.texture, self.rect, screen_rect)?;
        Ok(())
    }
}
