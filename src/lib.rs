use bevy::{color::palettes::tailwind::*, prelude::*};

pub fn game_plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
}

fn setup(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(Color::from(BLUE_200)));
    cmd.spawn(Camera2d);
}
