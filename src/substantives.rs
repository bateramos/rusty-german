use crate::read_file::read_file_lines;
use crate::types::{SubstantiveExercise};

pub fn get_substantives_list() -> Vec<SubstantiveExercise> {
    let mut list = Vec::new();
    if let Ok(lines) = read_file_lines("data/substantives.txt") {
        let mut article = "".to_owned();
        let mut tip = "".to_owned();

        for line in lines.iter() {
            if line == "" {
                article = "".to_owned();
                continue
            }

            if article == "" {
                let attr = line.split(";").collect::<Vec<&str>>();
                article = attr[0].to_owned();
                tip = attr[1].to_owned();
            } else {
                list.push(SubstantiveExercise { substantive: line.to_owned(), tip: tip.to_owned(), article: article.to_owned() });
            }

            //println!("{}", line);
        }
    }
    list
}
