use super::{get_starken_verben, get_schwachen_verben, create_verb, str};
use crate::types::{VerbType, ZeitType};

#[test]
fn it_create_verb() {
    let verb = create_verb("sein", VerbType::Starke, ZeitType::Perfekt, &["bin", "bist", "ist", "sind", "seid", "sind"].iter().map(|x| str(x)).collect::<Vec<String>>());
    assert_eq!(verb.name, "sein");
    assert_eq!(verb.conjugations.len(), 6);
}

#[test]
fn it_should_load_starken_verben_files() {
    get_starken_verben();
}

#[test]
fn it_should_load_schwachen_verben_files() {
    get_schwachen_verben();
}

mod create_schwache_verben_tests {
    use super::super::{create_schwache_verben, PrefixVerb};

    #[test]
    fn it_should_create_regular_verb() {
        let verb = create_schwache_verben("brauchen", PrefixVerb::Haben);
        assert_eq!(verb[0].conjugations, ["brauche", "brauchst", "braucht", "brauchen", "braucht", "brauchen"]);
        assert_eq!(verb[1].conjugations, ["brauchte", "brauchtest", "brauchte", "brauchten", "brauchtet", "brauchten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_t_alveolar() {
        let verb = create_schwache_verben("arbeiten", PrefixVerb::Haben);
        assert_eq!(verb[0].conjugations, ["arbeite", "arbeitest", "arbeitet", "arbeiten", "arbeitet", "arbeiten"]);
        assert_eq!(verb[1].conjugations, ["arbeitete", "arbeitetest", "arbeitete", "arbeiteten", "arbeitetet", "arbeiteten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_r_alveolar() {
        let verb = create_schwache_verben("reden", PrefixVerb::Haben);
        assert_eq!(verb[0].conjugations, ["rede", "redest", "redet", "reden", "redet", "reden"]);
        assert_eq!(verb[1].conjugations, ["redete", "redetest", "redete", "redeten", "redetet", "redeten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_gn_alveolar() {
        let verb = create_schwache_verben("regnen", PrefixVerb::Haben);
        assert_eq!(verb[0].conjugations, ["regne", "regnest", "regnet", "regnen", "regnet", "regnen"]);
        assert_eq!(verb[1].conjugations, ["regnete", "regnetest", "regnete", "regneten", "regnetet", "regneten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_rn() {
        let verb = create_schwache_verben("erinnern", PrefixVerb::Haben);
        assert_eq!(verb[0].conjugations, ["erinnere", "erinnerst", "erinnert", "erinnern", "erinnert", "erinnern"]);
        assert_eq!(verb[1].conjugations, ["erinnerte", "erinnertest", "erinnerte", "erinnerten", "erinnertet", "erinnerten"]);
    }

    #[test]
    fn it_should_create_past_tense_starting_with_ge() {
        let verb = create_schwache_verben("gehören", PrefixVerb::Haben);
        assert_eq!(verb[2].conjugations[0], "habe gehört");
    }

    #[test]
    fn it_should_create_past_tense_starting_with_er() {
        let verb = create_schwache_verben("erinnern", PrefixVerb::Haben);
        assert_eq!(verb[2].conjugations[0], "habe erinnert");
    }

    #[test]
    fn it_should_create_past_tense_ending_with_ieren() {
        let verb = create_schwache_verben("studieren", PrefixVerb::Haben);
        assert_eq!(verb[2].conjugations[0], "habe studiert");
    }

    #[test]
    fn it_should_create_past_tense_for_regular_verb() {
        let verb = create_schwache_verben("leben", PrefixVerb::Haben);
        assert_eq!(verb[2].conjugations[0], "habe gelebt");
    }
}
