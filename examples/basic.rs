extern crate glium;
#[macro_use]
extern crate korome;

use korome::{Game, Draw, GameLogic, LogicArgs, RenderArgs, Vector2};
use korome::draw::{Drawable, Texture};

fn main() {
    let draw = Draw::new("glium works!", 800, 600);

    let planet = draw.load_texture_from_bytes(include_bytes!("planet.png"), 64, 64).unwrap();

    let mut logic = Logic::new();

    logic.add_object(Object::new(&planet, (-400., 300.), 0.));

    let game = Game::new(logic, draw);

    game.run_until_closed();
}

struct Object<'a>{
    pos: Vector2<f32>,
    theta: f32,
    tex: &'a Texture
}

impl<'a> Object<'a>{
    fn new<V: Into<Vector2<f32>>>(tex: &'a Texture, pos: V, theta: f32) -> Self{
        Object{
            tex: tex,
            pos: pos.into(),
            theta: theta,
        }
    }
}

impl<'a> Drawable for Object<'a>{
    fn get_pos(&self) -> (f32, f32){
        self.pos.get_x_y()
    }

    fn get_rotation(&self) -> f32{
        self.theta
    }

    fn get_texture(&self) -> &Texture{
        self.tex
    }
}

struct Logic<'a> {
    objects: Vec<Object<'a>>,
}

impl<'a> Logic<'a>{
    pub fn new() -> Self{
        Logic{
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: Object<'a>){
        self.objects.push(obj)
    }
}

impl<'a> GameLogic for Logic<'a> {
    fn logic (&mut self, l_args: LogicArgs){
        let ref mut planet = self.objects[0];

        let delta = l_args.delta() as f32;

        let vel = 200.0 * delta;
        let pos = &mut planet.pos;

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

    fn render(&self, mut r_args: RenderArgs){
        //.rotate() doesn't actually work properly right now
        r_args.draw_drawables()
            .add_vec(&self.objects)
            .draw()
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
