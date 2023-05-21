# Snake Game

A simple implementation of the classic Snake game using Rust and the game engine Bevy.

## How to Play

Use arrow keys(←→↑↓) to control the snake's direction. The objective of the game is to eat the food (pink squares) to grow longer. The game is over if the snake runs into the wall or runs into its own body.

## Installation

To play the game, clone the repository:


`git clone https://github.com/jonpol01/SnakeRust.git
cd SnakeRust`

Ensure that you have Rust and Cargo installed. Then run the following command to start the game:


`cargo run --release`

## Game Design

The game board is a grid with a fixed size. The snake moves through the grid by moving one square at a time. The game loop ticks at a fixed rate, updating the game state and rendering the game in the window.

The Snake is made up of linked squares that grow longer as it eats food. Food is randomly generated on the board. Collisions occur when the snake's head collides with any part of its body or the walls.

The game has a simple scoring system where each piece of food that is eaten adds to the player's score. The game does not have a win condition, and the snake will continue to grow until it collides.

## Future Improvements

There are many ways to improve the game, including adding power-ups and obstacles, adding different game modes, or introducing multiplayer. There is also room to improve the graphics and audio.

## Acknowledgement

This game was inspired by the classic Snake game.


## Credits

This game was developed by [John Soliva] using Rust and the Piston game engine. The code is available on [GitHub](https://github.com/jonpol01/rust-snake-game).

## License

This game is released under the [MIT License](https://opensource.org/licenses/MIT).
