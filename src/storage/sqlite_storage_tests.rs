use super::{Operation, SqliteStorage, StorageInterface};

pub const DATABASE_PATH : &str = ":memory:";

fn count_table(table_name: &str, connection: &sqlite::Connection) -> i64 {
    let stmt = connection.prepare(format!("SELECT COUNT(*) FROM {}", table_name)).unwrap();
    let mut cursor = stmt.into_cursor();
    let result = &cursor.next().unwrap().unwrap().to_vec()[0];

    result.as_integer().unwrap()
}

#[test]
fn test_database_in_memory_creation() {
    let storage = SqliteStorage::initialize();
    storage.sender.send(Operation::ShutDown).unwrap();
    storage.thread.join().unwrap();

    let connection = storage.connection.lock().unwrap();

    count_table("Category", &connection);
}

#[test]
fn test_database_insertion() {
    let storage = SqliteStorage::initialize();
    storage.save_exercise_result("cat1".to_string(), "exec1".to_string(), true);
    storage.sender.send(Operation::ShutDown).unwrap();
    storage.thread.join().unwrap();

    let connection = storage.connection.lock().unwrap();

    assert_eq!(count_table("Category", &connection), 1);
    assert_eq!(count_table("Exercise", &connection), 1);
    assert_eq!(count_table("ExerciseResult", &connection), 1);
}

#[test]
fn test_dont_insert_duplicated_category() {
    let storage = SqliteStorage::initialize();
    storage.save_exercise_result("cat1".to_string(), "exec1".to_string(), true);
    storage.save_exercise_result("cat1".to_string(), "exec2".to_string(), true);
    storage.sender.send(Operation::ShutDown).unwrap();
    storage.thread.join().unwrap();

    let connection = storage.connection.lock().unwrap();

    assert_eq!(count_table("Category", &connection), 1);
    assert_eq!(count_table("Exercise", &connection), 2);
    assert_eq!(count_table("ExerciseResult", &connection), 2);
}

#[test]
fn test_insert_twice() {
    let storage = SqliteStorage::initialize();
    storage.save_exercise_result("cat1".to_string(), "exec1".to_string(), true);
    storage.save_exercise_result("cat2".to_string(), "exec2".to_string(), true);
    storage.sender.send(Operation::ShutDown).unwrap();
    storage.thread.join().unwrap();

    let connection = storage.connection.lock().unwrap();

    assert_eq!(count_table("Category", &connection), 2);
    assert_eq!(count_table("Exercise", &connection), 2);
    assert_eq!(count_table("ExerciseResult", &connection), 2);
}
