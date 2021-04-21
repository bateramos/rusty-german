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

pub struct ExpectDescriptionExercise {
    pub expect: String,
    pub description: String,
}

pub struct PrepositionExercise {
    pub phrase: String,
    pub case: String,
    pub preposition: String
}

pub struct PrepositionCaseExercise {
    pub case: String,
    pub preposition: String
}

pub struct SubstantiveExercise {
    pub substantive: String,
    pub article: String,
    pub tip: String,
}

pub struct SubstantiveTipExercise {
    pub tip: String,
    pub article: String,
}

pub struct ConjunctionExercise {
    pub phrase: String,
    pub conjunction: String,
}

pub struct TemporalSatzeExercise {
    pub verbindung: String,
    pub phrase: String,
    pub expected_phrase: String,
}

pub struct RelativPronomenExercise {
    pub phrase: String,
    pub expected_phrases: Vec<String>,
}

pub trait Exercise {
    fn get_description(&self) -> String;
    fn get_expected_result(&self) -> String {
        "".to_string()
    }
    fn get_expected_results(&self) -> Vec<String> {
        let expected_result = self.get_expected_result();
        vec![expected_result]
    }
    fn get_sort_property(&self) -> String {
        "".to_string()
    }
}

impl Exercise for ExpectDescriptionExercise {
    fn get_description(&self) -> String {
        self.description.to_string()
    }

    fn get_expected_result(&self) -> String {
        self.expect.to_string()
    }
}

impl Exercise for SubstantiveTipExercise {
    fn get_description(&self) -> String {
        self.tip.to_string()
    }

    fn get_expected_result(&self) -> String {
        self.article.to_string()
    }
}

impl Exercise for SubstantiveExercise {
    fn get_description(&self) -> String {
        self.substantive.to_string()
    }

    fn get_expected_result(&self) -> String {
        self.article.to_string()
    }

    fn get_sort_property(&self) -> String {
        self.substantive.to_string()
    }
}

impl Exercise for PrepositionExercise {
    fn get_description(&self) -> String {
        self.phrase.to_string()
    }

    fn get_expected_result(&self) -> String {
        self.preposition.to_string()
    }

    fn get_sort_property(&self) -> String {
        self.case.to_string()
    }
}

impl Exercise for PrepositionCaseExercise {
    fn get_description(&self) -> String {
        format!("akkusativ dativ genitiv\nPreposition: {}", self.preposition)
    }

    fn get_expected_result(&self) -> String {
        self.case.to_string()
    }
}

impl Exercise for TemporalSatzeExercise {
    fn get_description(&self) -> String {
        self.phrase.to_string()
    }

    fn get_expected_result(&self) -> String {
        self.expected_phrase.to_string()
    }

    fn get_sort_property(&self) -> String {
        self.verbindung.to_string()
    }
}

impl Exercise for ConjunctionExercise {
    fn get_description(&self) -> String {
        self.phrase.to_string()
    }

    fn get_expected_result(&self) -> String {
        self.conjunction.to_string()
    }

    fn get_sort_property(&self) -> String {
        self.conjunction.to_string()
    }
}

impl Exercise for RelativPronomenExercise {
    fn get_description(&self) -> String {
        self.phrase.to_string()
    }

    fn get_expected_results(&self) -> Vec<String> {
        self.expected_phrases.clone()
    }
}
