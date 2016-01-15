#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create the draw object, which creates a window with the given title and dimensions
    let draw = Draw::new("korome works!", 800, 600);

    // Load a texture, whose bytes have been loaded at compile-time with the given dimensions
    let planet = include_texture!(draw, "planet.png", 64, 64).unwrap();

    // Create a vector and push the objects to it
    let mut objs = Vec::new();
    objs.push(Object::new(&planet, -400., 300., 0.));

    // Create the game instance with objs as the shared
    let game = Game::with_shared(draw, objs, logic, render);
    game.run_until_closed();
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

impl<'a> Sprite for Object<'a>{
    fn get_pos(&self) -> (f32, f32){
        self.pos.into()
    }

    fn get_rotation(&self) -> f32{
        self.theta
    }

    fn get_texture(&self) -> &Texture{
        self.tex
    }
}

fn logic(objs: &mut Vec<Object>, l_args: LogicArgs){
    // Get a mutable reference so we can move it
    let ref mut planet = objs[0];

    let delta = l_args.delta as f32;

    // Set the velocity the 200 pixels per second
    let vel = 200.0 * delta;
    let pos = &mut planet.pos;

    // Make the planet move with WASD and the arrow keys and rotate with Q and E
    is_down!{
        l_args;

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
        E => {
            planet.theta += delta
        },
        Q => {
            planet.theta -= delta
        }
    }
}

fn render(objs: &Vec<Object>, mut r_args: RenderArgs){
    // Draw all sprites in objs
    r_args.draw_sprites(objs).unwrap();
}
