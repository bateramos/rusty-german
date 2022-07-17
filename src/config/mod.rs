mod options;

pub use options::{Args, VerbSubcommand};

pub struct MenuOption <'a> {
    pub text: &'a str,
    pub exec: &'a dyn Fn() -> (),
}

pub enum VerbExercise {
    OnlyPresent,
    All,
    OnlyVerb(String)
}

