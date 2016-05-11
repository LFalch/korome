#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create a Graphics object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("Just a single static texture", 800, 600).unwrap();

    // Load a texture, whose bytes have been loaded at compile-time
    let texture = include_texture!(graphics, "assets/planet.png").unwrap();

    // Create a GameManager with the Graphics object
    let gm = GameManager::new(graphics);

    // The first argument is ignored because
    // this example doesn't need any `FrameInfo`
    gm.run_until_closed(|_: FrameInfo, mut drawer: Drawer| {
        drawer.clear(0.1, 0., 1.);
        drawer.draw_texture_rigid(&texture, 0., 0.).unwrap();
        
        GameUpdate::nothing()
    })
}
