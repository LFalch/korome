#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create the `Graphics` object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("korome works!", 800, 600).unwrap();

    // Load a texture, whose bytes have been loaded at compile-time
    let planet = include_texture!(graphics, "assets/planet.png").unwrap();

    run_until_closed(graphics, Controllable{
        tex: &planet,
        x: -400.,
        y: 300.,
        theta: 0.,
    })
}

struct Controllable<'a>{
    x: f32,
    y: f32,
    theta: f32,
    tex: &'a Texture
}

impl<'a> Game for Controllable<'a>{
    fn frame(&mut self, info: &FrameInfo, drawer: &mut Drawer) -> GameUpdate{
        let delta = info.delta;
        let vel = 200.0 * delta;

        // Make the planet move with WASD and the arrow keys and rotate with Q and E
        is_down!{info;
            A, Left => {
                self.x -= vel
            },
            D, Right => {
                self.x += vel
            },
            S, Down => {
                self.y -= vel
            },
            W, Up => {
                self.y += vel
            },
            Q => {
                self.theta += delta
            },
            E => {
                self.theta -= delta
            }
        }

        drawer.clear(0., 0., 0.);

        self.tex.drawer()
            .pos((self.x, self.y))
            .rotation(self.theta)
            .draw(drawer);

        GameUpdate::Nothing
    }
}
