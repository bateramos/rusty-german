mod runner;

use crate::read_file_multi_options_exercise::get_multiple_options_exercise;
use crate::prepositions::{get_prepositions_exercises, get_prepositions_case_exercises};
use crate::conjunctions::get_conjunction_exercises;
use crate::substantives::{get_substantives_list, get_substantives_tips_exercises};
use crate::adjektivendungen::get_adjetivendungen_exercise;
use crate::verben_praposition::get_verb_preposition_exercises;
use crate::config::VerbExercise;

use runner::{run_phrase_translation, run_exercise, run_personal_pronoun_exercise, run_articles_exercise, run_verb_exercise, CreateOnAnswer};

pub use runner::ProcessInput;

const RANDOM_ORDER : bool = true;
const NOT_RANDOM : bool = false;

pub struct Exercises <'a> {
    process_input: ProcessInput<'a>,
    create_on_answer: CreateOnAnswer<'a>,
}

impl <'a> Exercises <'a> {
    pub fn init(process_input: ProcessInput<'a>, create_on_answer: CreateOnAnswer<'a>) -> Exercises<'a> {
        Exercises { process_input, create_on_answer }
    }

    pub fn preposition(&self) {
        run_exercise(&get_prepositions_case_exercises, ..5, RANDOM_ORDER, self.process_input, self.create_on_answer);
        run_exercise(&get_prepositions_exercises, ..8, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn conjunctions(&self) {
        run_exercise(&get_conjunction_exercises, ..10, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn relativ_pronomen(&self) {
        run_exercise(&|| get_multiple_options_exercise("data/relativ-pronomen.txt", "relativ pronomen"), ..2, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn nebensatze(&self) {
        run_exercise(&|| get_multiple_options_exercise("data/nebensatze.txt", "nebensatze"), ..2, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn substantive(&self) {
        run_exercise(&get_substantives_tips_exercises, ..5, RANDOM_ORDER, self.process_input, self.create_on_answer);
        run_exercise(&get_substantives_list, ..15, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn adjetiv(&self) {
        run_exercise(&get_adjetivendungen_exercise, .., NOT_RANDOM, self.process_input, self.create_on_answer);
    }

    pub fn verb_preposition(&self) {
        run_exercise(&get_verb_preposition_exercises, ..6, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn local_adverb(&self) {
        run_exercise(&|| get_multiple_options_exercise("data/lokaladverbien.txt", "lokaladverbien"), ..5, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn konjuntiv_ii(&self) {
        run_exercise(&|| get_multiple_options_exercise("data/konjuntiv-ii.txt", "konjuntiv II"), ..4, RANDOM_ORDER, self.process_input, self.create_on_answer);
    }

    pub fn verb(&self, exercise_type: VerbExercise) {
        run_verb_exercise(exercise_type, self.process_input, self.create_on_answer);
    }

    pub fn personal_pronoun(&self) {
        run_personal_pronoun_exercise(self.process_input, self.create_on_answer);
    }

    pub fn articles(&self) {
        run_articles_exercise(self.process_input, self.create_on_answer);
    }

    #[tokio::main]
    pub async fn translate_phrase_with_word(&self) {
        run_phrase_translation(None).await;
    }
}
