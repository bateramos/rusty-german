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

#[derive(Debug)]
pub struct PrepositionExercise {
    pub phrase: String,
    pub case: String,
    pub preposition: String
}

#[derive(Debug)]
pub struct SubstantiveExercise {
    pub substantive: String,
    pub article: String,
    pub tip: String
}
