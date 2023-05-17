# Rust Snake Game

This is a simple implementation of the classic Snake game in Rust, using the Piston game engine and the OpenGL graphics backend. The game features a snake that moves around the screen, eating food and growing in length as it does so. The player controls the movement of the snake using the arrow keys, and must avoid running into the walls or the snake's own body. The game ends when the snake collides with the walls or its own body, at which point the player's score is displayed on the screen.

## Installation and Usage

To play the game, you'll need to have Rust and the Cargo package manager installed on your system. Once you have these tools installed, you can run the game by executing the following command in your terminal:

`cargo run --release`

This will compile the game and start the game loop, displaying the game window on your screen.

## Gameplay

Once the game has started, you can control the snake using the arrow keys on your keyboard. The snake will move continuously in the direction that you specify using the arrow keys. The snake will grow in length as it eats food, which will appear randomly on the screen. The game ends when the snake collides with the walls or its own body, at which point the player's score is displayed on the screen.

## Credits

This game was developed by [John Soliva] using Rust and the Piston game engine. The code is available on [GitHub](https://github.com/jonpol01/rust-snake-game).

## License

This game is released under the [MIT License](https://opensource.org/licenses/MIT).
