#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create a draw object, which creates a window with the given title and dimensions
    let draw = Draw::new("Just a single static texture", 800, 600);

    // Load a texture, whose bytes have been loaded at compile-time with the given dimensions
    let texture = include_texture!(draw, "planet.png", 64, 64).unwrap();

    // Create a game object with the draw instance
    let mut game = Game::new(draw);

    // Run the game until the window is closed.
    while let Some((_, mut drawer)) = game.update() {
        drawer.draw_texture(&texture, 0., 0., 0.).unwrap()
    }
}
