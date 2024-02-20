use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use super::bar::Bar;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_bar(&mut self, x: i32, bar: Bar) -> Result<(), String> {
        self.canvas.fill_rect(Rect::new(
            x,
            bar.offset as i32,
            bar.bar_segment,
            bar.bar_height,
        ))?;
        Ok(())
    }

    pub fn draw(&mut self, vector: &Vec<Bar>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        let mut index = 0;
        for bar in vector {
            self.draw_bar(index * bar.bar_segment as i32, *bar)?;
            index += 1;
        }

        self.canvas.present();

        Ok(())
    }
}
