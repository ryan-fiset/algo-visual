use rand::seq::SliceRandom;
use std::str::FromStr;

use super::{bar::Bar, AppContext};

pub enum Algorithm {
    BubbleSort,
    QuickSort,
    BogoSort,
}

pub fn run_algo(context: &mut AppContext) {
    match context.algorithm {
        Algorithm::BubbleSort => bubble_sort(context),
        Algorithm::QuickSort => todo!(),
        Algorithm::BogoSort => bogo_sort(context),
    }
}

fn bubble_sort(context: &mut AppContext) {
    let len = context.vector.len();

    let mut i = 0;
    let mut j = 0;

    let mut swapped = false;

    while i < len - 1 {
        while j < len - 1 {
            if context.vector[j].y > context.vector[j + 1].y {
                context.vector.swap(j, j + 1);
                swapped = true;
            }
            j += 1;
        }

        if swapped == false {
            break;
        }

        i += 1;
    }
}

fn bogo_sort(context: &mut AppContext) {
    let mut rng = rand::thread_rng();
    context.vector.shuffle(&mut rng);
}

impl FromStr for Algorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUBBLESORT" => Ok(Algorithm::BubbleSort),
            "QUICKSORT" => Ok(Algorithm::QuickSort),
            "BOGOSORT" => Ok(Algorithm::BogoSort),
            _ => Err(()),
        }
    }
}
