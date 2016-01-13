#![warn(missing_docs)]
//! Abstraction over how a game can work

#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate toml;
#[macro_use]
extern crate quick_error;

mod draw;
mod game;
mod vector;
mod settings;

pub use draw::{Draw, Texture};
pub use game::{Game, Sprite, LogicArgs, RenderArgs};
pub use vector::{Vector2, FloatVector};
pub use settings::Settings;

/// Current engine version
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/version"));

/// Convenient `Result` type for `KoromeError`
pub type Result<T> = std::result::Result<T, KoromeError>;

quick_error! {
    /// Wraps together errors that can occur in this crate
    #[derive(Debug)]
    pub enum KoromeError{
        /// A `glium::DrawError`
        DrawError(err: glium::DrawError){
            from()
            description("error during rendering")
        }
        /// A `glium::texture::TextureCreationError`
        TextureCreationError(err: glium::texture::TextureCreationError){
            from()
            description("texture creation error occured")
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
