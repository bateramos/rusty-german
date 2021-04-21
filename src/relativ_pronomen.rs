use crate::read_file::read_file_lines;
use crate::types::RelativPronomenExercise;

pub fn get_relativ_pronomen_exercises() -> Vec<RelativPronomenExercise> {
    let mut exercise = vec![];
    if let Ok(lines) = read_file_lines("data/relativ-pronomen.txt") {

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

            exercise.push(RelativPronomenExercise { phrase, expected_phrases });
        }
    }
    exercise
}
