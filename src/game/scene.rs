use sdl2::render::WindowCanvas;
use crate::game::GameObject;

pub struct Scene<'a> {
    objects: Vec<GameObject<'a>>
}

impl Scene<'_> {
    pub fn new(objects: Vec<GameObject>) -> Scene {
        Scene { objects }
    }
    pub fn draw_onto_canvas(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for object in &self.objects {
            object.draw_onto_canvas(canvas)?
        }
        Ok(())
    }
}
