use regex::Regex;

use crate::types::AdjektivendungenExercise;
use crate::read_file::read_file;

pub fn get_adjetivendungen_exercise() -> Vec<AdjektivendungenExercise> {
    let mut vec = vec![];
    if let Ok(text) = read_file("data/adjektivendungen.txt") {

        let re = Regex::new(r"(?m)(^[^;\n]*\n)?((^\w+);(\w+);(\w+);(\w+);?(\w+)?)").unwrap();
        let mut category : Option<String> = None;
        for cap in re.captures_iter(&text) {
            if let Some(cat) = cap.get(1) {
                category.replace(cat.as_str().trim().to_owned());
            }

            let mut end;
            end = format!("{} {} {}", cap[4].trim(), cap[5].trim(), cap[6].trim());
            if let Some(pl) = cap.get(7) {
                end = format!("{} {}", end, pl.as_str());
            }

            vec.push(AdjektivendungenExercise {
                category: format!("Category: {} Case: {}", category.clone().unwrap(), &cap[3].trim()),
                end,
            });
        }
    }

    vec
}
