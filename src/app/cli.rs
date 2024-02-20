use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub vec_size: u32,
    #[arg(short, default_value_t = 50)]
    pub bar_segment: u32,
}
