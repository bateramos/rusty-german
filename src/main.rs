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
mod conjunctions;

use verben::{get_starken_verben, get_schwachen_verben};
use pronouns::get_personal_pronouns;
use prepositions::get_prepositions_exercises;
use articles::get_articles;
use substantives::{get_substantives_list, get_substantives_tips_exercises};
use conjunctions::get_conjunction_exercises;
use types::{ZeitType, PrepositionExercise, SubstantiveExercise, ConjunctionExercise};

fn main() {
    run_articles_exercise();
    run_personal_pronoun_exercise();
    run_verb_exercise();
    run_substantice_exercise();
    run_preposition_exercise();
    run_conjunction_exercise();
    run_verb_exercise();
}

fn run_substantice_exercise() {
    let mut rng = thread_rng();
    let mut substantives_tips_list = get_substantives_tips_exercises();
    substantives_tips_list.shuffle(&mut rng);

    for exercise in substantives_tips_list.iter() {
        println!("{}", exercise.tip.trim());
        wait_for_expected_input(exercise.article.to_string());
    }

    let mut substantives = get_substantives_list();
    substantives.shuffle(&mut rng);
    substantives = substantives.drain(..20).collect::<Vec<SubstantiveExercise>>();
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
    let mut prepositions_subset = prepositions.drain(..15).collect::<Vec<PrepositionExercise>>();
    prepositions_subset.sort_by_key(|a| a.case.to_owned());
    for preposition in prepositions_subset.iter() {
        println!("{}", preposition.phrase);
        wait_for_expected_input(preposition.preposition.to_string());
    }
}

fn run_conjunction_exercise() {
    let mut rng = thread_rng();
    let mut conjunctions = get_conjunction_exercises();
    conjunctions.shuffle(&mut rng);
    let conjunctions_subset = conjunctions.drain(..15).collect::<Vec<ConjunctionExercise>>();
    for conjunction in conjunctions_subset.iter() {
        println!("{}", conjunction.phrase);
        wait_for_expected_input(conjunction.conjunction.to_string());
    }
}

fn run_personal_pronoun_exercise() {
    let pronouns = get_personal_pronouns();

    for pronoun in pronouns.iter() {
        let mut conjugation_ite = 0;
        for subject in pronoun.subjects.iter() {
            let case = match conjugation_ite {
                0..=2 => "single",
                3..=5 => "plural",
                _ => panic!("something wrong")
            };
            println!(" --- {} --- ", pronoun.name);
            println!("{} person, {}:", (conjugation_ite % 3) + 1, case);
            conjugation_ite += 1;
            wait_for_expected_input(subject.to_string());
        }
    }
}

fn run_verb_exercise() {
    let mut stark_verb_list = get_starken_verben();
    let mut schwache_verb_list = get_schwachen_verben();
    let person = get_personal_pronouns()[0].subjects;

    let mut conjugation_ite;
    let mut rng = rand::thread_rng();

    let verben_list = vec![
        schwache_verb_list.remove(rng.gen_range(0, schwache_verb_list.len())),
        stark_verb_list.remove(rng.gen_range(0, stark_verb_list.len())),
        stark_verb_list.remove(rng.gen_range(0, stark_verb_list.len())),
    ];

    for exercise in verben_list.iter() {
        for verb in exercise.verben.iter() {
            if verb.zeit_type == ZeitType::Plusquamperfekt {
                break;
            }
            conjugation_ite = 0;
            for conjugation in verb.conjugations.iter() {
                println!(" --- {} ({:?} {:?}) --- ", exercise.verb, exercise.verb_type, verb.zeit_type);
                if let Some(obs) = &exercise.obs {
                    println!("Obs: {}", obs)
                };
                println!("{}:", person[conjugation_ite]);
                wait_for_expected_input(conjugation.to_string());
                conjugation_ite += 1;
            }
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
