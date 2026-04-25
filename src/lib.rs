pub mod resource;
use crate::resource::cross_resource_plugin;
use bevy::{color::palettes::css::BLACK, prelude::*};

pub fn game_plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);

    app.add_plugins(cross_resource_plugin);
}

fn setup(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::from(BLACK)));
    cmd.spawn(Camera2d);
}
