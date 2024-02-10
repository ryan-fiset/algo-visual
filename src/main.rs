extern crate sdl2;

use anyhow::Result;
use app::AppContext;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::Duration;

mod app;

const BAR_SEGMENT_IN_PXS: u32 = 60;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut context = AppContext::new();

    let window = video_subsystem
        .window(
            "algo-visual",
            (context.vector.len() as u32) * BAR_SEGMENT_IN_PXS,
            (context.vector.iter().max().unwrap() + 1) * BAR_SEGMENT_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
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

        canvas.clear();
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
