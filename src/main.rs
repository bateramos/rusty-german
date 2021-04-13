use std::io;
use std::env;
use std::ops::RangeBounds;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

mod types;
mod verben;
mod pronouns;
mod prepositions;
mod read_file;
mod articles;
mod substantives;
mod conjunctions;
mod temporal_satze;
mod clients;

use verben::{get_verben_phrase_exercise, get_starken_verben, get_schwachen_verben};
use pronouns::get_personal_pronouns;
use prepositions::{get_prepositions_exercises,get_prepositions_case_exercises};
use articles::get_articles;
use substantives::{get_substantives_list, get_substantives_tips_exercises};
use conjunctions::get_conjunction_exercises;
use temporal_satze::get_temporal_satze_exercises;
use types::{ZeitType, Exercise};

struct Options <'a> {
    text: &'a str,
    exec: &'a dyn Fn() -> (),
}

enum VerbExercise {
    OnlyPresent,
    All
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let randon_exercises = true;
    let mut input = String::new();

    let run_preposition = || {
        run_exercise(&get_prepositions_case_exercises, ..8, randon_exercises);
        run_exercise(&get_prepositions_exercises, ..15, randon_exercises);
    };
    let run_conjunctions = || run_exercise(&get_conjunction_exercises, ..15, randon_exercises);
    let run_temporal_satze = || run_exercise(&get_temporal_satze_exercises, ..3, randon_exercises);
    let run_substantive = || {
        run_exercise(&get_substantives_tips_exercises, .., randon_exercises);
        run_exercise(&get_substantives_list, ..20, randon_exercises);
    };
    let run_verben_all_times = || run_verb_exercise(VerbExercise::All);
    let run_verben_only_present = || run_verb_exercise(VerbExercise::OnlyPresent);

    let options = vec![
        Options { text: "verbs", exec: &run_verben_all_times },
        Options { text: "verbs (only present)", exec: &run_verben_only_present },
        Options { text: "personal pronoums", exec: &run_personal_pronoun_exercise },
        Options { text: "prepositions", exec: &run_preposition },
        Options { text: "articles", exec: &run_articles_exercise },
        Options { text: "substantives", exec: &run_substantive },
        Options { text: "conjunctions", exec: &run_conjunctions },
        Options { text: "temporal satze", exec: &run_temporal_satze },
    ];

    if args.len() == 2 && args[1] == "all" {
        for option in options.into_iter() {
            (option.exec)();
        }
    } else {
        for (index, option) in options.iter().enumerate() {
            println!("{} for {}", (index + 1).to_string(), option.text);
        }

        println!("\nTip: You can use ae, oe, ue for ä, ö, ü");

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                match input.trim().parse::<usize>() {
                    Ok(n) => (options[n - 1].exec)(),
                    Err(_e) => panic!("Invalid option")
                }
            }
            Err(error) => panic!("Error on receiving input {}", error)
        };
    }
}

fn run_exercise<T, R>(exercise_fn: &dyn Fn() -> Vec<T>, range: R, randon_exercises: bool)
    where
        T: Exercise,
        R: RangeBounds<usize>,
{
    let mut exercises = exercise_fn();
    if randon_exercises {
        let mut rng = thread_rng();
        exercises.shuffle(&mut rng);
    }
    let mut exercises_subset = exercises.drain(range).collect::<Vec<T>>();
    exercises_subset.sort_by_key(|a| a.get_sort_property());
    for exercise in exercises_subset.iter() {
        println!("{}", exercise.get_description());
        wait_for_expected_input(exercise.get_expected_result());
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

fn run_verb_exercise(exercise_run_type: VerbExercise) {
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

            match exercise_run_type {
                VerbExercise::OnlyPresent => break,
                _ => continue,
            };
        }
        run_phrase_verb_exercise(&exercise.verb);
    }
}

fn run_phrase_verb_exercise(verb: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let fut = async {
        get_verben_phrase_exercise(verb).await
    };
    let mut verb_phrases_exercises = rt.block_on(fut);

    if !verb_phrases_exercises.is_empty() {
        let mut rng = thread_rng();
        verb_phrases_exercises.shuffle(&mut rng);

        let phrase_exercise = &verb_phrases_exercises[0];

        println!("{}", phrase_exercise.description);

        wait_for_expected_input(phrase_exercise.expect.to_string());
    }
}

fn wait_for_expected_input(expected_input: String) {
    lazy_static! {
        static ref REPLACE_CHARS : HashMap<&'static str, Regex> = [
            ("ä", Regex::new(r"(ae)|(Ae)").unwrap()),
            ("ü", Regex::new(r"(ue)|(Ue)").unwrap()),
            ("ö", Regex::new(r"(oe)|(Oe)").unwrap()),
        ].iter().cloned().collect();
    }
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.trim() == "exit" {
                    panic!("Exiting");
                }
                for &umlaut_char in REPLACE_CHARS.keys() {
                    if expected_input.contains(umlaut_char) {
                        input = REPLACE_CHARS.get(umlaut_char).unwrap().replace_all(&input, umlaut_char).to_string();
                    }
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
