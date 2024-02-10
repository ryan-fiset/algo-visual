mod algorithm;

use algorithm::Algorithm;

pub enum AppState {
    Running,
    Suspended,
}

pub struct AppContext {
    pub algorithm: Algorithm,
    pub vector: Vec<u32>,
    pub state: AppState,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            algorithm: Algorithm::BubbleSort, // TODO: Make this modifiable via argument to fn
            vector: vec![6, 0, 3, 5, 7, 1, 4, 2, 8, 10, 11, 9], // TODO: Remove magic numbers
            state: AppState::Suspended,
        }
    }
}
