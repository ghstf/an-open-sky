use aos_sim::{Terrain, WorldGenerator};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_enhanced_input::prelude::*;

const TILE_SIZE: f32 = 12.0;
const VIEW_SIZE: i32 = 50;
const CAMERA_SPEED: f32 = 60.0;

#[derive(Component, Default, Clone)]
struct Controller;

#[derive(InputAction)]
#[action_output(Vec2)]
struct Movement;

#[derive(Resource)]
struct World(WorldGenerator);

#[derive(Resource)]
struct Tiles(HashMap<(i32, i32), Entity>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnhancedInputPlugin)
        .insert_resource(World(WorldGenerator::new(5., 42)))
        .insert_resource(Tiles(HashMap::new()))
        .add_systems(Startup, setup)
        .add_input_context::<Controller>()
        .add_observer(apply_movement)
        .run();
}

fn setup(mut cmd: Commands, generator: Res<World>, mut tiles: ResMut<Tiles>) {
    let center = VIEW_SIZE / 2;
    update_visible_tiles(&mut cmd, &generator.0, &mut tiles, center, center);

    let camera = cmd
        .spawn((
            Camera2d,
            Transform::from_xyz(center as f32 * TILE_SIZE, center as f32 * TILE_SIZE, 1000.0),
            Controller,
        ))
        .id();

    let movement = cmd
        .spawn((
            Action::<Movement>::new(),
            ActionOf::<Controller>::new(camera),
        ))
        .id();

    cmd.spawn((Binding::from(KeyCode::KeyD), BindingOf(movement)));
    cmd.spawn((
        Binding::from(KeyCode::KeyA),
        BindingOf(movement),
        Negate::all(),
    ));
    cmd.spawn((
        Binding::from(KeyCode::KeyW),
        BindingOf(movement),
        SwizzleAxis::YXZ,
    ));
    cmd.spawn((
        Binding::from(KeyCode::KeyS),
        BindingOf(movement),
        Negate::all(),
        SwizzleAxis::YXZ,
    ));
}

fn apply_movement(
    movement: On<Fire<Movement>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    generator: Res<World>,
    mut tiles: ResMut<Tiles>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    let mut transform = camera.get_mut(movement.context).unwrap();
    let previous_tile = (
        (transform.translation.x / TILE_SIZE).floor() as i32,
        (transform.translation.y / TILE_SIZE).floor() as i32,
    );

    transform.translation += movement.value.extend(0.0) * CAMERA_SPEED * time.delta_secs();

    let x = (transform.translation.x / TILE_SIZE).floor() as i32;
    let y = (transform.translation.y / TILE_SIZE).floor() as i32;

    if (x, y) != previous_tile {
        update_visible_tiles(&mut cmd, &generator.0, &mut tiles, x, y);
    }
}

fn update_visible_tiles(
    cmd: &mut Commands,
    generator: &WorldGenerator,
    tiles: &mut Tiles,
    center_x: i32,
    center_y: i32,
) {
    let min_x = center_x - VIEW_SIZE / 2;
    let min_y = center_y - VIEW_SIZE / 2;
    let max_x = min_x + VIEW_SIZE;
    let max_y = min_y + VIEW_SIZE;

    for (tile_x, tile_y) in tiles.0.keys().cloned().collect::<Vec<_>>() {
        if (tile_x < min_x || tile_x >= max_x) || (tile_y < min_y || tile_y >= max_y) {
            if let Some(entity) = tiles.0.remove(&(tile_x, tile_y)) {
                cmd.entity(entity).despawn();
            }
        }
    }

    for x in min_x..max_x {
        for y in min_y..max_y {
            if tiles.0.contains_key(&(x, y)) {
                continue;
            }

            let elevation = generator.elevation(x as f64, y as f64);
            let brightness = ((elevation + 1.) / 2.).clamp(0., 1.);

            let color = if generator.terrain(x as f64, y as f64) == Terrain::Ocean {
                Color::srgb(0.0, 0.0, 1.0)
            } else {
                Color::srgb(brightness as f32, brightness as f32, brightness as f32)
            };

            tiles.0.insert(
                (x, y),
                cmd.spawn((
                    Sprite::from_color(color, Vec2::splat(TILE_SIZE)),
                    Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                ))
                .id(),
            );
        }
    }
}
