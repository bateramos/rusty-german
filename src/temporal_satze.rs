use crate::read_file::read_file_lines;
use crate::types::TemporalSatzeExercise;

pub fn get_temporal_satze_exercises() -> Vec<TemporalSatzeExercise> {
    let mut exercises = Vec::new();
    if let Ok(lines) = read_file_lines("data/temporalsatze.txt") {
        let mut verbindung = "".to_owned();

        for line in lines.iter() {
            if line == "" {
                verbindung = "".to_owned();
                continue;
            }

            let attr = line.split(";").collect::<Vec<&str>>();
            if attr.len() == 1 {
                verbindung = attr[0].to_owned();
            } else if attr.len() == 2 {
                let phrase = attr[0].to_owned();
                let expected_phrase = attr[1].to_owned();

                exercises.push(TemporalSatzeExercise { phrase, verbindung: verbindung.to_owned(), expected_phrase });
            }
        }
    }

    exercises
}

#[cfg(test)]
#[path = "temporal_satze_tests.rs"]
mod tests;
