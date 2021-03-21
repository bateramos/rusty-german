use std::collections::HashSet;

use crate::read_file::read_file_lines;
use crate::types::{PrepositionExercise,PrepositionCaseExercise};

pub fn get_prepositions_exercises() -> Vec<PrepositionExercise> {
    let mut prepositions = Vec::new();
    if let Ok(lines) = read_file_lines("data/prepositions.txt") {
        let mut preposition = "".to_owned();
        let mut case = "".to_owned();
        let mut translation = "".to_owned();
        for line in lines.iter() {
            if line == "" {
                preposition = "".to_owned();
                case = "".to_owned();
                continue;
            }

            let configuration_line = preposition == "" && case == "";

            if configuration_line {
                let attr = line.split(";").collect::<Vec<&str>>();
                preposition = attr[0].to_owned();
                translation = attr[1].to_owned();
                case = attr[2].to_owned();
            } else {
                let phrase = line.to_owned() + "(" + &translation + " | " + &case + ")";

                prepositions.push(PrepositionExercise { phrase, case: case.to_owned(), preposition: preposition.to_owned() });
            }
        }
    }

    prepositions
}

pub fn get_prepositions_case_exercises() -> Vec<PrepositionCaseExercise> {
    let mut added_prepositions = HashSet::new();
    let mut prepositions = Vec::new();
    if let Ok(lines) = read_file_lines("data/prepositions.txt") {
        let mut preposition = "".to_owned();
        let mut case = "".to_owned();
        for line in lines.iter() {
            if line == "" {
                preposition = "".to_owned();
                case = "".to_owned();
                continue;
            }

            let configuration_line = preposition == "" && case == "";

            if configuration_line {
                let attr = line.split(";").collect::<Vec<&str>>();
                preposition = attr[0].to_owned();
                case = attr[2].to_owned();

                match added_prepositions.insert(preposition.to_owned()) {
                    true => prepositions.push(PrepositionCaseExercise { case: case.to_owned(), preposition: preposition.to_owned() }),
                    false => {
                        let exercise_index = prepositions.iter()
                            .position(|e| e.preposition == preposition)
                            .unwrap();
                        let exercise = prepositions.remove(exercise_index);
                        let mut cases = Vec::new();
                        cases.push(case.to_owned());
                        cases.push(exercise.case);
                        cases.sort();

                        let cases = cases.into_iter()
                            .map(|c| format!("{} ", c))
                            .collect::<String>()
                            .trim()
                            .to_string();
                        prepositions.push(PrepositionCaseExercise { case: cases, preposition: preposition.to_string() });
                    }
                }
            }
        }
    }

    prepositions
}

#[cfg(test)]
#[path = "prepositions_tests.rs"]
mod tests;
