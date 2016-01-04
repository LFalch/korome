extern crate glium;
#[macro_use]
extern crate korome;

use korome::{Game, Draw, GameLogic, LogicArgs, RenderArgs, Vector2, Result};
use korome::KoromeError::IoError;
use korome::draw::Drawable;

use std::io::ErrorKind::NotFound;
use std::error::Error;

fn main() {
    let mut draw = Draw::new("glium works!", 800, 600);

    if let Err(e) = load_textures(&mut draw){
        if let IoError(ref io) = e {
            match io.kind(){
                NotFound => println!("Not all textures files were found"),
                _        => println!("An unknown error occured when trying to load textures:\n{:?}", e)
            }
        }
        println!("{}", e.description());

        return
    }

    let mut logic = Logic::new();

    logic.add_object(Object::new((-400., 300.), 0.));

    let game = Game::new(logic, draw);

    game.run_until_closed();
}

fn load_textures(draw: &mut Draw) -> Result<()>{
    // try!(draw.load_texture("planet", 64, 64));

    draw.load_texture_from_bytes("planet", include_bytes!("planet.png"), 64, 64)
}

struct Object{
    pos: Vector2,
    theta: f32,
}

impl Object{
    fn new<V: Into<Vector2>>(pos: V, theta: f32) -> Self{
        Object{
            pos: pos.into(),
            theta: theta,
        }
    }
}

impl Drawable for Object{
    fn get_pos(&self) -> (f32, f32){
        self.pos.get_x_y()
    }

    fn get_rotation(&self) -> f32{
        self.theta
    }

    fn get_texture(&self) -> &str{
        "planet"
    }
}

struct Logic {
    objects: Vec<Object>,
}

impl Logic{
    pub fn new() -> Self{
        Logic{
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: Object){
        self.objects.push(obj)
    }
}

impl GameLogic for Logic {
    fn logic (&mut self, l_args: LogicArgs){
        let ref mut planet = self.objects[0];

        let delta = l_args.delta() as f32;

        let vel = 200.0 * delta;
        let pos = &mut planet.pos;
        // (17, 30, 31, 32) == (W, A, S, D)
        is_down!{
            l_args;

            75, 30 => {
                pos.x -= vel
            },
            77, 32 => {
                pos.x += vel
            },
            80, 31 => {
                pos.y -= vel
            },
            72, 17 => {
                pos.y += vel
            },
            18 => {
                planet.theta += delta
            },
            16 => {
                planet.theta -= delta
            }
        }

        for &(b, k) in l_args.keyevents(){
            if !b {println!("{}", k)}
        }
    }

    fn render(&self, mut r_args: RenderArgs){
        //.rotate() doesn't actually work properly right now
        r_args.draw_drawables(&self.objects).unwrap_or_else(|e| panic!("{}", e))
    }
}
