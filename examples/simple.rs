#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create a Graphics object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("Just a single static texture", 800, 600);

    // Load a texture, whose bytes have been loaded at compile-time
    let texture = include_texture!(graphics, "planet.png").unwrap();

    // Create a GameManager with the Graphics object
    let mut gm = GameManager::new(graphics);

    // Run the game until the window is closed.
    gm.run(Box::new(Simple{
        texture: texture
    }))

    /*
    while let Some((_, mut drawer)) = gm.next_frame() {
        drawer.clear(0.1, 0., 1.);
        drawer.draw_texture_rigid(&texture, 0., 0.).unwrap();
    }
    */
}

pub struct Simple{
    texture: Texture
}

impl State for Simple{
    fn frame(&mut self, frame: Option<(FrameInfo, Drawer)>) -> StateChange{
        match frame{
            Some((_, mut drawer)) => {
                drawer.clear(0.1, 0., 1.);
                drawer.draw_texture_rigid(&self.texture, 0., 0.).unwrap();
                StateChange::No
            },
            None => StateChange::Close
        }
    }
}
