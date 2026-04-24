use bevy::prelude::*;
use cw_bvy_v2::game_plugin;

fn main() {
    App::new().add_plugins(game_plugin).run();
}
