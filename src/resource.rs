use bevy::{
    color::palettes::tailwind::*,
    input::{
        ButtonState,
        common_conditions::{input_just_pressed, input_pressed},
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

pub fn cross_resource_plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (
            cboard_init,
            ctile_selector_init,
            fragrune_init,
            fragrune_spawn_sprites,
        )
            .chain(),
    );
    app.add_systems(
        Update,
        (
            fragrune_increment.run_if(input_pressed(KeyCode::Space)),
            fragrune_toggle_vis.run_if(input_just_pressed(KeyCode::Tab)),
            ctile_selector_move,
            ctile_selector_render,
            ctile_selector_letter_update,
            ctile_selector_letter_render,
        ),
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

fn fragrune_init(mut cmd: Commands) {
    for letter in 'A'..='Z' {
        let amount = 0;
        let frag = Fragment { letter, amount };
        let rune = Rune { letter, amount };

        cmd.spawn(frag);
        cmd.spawn(rune);
    }
}

fn fragrune_spawn_sprites(
    mut cmd: Commands,
    frag_q: Query<(Entity, &Fragment)>,
    rune_q: Query<(Entity, &Rune)>,
) {
    let tile_size = 50.;
    let tile_gap = tile_size / 20.;
    let size_multiplier = tile_size + tile_gap;
    let font_size = tile_size / 4.;
    let y_offset = -5. * size_multiplier;

    let mut pos_vec = Vec::with_capacity(26);
    for (x, y) in (-6..=6).zip([0; 13]) {
        pos_vec.push((x as f32, y as f32));
    }
    for (x, y) in (-6..=6).zip([-1; 13]) {
        pos_vec.push((x as f32, y as f32));
    }

    for ((e, frag), (x, y)) in frag_q.iter().zip(pos_vec.as_slice()) {
        cmd.entity(e).insert((
            Sprite::from_color(BLUE_800, Vec2::splat(tile_size)),
            Text2d::new(format!("{} {}", frag.letter, frag.amount)),
            TextFont::default().with_font_size(font_size),
            Transform::from_xyz(x * size_multiplier, y * size_multiplier + y_offset, 3.),
            Visibility::Visible,
        ));
    }

    for ((e, rune), (x, y)) in rune_q.iter().zip(pos_vec.as_slice()) {
        cmd.entity(e).insert((
            Sprite::from_color(PURPLE_800, Vec2::splat(tile_size)),
            Text2d::new(format!("{} {}", rune.letter, rune.amount)),
            TextFont::default().with_font_size(font_size),
            Transform::from_xyz(x * size_multiplier, y * size_multiplier + y_offset, 2.),
            Visibility::Visible,
        ));
    }
}

fn fragrune_increment(
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

fn fragrune_toggle_vis(mut frag_q: Query<&mut Visibility, With<Fragment>>) {
    for mut vis in frag_q.iter_mut() {
        *vis = match *vis {
            Visibility::Visible => Visibility::Hidden,
            _ => Visibility::Visible,
        };
    }
}

#[derive(Component)]
struct CrossTile {
    letter: Option<char>,
    x: i32,
    y: i32,
}

#[derive(Component)]
struct CrossBoard;

fn cboard_init(mut cmd: Commands) {
    let tile_size = 100.;
    let tile_gap = tile_size / 20.;
    let size_multiplier = tile_size + tile_gap;
    let font_size = tile_size / 2.;

    let range_y = 5;
    let range_x = 11;

    cmd.spawn((
        CrossBoard,
        Transform::from_xyz(0., size_multiplier / 2., 0.),
        Visibility::Visible,
    ))
    .with_children(|p| {
        for x in -range_x / 2..=range_x / 2 {
            for y in -range_y / 2..=range_y / 2 {
                p.spawn((
                    CrossTile { letter: None, x, y },
                    Sprite::from_color(GRAY_800, Vec2::splat(tile_size)),
                    Text2d::new(""),
                    TextFont::default().with_font_size(font_size),
                    Transform::from_xyz(x as f32 * size_multiplier, y as f32 * size_multiplier, 1.),
                    Visibility::Inherited,
                ));
            }
        }
    });
}

#[derive(Component)]
struct CrossTileSelector {
    e: Entity,
    x: i32,
    y: i32,
    letter: Option<char>,
}

fn ctile_selector_init(
    mut cmd: Commands,
    ctile_q: Query<(Entity, &CrossTile, &Transform)>,
    cboard_s: Single<Entity, With<CrossBoard>>,
) {
    let sprite_size = 110.;
    let font_size = sprite_size / 4.;
    for (e, ctile, ctile_t) in ctile_q.iter() {
        if ctile.x == 0 && ctile.y == 0 {
            let mut selector_transform = *ctile_t;
            selector_transform.translation.z += 1.;

            cmd.entity(cboard_s.entity()).with_child((
                CrossTileSelector {
                    e,
                    x: ctile.x,
                    y: ctile.y,
                    letter: None,
                },
                Sprite::from_color(GRAY_200.with_alpha(0.02), Vec2::splat(sprite_size)),
                selector_transform,
                Text2d::new("Z"),
                TextFont::default().with_font_size(font_size),
            ));
            break;
        }
    }
}

fn ctile_selector_move(
    ctile_q: Query<(Entity, &CrossTile)>,
    mut ctile_selector_s: Single<&mut CrossTileSelector>,
    mut arrow_input: MessageReader<KeyboardInput>,
) {
    for key in arrow_input.read() {
        let (x_add, y_add) = match (key.key_code, key.state) {
            (KeyCode::ArrowUp, ButtonState::Pressed) => (0, 1),
            (KeyCode::ArrowDown, ButtonState::Pressed) => (0, -1),
            (KeyCode::ArrowLeft, ButtonState::Pressed) => (-1, 0),
            (KeyCode::ArrowRight, ButtonState::Pressed) => (1, 0),
            _ => (0, 0),
        };

        let x = ctile_selector_s.x + x_add;
        let y = ctile_selector_s.y + y_add;
        for (e, ctile) in ctile_q.iter() {
            if x == ctile.x && y == ctile.y {
                ctile_selector_s.e = e;
                ctile_selector_s.x = x;
                ctile_selector_s.y = y;
            }
        }
    }
}

fn ctile_selector_render(
    transform_q: Query<&Transform, Without<CrossTileSelector>>,
    ctile_selector_s: Single<(&mut Transform, &CrossTileSelector)>,
) {
    let (mut ctile_sel_t, ctile_sel) = ctile_selector_s.into_inner();
    let Ok(new_t) = transform_q.get(ctile_sel.e) else {
        return;
    };

    *ctile_sel_t = *new_t;
}

fn ctile_selector_letter_update(
    mut ctile_selector_s: Single<&mut CrossTileSelector>,
    mut keyboard: MessageReader<KeyboardInput>,
) {
    for letter in keyboard.read() {
        if letter.state == ButtonState::Released {
            return;
        }

        match &letter.logical_key {
            Key::Character(smol_str) => {
                let l = smol_str.chars().next().unwrap().to_ascii_uppercase();
                if l >= 'A' && l <= 'Z' {
                    ctile_selector_s.letter = Some(l);
                }
            }
            Key::Backspace => {
                ctile_selector_s.letter = None;
            }
            _ => {}
        }
    }
}

fn ctile_selector_letter_render(ctile_selector_s: Single<(&mut Text2d, &CrossTileSelector)>) {
    let (mut text_2d, ctile_sel) = ctile_selector_s.into_inner();
    match ctile_sel.letter {
        Some(l) => {
            text_2d.0 = l.into();
        }
        None => {
            text_2d.0 = "".to_string();
        }
    }
}
