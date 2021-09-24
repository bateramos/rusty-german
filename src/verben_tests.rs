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

mod create_regular_stark_verben_tests {
    use super::super::{create_regular_stark_verben, PrefixVerb, str};

    #[test]
    fn it_should_create_regular_stark_verb() {
        let verb = create_regular_stark_verben("beginnen", "begannen", "begonnen", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["beginne", "beginnst", "beginnt", "beginnen", "beginnt", "beginnen"]);
        assert_eq!(verb.verben[1].conjugations, ["begann", "begannst", "begann", "begannen", "begannt", "begannen"]);
    }

    #[test]
    fn it_should_create_regular_changing_in_second_and_terth_person() {
        let verb = create_regular_stark_verben("tragen", "trugen", "betragen", PrefixVerb::Haben, Some(str("a>ä")));
        assert_eq!(verb.verben[0].conjugations, ["trage", "trägst", "trägt", "tragen", "tragt", "tragen"]);
        assert_eq!(verb.verben[1].conjugations, ["trug", "trugst", "trug", "trugen", "trugt", "trugen"]);
    }

    #[test]
    fn it_should_create_regular_last_consonant_is_s() {
        let verb = create_regular_stark_verben("wachsen", "wuchsen", "gewachsen", PrefixVerb::Haben, Some(str("a>ä")));
        assert_eq!(verb.verben[0].conjugations, ["wachse", "wächst", "wächst", "wachsen", "wachst", "wachsen"]);
        assert_eq!(verb.verben[1].conjugations, ["wuchs", "wuchsest", "wuchs", "wuchsen", "wuchst", "wuchsen"]);
    }

    #[test]
    fn it_should_create_separated_verb() {
        let verb = create_regular_stark_verben("[an]nehmen", "[an]nahmen", "angenommen", PrefixVerb::Haben, Some(str("e>i")));
        assert_eq!(verb.verben[0].conjugations, ["nehme an", "nihmst an", "nihmt an", "nehmen an", "nehmt an", "nehmen an"]);
        assert_eq!(verb.verben[1].conjugations, ["nahm an", "nahmst an", "nahm an", "nahmen an", "nahmt an", "nahmen an"]);
    }

    #[test]
    fn it_should_create_verb_with_eszett() {
        let verb = create_regular_stark_verben("genießen", "genossen", "genossen", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["genieße", "genießt", "genießt", "genießen", "genießt", "genießen"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_t_alveolar() {
        let verb = create_regular_stark_verben("[an]bieten", "[an]boten", "angeboten", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["biete an", "bietest an", "bietet an", "bieten an", "bietet an", "bieten an"]);
        assert_eq!(verb.verben[1].conjugations, ["bot an", "botest an", "bot an", "boten an", "botet an", "boten an"]);
    }
}

mod create_schwache_verben_tests {
    use super::super::{create_schwache_verben, PrefixVerb};

    #[test]
    fn it_should_create_regular_verb() {
        let verb = create_schwache_verben("brauchen", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["brauche", "brauchst", "braucht", "brauchen", "braucht", "brauchen"]);
        assert_eq!(verb.verben[1].conjugations, ["brauchte", "brauchtest", "brauchte", "brauchten", "brauchtet", "brauchten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_t_alveolar() {
        let verb = create_schwache_verben("arbeiten", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["arbeite", "arbeitest", "arbeitet", "arbeiten", "arbeitet", "arbeiten"]);
        assert_eq!(verb.verben[1].conjugations, ["arbeitete", "arbeitetest", "arbeitete", "arbeiteten", "arbeitetet", "arbeiteten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_r_alveolar() {
        let verb = create_schwache_verben("reden", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["rede", "redest", "redet", "reden", "redet", "reden"]);
        assert_eq!(verb.verben[1].conjugations, ["redete", "redetest", "redete", "redeten", "redetet", "redeten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_gn_alveolar() {
        let verb = create_schwache_verben("regnen", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["regne", "regnest", "regnet", "regnen", "regnet", "regnen"]);
        assert_eq!(verb.verben[1].conjugations, ["regnete", "regnetest", "regnete", "regneten", "regnetet", "regneten"]);
    }

    #[test]
    fn it_should_create_verb_ending_with_rn() {
        let verb = create_schwache_verben("erinnern", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[0].conjugations, ["erinnere", "erinnerst", "erinnert", "erinnern", "erinnert", "erinnern"]);
        assert_eq!(verb.verben[1].conjugations, ["erinnerte", "erinnertest", "erinnerte", "erinnerten", "erinnertet", "erinnerten"]);
    }

    #[test]
    fn it_should_create_past_tense_starting_with_ge() {
        let verb = create_schwache_verben("gehören", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[2].conjugations[0], "habe gehört");
    }

    #[test]
    fn it_should_create_past_tense_starting_with_er() {
        let verb = create_schwache_verben("erinnern", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[2].conjugations[0], "habe erinnert");
    }

    #[test]
    fn it_should_create_past_tense_ending_with_ieren() {
        let verb = create_schwache_verben("studieren", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[2].conjugations[0], "habe studiert");
    }

    #[test]
    fn it_should_create_past_tense_for_regular_verb() {
        let verb = create_schwache_verben("leben", PrefixVerb::Haben, None);
        assert_eq!(verb.verben[2].conjugations[0], "habe gelebt");
    }
}
