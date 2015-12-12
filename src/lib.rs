#[macro_use]
extern crate glium;
extern crate time;

pub mod draw;

pub use self::draw::Draw as Draw;

use self::draw::Vertex;

use glium::{Display, DisplayBuild, Frame, Surface};

implement_vertex!(Vertex, position, tex_coords);

pub fn create_window(title: &'static str) -> Display {
    glium::glutin::WindowBuilder::new()
        // .with_sync()
        .with_title(title.to_string())
        .build_glium().expect("Failed to build the window")
}

pub struct InfoPacket{
    delta    : f64,
    keyevents: Vec<u16>,
    mousepos : (i32, i32)
}

impl InfoPacket{
    pub fn new(delta: f64) -> InfoPacket{
        InfoPacket{
            delta    : delta,
            keyevents: vec![],
            mousepos : (0, 0),
        }
    }

    pub fn delta(&self) -> &f64{
        &self.delta
    }
}

pub trait GameFunc {
    fn logic (&mut self, infopacket: InfoPacket);
    fn render(&self, display: &mut glium::Frame, draw: &Draw);
}

pub struct Game<'a> {
    g: &'a mut GameFunc,
    draw: Draw<'a>,
}

use glium::glutin::Event::*;

impl<'a> Game<'a> {
    pub fn new<GF>(gf: &'a mut GF, draw: Draw<'a>) -> Game<'a>
        where GF: GameFunc{
        Game{
            g: gf,
            draw: draw,
        }
    }

    pub fn run_until_closed(&'a mut self, display: &'a Display){
        let mut last = time::precise_time_s();
        'main: loop {
            let now = time::precise_time_s();

            let delta = now-last;

            last = now;

            self.render(display, &self.draw);
            self.g.logic(InfoPacket::new(delta));

            for ev in display.poll_events() {
                match ev {
                    Closed => break 'main,
                    _ => ()
                }
            }
        }
    }

    pub fn render<'b>(&self, display: &'b Display, draw: &'a Draw<'a>){
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        self.g.render(&mut target, draw);

        target.finish().unwrap()
    }
}
