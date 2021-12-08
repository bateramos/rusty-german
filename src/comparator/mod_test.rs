use super::is_match;

#[test]
fn should_match_first() {
    assert!(is_match("versuchst".to_string(), &vec!["versuchst".to_string()]));
}

#[test]
fn should_match_second() {
    assert!(is_match("versuchst".to_string(), &vec!["versuchen".to_string(), "versuchst".to_string()]));
}

#[test]
fn should_match_replacing_a_with_umlaut() {
    assert!(is_match("ae".to_string(), &vec!["ä".to_string()]));
    assert!(is_match("Ae".to_string(), &vec!["ä".to_string()]));
}

#[test]
fn should_match_replacing_u_with_umlaut() {
    assert!(is_match("ue".to_string(), &vec!["ü".to_string()]));
    assert!(is_match("Ue".to_string(), &vec!["ü".to_string()]));
}

#[test]
fn should_match_replacing_o_with_umlaut() {
    assert!(is_match("oe".to_string(), &vec!["ö".to_string()]));
    assert!(is_match("oe".to_string(), &vec!["ö".to_string()]));
}
