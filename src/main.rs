use std::io;
use std::env;
use std::ops::RangeBounds;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use regex::Regex;
use rusty_german_types::Exercise;

#[macro_use]
extern crate lazy_static;

mod types;
mod verben;
mod pronouns;
mod prepositions;
mod read_file;
mod read_file_multi_options_exercise;
mod articles;
mod substantives;
mod conjunctions;
mod clients;
mod relativ_pronomen;
mod nebensatze;
mod storage;
mod adjektivendungen;
mod verben_praposition;

use verben::{get_verben_phrase_exercise, get_starken_verben, get_schwachen_verben};
use pronouns::get_personal_pronouns;
use prepositions::{get_prepositions_exercises,get_prepositions_case_exercises};
use articles::get_articles;
use substantives::{get_substantives_list, get_substantives_tips_exercises};
use conjunctions::get_conjunction_exercises;
use relativ_pronomen::get_relativ_pronomen_exercises;
use nebensatze::get_nebensatze_exercise;
use types::ZeitType;
use storage::{TextStorage, StorageInterface};
use adjektivendungen::get_adjetivendungen_exercise;
use verben_praposition::get_verb_preposition_exercises;

struct MenuOption <'a> {
    text: &'a str,
    exec: &'a dyn Fn() -> (),
}

enum VerbExercise {
    OnlyPresent,
    All
}

type OnAnswer <'a> = Box<dyn Fn(bool) + 'a>;
type CreateOnAnswer <'a> = &'a dyn Fn(String, String) -> OnAnswer<'a>;

fn main() {
    menu();
}

fn menu() {
    let ts = TextStorage::initialize();

    clean_screen();

    let args: Vec<String> = env::args().collect();
    let random_exercises = true;

    let on_answer : CreateOnAnswer = &|category, exercise| -> OnAnswer {
        let ts = &ts;

        Box::new(move |is_correct| {
            ts.save_exercise_result(&category, &exercise, is_correct);
        })
    };

    let run_preposition = || {
        run_exercise(&get_prepositions_case_exercises, ..5, random_exercises, &on_answer);
        run_exercise(&get_prepositions_exercises, ..8, random_exercises, &on_answer);
    };
    let run_conjunctions = || run_exercise(&get_conjunction_exercises, ..10, random_exercises, &on_answer);
    let run_relativ_pronomen = || run_exercise(&get_relativ_pronomen_exercises, ..2, random_exercises, &on_answer);
    let run_nenbensatze = || run_exercise(&get_nebensatze_exercise, ..2, random_exercises, &on_answer);
    let run_substantive = || {
        run_exercise(&get_substantives_tips_exercises, ..5, random_exercises, &on_answer);
        run_exercise(&get_substantives_list, ..15, random_exercises, &on_answer);
    };
    let run_verben_all_times = || run_verb_exercise(VerbExercise::All, &on_answer);
    let run_verben_only_present = || run_verb_exercise(VerbExercise::OnlyPresent, &on_answer);
    let run_personal_pronoun = || run_personal_pronoun_exercise(&on_answer);
    let run_articles = || run_articles_exercise(&on_answer);
    let run_adjetiv = || run_exercise(&get_adjetivendungen_exercise, .., false, &on_answer);
    let run_verb_prap = || run_exercise(&get_verb_preposition_exercises, ..6, random_exercises, &on_answer);

    let options = vec![
        MenuOption { text: "verbs", exec: &run_verben_all_times },
        MenuOption { text: "verbs (only present)", exec: &run_verben_only_present },
        MenuOption { text: "personal pronoums", exec: &run_personal_pronoun },
        MenuOption { text: "prepositions", exec: &run_preposition },
        MenuOption { text: "articles", exec: &run_articles },
        MenuOption { text: "substantives", exec: &run_substantive },
        MenuOption { text: "conjunctions", exec: &run_conjunctions },
        MenuOption { text: "relativ pronomen", exec: &run_relativ_pronomen },
        MenuOption { text: "nebensätze", exec: &run_nenbensatze },
        MenuOption { text: "adjetivendungen", exec: &run_adjetiv },
        MenuOption { text: "verben praposition", exec: &run_verb_prap },
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
        println!("\nexit: to exit");
        println!("skip: to skip an exercise");
        println!("menu: to go back to menu");

        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    match input.trim().parse::<usize>() {
                        Ok(n) => {
                            clean_screen();
                            (options[n - 1].exec)();
                            break;
                        }
                        Err(_e) => println!("Invalid option, only nummerals are allowed.")
                    }
                }
                Err(error) => panic!("Error on receiving input {}", error)
            };
        }
    }
}

