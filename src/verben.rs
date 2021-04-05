use arrayvec::ArrayVec;
use select::document::Document;
use select::predicate::{Attr, Name};

use crate::types::{VerbExercise, ExpectDescriptionExercise, Verb, VerbType, ZeitType};
use crate::read_file::read_file_lines;

#[derive(Debug, Copy, Clone)]
enum PrefixVerb {
    Sein, Haben
}

fn str(s: &str) -> String {
    String::from(s)
}

fn create_verb(name: &str, verb_type: VerbType, zeit_type: ZeitType, conjugations: &Vec<String>) -> Verb {
    let vec_conj : ArrayVec<[String; 6]> = conjugations.iter().map(|x| x.to_owned()).collect();
    Verb { name: str(name), verb_type, zeit_type, conjugations: vec_conj.into_inner().unwrap() }
}

fn create_verb_prefix(name: &str, perfect_form: &str, prefix_verb: PrefixVerb, verb_type: VerbType, zeit_type: ZeitType) -> Verb {
    let conjugations : [&str; 6] = match prefix_verb {
        PrefixVerb::Sein => {
            match zeit_type {
                ZeitType::Perfekt => ["bin", "bist", "ist", "sind", "seid", "sind"],
                ZeitType::Plusquamperfekt => ["war", "warst", "war", "waren", "wart", "waren"],
                ZeitType::Futur => ["werde", "wirst", "wird", "werden", "werdet", "werden"],
                _ => panic!("Wrong ZeitType ({:?}) for {}", zeit_type, name)
                
            }
        }
        PrefixVerb::Haben => {
            match zeit_type {
                ZeitType::Perfekt => ["habe", "hast", "hat", "haben", "habt", "haben"],
                ZeitType::Plusquamperfekt => ["hatte", "hattest", "hatte", "hatten", "hattet", "hatten"],
                ZeitType::Futur => ["werde", "wirst", "wird", "werden", "werdet", "werden"],
                _ => panic!("Wrong ZeitType ({:?}) for {}", zeit_type, name)
            }
        }
    };

    let vec_conj : ArrayVec<[String; 6]> = conjugations.iter().map(|x| format!("{} {}", x, perfect_form)).collect();
    Verb { name: str(name), verb_type, zeit_type, conjugations: vec_conj.into_inner().unwrap() }
}

fn create_schwache_verben(name: &str, prefix_verb: PrefixVerb, obs: Option<String>) -> VerbExercise {
    let mut prefix = str(name);
    let sufix = prefix.split_off(name.len() - 2);

    let ending_with_rn = sufix == "rn";
    let ending_with_alveolar = prefix.ends_with("gn") || prefix.ends_with("d") || prefix.ends_with("t");

    let person = if ending_with_alveolar {
        ["e", "est", "et", "en", "et", "en"]
    } else if ending_with_rn {
        ["re", "rst", "rt", "rn", "rt", "rn"]
    } else {
        ["e", "st", "t", "en", "t", "en"]
    };

    let person_past = if ending_with_alveolar {
        ["ete", "etest", "ete", "eten", "etet", "eten"]
    } else if ending_with_rn {
        ["rte", "rtest", "rte", "rten", "rtet", "rten"]
    } else {
        ["te", "test", "te", "ten", "tet", "ten"]
    };

    let present_form : Vec<String> = person.iter().map(|x| prefix.to_owned() + x).collect();
    let past_form : Vec<String> = person_past.iter().map(|x| prefix.to_owned() + x).collect();

    let verb_has_prefix = name.starts_with("ge") || name.starts_with("er") || name.starts_with("be") || name.starts_with("ver");

    let past_tense = if verb_has_prefix || name.ends_with("ieren") {
        present_form[4].to_owned()
    } else {
        str("ge") + &present_form[4].to_owned()
    };

    VerbExercise { verb: str(name), verb_type: VerbType::Schwache, obs, verben: [
        create_verb(name, VerbType::Schwache, ZeitType::Praesens, &present_form),
        create_verb(name, VerbType::Schwache, ZeitType::Praeteritum, &past_form),
        create_verb_prefix(name, &past_tense, prefix_verb, VerbType::Schwache, ZeitType::Perfekt),
        create_verb_prefix(name, &past_tense, prefix_verb, VerbType::Schwache, ZeitType::Plusquamperfekt),
        create_verb_prefix(name, name, prefix_verb, VerbType::Schwache, ZeitType::Futur)
    ] }
}

fn create_regular_stark_verben(name: &str, perfect_form: &str, past_tense: &str, prefix_verb: PrefixVerb, present_2_3_form_config: Option<String>) -> VerbExercise {
    let mut present_prefix = str(name);
    present_prefix.truncate(present_prefix.len() - 2);
    let mut perfect_prefix = str(perfect_form);
    perfect_prefix.truncate(perfect_prefix.len() - 2);

    let ending_with_s = present_prefix.ends_with("s");

    let person = if ending_with_s {
        ["e", "t", "t", "en", "t", "en"]
    } else {
        ["e", "st", "t", "en", "t", "en"]
    };
    let person_past = if ending_with_s {
        ["", "est", "", "en", "t", "en"]
    } else {
        ["", "st", "", "en", "t", "en"]
    };

    let mut present_form : Vec<String> = person.iter().map(|x| present_prefix.to_owned() + x).collect();
    let past_form : Vec<String> = person_past.iter().map(|x| perfect_prefix.to_owned() + x).collect();

    if let Some(config) = present_2_3_form_config {
        let config_vec = config.split(">").collect::<Vec<&str>>();
        let from = config_vec[0];
        let to = config_vec[1];
        present_form[1] = present_form[1].replace(from, to);
        present_form[2] = present_form[2].replace(from, to);
    }

    create_starke_verben(name, past_tense, prefix_verb, &present_form, &past_form, None)
}

