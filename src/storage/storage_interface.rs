pub trait StorageInterface <S> {
    fn initialize() -> Self;
    fn save_exercise_result(&self, category: S, exercise: S, result: bool);
}
