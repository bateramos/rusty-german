use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap()]
pub struct Args {
    /// Run all exercises in sequence
    #[clap(long)]
    pub all: bool,
    #[clap(subcommand)]
    pub verb: Option<VerbSubcommand>,
}

#[derive(Subcommand)]
pub enum VerbSubcommand {
    /// Run exercise for a specific verb or random verbs if black
    Verb { verb: Option<String> }
}
