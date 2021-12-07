mod options;

pub use options::Args;

pub struct MenuOption <'a> {
    pub text: &'a str,
    pub exec: &'a dyn Fn() -> (),
}

pub enum VerbExercise {
    OnlyPresent,
    All
}

