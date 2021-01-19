use crate::read_file::read_file_lines;
use crate::types::PrepositionExercise;

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

#[cfg(test)]
#[path = "prepositions_tests.rs"]
mod tests;
