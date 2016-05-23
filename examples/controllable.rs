#[macro_use]
extern crate korome;

use korome::*;
use korome::easy::*;

fn main() {
    // Create the `Graphics` object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("korome works!", 800, 600).unwrap();

    // Load a texture, whose bytes have been loaded at compile-time
    let planet = include_texture!(graphics, "assets/planet.png").unwrap();

    // Create an `EasyGame` with a planet object using the texture
    let g = EasyGame::with_vec(vec![
        Object{
            tex: &planet,
            x: -400.,
            y: 300.,
            theta: 0.
        }
    ]);

    run_until_closed(graphics, g)
}

struct Object<'a>{
    x: f32,
    y: f32,
    theta: f32,
    tex: &'a Texture
}

impl<'a> Obj for Object<'a>{
    fn update(&mut self, info: &FrameInfo){
        let delta = info.delta as f32;

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
    }
    fn draw(&self, drawer: &mut Drawer) -> DrawResult{
        drawer.draw_texture(self.tex, self.x, self.y, self.theta)
    }
}
