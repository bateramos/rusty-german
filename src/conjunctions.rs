use crate::read_file::read_file_lines;
use crate::types::ConjunctionExercise;

pub fn get_conjunction_exercises() -> Vec<ConjunctionExercise> {
    let mut conjunctions = Vec::new();
    if let Ok(lines) = read_file_lines("data/conjunctions.txt") {
        let mut conjunction = "".to_owned();
        let mut translation = "".to_owned();
        for line in lines.iter() {
            if line == "" {
                conjunction = "".to_owned();
                continue;
            }

            let configuration_line = conjunction == "";

            if configuration_line {
                let attr = line.split(";").collect::<Vec<&str>>();
                conjunction = attr[0].to_owned();
                translation = attr[1].to_owned();
            } else {
                let phrase = line.to_owned() + "(" + &translation + ")";

                conjunctions.push(ConjunctionExercise { phrase, conjunction: conjunction.to_owned() });
            }
        }
    }

    conjunctions
}

#[cfg(test)]
#[path = "conjunctions_tests.rs"]
mod tests;
