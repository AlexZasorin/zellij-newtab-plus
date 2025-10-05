use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Debug, Default)]
struct State {
    new_tab_name: String,
    awaiting_input: bool,
    permissions_granted: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::Reconfigure,
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::ModeUpdate,
            EventType::TabUpdate,
            EventType::Key,
            EventType::PermissionRequestResult,
        ]);

        if self.permissions_granted {
            hide_self();
        }
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;

        // TODO: Handle situation where user closes the plugin pane while in awaiting input or waiting for new tab state
        match event {
            Event::ModeUpdate(_) => {
                let plugin_ids = get_plugin_ids();

                // FIXME: Avoid rebinding if already bound? Other scenarios where this should be
                // bound?
                bind_key(plugin_ids.plugin_id);
            }
            Event::PermissionRequestResult(permission) => {
                self.permissions_granted = match permission {
                    PermissionStatus::Granted => true,
                    PermissionStatus::Denied => false,
                };

                if self.permissions_granted {
                    hide_self();
                }
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Char(char) if char.is_ascii() => {
                    self.new_tab_name.push(char);
                    should_render = true;
                }
                BareKey::Enter => {
                    if self.awaiting_input && !self.new_tab_name.trim().is_empty() {
                        new_tab(Some(&self.new_tab_name), None);

                        self.awaiting_input = false;
                        self.new_tab_name = String::new();
                        should_render = true;
                        hide_self();
                    }
                }
                BareKey::Esc => {
                    if self.awaiting_input {
                        self.awaiting_input = false;
                        self.new_tab_name = String::new();
                        should_render = true;
                        hide_self();
                    }
                }
                BareKey::Backspace => {
                    if self.awaiting_input && !self.new_tab_name.is_empty() {
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

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let mut should_render = false;
        if pipe_message.name == "new_tab" {
            if self.awaiting_input {
                self.awaiting_input = false;
                self.new_tab_name = String::new();
                hide_self();
            } else {
                self.awaiting_input = true;

                show_self(true);

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

                should_render = true;
            }
        }

        should_render
    }

    fn render(&mut self, _: usize, _: usize) {
        // FIXME: Center this somehow?
        print!("{}", self.new_tab_name);
    }
}

pub fn bind_key(own_plugin_id: u32) {
    let new_config = format!(
        "
        keybinds {{
            shared {{
                bind \"Ctrl n\" {{
                    MessagePluginId {} {{
                        name \"new_tab\"
                    }}
                }}
            }}
        }}
        ",
        own_plugin_id
    );
    reconfigure(new_config, false);
}
