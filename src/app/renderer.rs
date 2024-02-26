use std::time::Duration;

use anyhow::Result;
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use super::{bar::Bar, AppContext};

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_bar(&mut self, x: u32, bar: Bar) -> Result<(), String> {
        let bar_rect = Rect::new(
            x.try_into().unwrap(),
            bar.offset as i32,
            bar.bar_segment,
            bar.bar_height,
        );
        self.canvas.fill_rect(bar_rect)?;
        Ok(())
    }

    fn draw_bars(&mut self, context: &AppContext, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);

        let mut index: u32 = 0;
        for bar in &context.vector {
            self.draw_bar(index * context.bar_segment, *bar)?;
            index += 1;
        }

        Ok(())
    }

    pub fn draw(&mut self, context: &AppContext) -> Result<(), String> {
        // Set background
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.draw_bars(&context, Color::WHITE)?;
        if context.is_sorted() {
            self.draw_bars(&context, Color::GREEN)?;
        }

        self.canvas.present();

        Ok(())
    }
}
