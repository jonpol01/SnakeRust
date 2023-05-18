//use std::slice::Windows;

//use bevy::core::FixedTimestep;
use bevy::{prelude::*, render::view::window};
use rand::prelude::random;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

const ARENA_HEIGHT: u32 = 10;
const ARENA_WIDTH: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

struct GameOverEvent;
struct GrowthEvent;

#[derive(Default)]
struct LastTailPosition(Option<Position>);

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

#[derive(Component)]
struct Food;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

// fn setup_camera(mut commands: Commands) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
// }
fn setup_camera(mut commands: Commands) {
    // commands spawn bundle orthographic camera bundle new 2d
    commands.spawn(Camera2dBundle::default());
}
fn spawn_snake(mut commands: Commands, mut segments: Local<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}
// fn spawn_snake(mut commands: Commands) {
//     // commands spawn bundle sprite bundle
//     commands.spawn(SpriteBundle {
//         // sprite sprite
//         sprite: Sprite {
//             // color snake head color
//             color: SNAKE_HEAD_COLOR,
//             // ..default
//             ..Default::default()
//         },
//         // transform transform
//         transform: Transform {
//             // scale vec3 new 10.0 10.0 10.0
//             scale: Vec3::new(10.0, 10.0, 10.0),
//             // ..default
//             ..Default::default()
//         },
//         // ..default
//         ..Default::default()
//     });
// }

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn snake_movement(
    //mut last_tail_position: ResMut<LastTailPosition>,
    mut last_tail_position: Local<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    //segments: ResMut<SnakeSegments>,
    segments: Local<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    //segments_res: ResMut<SnakeSegments>,
    segments_res: Local<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    //last_tail_position: Res<LastTailPosition>,
    last_tail_position: Local<LastTailPosition>,
    //mut segments: ResMut<SnakeSegments>,
    mut segments: Local<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

//fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
fn size_scaling(windows: Local<Window>, mut q: Query<(&Size, &mut Transform)>) {
        //let window = windows.get_primary().unwrap();
        let window = windows;
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Local<Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    //let window = windows.get_primary().unwrap();
    let window = windows;
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn food_spawner(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        //.insert_resource(WindowDescriptor {
        // .insert_resource(WindowPlugin::default().with(WindowDescriptor {
        //     title: "Snake!".to_string(),
        //     width: 500.0,
        //     height: 500.0,
        //     ..default()
        // })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            //window: WindowDescriptor { 
            primary_window: Some(Window {
                resolution: (500., 500.).into(),
                title: "Game of Life".to_string(),
              ..default()
            }),
            ..default()
        }))

//        .add_plugins(WindowPlugins)

        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_system(snake_movement_input.before(snake_movement))
        .add_event::<GameOverEvent>()
        // .add_system(
        //     // SystemSet::new()
        //     //     .with_run_criteria(FixedTimestep::step(0.150))
        //     //systemset
        //     systemset::SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(0.150))
        //         .with_system(snake_movement)
        //         .with_system(snake_eating.after(snake_movement))
        //         .with_system(snake_growth.after(snake_eating)),
        // )
        
        //add system for snake movement with fixed timestep
        .add_systems(<dyn SystemSet>::new()
            .with_run_criteria(FixedTime::new_from_secs(0.150))
            .with_system(snake_movement)
            .with_system(snake_eating.after(snake_movement))
            .with_system(snake_growth.after(snake_eating))
        )
        .add_system(game_over.after(snake_movement))
        .add_systems(
            SystemSet::new()
                .with_run_criteria(FixedTime::new_from_secs(1.0))
                .with_system(food_spawner),
        )
        
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}