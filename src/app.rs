use std::str::FromStr;

use clap::Parser;
use rand::seq::SliceRandom;

mod algorithm;
pub mod bar;
mod cli;
pub mod renderer;

use algorithm::Algorithm;
use bar::Bar;
use cli::Args;

use crate::app::algorithm::run_algo;
#[derive(PartialEq, Debug)]
pub enum AppState {
    Running,
    Suspended,
}

pub struct AppContext {
    pub algorithm: Algorithm,
    pub vector: Vec<Bar>,
    pub state: AppState,
    pub screen_height: u32,
    pub bar_segment: u32,
}

impl AppContext {
    pub fn new() -> Self {
        let args = Args::parse();

        let mut vector: Vec<Bar> = Vec::new();

        for i in 1..=args.vec_size {
            vector.push(Bar::new(i))
        }

        let mut rng = rand::thread_rng();
        vector.shuffle(&mut rng);

        Self {
            algorithm: Algorithm::from_str(&(args.algo).to_uppercase()).unwrap(),
            vector,
            state: AppState::Suspended,
            screen_height: args.bar_segment * args.vec_size,
            bar_segment: args.bar_segment,
        }
    }

    pub fn change_state(&mut self) {
        self.state = match self.state {
            AppState::Running => AppState::Suspended,
            AppState::Suspended => AppState::Running,
        };
    }

    pub fn next_tick(&mut self) {
        if let AppState::Suspended = self.state {
            println!("Paused");
            return;
        }

        run_algo(self);

        println!("Running");
    }

    pub fn is_sorted(&self) -> bool {
        self.vector.windows(2).all(|w| w[0] < w[1])
    }
}
