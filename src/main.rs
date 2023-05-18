use bevy::{prelude::*};

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct SnakeHead;

fn setup_camera(mut commands: Commands) {
    // commands spawn bundle orthographic camera bundle new 2d
    commands.spawn(Camera2dBundle::default());
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

fn snake_movement(mut head_positions: Query<(&SnakeHead, &mut Transform)>) {

    for (_head, mut transform) in &mut head_positions {
        info!("Head: {:?}", transform.translation);
        transform.translation.y += 2.;
    }
}

fn snake_move(
    keyboard_input: Res<Input<KeyCode>>, 
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
){
    for mut transform in &mut head_positions {
        if keyboard_input.pressed(KeyCode::Left){
            info!("Left");
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right){
            info!("Right");
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down){
            info!("Down");
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up){
            info!("Up");
            transform.translation.y += 2.;
        }
    }

}

// This system will move the cube forward.
// fn move_cube(mut cubes: Query<(&mut Transform, &mut CubeState)>, timer: Res<Time>) {
//     for (mut transform, cube) in &mut cubes {
//         // Move the cube forward smoothly at a given move_speed.
//         let forward = transform.forward();
//         transform.translation += forward * cube.move_speed * timer.delta_seconds();
//     }
// }

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_snake)
    .add_system(snake_movement)
    .run();
}