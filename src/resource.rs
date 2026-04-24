use bevy::{color::palettes::tailwind::*, prelude::*};

pub fn cross_resource_plugin(app: &mut App) {
    app.add_systems(Startup, (init_cross_resources, spawn_sprites).chain());
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

        cmd.spawn(frag);
        cmd.spawn(rune);
    }
}

fn spawn_sprites(
    mut cmd: Commands,
    frag_q: Query<(Entity, &Fragment)>,
    rune_q: Query<(Entity, &Rune)>,
) {
    let tile_size = 64.;
    let tile_gap = tile_size / 20.;
    let offset = tile_size + tile_gap;
    let font_size = tile_size / 4.;

    let mut pos_vec = Vec::with_capacity(26);
    for (x, y) in (-6..=6).zip([0; 13]) {
        pos_vec.push((x as f32, y as f32));
    }
    for (x, y) in (-6..=6).zip([-1; 13]) {
        pos_vec.push((x as f32, y as f32));
    }

    for ((e, frag), (x, y)) in frag_q.iter().zip(pos_vec.as_slice()) {
        cmd.entity(e).insert((
            Sprite::from_color(BLUE_700, Vec2::splat(tile_size)),
            Text2d::new(format!("{} {}", frag.letter, frag.amount)),
            TextFont::default().with_font_size(font_size),
            Transform::from_xyz(x * offset, y * offset, 1.),
        ));
    }

    for ((e, frag), (x, y)) in rune_q.iter().zip(pos_vec.as_slice()) {
        cmd.entity(e).insert((
            Sprite::from_color(PURPLE_700, Vec2::splat(tile_size)),
            Text2d::new(format!("{} {}", frag.letter, frag.amount)),
            TextFont::default().with_font_size(font_size),
            Transform::from_xyz(x * offset, y * offset - offset * 2., 2.),
        ));
    }
}
