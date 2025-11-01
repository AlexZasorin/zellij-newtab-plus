use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Debug, Default)]
struct State {
    new_tab_name: String,
    permissions_granted: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        hide_self();
        request_permission(&[
            PermissionType::Reconfigure,
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::Key, EventType::PermissionRequestResult]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;

        match event {
            Event::PermissionRequestResult(permission) => {
                self.permissions_granted = match permission {
                    PermissionStatus::Granted => true,
                    PermissionStatus::Denied => false,
                };

                if self.permissions_granted {
                    setup_plugin_pane();
                    show_self(true);
                }
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Char(char) if char.is_ascii() => {
                    self.new_tab_name.push(char);
                    should_render = true;
                }
                BareKey::Enter => {
                    if !self.new_tab_name.trim().is_empty() {
                        new_tab(Some(&self.new_tab_name), None);

                        self.new_tab_name = String::new();
                        should_render = true;
                        close_self();
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
        print_text_with_coordinates(Text::new(self.new_tab_name.to_string()), 1, 0, None, None);
    }
}

fn setup_plugin_pane() {
    let plugin_id = get_plugin_ids().plugin_id;

    let coordinates = FloatingPaneCoordinates::new(
        Some(String::from("40%")),
        Some(String::from("6")),
        Some(String::from("20%")),
        Some(String::from("3")),
        Some(false),
    );

    match coordinates {
        None => eprintln!("Invalid floating pane coordinates."),
        Some(floating_coordinates) => {
            change_floating_panes_coordinates(vec![(
                PaneId::Plugin(plugin_id),
                floating_coordinates,
            )]);
        }
    }

    rename_pane_with_id(
        PaneId::Plugin(plugin_id),
        String::from("Enter new tab name:"),
    );
}
