#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create a Graphics object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("Just a single static texture", 800, 600).unwrap();

    // Load a texture, whose bytes have been loaded at compile-time
    let texture = include_texture!(graphics, "assets/planet.png").unwrap();
    let quad = Quad::new(&graphics, [1., 0., 0., 1.], [[-64., -64.], [64., -64.], [64., 64.], [-64., 64.]]).unwrap();

    // The first argument is ignored because
    // this example doesn't need any `FrameInfo`
    run_until_closed(graphics, |_: FrameInfo, mut drawer: Drawer| {
        drawer.clear(0.1, 0., 1.);
        texture.drawer().draw(&mut drawer).unwrap();
        quad.drawer()
            .pos((-154., 154.))
            .draw(&mut drawer).unwrap();

        GameUpdate::nothing()
    })
}
