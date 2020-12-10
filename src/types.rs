#[derive(Debug, Copy, Clone)]
pub enum VerbType {
    Starke, Schwache
}

#[derive(Debug, PartialEq)]
pub enum ZeitType {
    Praesens, Praeteritum, Perfekt, Plusquamperfekt, Futur
}

pub struct Verb {
    pub name: String,
    pub verb_type: VerbType,
    pub zeit_type: ZeitType,
    pub conjugations: [String; 6]
}
