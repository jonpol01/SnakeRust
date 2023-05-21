use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::random;

use bevy::sprite::Sprite;
use bevy::prelude::{SpriteBundle, Color};

//snake head color red
const SNAKE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const SNAKE_TAIL_COLOR: Color = Color::rgb(1.0, 0.7, 0.6);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

const ARENA_HEIGHT: u32 = 30; // screen height / arena height
const ARENA_WIDTH: u32 = 30; // screen width / arena width


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
struct FoodSpawnEvent;

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

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>, mut score: ResMut<u32>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let score_entity = commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format!("Score: {}", *score).as_str(), // display the score
            TextStyle {
                font: asset_server.load("dejavu-sans-mono/DejaVuSansMono.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform::from_translation(Vec3::new(-350.0, 400.0, 0.0)),
        ..Default::default()
    }).id();
    commands.insert_resource(score_entity); // keep a reference to the score entity
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {

    let x = (random::<f32>() * ARENA_WIDTH as f32) as i32;
    let y = (random::<f32>() * ARENA_HEIGHT as f32) as i32;
    let direction = match random::<u8>() % 4 {
        0 => Direction::Left,
        1 => Direction::Up,
        2 => Direction::Right,
        _ => Direction::Down,
    };

    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            })
            .insert(SnakeHead {
                direction,
            })
            .insert(SnakeSegment)
            .insert(Position { x: x, y: y })
            .insert(Size::square(0.8))
            .id(),
        //spawn_segment(commands, Position { x: 3, y: 3 }),
    ]);

    
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_TAIL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.8))
        .id()

}

fn snake_movement(
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    segments: ResMut<SnakeSegments>,
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
    score_entity: Res<Entity>,
    mut query: Query<&mut Text, With<Text>>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
            // retrieve the score text and update it
            if let Ok(mut text) = query.get_mut(*score_entity) {
                text.sections[0].value = format!("Score: {}", 0);
                info!("Game Over! Score: {}", 0);
            }
        }
        spawn_snake(commands, segments_res);
    }
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    mut score: ResMut<u32>,
    mut query: Query<&mut Text, With<Text>>,
    mut reader: EventReader<GameOverEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
    score_entity: Res<Entity>, // add a reference to the score entity
) {
    if reader.iter().next().is_some() {
        *score = 0;
    }

    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
                *score += 1;
                info!("Score: {}", *score);
                // retrieve the score text and update it
                if let Ok(mut text) = query.get_mut(*score_entity) {
                    text.sections[0].value = format!("Score: {}", *score);
                }
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    // fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    //     let tile_size = bound_window / bound_game; 
    //     pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    // }
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / (bound_game / 2.0); // Divide by 2 to use half of the screen
        // screen width is 1200.. use only have if it and center it
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)        
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}


fn food_spawner(
    mut commands: Commands,
    mut writer: EventWriter<FoodSpawnEvent>,
    food: Query<Entity, With<Food>>,
) {
    if food.iter().next().is_some() {
        return;
    }

    commands
        .spawn_bundle(SpriteBundle {
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
    writer.send(FoodSpawnEvent);
}

fn main() {

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Snake-rust".to_string(),
            width: 1200.0,
            height: 900.0,
            ..default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource::<u32>(0)
        .add_event::<GrowthEvent>()
        .add_system(snake_movement_input.before(snake_movement))
        .add_event::<GameOverEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement)
                .with_system(snake_eating.after(snake_movement))
                .with_system(snake_growth.after(snake_eating)),
        )
        .add_system(game_over.after(snake_movement))
        .add_event::<FoodSpawnEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(food_spawner)
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
