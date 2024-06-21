use rand::Rng;

use std::ops::RangeBounds;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rusty_german_types::Exercise;

use crate::config::VerbExercise;
use crate::verben::{get_starken_verben, get_schwachen_verben};
use crate::pronouns::get_personal_pronouns;
use crate::types::ZeitType;
use crate::articles::get_articles;
use crate::clients;

pub type OnAnswer <'a> = Box<dyn Fn(bool) + 'a>;
pub type CreateOnAnswer <'a> = &'a dyn Fn(String, String, String) -> OnAnswer<'a>;
pub type ProcessInput <'a> = &'a dyn Fn(Vec<String>, Option<OnAnswer>);

pub fn run_exercise<T, R>(exercise_fn: &dyn Fn() -> Vec<T>, range: R, random_exercises: bool, process_input: ProcessInput, on_answer: CreateOnAnswer)
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
        let expected_values = exercise.get_expected_results().into_iter().fold("".to_owned(), |acc, item| {
            format!("{}|{}", acc, item)
        });

        process_input(exercise.get_expected_results(), Some(on_answer(category, exercise.get_description(), expected_values)));
    }
}

#[tokio::main]
pub async fn run_verb_exercise(exercise_run_type: VerbExercise, process_input: ProcessInput, on_answer: CreateOnAnswer) {
    let mut stark_verb_list = get_starken_verben();
    let mut schwache_verb_list = get_schwachen_verben();
    let person = get_personal_pronouns()[0].subjects;

    let mut rng = rand::thread_rng();

    let verben_list = match exercise_run_type {
        VerbExercise::OnlyVerb(ref verb) => {
            if let Some(position) = stark_verb_list.iter().position(|v| v.verb.eq(verb)) {
                vec![stark_verb_list.remove(position)]
            } else if let Some(position) = schwache_verb_list.iter().position(|v| v.verb.eq(verb)) {
                vec![schwache_verb_list.remove(position)]
            } else {
                panic!("No verb found for string {}", verb);
            }
        },
        _ => vec![
            schwache_verb_list.remove(rng.gen_range(0, schwache_verb_list.len())),
            stark_verb_list.remove(rng.gen_range(0, stark_verb_list.len())),
            stark_verb_list.remove(rng.gen_range(0, stark_verb_list.len())),
        ],
    };

    for exercise in verben_list.iter() {
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
                process_input(vec![conjugation.into()], Some(on_answer("verb_exercise".into(), verb_exercise, conjugation.into())));
                conjugation_ite += 1;
            }

            match exercise_run_type {
                VerbExercise::OnlyPresent => break,
                _ => continue,
            };
        }

        run_phrase_translation(Some(exercise.verb.to_owned())).await;
    }
}

pub fn run_personal_pronoun_exercise(process_input: ProcessInput, on_answer: CreateOnAnswer) {
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
            process_input(vec![subject.to_string()], Some(on_answer("personal_pronoun".into(), pronoun.name.to_owned(), subject.to_string())));
        }
    }
}

pub fn run_articles_exercise(process_input: ProcessInput, on_answer: CreateOnAnswer) {
    for articles in get_articles().iter() {
        for article in articles.iter() {
            let exercise = format!("{} {}", article.case, article.gender);
            println!("{}:", exercise);
            process_input(vec![article.name.to_string()], Some(on_answer("article".to_owned(), exercise, article.name.to_string())));
        }
    }
}

pub async fn run_phrase_translation(verb: Option<String>) {
    let verb = if let Some(verb) = verb {
        verb
    } else {
        println!("type a german verb");
        crate::get_next_input()
    };

    let phrase = clients::fetch_phrase_for(verb.clone()).await;

    let phrase = phrase.unwrap();
    println!("{}", phrase);

    loop {
        let verb = verb.clone();
        let translation = crate::get_next_input();

        let result = clients::verify_translation(verb, &phrase, translation).await;
        let result = result.unwrap();

        if result.to_lowercase().replace(".", "").contains("true") {
            break
        } else {
            println!("{}", result);
            println!("try again");
        }
    }
}
