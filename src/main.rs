extern crate sdl2;

use anyhow::Result;
use app::{renderer::Renderer, AppContext};
use sdl2::{event::Event, keyboard::Keycode};
use std::time::Duration;

mod app;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let context = AppContext::new();

    let window = video_subsystem
        .window("algo-visual", context.screen_height, context.screen_height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    println!("{}", context.screen_height);

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        renderer.draw(&context)?;
    }

    Ok(())
}
