use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Window {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    _image: sdl2::image::Sdl2ImageContext,
    window: sdl2::video::Window,
}
impl Window {
    pub fn video(&self) -> &sdl2::VideoSubsystem {
        &self.video
    }
    pub fn sdl(&self) -> &sdl2::Sdl {
        &self.sdl
    }
    pub fn sdl_window(&self) -> &sdl2::video::Window {
        &self.window
    }
    pub fn new(title: &str, width: u32, height: u32) -> Result<Window, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let _image = image::init(InitFlag::PNG | InitFlag::JPG)?;

        let window = video
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        Ok(Window {
            sdl,
            video,
            _image,
            window,
        })
    }
    pub fn into_canvas(self) -> sdl2::render::WindowCanvas {
        self.window
            .into_canvas()
            .build()
            .expect("Error creating canvas...")
    }
    pub fn handle_default_events(event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                _ => {}
            }
        }
        false
    }
}
