use an_open_sky_sim::WorldGenerator;
use bevy::prelude::*;

const SIZE: f32 = 32.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2d);

    let generator = WorldGenerator::new(5., 42);

    for x in 0..10 {
        for y in 0..10 {
            let elevation = generator.elevation(x as f64, y as f64);
            let brightness = ((elevation + 1.) / 2.).clamp(0., 1.);

            cmd.spawn(Sprite::from_color(
                Color::srgb(brightness as f32, brightness as f32, brightness as f32),
                Vec2::splat(SIZE),
            ))
            .insert(Transform::from_xyz(x as f32 * SIZE, y as f32 * SIZE, 0.0));
        }
    }
}
