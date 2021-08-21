use super::get_verb_preposition_exercises;

#[test]
fn it_should_load_file() {
    assert!(!get_verb_preposition_exercises().is_empty());
}

#[test]
fn it_should_set_fields_for_all_entries() {
    for entry in get_verb_preposition_exercises() {
        assert!(!entry.category.is_empty());
        assert!(!entry.verb.is_empty());
        assert!(!entry.preposition.is_empty());
    }
}
