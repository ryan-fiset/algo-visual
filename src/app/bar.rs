use clap::Parser;

use crate::app::Args;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Bar {
    pub y: u32,
    pub offset: u32,
    pub bar_height: u32,
    pub bar_segment: u32,
}

impl Bar {
    pub fn new(y: u32) -> Self {
        let args = Args::parse();

        let bar_segment = args.bar_segment;
        let max_height = args.bar_segment * args.vec_size;

        let bar_height = y * bar_segment;
        let offset = max_height - bar_height;

        Self {
            y,
            offset,
            bar_height,
            bar_segment,
        }
    }
}
