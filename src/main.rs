mod game;
use game::GameObject;
use game::Scene;

mod rendering;
use rendering::Window;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas};
use std::time::Duration;

fn render(
    canvas: &mut WindowCanvas,
    background_color: Color,
    scene: &Scene,
) -> Result<(), String> {
    canvas.set_draw_color(background_color);
    canvas.clear();

    scene.draw_onto_canvas(canvas)?;
    canvas.present();

    Ok(())
}
fn main() -> Result<(), String> {
    let window = Window::new("exopirate", 1280, 720)?;
    let mut event_pump = window.sdl().event_pump()?;
    let mut canvas = window.into_canvas();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("racoon.png")?;
    
    let objects = vec![
        GameObject::new(
            texture,
            Point::new(0, 0),
            Rect::new(0, 0, 640, 320)
        )
    ];
    let scene = Scene::new(objects);

    loop {
        if Window::handle_default_events(&mut event_pump) {
            break;
        }
        render(&mut canvas, Color::RGB(255, 255, 255), &scene)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
