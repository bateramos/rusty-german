pub trait StorageInterface {
    fn initialize() -> Self;
    fn save_exercise_result<S: Into<String>>(&self, category: S, exercise: S, result: bool);
}
