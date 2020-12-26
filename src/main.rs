use std::io;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

mod types;
mod verben;
mod pronouns;
mod prepositions;
mod read_file;
mod articles;
mod substantives;

use verben::get_verben;
use pronouns::get_personal_pronouns;
use prepositions::get_prepositions_exercises;
use articles::get_articles;
use substantives::{get_substantives_list, get_substantives_tips_exercises};

fn main() {
    let mut input = String::new();

    println!("1 for verbs, 2 for personal pronouns, 3 for prepositions, 4 for articles, 5 for substantives");
    println!("type exit to quit");
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            match input.trim() {
                "1" => run_verb_exercise(),
                "2" => run_personal_pronoun_exercise(),
                "3" => run_preposition_exercise(),
                "4" => run_articles_exercise(),
                "5" => run_substantice_exercise(),
                _ => panic!("Invalid option {}", input)
            }
        }
        Err(error) => panic!("Error on receiving input {}", error)
    };
}

fn run_substantice_exercise() {
    let mut rng = thread_rng();
    let mut substantives_tips_list = get_substantives_tips_exercises();
    substantives_tips_list.shuffle(&mut rng);

    for exercise in substantives_tips_list.iter() {
        println!("{}", exercise.tip);
        wait_for_expected_input(exercise.article.to_string());
    }

    let mut substantives = get_substantives_list();
    substantives.shuffle(&mut rng);
    substantives = substantives[..40].to_vec();
    for exercise in substantives.iter() {
        println!("{}", exercise.substantive);
        wait_for_expected_input(exercise.article.to_string());
    }
}

fn run_articles_exercise() {
    for articles in get_articles().iter() {
        for article in articles.iter() {
            println!("{} {}:", article.case, article.gender);
            wait_for_expected_input(article.name.to_string());
        }
    }
}

fn run_preposition_exercise() {
    let mut rng = thread_rng();
    let mut prepositions = get_prepositions_exercises();
    prepositions.shuffle(&mut rng);
    prepositions.sort_by_key(|a| a.case.to_owned());
    for preposition in prepositions.iter() {
        println!("{}", preposition.phrase);
        wait_for_expected_input(preposition.preposition.to_string());
    }
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
        for conjugation in verb.conjugations.iter() {
            println!(" --- {} ({:?} {:?}) --- ", verb.name, verb.verb_type, verb.zeit_type);
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
                if input.trim() == "exit" {
                    panic!("Exiting");
                }
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
    print!("\x1B[2J\x1B[1;1H");
}
