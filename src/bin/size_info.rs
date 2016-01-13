extern crate korome;
extern crate glium;

use korome::*;

use glium::texture::Texture2d;

use glium::backend::glutin_backend::GlutinFacade;

macro_rules! print_type_info{
    ($($t:ty)*) => {
        $({
            use std::mem::{align_of, size_of};

            let (size, align) = (size_of::<$t>(), align_of::<$t>());
            println!("{}\t: {} <= {} * {}", stringify!($t), size, size/align, align);
        })*
    }
}

type LogicFn = fn(&mut(), LogicArgs);
type RenderFn = fn(&(), RenderArgs);

fn main(){
    println!("Version: {}", korome::VERSION);

    print_type_info!(
        Draw
        Texture
        Texture2d
        LogicArgs
        RenderArgs
        Vector2<f32>
        Vector2<f64>
        GlutinFacade
        fn(&(), RenderArgs)
        fn(&mut (), LogicArgs)
        Game<(), LogicFn, RenderFn>
    );
}
