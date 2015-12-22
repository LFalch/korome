extern crate glium;
#[macro_use]
extern crate korome;

use glium::{Display, Surface};
use korome::{Draw, GameLogic, InfoPacket, Vector2};

use std::io;
use std::io::ErrorKind::NotFound;
use std::error::Error;

fn main() {
    let display = korome::create_window("glium works!", 1200, 900);
    let mut draw = Draw::new(&display);

    if let Err(e) = load_textures(&mut draw, &display){
        match e.kind(){
            NotFound => println!("Not all textures files were found"),
            _        => println!("An unknown error occured when trying to load textures:\n{:?}", e)
        }
        println!("{}", e.description());

        return
    }

    let mut game = Logic{
        pos: Vector2::new(-0.5, 0.0),
        theta: 0.0
    }.into_game(draw);

    game.run_until_closed(&display);
}

fn load_textures(draw: &mut Draw, display: &Display) -> io::Result<()>{
    try!(draw.load_texture(display, "planet", 64, 64));

    // draw.load_texture_from_bytes(display, "planet", include_bytes!("../planet.png"), 32, 32);

    Ok(())
}

struct Logic {
    pos: Vector2,
    theta: f32,
}

impl GameLogic for Logic {
    fn logic (&mut self, ip: InfoPacket){
        let delta = *ip.delta() as f32;

        let vel = 0.22 * delta;
        let pos = &mut self.pos;
        // (17, 30, 31, 32) == (W, A, S, D)
        is_down!{
            ip;

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
                self.theta += delta
            },
            16 => {
                self.theta -= delta
            }
        }

        for &(b, k) in ip.keyevents(){
            if !b {println!("{}", k)}
        }
    }

    fn render(&self, target: &mut glium::Frame, draw: &Draw){
        //.rotate() doesn't actually work properly right now

        draw.texture("planet").unwrap()
            .scale(2.0, 2.0)
            .rotate(self.theta as f32)
            .translate(self.pos.get_x(), self.pos.get_y())
            .draw(target);
    }
}
