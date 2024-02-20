use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub vec_size: u32,
}
