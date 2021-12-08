use std::io;
use rand::Rng;
use clap::Parser;

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
mod storage;
mod adjektivendungen;
mod verben_praposition;
mod config;
mod comparator;
mod exercise;

use verben::{get_verben_phrase_exercise, get_starken_verben, get_schwachen_verben};
use pronouns::get_personal_pronouns;
use articles::get_articles;
use types::ZeitType;
use storage::{SqliteStorage, StorageInterface};
use config::{Args, MenuOption, VerbExercise};
use comparator::is_match;
use exercise::{Exercises, ProcessInput};

type OnAnswer <'a> = Box<dyn Fn(bool) + 'a>;
type CreateOnAnswer <'a> = &'a dyn Fn(String, String, String) -> OnAnswer<'a>;

fn main() {
    menu();
}

fn menu() {
    let args = Args::parse();

    let ts = SqliteStorage::initialize();

    clean_screen();

    let on_answer : CreateOnAnswer = &|category, exercise, expected_values| -> OnAnswer {
        let ts = &ts;

        Box::new(move |is_correct| {
            ts.save_exercise_result(category.to_string(), format!("{}; {}", exercise.to_string(), expected_values.to_string()), is_correct);
        })
    };

    let process_input : ProcessInput = &|exercise, on_answer| {
        wait_for_expected_inputs(exercise, on_answer);
    };

    let exercises = Exercises::init(&process_input, &on_answer);

    let run_preposition = || exercises.preposition();
    let run_conjunctions = || exercises.conjunctions();
    let run_relativ_pronomen = || exercises.relativ_pronomen();
    let run_nenbensatze = || exercises.nebensatze();
    let run_substantive = || exercises.substantive();
    let run_verben_all_times = || run_verb_exercise(VerbExercise::All, &on_answer);
    let run_verben_only_present = || run_verb_exercise(VerbExercise::OnlyPresent, &on_answer);
    let run_personal_pronoun = || run_personal_pronoun_exercise(&on_answer);
    let run_articles = || run_articles_exercise(&on_answer);
    let run_adjetiv = || exercises.adjetiv();
    let run_verb_prap = || exercises.verb_preposition();
    let run_lokaladverbien = || exercises.local_adverb();
    let run_konjuntiv_ii  = || exercises.konjuntiv_ii();
    let run_review_exercises_menu = || run_review_exercises(&ts);

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
        MenuOption { text: "lokaladverbien", exec: &run_lokaladverbien  },
        MenuOption { text: "Konjuntiv II", exec: &run_konjuntiv_ii },
        MenuOption { text: "Review Exercises", exec: &run_review_exercises_menu },
    ];

    if args.all {
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
                    input = input.trim().to_string();

                    if input == "exit" {
                        break;
                    }

                    match input.parse::<usize>() {
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

fn run_review_exercises(ts: &SqliteStorage) {
    let reviews = ts.fetch_exercises_with_result_false();

    reviews.iter().for_each(|review| {
        let parts : Vec<&str> = review.split(";").collect();
        let description = parts[0];
        let expected_results : Vec<String> = parts[1].split("|").filter_map(|item| {
            if item.trim().is_empty() {
                None
            } else {
                Some(item.trim().to_string())
            }
        }).collect();
        println!("{}", description);
        wait_for_expected_inputs(expected_results, None);
    });
}

fn run_articles_exercise(on_answer: CreateOnAnswer) {
    for articles in get_articles().iter() {
        for article in articles.iter() {
            let exercise = format!("{} {}", article.case, article.gender);
            println!("{}:", exercise);
            wait_for_expected_input(article.name.to_string(), on_answer("article".to_owned(), exercise, article.name.to_string()));
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
            wait_for_expected_input(subject.to_string(), on_answer("personal_pronoun".into(), pronoun.name.to_owned(), subject.to_string()));
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
                    let mut verb_exercise = format!(" --- {} ({:?} {:?}) --- ", exercise.verb, exercise.verb_type, verb.zeit_type);
                    if let Some(obs) = &exercise.obs {
                        verb_exercise = format!("{}\n Obs: {}", verb_exercise, obs);
                    };
                    let time = person[conjugation_ite];
                    verb_exercise = format!("{}\n{}:", verb_exercise, time);
                    println!("{}", verb_exercise);
                    wait_for_expected_input(conjugation.into(), on_answer("verb_exercise".into(), verb_exercise, conjugation.into()));
                    conjugation_ite += 1;
                }

                match exercise_run_type {
                    VerbExercise::OnlyPresent => break,
                    _ => continue,
                };
            }
        };

        let (_, verb_phrases) = tokio::join!(run_exercise_async, verb_phrases_exercises);

        let verb_phrases_exercises = verb_phrases.unwrap();
        if !verb_phrases_exercises.is_empty() {
            let index = rng.gen_range(0, verb_phrases_exercises.len().min(5));

            let category = "verb_translation";

            let phrase_exercise = &verb_phrases_exercises[index];

            println!("{}\n{}", category, phrase_exercise.description);

            let expect = phrase_exercise.expect.to_string();
            wait_for_expected_input(expect.to_string(), on_answer(category.into(), phrase_exercise.description.to_string(), expect));
        }
    }
}

fn clean_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn wait_for_expected_input(expected_input: String, on_answer: OnAnswer) {
    wait_for_expected_inputs(vec![expected_input], Some(on_answer));
}

fn wait_for_expected_inputs(expected_inputs: Vec<String>, on_answer: Option<OnAnswer>) {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.trim() == "exit" {
                    std::process::exit(1);
                }
                if input.trim() == "skip" {
                    break;
                }
                if input.trim() == "menu" {
                    menu();
                    break;
                }
                let correct_input = is_match(input.to_string(), &expected_inputs);

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
