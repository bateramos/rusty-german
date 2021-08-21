use rusty_german_entity_macro::RustyEntity;
use rusty_german_types::Exercise;

#[derive(Debug)]
pub enum VerbType {
    Starke, Schwache
}

#[derive(Debug, PartialEq)]
pub enum ZeitType {
    Praesens, Praeteritum, Perfekt, Plusquamperfekt, Futur
}

pub struct VerbExercise {
    pub verb: String,
    pub verben: [Verb; 5],
    pub verb_type: VerbType,
    pub obs: Option<String>
}

pub struct Verb {
    pub name: String,
    pub verb_type: VerbType,
    pub zeit_type: ZeitType,
    pub conjugations: [String; 6],
}

pub struct Pronoun {
    pub name: &'static str,
    pub subjects: [&'static str; 6]
}

pub struct Article {
    pub name: &'static str,
    pub case: &'static str,
    pub gender: &'static str
}

#[derive(RustyEntity)]
pub struct ExpectDescriptionExercise {
    #[entity(expected_result)]
    pub expect: String,
    #[entity(description)]
    pub description: String,
}

#[derive(RustyEntity)]
pub struct PrepositionExercise {
    #[entity(description)]
    pub phrase: String,
    #[entity(sort)]
    pub case: String,
    #[entity(expected_result)]
    pub preposition: String
}

pub struct PrepositionCaseExercise {
    pub case: String,
    pub preposition: String
}

#[derive(RustyEntity)]
pub struct SubstantiveExercise {
    #[entity(sort)]
    #[entity(description)]
    pub substantive: String,
    #[entity(expected_result)]
    pub article: String,
    pub tip: String,
}

#[derive(RustyEntity)]
pub struct SubstantiveTipExercise {
    #[entity(description)]
    pub tip: String,
    #[entity(expected_result)]
    pub article: String,
}

#[derive(RustyEntity)]
pub struct ConjunctionExercise {
    #[entity(description)]
    pub phrase: String,
    #[entity(expected_result)]
    pub conjunction: String,
}

#[derive(RustyEntity)]
pub struct TemporalSatzeExercise {
    #[entity(sort)]
    pub verbindung: String,
    #[entity(description)]
    pub phrase: String,
    #[entity(expected_result)]
    pub expected_phrase: String,
}

#[derive(RustyEntity)]
pub struct MultiOptionsExercise {
    #[entity(description)]
    pub phrase: String,
    #[entity(expected_results)]
    pub expected_phrases: Vec<String>,
}

// 
// [derive(RustyParser(
//    file = data/adjektivendungen.txt
//    regex = (?m)(^[a-zA-Z0-9_ ]*\n)?((^\w+);(\w+);(\w+);(\w+);?(\w+)?)
//    map = {{ category = {1}, case = {3}, end = {4..7} }}
// )]
#[derive(RustyEntity)]
pub struct AdjektivendungenExercise {
    #[entity(description)]
    pub category: String,
    #[entity(expected_result)]
    pub end: String,
}

#[derive(RustyEntity)]
pub struct VerbPrapositionExercise {
    pub category: String,
    #[entity(description)]
    pub verb: String,
    #[entity(expected_result)]
    pub preposition: String,
}

impl Exercise for PrepositionCaseExercise {
    fn get_description(&self) -> String {
        format!("akkusativ dativ genitiv\nPreposition: {}", self.preposition)
    }

    fn get_expected_result(&self) -> String {
        self.case.to_string()
    }
}
