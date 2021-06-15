use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use chrono::prelude::*;

use crate::storage::storage_interface::StorageInterface;

pub struct TextStorage {
    tx: Sender<String>
}

impl StorageInterface for TextStorage {
    fn initialize() -> Self {
        let (tx, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("storage.txt")
                .unwrap();

            for message in rx {
                file.write(message.as_bytes()).unwrap();
            }
        });

        TextStorage { tx }
    }

    fn save_exercise_result(&self, category: &str, exercise: &str, result: bool) {
        self.tx.send(format!("\n[{:?}] {} {} result: {}", Utc::now(), category, exercise, result)).unwrap();
    }
}
