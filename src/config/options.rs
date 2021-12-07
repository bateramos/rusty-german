use clap::Parser;

#[derive(Parser)]
#[clap()]
pub struct Args {
    /// Run all exercises in sequence
    #[clap(long)]
    pub all: bool,
}
