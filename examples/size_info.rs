extern crate korome;
extern crate glium;

use korome::{Draw, Vector2, Game, InfoPacket, GameLogic};
use korome::draw::{Texture, TextureDrawer, Vertex, VertexBuffers};
use glium::VertexBuffer;

macro_rules! print_type_info{
    ($($t:ty),*) => {
        use std::mem::{align_of, size_of};
        $({
            let info = (size_of::<$t>(), align_of::<$t>());
            println!("{}\t: {} === {} * {}", stringify!($t), info.0, info.0/info.1, info.1);
        })*
    }
}

fn main(){
    print_type_info!(Logic, Draw, Vector2, Texture, Vertex, Game<Logic>, InfoPacket, TextureDrawer, VertexBuffers, VertexBuffer<Vertex>);
}

struct Logic;

impl GameLogic for Logic {
    fn logic (&mut self, _ip: InfoPacket){

    }

    fn render(&self, _target: &mut glium::Frame, _draw: &Draw){

    }
}
