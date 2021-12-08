use std::ops::RangeBounds;
use rand::thread_rng;
use rand::seq::SliceRandom;

use rusty_german_types::Exercise;

pub type OnAnswer <'a> = Box<dyn Fn(bool) + 'a>;
pub type CreateOnAnswer <'a> = &'a dyn Fn(String, String, String) -> OnAnswer<'a>;
pub type ProcessInput <'a> = &'a dyn Fn(Vec<String>, Option<OnAnswer>);

pub fn run_exercise<T, R>(exercise_fn: &dyn Fn() -> Vec<T>, range: R, random_exercises: bool, process_input: ProcessInput, on_answer: CreateOnAnswer)
    where
        T: Exercise,
        R: RangeBounds<usize>,
{
    let mut exercises = exercise_fn();
    if random_exercises {
        let mut rng = thread_rng();
        exercises.shuffle(&mut rng);
    }
    let mut exercises_subset = exercises.drain(range).collect::<Vec<T>>();
    exercises_subset.sort_by_key(|a| a.get_sort_property());
    for exercise in exercises_subset.iter() {
        println!("{}", exercise.get_description());
        let category = std::any::type_name::<T>().to_string();
        let expected_values = exercise.get_expected_results().into_iter().fold("".to_owned(), |acc, item| {
            format!("{}|{}", acc, item)
        });

        process_input(exercise.get_expected_results(), Some(on_answer(category, exercise.get_description(), expected_values)));
    }
}

