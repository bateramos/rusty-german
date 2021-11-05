use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use chrono::prelude::*;

use crate::storage::storage_interface::StorageInterface;

pub struct TextStorage {
    sender: Sender<String>
}

impl StorageInterface <String> for TextStorage {
    fn initialize() -> Self {
        let (sender, receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("storage.txt")
                .unwrap();

            for message in receiver {
                file.write(message.as_bytes()).unwrap();
            }
        });

        TextStorage { sender }
    }

    fn save_exercise_result(&self, category: String, exercise: String, result: bool) {
        self.sender.send(format!("\n[{:?}] {} {} result: {}", Utc::now(), category, exercise, result)).unwrap();
    }
}
