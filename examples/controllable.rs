#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create the draw object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("korome works!", 800, 600);

    // Load a texture, whose bytes have been loaded at compile-time
    let planet = include_texture!(graphics, "planet.png").unwrap();

    // Create a planet Object with the texture
    let mut planet = Object::new(&planet, -400., 300., 0.);

    // Create the game instance
    let mut gm = GameManager::new(graphics);

    while let Some((info, mut drawer)) = gm.next_frame() {
        logic(info, &mut planet);

        drawer.clear(0., 0., 1.);
        planet.draw(&mut drawer).unwrap();
    }
}

fn logic(info: FrameInfo, planet: &mut Object){
    let delta = info.delta as f32;

    let vel = 200.0 * delta;
    let ref mut pos = planet.pos;

    // Make the planet move with WASD and the arrow keys and rotate with Q and E
    is_down!{info;
        Left, A => {
            pos.0 -= vel
        },
        Right, D => {
            pos.0 += vel
        },
        Down , S => {
            pos.1 -= vel
        },
        Up   , W => {
            pos.1 += vel
        },
        Q => {
            planet.theta += delta
        },
        E => {
            planet.theta -= delta
        }
    }
}

struct Object<'a>{
    pos: Vector2<f32>,
    theta: f32,
    tex: &'a Texture
}

impl<'a> Object<'a>{
    fn new(tex: &'a Texture, x: f32, y: f32, theta: f32) -> Self{
        Object{
            tex: tex,
            pos: Vector2(x, y),
            theta: theta,
        }
    }
}

impl<'a> Draw for Object<'a>{
    fn draw(&self, drawer: &mut Drawer) -> DrawResult<()>{
        let (x, y) = self.pos.into();
        drawer.draw_texture(self.tex, x, y, self.theta)
    }
}
