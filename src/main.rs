use std::io;
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

use storage::{SqliteStorage, StorageInterface};
use config::{Args, VerbSubcommand, MenuOption, VerbExercise};
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

    ts.fetch_verb_exercises_count();

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
    let run_verben_all_times = || exercises.verb(VerbExercise::All);
    let run_verben_only_present = || exercises.verb(VerbExercise::OnlyPresent);
    let run_personal_pronoun = || exercises.personal_pronoun();
    let run_articles = || exercises.articles();
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

    if let Some(verb_args) = args.verb {
        match verb_args {
            VerbSubcommand::Verb{ verb } => {
                if let Some(verb) = verb {
                    exercises.verb(VerbExercise::OnlyVerb(verb));
                } else {
                    exercises.verb(VerbExercise::All);
                }
            }
        }
    } else if args.all {
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

fn clean_screen() {
    print!("\x1B[2J\x1B[1;1H");
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
