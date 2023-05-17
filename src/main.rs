extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use piston::input::UpdateArgs;
use piston::window::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;
use std::{thread, string};
use std::time::Duration;


pub struct Game {
    gl: GlGraphics,
    rows: u32,
    cols: u32,
    snake: Snake,
    just_eaten: bool,
    square_width: u32,
    food: Food,
    score: u32,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
    
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    
        let half_height = args.window_size[1] as f64 / 2.0;

        let board_width = self.square_width * self.cols;
        let board_height = self.square_width * self.rows;
        // Define the border rectangle parameters
        let border_color = WHITE;
        let border_width = 5.0;
        let border_padding = 5.0;
    
        // Define the border rectangle
        let border_rect = [
            border_padding,
            border_padding,
            board_width as f64 + border_width - 2.0 * border_padding,
            board_height as f64 + border_width - 2.0 * border_padding,
        ];

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BLACK, gl);
            
            // Draw the border rectangle inside the screen
            let border = rectangle::Rectangle::new_border(border_color, border_width);
            border.draw(border_rect, &c.draw_state, c.transform, gl);
        });

        // Only iterate over the snake parts that are in the top half of the screen
        let top_snake_parts: Vec<Snake_Piece> = self.snake.snake_parts
            .iter()
            //.filter(|p| p.1 < self.rows / 2)
            .filter(|p| p.1 < self.rows / 1)
            .map(|p| Snake_Piece(p.0, p.1))
            .collect();
    
        // Render the snake
        for p in &top_snake_parts {
            let x = p.0 * self.square_width;
            //let y = (p.1 - self.rows / 4) * self.square_width; // screen hight use only
            let y = p.1 * self.square_width;

            let square = rectangle::square(x as f64, y as f64, self.square_width as f64);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform;
                rectangle(GREEN, square, transform, gl);
            });
        }
    
        // Render the food
        if self.food.y < self.rows / 1 { // devide by 2
            self.food.render(&mut self.gl, args, self.square_width);
        }
    }

    fn update(&mut self, _args: &UpdateArgs) -> bool {
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.score += 1;
            self.just_eaten = false;
        }

        self.just_eaten = self.food.update(&self.snake);
        if self.just_eaten {
            use rand::Rng;
            use rand::thread_rng;
            // try my luck
            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0, self.cols);
                let new_y = r.gen_range(0, self.rows);
                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food { x: new_x, y: new_y };
                    break;
                }
            }
        }

        true
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.d.clone();
        self.snake.d = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}

/// The direction the snake moves in.
#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Snake {
    gl: GlGraphics,
    snake_parts: LinkedList<Snake_Piece>,
    width: u32,
    d: Direction,
}

#[derive(Clone)]
pub struct Snake_Piece(u32, u32);

impl Snake {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.snake_parts
            .iter()
            .map(|p| Snake_Piece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }

    /// Move the snake if valid, otherwise returns false.
    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: Snake_Piece =
            (*self.snake_parts.front().expect("No front of snake found.")).clone();

        if (self.d == Direction::UP && new_front.1 == 0)
            || (self.d == Direction::LEFT && new_front.0 == 0)
            || (self.d == Direction::DOWN && new_front.1 == rows - 1)
            || (self.d == Direction::RIGHT && new_front.0 == cols - 1)
        {
            return false;
        }

        match self.d {
            Direction::UP => new_front.1 -= 1,
            Direction::DOWN => new_front.1 += 1,
            Direction::LEFT => new_front.0 -= 1,
            Direction::RIGHT => new_front.0 += 1,
        }

        if !just_eaten {
            self.snake_parts.pop_back();
        }

        // Checks self collision.
        if self.is_collide(new_front.0, new_front.1) {
            return false;
        }

        self.snake_parts.push_front(new_front);
        true
    }

    fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    // Return true if snake ate food this update
    fn update(&mut self, s: &Snake) -> bool {
        let front = s.snake_parts.front().unwrap();
        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        use graphics;

        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        });
    }
}



fn start_game(opengl: OpenGL) {
    const SQUARE_WIDTH: u32 = 20;
    let WIDTH = 1280;
    let HEIGHT = 720;
    let COLS = (WIDTH as f64 / SQUARE_WIDTH as f64).floor() as u32;
    let ROWS = (HEIGHT as f64 / SQUARE_WIDTH as f64).floor() as u32;

    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [WIDTH, HEIGHT]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
    .for_folder("dejavu-sans-mono").unwrap();
    println!("{:?}", assets);
    let mut glyphs = window.load_font(assets.join("DejavuSansMono.ttf")).unwrap();


    loop {
        let mut game = Game {
            gl: GlGraphics::new(opengl),
            rows: ROWS,
            cols: COLS,
            square_width: SQUARE_WIDTH,
            just_eaten: false,
            food: Food {
                x: rand::thread_rng().gen_range(0, COLS),
                y: rand::thread_rng().gen_range(0, ROWS),
            },
            score: 0,
            snake: Snake {
                gl: GlGraphics::new(opengl),
                snake_parts: LinkedList::from_iter((vec![Snake_Piece(COLS / 2, ROWS / 2)]).into_iter()),
                width: SQUARE_WIDTH,
                d: Direction::DOWN,
            },
        };

        let mut events = Events::new(EventSettings::new()).ups(10);
        while let Some(e) = events.next(&mut window) {

            if let Some(r) = e.render_args() {

                game.render(&r);

                // let score = game.score.to_string();
                // let transform = r.draw_size.to_matrix();
                // let text_color = [1.0, 1.0, 1.0, 1.0];
                // let text_size = 32;
                // let text = graphics::Text::new_color(text_color, text_size);
                // graphics::text::Text::new_color(text_color, text_size)
                //     .draw(
                //         &score,
                //         &mut glyphs,
                //         &c.draw_state,
                //         transform,
                //         g
                //     ).unwrap();

                

            }

            if let Some(u) = e.update_args() {
                if !game.update(&u) {
                    break;
                }
            }

            if let Some(k) = e.button_args() {
                if k.state == ButtonState::Press {
                    game.pressed(&k.button);
                }
            }
        }

        println!("Congratulations, your score was: {}", game.score);
        // Pause for 3 seconds before restarting the game
        println!("Restarting the game in 3 seconds...");
        thread::sleep(Duration::from_secs(3));
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    start_game(opengl);
}