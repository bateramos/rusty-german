use arrayvec::ArrayVec;

use crate::types::{Verb, VerbType, ZeitType};
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

fn create_schwache_verben(name: &str, past_tense: &str, prefix_verb: PrefixVerb) -> [Verb; 5] {
    let person = ["e", "st", "t", "en", "t", "en"];
    let person_past = ["te", "test", "te", "ten", "tet", "ten"];
    let mut prefix = str(name);

    prefix.truncate(name.len() - 2);

    let present_form : Vec<String> = person.iter().map(|x| prefix.to_owned() + x).collect();
    let past_form : Vec<String> = person_past.iter().map(|x| prefix.to_owned() + x).collect();

    [
        create_verb(name, VerbType::Schwache, ZeitType::Praesens, &present_form),
        create_verb(name, VerbType::Schwache, ZeitType::Praeteritum, &past_form),
        create_verb_prefix(name, past_tense, prefix_verb, VerbType::Schwache, ZeitType::Perfekt),
        create_verb_prefix(name, past_tense, prefix_verb, VerbType::Schwache, ZeitType::Plusquamperfekt),
        create_verb_prefix(name, name, prefix_verb, VerbType::Schwache, ZeitType::Futur)
    ]
}

fn create_starke_verben(name: &str, past_tense: &str, prefix_verb: PrefixVerb, present_form: &Vec<String>, past_form: &Vec<String>) -> [Verb; 5] {
    [
        create_verb(name, VerbType::Starke, ZeitType::Praesens, present_form),
        create_verb(name, VerbType::Starke, ZeitType::Praeteritum, past_form),
        create_verb_prefix(name, past_tense, prefix_verb, VerbType::Starke, ZeitType::Perfekt),
        create_verb_prefix(name, past_tense, prefix_verb, VerbType::Starke, ZeitType::Plusquamperfekt),
        create_verb_prefix(name, name, prefix_verb, VerbType::Starke, ZeitType::Futur)
    ]
}

pub fn get_verben() -> Vec<[Verb; 5]> {
    let mut verben = Vec::new();
    if let Ok(lines) = read_file_lines("data/starke_verben.txt") {
        let mut line_number = 0;
        let mut verb_name = "";
        let mut verb_name_past = "";
        let mut prefix_verb = PrefixVerb::Sein;
        let mut present_conjugation = Vec::new();
        let mut past_conjugation = Vec::new();
        for line in lines.iter() {
            if line_number == 0 {
                let attr = line.split(";").collect::<Vec<&str>>();
                verb_name = attr[0];
                verb_name_past = attr[1];
                prefix_verb = match attr[2] {
                    "sein" => PrefixVerb::Sein,
                    "haben" => PrefixVerb::Haben,
                    _ => panic!("Wrong prefix verb for {}. value: {}", verb_name, attr[2])
                };
            } else if line_number == 1 {
                present_conjugation = line.split(";").map(|x| x.to_owned()).collect();
            } else if line_number == 2 {
                past_conjugation = line.split(";").map(|x| x.to_owned()).collect();
            }

            if line_number == 3 {
                verben.push(create_starke_verben(verb_name, verb_name_past, prefix_verb, &present_conjugation, &past_conjugation));
                line_number = 0;
            } else {
                line_number += 1;
            }
        }
    }
    if let Ok(lines) = read_file_lines("data/schwachen_verben.txt") {
        for line in lines.iter() {
            if line != "" {
                let attr = line.split(";").collect::<Vec<&str>>();
                let verb_name = attr[0];
                let verb_name_past = attr[1];
                let prefix_verb = match attr[2] {
                    "sein" => PrefixVerb::Sein,
                    "haben" => PrefixVerb::Haben,
                    _ => panic!("Wrong prefix verb for {}. value: {}", verb_name, attr[2])
                };
                verben.push(create_schwache_verben(verb_name, verb_name_past, prefix_verb));
            }
        }
    }

    verben
}

#[cfg(test)]
#[path = "verben_tests.rs"]
mod tests;
