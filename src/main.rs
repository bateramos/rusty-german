use std::io;
use rand::Rng;

mod types;
mod verben;
mod pronouns;

use verben::get_verben;
use pronouns::get_personal_pronouns;

fn main() {
    let mut input = String::new();

    println!("1 to verbs, 2 for personal pronouns");
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            match input.trim() {
                "1" => run_verb_exercise(),
                "2" => run_personal_pronoun_exercise(),
                _ => panic!("Invalid option {}", input)
            }
        }
        Err(error) => panic!("Error on receiving input {}", error)
    };
}

fn run_personal_pronoun_exercise() {
    let pronouns = get_personal_pronouns();

    for pronoun in pronouns.iter() {
        let mut conjugation_ite = 0;
        println!(" --- {} --- ", pronoun.name);
        for subject in pronoun.subjects.iter() {
            let case = match conjugation_ite {
                0..=2 => "single",
                3..=5 => "plural",
                _ => panic!("something wrong")
            };
            println!("{} person, {}:", (conjugation_ite % 3) + 1, case);
            conjugation_ite += 1;
            wait_for_expected_input(subject.to_string());
        }
    }
}

fn run_verb_exercise() {
    let verben_list = get_verben();
    let person = get_personal_pronouns()[0].subjects;

    let mut conjugation_ite;
    let mut rng = rand::thread_rng();

    let verben = &verben_list[rng.gen_range(0, verben_list.len())];

    for verb in verben.iter() {
        conjugation_ite = 0;
        println!(" --- {} ({:?} {:?}) --- ", verb.name, verb.verb_type, verb.zeit_type);
        for conjugation in verb.conjugations.iter() {
            println!("{}:", person[conjugation_ite]);
            wait_for_expected_input(conjugation.to_string());
            conjugation_ite += 1;
        }
    }
}

fn wait_for_expected_input(expected_input: String) {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                match input.trim() == expected_input {
                    true => {
                        break;
                    }
                    _ => println!("{} != {}", input.trim(), expected_input)
                }
            }
            Err(error) => {
                println!("error: {}", error);
            }
        }
    }
}
