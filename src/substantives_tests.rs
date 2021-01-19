use super::{get_substantives_list, get_substantives_tips_exercises};

#[test]
fn it_should_load_substantive_file() {
    get_substantives_list();
}

#[test]
fn it_should_load_substantive_tips_file() {
    get_substantives_tips_exercises();
}
