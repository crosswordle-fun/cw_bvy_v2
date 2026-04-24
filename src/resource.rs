use bevy::{
    color::palettes::tailwind::*,
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};

pub fn cross_resource_plugin(app: &mut App) {
    app.add_systems(Startup, (init_cross_resources, spawn_sprites).chain());
    app.add_systems(
        Update,
        debug_increment_frag_and_rune.run_if(input_pressed(KeyCode::Space)),
    );
    app.add_systems(
        Update,
        toggle_frag_vis.run_if(input_just_pressed(KeyCode::Tab)),
    );
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
    let tile_size = 75.;
    let tile_gap = tile_size / 20.;
    let size_multiplier = tile_size + tile_gap;
    let font_size = tile_size / 4.;
    let y_offset = -3. * size_multiplier;

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
            Transform::from_xyz(x * size_multiplier, y * size_multiplier + y_offset, 2.),
            Visibility::Visible,
        ));
    }

    for ((e, rune), (x, y)) in rune_q.iter().zip(pos_vec.as_slice()) {
        cmd.entity(e).insert((
            Sprite::from_color(PURPLE_700, Vec2::splat(tile_size)),
            Text2d::new(format!("{} {}", rune.letter, rune.amount)),
            TextFont::default().with_font_size(font_size),
            Transform::from_xyz(x * size_multiplier, y * size_multiplier + y_offset, 1.),
            Visibility::Visible,
        ));
    }
}

fn debug_increment_frag_and_rune(
    mut frag_q: Query<(&mut Fragment, &mut Text2d), Without<Rune>>,
    mut rune_q: Query<(&mut Rune, &mut Text2d)>,
) {
    for (mut frag, mut text_2d) in frag_q.iter_mut() {
        frag.amount += 1;
        text_2d.0 = format!("{} {}", frag.letter, frag.amount);
    }
    for (mut rune, mut text_2d) in rune_q.iter_mut() {
        rune.amount += 1;
        text_2d.0 = format!("{} {}", rune.letter, rune.amount);
    }
}

fn toggle_frag_vis(mut frag_q: Query<&mut Visibility, With<Fragment>>) {
    for mut vis in frag_q.iter_mut() {
        *vis = match *vis {
            Visibility::Visible => Visibility::Hidden,
            _ => Visibility::Visible,
        };
    }
}
