#![warn(missing_docs, trivial_casts, trivial_numeric_casts)]
//! A small game engine written in Rust.

#[macro_use]
pub extern crate glium;
extern crate num_traits;
extern crate image;
#[macro_use]
extern crate quick_error;

/// Module with an easy to use struct quick games with very simple logic
pub mod easy;

mod vertex;
mod draw;
mod logic;
mod vector;

pub use draw::{Graphics, Texture, Drawer, TextureDrawer, Quad, QuadDrawer};
pub use logic::{run_until_closed, Game, GameUpdate, FrameInfo, VirtualKeyCode, MouseButton};
pub use vector::Vector2;

/// Result type for `korome::TextureError`
pub type TextureResult = Result<Texture, TextureError>;

quick_error! {
    /// Wraps together all errors that can occur creating `Texture`s
    #[derive(Debug)]
    pub enum TextureError{
        /// A `glium::texture::TextureCreationError`
        TextureCreationError(err: glium::texture::TextureCreationError){
            from()
            cause(err)
            description("texture creation error")
        }
        /// An `image::ImageError`
        ImageError(err: image::ImageError){
            from()
            cause(err)
            description(err.description())
        }
        /// A `glium::vertex::buffer::CreationError`
        BufferCreationError(err: glium::vertex::BufferCreationError){
            from()
            cause(err)
            description(err.description())
        }
        /// An `std::io::Error`
        IoError(err: std::io::Error){
            from()
            cause(err)
            description(err.description())
        }
    }
}
