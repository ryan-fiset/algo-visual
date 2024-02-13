use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use super::BAR_SEGMENT_IN_PXS;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_bar(&mut self, x: i32, y: i32, screen_height: i32) -> Result<(), String> {
        let bar_height = y as u32 * BAR_SEGMENT_IN_PXS;
        let offset = screen_height - bar_height as i32;

        self.canvas.fill_rect(Rect::new(
            x * BAR_SEGMENT_IN_PXS as i32,
            y + offset,
            BAR_SEGMENT_IN_PXS,
            y as u32 * BAR_SEGMENT_IN_PXS,
        ))?;
        Ok(())
    }

    pub fn draw(&mut self, vector: &Vec<u32>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        let mut index = 0;
        let highest_value = vector.iter().max().unwrap();
        for value in vector {
            self.draw_bar(
                index,
                *value as i32,
                *highest_value as i32 * BAR_SEGMENT_IN_PXS as i32,
            )?;
            index += 1;
        }

        self.canvas.present();

        Ok(())
    }
}
