#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create a draw object, which creates a window with the given title and dimensions
    let draw = Draw::new("Just a single static texture", 800, 600);

    // Load a texture, whose bytes have been loaded at compile-time with the given dimensions
    let texture = include_texture!(draw, "planet.png", 64, 64).unwrap();

    // Create a game object with an empty logic function
    // and a render function that draws the texture unrotatedly in the middle of the window
    let game = Game::new(draw, |_, _| {}, |_, mut args|{
        args.draw_texture(&texture, 0., 0., 0.).unwrap();
    });

    // Run the game until the window is closed.
    game.run_until_closed();
}
