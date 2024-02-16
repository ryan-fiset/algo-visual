pub use anyhow;

mod algorithm;
pub mod bar;
pub mod renderer;

use algorithm::Algorithm;

use self::bar::Bar;

pub enum AppState {
    Running,
    Suspended,
}

pub struct AppContext {
    pub algorithm: Algorithm,
    pub vector: Vec<Bar>,
    pub state: AppState,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            algorithm: Algorithm::BubbleSort, // TODO: Make this modifiable via argument to fn
            vector: vec![
                Bar::new(6),
                Bar::new(13),
                Bar::new(3),
                Bar::new(5),
                Bar::new(12),
                Bar::new(7),
                Bar::new(1),
                Bar::new(4),
                Bar::new(2),
                Bar::new(8),
                Bar::new(10),
                Bar::new(11),
                Bar::new(9),
            ], // TODO: Remove magic numbers
            state: AppState::Suspended,
        }
    }
}
