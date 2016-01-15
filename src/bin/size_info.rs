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


fn main(){
    print_type_info!(
        Game
        Draw
        Update
        Drawer
        Texture
        Texture2d
        Vector2<f32>
        Vector2<f64>
        GlutinFacade
        TextureError
        VirtualKeyCode
    );
}
