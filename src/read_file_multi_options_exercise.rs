use crate::read_file::read_file_lines;
use crate::types::MultiOptionsExercise;

pub fn get_multiple_options_exercise(file_path: &str) -> Vec<MultiOptionsExercise> {
    let mut exercise = vec![];
    if let Ok(lines) = read_file_lines(file_path) {
        let mut iter = lines.iter();

        loop {
            let mut expected_phrases = vec![];
            let phrase = match iter.next() {
                Some(s) => s.to_string(),
                None => break,
            };

            loop {
                match iter.next() {
                    Some(s) => {
                        if s == "" {
                            break;
                        }
                        expected_phrases.push(s.to_string());
                    },
                    None => break,
                }
            }

            exercise.push(MultiOptionsExercise { phrase, expected_phrases });
        }
    }
    exercise
}
