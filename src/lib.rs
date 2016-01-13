#![warn(missing_docs)]
//! Abstraction over how a game can work

#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
#[macro_use]
extern crate quick_error;

mod draw;
mod game;
mod vector;

pub use draw::{Draw, Texture};
pub use game::{Game, Sprite, LogicArgs, RenderArgs};
pub use vector::{Vector2, FloatVector};

/// Current engine version
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/version"));

/// Result type for `korome::TextureError`
pub type TextureResult<T> = Result<T, TextureError>;
/// Result type for `glium::DrawError`
pub type DrawResult<T> = Result<T, glium::DrawError>;

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