fn run_exercise<T, R>(exercise_fn: &dyn Fn() -> Vec<T>, range: R, random_exercises: bool, on_answer: CreateOnAnswer)
    where
        T: Exercise,
        R: RangeBounds<usize>,
{
    let mut exercises = exercise_fn();
    if random_exercises {
        let mut rng = thread_rng();
        exercises.shuffle(&mut rng);
    }
    let mut exercises_subset = exercises.drain(range).collect::<Vec<T>>();
    exercises_subset.sort_by_key(|a| a.get_sort_property());
    for exercise in exercises_subset.iter() {
        println!("{}", exercise.get_description());
        let category = std::any::type_name::<T>().to_string();
        let exec = exercise.get_description();
        wait_for_expected_inputs(exercise.get_expected_results(), Some(on_answer(category, exec)));
    }
}

fn run_articles_exercise(on_answer: CreateOnAnswer) {
    for articles in get_articles().iter() {
        for article in articles.iter() {
            let exercise = format!("{} {}", article.case, article.gender);
            println!("{}:", exercise);
            wait_for_expected_input(article.name.to_string(), on_answer("article".to_owned(), exercise));
        }
    }
}

fn run_personal_pronoun_exercise(on_answer: CreateOnAnswer) {
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
            wait_for_expected_input(subject.to_string(), on_answer("personal_pronoun".into(), pronoun.name.to_owned()));
        }
    }
}

#[tokio::main]
async fn run_verb_exercise(exercise_run_type: VerbExercise, on_answer: CreateOnAnswer) {
    let mut stark_verb_list = get_starken_verben();
    let mut schwache_verb_list = get_schwachen_verben();
    let person = get_personal_pronouns()[0].subjects;

    let mut rng = rand::thread_rng();

    let verben_list = vec![
        schwache_verb_list.remove(rng.gen_range(0, schwache_verb_list.len())),
        stark_verb_list.remove(rng.gen_range(0, stark_verb_list.len())),
        stark_verb_list.remove(rng.gen_range(0, stark_verb_list.len())),
    ];

    for exercise in verben_list.iter() {
        let search_verb = exercise.verb.to_owned();
        let verb_phrases_exercises = tokio::spawn(async move {
            get_verben_phrase_exercise(&search_verb).await
        });

        let run_exercise_async = async {
            for verb in exercise.verben.iter() {

                if verb.zeit_type == ZeitType::Plusquamperfekt {
                    break;
                }
                let mut conjugation_ite = 0;
                for conjugation in verb.conjugations.iter() {
                    let verb_exercise = format!("{} ({:?} {:?})", exercise.verb, exercise.verb_type, verb.zeit_type);
                    println!(" --- {} --- ", verb_exercise);
                    if let Some(obs) = &exercise.obs {
                        println!("Obs: {}", obs)
                    };
                    let time = person[conjugation_ite];
                    println!("{}:", time);
                    wait_for_expected_input(conjugation.to_string(), on_answer("verb_exercise".into(), time.into()));
                    conjugation_ite += 1;
                }

                match exercise_run_type {
                    VerbExercise::OnlyPresent => break,
                    _ => continue,
                };
            }
        };

        let (_, verb_phrases) = tokio::join!(run_exercise_async, verb_phrases_exercises);

        let mut verb_phrases_exercises = verb_phrases.unwrap();
        if !verb_phrases_exercises.is_empty() {
            let mut rng = thread_rng();
            verb_phrases_exercises.shuffle(&mut rng);

            let phrase_exercise = &verb_phrases_exercises[0];

            println!("{}", phrase_exercise.description);

            wait_for_expected_input(phrase_exercise.expect.to_string(), on_answer("verb_translation".into(), exercise.verb.to_owned()));
        }
    }
}

fn clean_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn normalize_string(string: String) -> String {
    Regex::new("[.]$").unwrap().replace(&string.trim().to_lowercase(), "").to_string()
}

fn wait_for_expected_input(expected_input: String, on_answer: OnAnswer) {
    wait_for_expected_inputs(vec![expected_input], Some(on_answer));
}

fn wait_for_expected_inputs(expected_inputs: Vec<String>, on_answer: Option<OnAnswer>) {
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
                if input.trim() == "skip" {
                    break;
                }
                if input.trim() == "menu" {
                    menu();
                    break;
                }
                let mut correct_input = false;

                for expected_input in &expected_inputs {
                    let mut input = input.to_string();
                    for &umlaut_char in REPLACE_CHARS.keys() {
                        if expected_input.contains(umlaut_char) {
                            input = REPLACE_CHARS.get(umlaut_char).unwrap().replace_all(&input, umlaut_char).to_string();
                        }
                    }

                    correct_input = normalize_string(input) == normalize_string(expected_input.to_owned());

                    if correct_input {
                        break;
                    } else {
                        println!("Correct Option:\n{}", expected_input);
                    }
                }

                match on_answer {
                    Some(ref f) => f(correct_input),
                    None => {}
                }
                if correct_input {
                    break;
                }
            }
            Err(error) => {
                println!("error: {}", error);
            }
        }
    }
    clean_screen();
}
