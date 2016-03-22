#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create the `Graphics` object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("korome works!", 800, 600);

    // Load a texture, whose bytes have been loaded at compile-time
    let planet = include_texture!(graphics, "assets/planet.png").unwrap();

    // Create a planet object with the texture
    let mut planet = Object{
        tex: &planet,
        x: -400.,
        y: 300.,
        theta: 0.
    };

    // Create the GameManager
    let mut gm = GameManager::new(graphics);

    gm.run_until_closed(|info, mut drawer| {
        planet.update(&info);

        drawer.clear(0., 0., 1.);
        planet.draw(&mut drawer).unwrap();
    })
}

struct Object<'a>{
    x: f32,
    y: f32,
    theta: f32,
    tex: &'a Texture
}

impl<'a> Update for Object<'a>{
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
}

impl<'a> Draw for Object<'a>{
    fn draw(&self, drawer: &mut Drawer) -> DrawResult{
        drawer.draw_texture(self.tex, self.x, self.y, self.theta)
    }
}
