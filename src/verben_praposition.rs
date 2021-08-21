use regex::Regex;

use crate::types::VerbPrapositionExercise;
use crate::read_file::read_file;

pub fn get_verb_preposition_exercises() -> Vec<VerbPrapositionExercise> {
    let mut vec = vec![];
    if let Ok(text) = read_file("data/verben_praposition.txt") {

        let re = Regex::new(r"(?m)(.*)\n((sich )?([a-zA-Zä-ü]*) (.*))").unwrap();
        let mut category : Option<String> = None;
        for cap in re.captures_iter(&text) {
            if let Some(cat) = cap.get(1) {
                if cat.as_str() != "" {
                    category.replace(cat.as_str().trim().to_owned());
                }
            }

            let mut verb;
            verb = cap[4].trim().to_owned();
            if let Some(replexive) = cap.get(3) {
                verb = format!("{} {}", replexive.as_str().trim(), verb);
            }
            verb = format!("[{}] {} __", category.clone().unwrap(), verb);

            vec.push(VerbPrapositionExercise {
                category: category.clone().unwrap().to_owned(),
                preposition: cap[5].trim().to_owned(),
                verb,
            });
        }
    }

    vec
}

#[cfg(test)]
#[path = "verben_praposition_tests.rs"]
mod tests;