fn create_starke_verben(name: &str, past_tense: &str, prefix_verb: PrefixVerb, present_form: &Vec<String>, past_form: &Vec<String>, obs: Option<String>) -> VerbExercise {
    VerbExercise { verb: str(name), verb_type: VerbType::Starke, obs, verben: [
        create_verb(name, VerbType::Starke, ZeitType::Praesens, present_form),
        create_verb(name, VerbType::Starke, ZeitType::Praeteritum, past_form),
        create_verb_prefix(name, past_tense, prefix_verb, VerbType::Starke, ZeitType::Perfekt),
        create_verb_prefix(name, past_tense, prefix_verb, VerbType::Starke, ZeitType::Plusquamperfekt),
        create_verb_prefix(name, name, prefix_verb, VerbType::Starke, ZeitType::Futur)
    ] }
}

pub fn get_starken_verben() -> Vec<VerbExercise> {
    let mut verben = Vec::new();
    if let Ok(lines) = read_file_lines("data/starke_verben.txt") {
        let mut line_number = 0;
        let mut verb_name = "";
        let mut verb_name_past = "";
        let mut verb_obs = None;
        let mut prefix_verb = PrefixVerb::Sein;
        let mut present_conjugation = Vec::new();
        let mut past_conjugation = Vec::new();
        for line in lines.iter() {
            if line == "" {
                continue;
            }

            if line.starts_with("#") {
                let attr = line.split("#").collect::<Vec<&str>>()[1]
                    .split(";").collect::<Vec<&str>>();
                verb_name = attr[0];
                let verb_name_perfect = attr[1];
                verb_name_past = attr[2];
                prefix_verb = match attr[3] {
                    "sein" => PrefixVerb::Sein,
                    "haben" => PrefixVerb::Haben,
                    _ => panic!("Wrong prefix verb for {}. value: {}", verb_name, attr[2])
                };
                let options = match attr.len() {
                    5 => Some(str(attr[4])),
                    _ => None
                };
                verben.push(create_regular_stark_verben(verb_name, verb_name_perfect, verb_name_past, prefix_verb, options));
                continue;
            }

            if line_number == 0 {
                let attr = line.split(";").collect::<Vec<&str>>();
                verb_obs = match attr.len() {
                    4 => Some(attr[3].to_owned()),
                    3 => None,
                    _ => panic!("Line with wrong format. {}", line)
                };
                verb_name = attr[0];
                verb_name_past = attr[1];
                prefix_verb = match attr[2] {
                    "sein" => PrefixVerb::Sein,
                    "haben" => PrefixVerb::Haben,
                    _ => panic!("Wrong prefix verb for {}. value: {}", verb_name, attr[2])
                };
                line_number += 1;
            } else if line_number == 1 {
                present_conjugation = line.split(";").map(|x| x.to_owned()).collect();
                assert!(present_conjugation.len() == 6, "Line with wrong format: {}", line);
                line_number += 1;
            } else if line_number == 2 {
                past_conjugation = line.split(";").map(|x| x.to_owned()).collect();
                assert!(past_conjugation.len() == 6, "Line with wrong format: {}", line);
                line_number += 1;
            }

            if line_number == 3 {
                verben.push(create_starke_verben(verb_name, verb_name_past, prefix_verb, &present_conjugation, &past_conjugation, verb_obs));

                verb_name = "";
                verb_obs = None;

                line_number = 0;
            }
        }
    }

    verben
}

pub fn get_schwachen_verben() -> Vec<VerbExercise> {
    let mut verben = Vec::new();
    if let Ok(lines) = read_file_lines("data/schwachen_verben.txt") {
        for line in lines.iter() {
            if line != "" {
                let attr = line.split(";").collect::<Vec<&str>>();
                let obs = match attr.len() {
                    2 => None,
                    3 => Some(attr[2].to_owned()),
                    _ => panic!("Line with the wrong format. {}", line)
                };
                let verb_name = attr[0];
                let prefix_verb = match attr[1] {
                    "sein" => PrefixVerb::Sein,
                    "haben" => PrefixVerb::Haben,
                    _ => panic!("Wrong prefix verb for {}. value: {}", verb_name, attr[1])
                };
                verben.push(create_schwache_verben(verb_name, prefix_verb, obs));
            }
        }
    }

    verben
}

pub async fn get_verben_phrase_exercise(verb: &str) -> Result<Vec<ExpectDescriptionExercise>, Box<dyn std::error::Error>> {
    let html = reqwest::get(format!("https://dict.leo.org/englisch-deutsch/{}", verb))
        .await?
        .text()
        .await?;

    let document = Document::from(&html[..]);

    let mut phrases : Vec<String> = vec![];
    for node in document.find(Attr("id", "section-example")) {
        for samp in node.find(Name("samp")) {
            phrases.push(samp.text());
        }
    }

    let mut exercises : Vec<ExpectDescriptionExercise> = vec![];

    let mut n = 0;
    while n < phrases.len() {
        let description = format!("{}\nWrite translation to:\n{}", verb, phrases[n]);
        let expect = phrases[n + 1].to_string();

        exercises.push(ExpectDescriptionExercise { expect, description });
        n += 2;
    }

    Ok(exercises)
}

#[cfg(test)]
#[path = "verben_tests.rs"]
mod tests;
