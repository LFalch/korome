#![warn(missing_docs, trivial_casts, trivial_numeric_casts)]
//! A small game engine written in Rust.
//!
//! This crate is constantly changing and therefore highly unstable

#[macro_use]
pub extern crate glium;
extern crate image;
#[macro_use]
extern crate quick_error;

mod vertex;
mod draw;
mod logic;

pub use draw::{Graphics, Texture, Drawer, TextureDrawer, Quad, QuadDrawer, GraphicsCreationError};
pub use logic::{run_until_closed, Game, GameUpdate, FrameInfo, VirtualKeyCode, MouseButton, Event};

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
