pub const BAR_SEGMENT_IN_PXS: u32 = 60;
pub const SCREEN_HEIGHT: u32 = 780; // TODO: Remove magic value here

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Bar {
    pub y: u32,
    pub offset: u32,
    pub bar_height: u32,
}

impl Bar {
    pub fn new(y: u32) -> Self {
        let bar_height = y * BAR_SEGMENT_IN_PXS;
        let offset = SCREEN_HEIGHT - bar_height;

        Self {
            y,
            offset,
            bar_height,
        }
    }
}
