use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn setup_camera(mut commands: Commands) {
    // commands spawn bundle orthographic camera bundle new 2d
    commands.spawn(Camera2dBundle::default());


//    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut commands: Commands) {
    // commands spawn bundle sprite bundle
    commands.spawn(SpriteBundle {
        // sprite sprite
        sprite: Sprite {
            // color snake head color
            color: SNAKE_HEAD_COLOR,
            // ..default
            ..Default::default()
        },
        // transform transform
        transform: Transform {
            // scale vec3 new 10.0 10.0 10.0
            scale: Vec3::new(10.0, 10.0, 10.0),
            // ..default
            ..Default::default()
        },
        // ..default
        ..Default::default()
    });
}

#[derive(Component)]
struct SnakeHead;

fn main() {
    //App::new().add_plugins(DefaultPlugins).run();
    App::new()
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_snake)
    .add_plugins(DefaultPlugins)
    .run();
}