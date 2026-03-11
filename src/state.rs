use crate::{
    input_state::InputState,
    ui::{CURSOR, PROMPT, setup_plugin_pane},
};
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Debug)]
pub struct State {
    use_zoxide: bool,
    input: InputState,
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
                    self.input.clear_name();
                    close_self();
                }
                BareKey::Char('d') if key.has_modifiers(&[KeyModifier::Alt]) => {
                    self.input.delete_entry();

                    dbg!("Should have deleted!");
                    should_render = true;
                }
                BareKey::Char(char) if char.is_ascii() => {
                    self.input.add_char(char);
                    should_render = true;
                }
                BareKey::Enter => {
                    if !self.input.new_tab_name().trim().is_empty() {
                        if self.use_zoxide {
                            let mut context = BTreeMap::new();
                            context.insert("zoxide_query".to_string(), "true".to_string());

                            let tab_name = self.input.new_tab_name();
                            let command: Vec<&str> = ["zoxide", "query"]
                                .into_iter()
                                .chain(tab_name.split_whitespace())
                                .collect();

                            run_command(&command, context);

                            should_render = true;
                        } else {
                            self.new_named_tab(None);
                            should_render = true;
                        }

                        self.input.reset_state();
                    }
                }
                BareKey::Esc => {
                    self.input.clear_name();
                    close_self();
                }
                BareKey::Backspace if key.has_modifiers(&[KeyModifier::Alt]) => {
                    self.input.delete_prev_word();

                    should_render = true;
                }
                BareKey::Backspace => {
                    should_render = self.input.del_char();
                }
                BareKey::Up => {
                    should_render = self.input.up();
                }
                BareKey::Down => {
                    should_render = self.input.down();
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
        let text = format!("{PROMPT}{}{CURSOR}", self.input.new_tab_name());
        print_text_with_coordinates(Text::new(text), 1, 0, None, None);
    }
}

impl Default for State {
    fn default() -> State {
        let input_state = InputState::init().unwrap_or_default();
        State {
            use_zoxide: false,
            input: input_state,
        }
    }
}

impl State {
    fn parse_configuration(&mut self, configuration: BTreeMap<String, String>) {
        self.use_zoxide = configuration
            .get("use_zoxide")
            .is_some_and(|v| v.to_lowercase() == "true");
    }

    fn new_named_tab(&mut self, cwd: Option<&String>) {
        new_tab(Some(&self.input.new_tab_name().to_string()), cwd);

        self.input.push_history();
        self.input.clear_name();

        close_self();
    }
}
