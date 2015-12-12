extern crate glium;
extern crate korome;

use glium::{Display, Surface};
use korome::{Draw, Game, GameFunc, InfoPacket};

use std::io;
use std::io::ErrorKind::NotFound;
use std::error::Error;

fn main() {
    let display = korome::create_window("glium works!");
    let mut draw = Draw::new(&display);
    let res_textures = load_textures(&mut draw, &display);

    if let Err(e) = res_textures{
        match e.kind(){
            NotFound => println!("Not all textures files were found"),
            _        => println!("An unknown error occured when trying to load textures:\n{:?}", e)
        }
        println!("{}", e.description());

        return
    }

    let mut i = Instance{
        t: -0.5
    };

    let mut game = Game::new(&mut i, draw);

    game.run_until_closed(&display);
}

fn load_textures(draw: &mut Draw, display: &Display) -> io::Result<()>{
    try!(draw.load_texture(display, "planet", 32, 32));

    // draw.load_texture_from_bytes(display, "planet", include_bytes!("../planet.png"), 32, 32);

    Ok(())
}

struct Instance {
    t: f32,
}

impl GameFunc for Instance {
    fn logic (&mut self, ip: InfoPacket){
        self.t += (0.01 * ip.delta()) as f32
    }

    fn render(&self, target: &mut glium::Frame, draw: &Draw){
        draw.texture("planet").unwrap()
            // .rotate(45.0)
            .scale(4.0, 4.0)
            .translate(self.t, 0.0)
            .draw(target);
        draw.texture("planet").unwrap()
            // .rotate(45.0)
            .scale(4.0, 4.0)
            .translate(self.t, 0.2)
            .draw(target);
    }
}
