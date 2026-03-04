use crate::{
    history::History,
    ui::{CURSOR, PROMPT, setup_plugin_pane},
};
use regex::Regex;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Debug, Default)]
pub struct State {
    new_tab_name: String,
    use_zoxide: bool,
    history: Option<History>,
    history_idx: usize,
    stashed_input: Option<String>,
}

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.parse_configuration(configuration);

        hide_self();

        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::Reconfigure,
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::Key,
            EventType::PermissionRequestResult,
            EventType::RunCommandResult,
        ]);

        self.history = match History::new() {
            Ok(history) => Some(history),
            Err(e) => {
                eprintln!("Failed to load history: {e}");
                return;
            }
        }
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;

        match event {
            Event::PermissionRequestResult(permission) => {
                if matches!(permission, PermissionStatus::Granted) {
                    setup_plugin_pane();
                    show_self(true);

                    should_render = true;
                }
            }
            Event::RunCommandResult(error_code, stdout, _stderr, context) => {
                if context.contains_key("zoxide_query") {
                    if error_code == Some(0) {
                        let stdout_str = String::from_utf8_lossy(&stdout).trim().to_string();
                        self.new_named_tab(Some(&stdout_str.to_string()));
                    } else {
                        self.new_named_tab(None);
                    }

                    should_render = true;
                }
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Char('c') | BareKey::Char('d')
                    if key.has_modifiers(&[KeyModifier::Ctrl]) =>
                {
                    self.new_tab_name = String::new();
                    close_self();
                }
                BareKey::Char(char) if char.is_ascii() => {
                    self.new_tab_name.push(char);
                    should_render = true;
                }
                BareKey::Enter => {
                    if !self.new_tab_name.trim().is_empty() {
                        if self.use_zoxide {
                            let mut context = BTreeMap::new();
                            context.insert("zoxide_query".to_string(), "true".to_string());

                            let command: Vec<&str> = ["zoxide", "query"]
                                .into_iter()
                                .chain(self.new_tab_name.split_whitespace())
                                .collect();

                            run_command(&command, context);

                            should_render = true;
                        } else {
                            self.new_named_tab(None);
                            should_render = true;
                        }

                        self.stashed_input = None;
                        self.history_idx = 0;
                    }
                }
                BareKey::Esc => {
                    self.new_tab_name = String::new();
                    close_self();
                }
                BareKey::Backspace if key.has_modifiers(&[KeyModifier::Alt]) => {
                    let re = Regex::new(r"\b(\w+|[^\w\s])\s*$").unwrap();
                    self.new_tab_name = re.replace(&self.new_tab_name, "").to_string();

                    should_render = true;
                }
                BareKey::Backspace => {
                    if !self.new_tab_name.is_empty() {
                        self.new_tab_name.truncate(self.new_tab_name.len() - 1);
                        should_render = true;
                    }
                }
                BareKey::Up => {
                    let entries = match &self.history {
                        Some(contents) => contents.entries(),
                        None => {
                            return false;
                        }
                    };

                    if entries.is_empty() {
                        return false;
                    }

                    if self.stashed_input.is_some() {
                        self.history_idx = (self.history_idx + 1).min(entries.len() - 1);
                    } else {
                        self.stashed_input = Some(self.new_tab_name.clone());
                    }

                    self.new_tab_name = entries[entries.len() - self.history_idx - 1].clone();

                    should_render = true;
                }
                BareKey::Down => {
                    let entries = match &self.history {
                        Some(contents) => contents.entries(),
                        None => {
                            return false;
                        }
                    };

                    if entries.is_empty() || self.stashed_input.is_none() {
                        return false;
                    }

                    if self.history_idx == 0 {
                        self.new_tab_name = self.stashed_input.take().unwrap_or_default();
                        return true;
                    }

                    self.history_idx = self.history_idx.saturating_sub(1);
                    self.new_tab_name = entries[entries.len() - self.history_idx - 1].clone();

                    should_render = true;
                }
                _ => {}
            },
            _ => {
                dbg!(&self);
                dbg!("{}", should_render);
            }
        }

        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        let text = format!("{PROMPT}{}{CURSOR}", self.new_tab_name);
        print_text_with_coordinates(Text::new(text), 1, 0, None, None);
    }
}

impl State {
    fn parse_configuration(&mut self, configuration: BTreeMap<String, String>) {
        self.use_zoxide = configuration
            .get("use_zoxide")
            .is_some_and(|v| v.to_lowercase() == "true");
    }

    fn new_named_tab(&mut self, cwd: Option<&String>) {
        new_tab(Some(&self.new_tab_name), cwd);

        if let Some(history) = &mut self.history {
            history.push(self.new_tab_name.trim());
        };

        self.new_tab_name = String::new();

        close_self();
    }
}
