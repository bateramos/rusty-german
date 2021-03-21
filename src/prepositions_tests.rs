use super::{get_prepositions_exercises,get_prepositions_case_exercises};

#[test]
fn it_should_load_preposition_file() {
    get_prepositions_exercises();
}

mod preposition_case_exercise {
    use super::{get_prepositions_case_exercises};
    use crate::types::{PrepositionCaseExercise};

    #[test]
    fn it_should_load_preposition_case_exercise() {
        get_prepositions_case_exercises();
    }

    #[test]
    fn it_should_join_two_case_prepositions() {
        let exercises = get_prepositions_case_exercises();
        let uber_exercises = exercises
            .into_iter()
            .filter(|e| e.preposition == "über")
            .collect::<Vec<PrepositionCaseExercise>>();

        assert_eq!(uber_exercises.len(), 1);
        assert_eq!(uber_exercises[0].case, "akkusativ dativ");
    }
}
