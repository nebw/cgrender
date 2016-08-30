extern crate rand;
extern crate sdl2;
extern crate stopwatch;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Renderer, Texture};
use sdl2::EventPump;
use std::cmp;
use std::time::Duration;
use std::u8;
use stopwatch::Stopwatch;
use rand::{Rng, XorShiftRng};

struct Context<'a> {
    renderer: Renderer<'a>,
    event_pump: EventPump,
    texture: Texture,
}

struct Scene {
    width: usize,
    height: usize,
    rng: XorShiftRng,
}

impl<'a> Context<'a> {
    fn new(title: &str, width: u32, height: u32) -> Context {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, width, height).unwrap();

        Context { 
            renderer: renderer, 
            event_pump: event_pump,
            texture: texture
        }
    }

    fn scene_rect(&self, scene: &Scene) -> Rect {
        Rect::new(0, 0, scene.width as u32, scene.height as u32)
    }

    fn draw_scene(&mut self, scene: &mut Scene) {
        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| { 
            scene.draw(buffer, pitch) }).unwrap();

        self.renderer.clear();
        let rect = self.scene_rect(scene);
        self.renderer.copy(&self.texture, None, Some(rect));
        self.renderer.present();
    }
}

impl Scene {
    fn new(width: usize, height: usize) -> Scene {
        let rng = rand::weak_rng();
        Scene {
            width: width,
            height: height,
            rng: rng
        }
    }

    fn rnd_u8(&mut self) -> u8 {
        (self.rng.next_f32() * (u8::MAX as f32)).round() as u8
    }

    fn draw(&mut self, buffer: &mut [u8], pitch: usize) {
        let (r, g, b) = (self.rnd_u8(), self.rnd_u8(), self.rnd_u8());
        for y in 0..self.width {
            for x in 0..self.height {
                let offset = y * pitch + x * 3;
                buffer[offset + 0] = r;
                buffer[offset + 1] = g;
                buffer[offset + 2] = b;
            }
        }
    }
}

pub fn main() {
    let (width, height) = (512, 512);
    let mut context = Context::new("Test", width, height);

    let mut scene = Scene::new(width as usize, height as usize);

    let fps: f64 = 1.;
    let frame_duration: i64 = ((1. / fps) * 1000.).round() as i64;

    'running: loop {
        let sw = Stopwatch::start_new();

        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        context.draw_scene(&mut scene);

        let sleep_time = cmp::max(0, frame_duration - sw.elapsed_ms()) as u64;
        std::thread::sleep(Duration::from_millis(sleep_time));
    }
}
