// ゲームの固定タイムステップを定義
use bevy::core::FixedTimestep;
// Bevyの前準備
use bevy::prelude::*;
// ランダムな数値を生成
use rand::prelude::random;

// ゲームオブジェクトの色を定義
const SNAKE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const SNAKE_TAIL_COLOR: Color = Color::rgb(1.0, 0.7, 0.6);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
// スネークの速度を定義
const SNAKE_SPEED: u32 = 1;

// アリーナの境界線とサイズを定義
const ARENA_BORDER: u32 = 1;
const ARENA_HEIGHT: u32 = 30; // 画面の高さ / アリーナの高さ
const ARENA_WIDTH: u32 = 30; // 画面の幅 / アリーナの幅

// ゲームオブジェクトの構造体とそれに付随するコンポーネントの定義
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u32,
    y: u32,
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
// ゲームオーバー時にトリガーするイベント
struct GameOverEvent;
// スネークが成長したときにトリガーするイベント
struct GrowthEvent;

#[derive(Default)]
struct FoodSpawnEvent;

#[derive(Default)]
// 最終的なTailの位置を保持
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
// 方向を逆転させる
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
// カメラを作成するための関数
fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>, mut score: ResMut<u32>) {
    // カメラを2Dに設定
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // スコアを表示するテキストを作成
    let score_entity = commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format!("Score: {}", *score).as_str(), // スコアを表示
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
    // スコアエンティティへの参照を保持
    commands.insert_resource(score_entity);
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    // ヘッドの初期座標をランダムに決定
    let x = (random::<f32>() * (ARENA_WIDTH - ARENA_BORDER)  as f32) as u32;
    let y = (random::<f32>() * (ARENA_HEIGHT - ARENA_BORDER) as f32) as u32;
    // 方向をランダムに決定
    let direction = match random::<u8>() % 4 {
        0 => Direction::Left,
        1 => Direction::Up,
        2 => Direction::Right,
        _ => Direction::Down,
    };

    // スネークのヘッドを生成し、初期位置と方向を設定
    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR, // スネークの色を定義
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
    ]);
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    // スネークのセグメントを生成し、位置を設定
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_TAIL_COLOR, // スネークのセグメントの色を定義
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
    // スネークのヘッドとボディを移動させる
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= SNAKE_SPEED;
            }
            Direction::Right => {
                head_pos.x += SNAKE_SPEED;
            }
            Direction::Up => {
                head_pos.y += SNAKE_SPEED;
            }
            Direction::Down => {
                head_pos.y -= SNAKE_SPEED;
            }
        };
        // スネークが画面端かどうかを判定し、端を越えた場合はゲームオーバーにする
        if head_pos.x < 1
            || head_pos.y < 1
            || head_pos.x as u32 >= ARENA_WIDTH - 1
            || head_pos.y as u32 >= ARENA_HEIGHT - 1
        {
            game_over_writer.send(GameOverEvent);
        }
        // スネークが自分自身に当たった場合はゲームオーバーにする
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        // スネークのボディを移動
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
    // もしSnakeHeadが存在したら、キーボードの入力を受け取り、SnakeHeadの方向を変更する
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
        // SnakeHeadが反対の方向に動くことがないようにする
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn game_over(
    mut commands: Commands,
    // ゲームオーバーイベントを受け取る
    mut reader: EventReader<GameOverEvent>,
    score_entity: Res<Entity>,
    mut query: Query<&mut Text, With<Text>>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    // ゲームオーバーイベントがあれば、全てのFoodとSnakeSegmentを削除し、新しいSnakeを生成する
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
            // スコアテキストを取得して更新する
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
    score_entity: Res<Entity>, // スコアテキストのEntityを参照する
) {
    // ゲームオーバーイベントがあれば、スコアを0にリセットする
    if reader.iter().next().is_some() {
        *score = 0;
    }

    // SnakeHeadがFoodに接触したら、Foodを削除し、Snakeを成長させ、スコアを更新する
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
                *score += 1;
                info!("Score: {}", *score);
                // スコアテキストを取得して更新する
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
    // 成長イベントがあれば、Snakeを成長させる
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        // ウィンドウサイズに合わせてスプライトサイズを変更する
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    // ゲーム内の位置とウィンドウサイズの範囲を変換する関数
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / (bound_game / 2.0); // 画面の半分を使うために2で割る
        // ウィンドウサイズを合わせて中央に揃える
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)        
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        // Entityの位置をウィンドウサイズに合わせて変換してセットする
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
    // まだFoodが存在していなければ、ランダムな場所に新しいFoodを生成する
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
            x: (random::<f32>() * ARENA_WIDTH as f32) as u32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as u32,
        })
        .insert(Size::square(0.8));
    writer.send(FoodSpawnEvent);
}

fn main() {

    // Appを作成する
     App::new()
         // 背景色を設定する
         .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
         .insert_resource(WindowDescriptor {
             // ウィンドウのタイトルを設定する
             title: "Snake-rust".to_string(),
             // ウィンドウのサイズを設定する
             width: 1200.0,
             height: 900.0,
             ..default()
         })
         // カメラをセットアップするStartupSystemを登録する
         .add_startup_system(setup_camera)
         // Snakeを生成するStartupSystemを登録する
         .add_startup_system(spawn_snake)
         // Snakeのセグメントを管理するSnakeSegmentsを初期化する
         .insert_resource(SnakeSegments::default())
         // 最後尾のSnakeSegmentの位置を管理するLastTailPositionを初期化する
         .insert_resource(LastTailPosition::default())
         // スコアを0で初期化する
         .insert_resource::<u32>(0)
         // 成長イベントを登録する
         .add_event::<GrowthEvent>()
         // Snakeの動きを処理するSystemを登録する
         .add_system(snake_movement_input.before(snake_movement))
         // ゲームオーバーイベントを登録する
         .add_event::<GameOverEvent>()
         // Snakeの移動、食事、成長を処理するSystemSetを登録する
         .add_system_set(
             SystemSet::new()
                 // Snakeの移動を処理する
                 .with_run_criteria(FixedTimestep::step(0.150))
                 .with_system(snake_movement)
                 // SnakeがFoodを食べたときの処理をする
                 .with_system(snake_eating.after(snake_movement))
                 // Snakeが成長したときの処理をする
                 .with_system(snake_growth.after(snake_eating)),
         )
         // ゲームオーバー時の処理をする
         .add_system(game_over.after(snake_movement))
         // Foodの生成イベントを登録する
         .add_event::<FoodSpawnEvent>()
         // Foodを生成するSystemSetを登録する
         .add_system_set(
             SystemSet::new()
                 // 一定時間ごとにFoodを生成する
                 .with_run_criteria(FixedTimestep::step(0.5))
                 .with_system(food_spawner)
         )
         // PostUpdateステージに位置の変換やサイズの調整をするSystemを登録する
         .add_system_set_to_stage(
             CoreStage::PostUpdate,
             SystemSet::new()
                 // 位置を画面に合わせて変換する
                 .with_system(position_translation)
                 // サイズを画面に合わせて調整する
                 .with_system(size_scaling),
         )
         // デフォルトプラグインを追加する
         .add_plugins(DefaultPlugins)
         // アプリを実行する
         .run();
 }