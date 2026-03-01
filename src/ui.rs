use zellij_tile::prelude::*;

pub const PROMPT: &str = "> ";
pub const CURSOR: char = '█';

pub fn setup_plugin_pane() {
    let plugin_id = get_plugin_ids().plugin_id;

    let coordinates = FloatingPaneCoordinates::default()
        .with_x_percent(40)
        .with_y_fixed(6)
        .with_width_percent(20)
        .with_height_fixed(3);

    change_floating_panes_coordinates(vec![(PaneId::Plugin(plugin_id), coordinates)]);

    rename_pane_with_id(PaneId::Plugin(plugin_id), "New tab name:");
}
