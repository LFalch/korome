extern crate korome;
extern crate glium;

use korome::*;

use glium::Display;
use glium::texture::Texture2d;

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
        Drawer
        Texture
        Display
        Graphics
        FrameInfo
        Texture2d
        GameManager
        Vector2<f32>
        Vector2<f64>
        TextureError
        VirtualKeyCode
    );
}
