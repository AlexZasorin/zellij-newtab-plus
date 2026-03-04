use indexmap::IndexSet;
use regex::Regex;
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub struct InputState {
    new_tab_name: String,
    // Current tab name is stashed here while browsing history
    stashed_input: Option<String>,
    history: IndexSet<String>,
    index: usize,
    path: PathBuf,
    // If we fail to load the history for a session, we want to avoid trying to write to the file
    loaded_history: bool,
}

impl InputState {
    pub fn add_char(&mut self, ch: char) {
        self.new_tab_name.push(ch)
    }

    pub fn del_char(&mut self) -> bool {
        if !self.new_tab_name.is_empty() {
            self.new_tab_name.pop();

            return true;
        }

        false
    }

    pub fn push_history(&mut self) -> bool {
        if self.history.contains(self.new_tab_name.trim()) {
            self.history.shift_remove(self.new_tab_name.trim());
        }
        self.history.insert(self.new_tab_name.trim().to_string());

        if !self.loaded_history {
            return false;
        }

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
            self.history
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

    pub fn new_tab_name(&self) -> &str {
        &self.new_tab_name
    }

    pub fn clear_name(&mut self) {
        self.new_tab_name = String::new();
    }

    pub fn reset_state(&mut self) {
        self.stashed_input = None;
        self.index = 0;
    }

    pub fn delete_prev_word(&mut self) {
        let re = Regex::new(r"\b(\w+|[^\w\s])\s*$").unwrap();
        self.new_tab_name = re.replace(&self.new_tab_name, "").to_string();
    }

    pub fn up(&mut self) -> bool {
        if self.history.is_empty() {
            return false;
        }

        if self.stashed_input.is_some() {
            self.index = (self.index + 1).min(self.history.len() - 1);
        } else {
            self.stashed_input = Some(self.new_tab_name.clone());
        }

        self.new_tab_name = self.history[self.history.len() - self.index - 1].clone();

        true
    }

    pub fn down(&mut self) -> bool {
        if self.history.is_empty() || self.stashed_input.is_none() {
            return false;
        }

        if self.index == 0 {
            self.new_tab_name = self.stashed_input.take().unwrap_or_default();
            return false;
        }

        self.index = self.index.saturating_sub(1);
        self.new_tab_name = self.history[self.history.len() - self.index - 1].clone();

        true
    }
}

impl Default for InputState {
    fn default() -> InputState {
        let _ = fs::create_dir_all("/cache/zellij-newtab-plus");

        let mut file_result = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .open("/cache/zellij-newtab-plus/history");

        let mut loaded_history = false;
        let mut raw_history: String = String::new();
        if let Ok(file) = &mut file_result {
            let read_result = file.read_to_string(&mut raw_history);
            if read_result.is_ok() {
                loaded_history = true;
            }
        }

        let entries: IndexSet<String> = raw_history.lines().map(|s| s.to_string()).collect();

        InputState {
            new_tab_name: String::new(),
            stashed_input: None,
            history: entries,
            index: 0,
            path: PathBuf::from("/cache/zellij-newtab-plus/history"),
            loaded_history,
        }
    }
}
