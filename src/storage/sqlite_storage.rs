use sqlite;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use crate::storage::storage_interface::StorageInterface;

#[cfg(not(test))]
const DATABASE_PATH : &str = "results.sqlite";

enum Operation {
    #[allow(dead_code)]
    ShutDown,
    Save { category: String, exercise: String, result: bool },
}

pub struct SqliteStorage {
    sender: mpsc::Sender<Operation>,
    #[allow(dead_code)]
    thread: Option<thread::JoinHandle<()>>,
    #[allow(dead_code)]
    connection: Arc<Mutex<sqlite::Connection>>,
}

impl StorageInterface <String> for SqliteStorage {
    fn initialize() -> Self {
        let (sender, receiver) = mpsc::channel();
        let connection = Arc::new(Mutex::new(sqlite::open(DATABASE_PATH).unwrap()));

        let thread_conn = Arc::clone(&connection);
        let thread = thread::spawn(move || {
            if let Err(error) = create_database_tables(&thread_conn.lock().unwrap()) {
                println!("Error on initializing database: {}", error);
                return;
            }

            for operation in receiver {
                match operation {
                    Operation::ShutDown => {
                        return;
                    },
                    Operation::Save { category, exercise, result } => {
                        let conn = thread_conn.lock().unwrap();
                        let category_id = generate_hash(&category) as i64;
                        let exercise_id = generate_hash(&exercise) as i64;

                        let mut stmt = conn.prepare("
                            INSERT OR IGNORE INTO Category (id, name) VALUES (:id, :name);
                        ").unwrap();

                        stmt.bind_by_name(":id", category_id).unwrap();
                        stmt.bind_by_name(":name", category.as_str()).unwrap();

                        stmt.next().unwrap();

                        let mut stmt = conn.prepare("
                            INSERT OR IGNORE INTO Exercise (id, name, category) VALUES (:id, :name, :category);
                        ").unwrap();

                        stmt.bind_by_name(":id", exercise_id).unwrap();
                        stmt.bind_by_name(":name", exercise.as_str()).unwrap();
                        stmt.bind_by_name(":category", category_id).unwrap();

                        stmt.next().unwrap();

                        let mut stmt = conn.prepare("
                            INSERT INTO ExerciseResult (exercise, date, result) VALUES (:exercise, date('now'), :result);
                        ").unwrap();

                        stmt.bind_by_name(":exercise", exercise_id).unwrap();
                        stmt.bind_by_name(":result", result.to_string().as_str()).unwrap();

                        stmt.next().unwrap();
                    },
                }
            }
        });

        SqliteStorage { sender, thread: Some(thread), connection }
    }

    fn save_exercise_result(&self, category: String, exercise: String, result: bool) {
        self.sender.send(Operation::Save { category, exercise, result }).unwrap();
    }
}

impl SqliteStorage {
    pub fn fetch_verb_exercises_count(&self) -> Vec<(String, i64)> {
        let mut exercises_count = Vec::new();
        let connection = self.connection.lock().unwrap();
        let mut cursor = connection
            .prepare("select SUBSTR(e.name, 6, INSTR(e.name, ' (') - 6) as VERB, COUNT(er.id) from ExerciseResult er
                left join Exercise e on e.id = er.exercise
                left join Category c on c.id = e.category
                where c.name = 'verb_exercise'
                group by VERB
            ")
            .unwrap()
            .into_cursor();

        while let Some(row) = cursor.next().unwrap() {
            exercises_count.push((row[0].as_string().unwrap().to_string(), row[1].as_integer().unwrap()));
        }

        exercises_count
    }

    pub fn fetch_exercises_with_result_false(&self) -> Vec<String> {
        let mut exercises = Vec::new();
        let connection = self.connection.lock().unwrap();
        let mut cursor = connection
            .prepare("select distinct e.name from ExerciseResult er
                left join Exercise e on e.id = er.exercise
                where er.result = 'false' AND er.date > date('now', '-2 days')
            ")
            .unwrap()
            .into_cursor();


        while let Some(row) = cursor.next().unwrap() {
            exercises.push(row[0].as_string().unwrap().to_string());
        }

        exercises
    }

    #[allow(dead_code)]
    fn drop_thread(&mut self) {
        self.sender.send(Operation::ShutDown).unwrap();

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

fn create_database_tables(connection: &sqlite::Connection) -> Result<(), sqlite::Error> {
    connection.execute("
        CREATE TABLE IF NOT EXISTS Category (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS Exercise (
            id INTEGER NOT NULL,
            category INTEGER NOT NULL,
            name TEXT NOT NULL,
            PRIMARY KEY (id, category),
            FOREIGN KEY (category) REFERENCES Category (id)
        );
        CREATE TABLE IF NOT EXISTS ExerciseResult (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            exercise INTEGER NOT NULL,
            date TEXT NOT NULL,
            result BOOL NOT NULL,
            FOREIGN KEY (exercise) REFERENCES Exercise (id)
        );
    ")
}

fn generate_hash<K>(key: K) -> u64 where K: Hash {
    let mut s = DefaultHasher::new();
    key.hash(&mut s);
    s.finish()
}

#[cfg(test)]
#[path = "sqlite_storage_tests.rs"]
mod tests;
#[cfg(test)]
use tests::DATABASE_PATH;
