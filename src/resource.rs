use bevy::prelude::*;

pub fn cross_resource_plugin(app: &mut App) {
    app.add_systems(Startup, init_cross_resources);
}

#[derive(Component, Debug)]
struct Fragment {
    letter: char,
    amount: u32,
}

#[derive(Component, Debug)]
struct Rune {
    letter: char,
    amount: u32,
}

fn init_cross_resources(mut cmd: Commands) {
    for letter in 'A'..='Z' {
        let amount = 0;
        let frag = Fragment { letter, amount };
        let rune = Rune { letter, amount };

        println!("{}: {:?} and {:?}", letter, frag, rune);
        cmd.spawn(frag);
        cmd.spawn(rune);
    }
}
