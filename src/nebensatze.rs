use crate::read_file_multi_options_exercise::get_multiple_options_exercise;
use crate::types::MultiOptionsExercise;

pub fn get_nebensatze_exercise() -> Vec<MultiOptionsExercise> {
    get_multiple_options_exercise("data/nebensatze.txt")
}
