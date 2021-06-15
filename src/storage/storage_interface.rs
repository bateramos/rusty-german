pub trait StorageInterface {
    fn initialize() -> Self;
    fn save_exercise_result(&self, category: &str, exercise: &str, result: bool);
}
