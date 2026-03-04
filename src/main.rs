mod history;
mod state;
mod ui;

use state::State;
use zellij_tile::prelude::*;

register_plugin!(State);
