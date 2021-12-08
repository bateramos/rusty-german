use std::collections::HashMap;

use regex::Regex;

fn normalize_string(string: String) -> String {
    Regex::new("[.]$").unwrap().replace(&string.trim().to_lowercase(), "").to_string()
}

pub fn is_match(input: String, match_agains: &Vec<String>) -> bool {
    lazy_static! {
        static ref REPLACE_CHARS : HashMap<&'static str, Regex> = [
            ("ä", Regex::new(r"(ae)|(Ae)").unwrap()),
            ("ü", Regex::new(r"(ue)|(Ue)").unwrap()),
            ("ö", Regex::new(r"(oe)|(Oe)").unwrap()),
        ].iter().cloned().collect();
    }

    let mut match_found = false;
    for expected_input in match_agains {
        let mut input = input.to_string();
        for &umlaut_char in REPLACE_CHARS.keys() {
            if expected_input.contains(umlaut_char) {
                input = REPLACE_CHARS.get(umlaut_char).unwrap().replace_all(&input, umlaut_char).to_string();
            }
        }

        match_found = normalize_string(input) == normalize_string(expected_input.to_owned());

        if match_found  {
            break;
        } else {
            println!("Correct Option:\n{}", expected_input);
        }
    }

    match_found
}

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;
