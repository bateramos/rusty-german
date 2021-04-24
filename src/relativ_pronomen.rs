use crate::read_file_multi_options_exercise::get_multiple_options_exercise;
use crate::types::MultiOptionsExercise;

pub fn get_relativ_pronomen_exercises() -> Vec<MultiOptionsExercise> {
    get_multiple_options_exercise("data/relativ-pronomen.txt")
}
