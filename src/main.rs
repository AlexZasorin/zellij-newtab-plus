use zellij_tile::prelude::*;

use std::collections::BTreeMap;

const PROMPT: &str = "> ";
const CURSOR: char = '█';

#[derive(Debug, Default)]
struct State {
    new_tab_name: String,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
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
                if context.contains_key("zoxide_query") && error_code == Some(0) {
                    let stdout_str = String::from_utf8_lossy(&stdout);
                    new_tab(
                        Some(&self.new_tab_name),
                        Some(&stdout_str.trim().to_string()),
                    );
                    self.new_tab_name = String::new();
                    close_self();

                    should_render = true;
                }
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Char(char) if char.is_ascii() => {
                    self.new_tab_name.push(char);
                    should_render = true;
                }
                BareKey::Enter => {
                    if !self.new_tab_name.trim().is_empty() {
                        let mut context = BTreeMap::new();
                        context.insert("zoxide_query".to_string(), "true".to_string());
                        run_command(&["zoxide", "query", &self.new_tab_name], context);

                        should_render = true;
                    }
                }
                BareKey::Esc => {
                    self.new_tab_name = String::new();
                    close_self();
                }
                BareKey::Backspace => {
                    if !self.new_tab_name.is_empty() {
                        self.new_tab_name.truncate(self.new_tab_name.len() - 1);
                        should_render = true;
                    }
                }
                _ => {}
            },
            _ => {
                dbg!("{}", &self);
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

fn setup_plugin_pane() {
    let plugin_id = get_plugin_ids().plugin_id;

    let coordinates = FloatingPaneCoordinates::default()
        .with_x_percent(40)
        .with_y_fixed(6)
        .with_width_percent(20)
        .with_height_fixed(3);

    change_floating_panes_coordinates(vec![(PaneId::Plugin(plugin_id), coordinates)]);

    rename_pane_with_id(PaneId::Plugin(plugin_id), "New tab name:");
}
