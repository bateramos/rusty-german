use super::{create_verb, str};
use crate::types::{VerbType, ZeitType};

#[test]
fn it_create_verb() {
    let verb = create_verb("sein", VerbType::Starke, ZeitType::Perfekt, &["bin", "bist", "ist", "sind", "seid", "sind"].iter().map(|x| str(x)).collect::<Vec<String>>());
    assert_eq!(verb.name, "sein");
    assert_eq!(verb.conjugations.len(), 6);
}

mod create_schwache_verben_tests {
    use super::super::{create_schwache_verben, PrefixVerb};

    #[test]
    fn it_should_create_regular_verb() {
        let verb = create_schwache_verben("brauchen", "gebraucht", PrefixVerb::Haben);
        assert_eq!(verb[0].conjugations, ["brauche", "brauchst", "braucht", "brauchen", "braucht", "brauchen"]);
        assert_eq!(verb[1].conjugations, ["brauchte", "brauchtest", "brauchte", "brauchten", "brauchtet", "brauchten"]);
    }
}
