use crate::read_file::read_file_lines;
use crate::types::PrepositionExercise;

pub fn get_prepositions_exercises() -> Vec<PrepositionExercise> {
    let mut prepositions = Vec::new();
    if let Ok(lines) = read_file_lines("data/prepositions.txt") {
        for line in lines.iter() {
            if line == "" {
                continue;
            }

            let attr = line.split(";").collect::<Vec<&str>>();
            let phrase = attr[0].to_owned();
            let preposition = attr[1].to_owned();

            prepositions.push(PrepositionExercise { phrase, preposition });
        }
    }

    prepositions
}
