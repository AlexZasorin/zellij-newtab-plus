use indexmap::IndexSet;
use std::{
    fs::{self, OpenOptions},
    io::{Error, Read, Write},
    path::PathBuf,
    result::Result,
};

#[derive(Debug)]
pub struct History {
    entries: IndexSet<String>,
    path: PathBuf,
}

impl History {
    pub fn new() -> Result<History, Error> {
        fs::create_dir_all("/cache/zellij-newtab-plus")?;

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .open("/cache/zellij-newtab-plus/history")?;

        let mut raw_history: String = String::new();
        file.read_to_string(&mut raw_history)?;

        let entries: IndexSet<String> = raw_history.lines().map(|s| s.to_string()).collect();

        Ok(History {
            entries,
            path: PathBuf::from("/cache/zellij-newtab-plus/history"),
        })
    }

    pub fn push(&mut self, entry: &str) -> bool {
        if self.entries.contains(entry) {
            self.entries.shift_remove(entry);
        }
        self.entries.insert(entry.to_string());

        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)
        {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("Failed to open history file: {e}");
                return false;
            }
        };

        match file.write_all(
            self.entries
                .iter()
                .fold(String::new(), |acc, x| acc + x + "\n")
                .as_bytes(),
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to write to history file: {e}");
                return false;
            }
        };

        true
    }

    pub fn entries(&self) -> &IndexSet<String> {
        &self.entries
    }
}
