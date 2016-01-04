#![warn(missing_docs)]
//! Abstraction over how a game can work

#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate toml;
#[macro_use]
extern crate quick_error;

/// Provides all rendering functionality
pub mod draw;

mod vector;
mod settings;

pub use draw::Draw;
pub use vector::Vector2;
pub use settings::Settings;

use draw::Drawable;

use glium::{Frame, Surface};
use glium::glutin::{Event, ElementState, /*VirtualKeyCode*/};
use glium::texture::*;

use time::precise_time_s as time_s;

use std::error::Error;
use std::collections::HashSet;

/// Current engine version
pub const VERSION: &'static str = include_str!("../version.txt");

/// Convenient `Result` type for `KoromeError`
pub type Result<T> = std::result::Result<T, KoromeError>;

quick_error! {
    /// Describes errors that can occur in this crate
    #[derive(Debug)]
    pub enum KoromeError{
        /// Texture wasn't found in cache
        TextureNotFound {
            display("The Texture with the given identifier didn't exist in the cache.")
        }
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

/// Wraps all useful info about what has happened (e.g. events) together
#[derive(Debug)]
pub struct LogicArgs<'a>{
    /// The delta time since last frame
    delta    : f64,
    /// A vector of all key events that happpened
    keyevents: &'a [(bool, u8)],
    /// A `HashSet` of all keys that are pressed down
    down_keys: &'a HashSet<u8>,
    /// The current position of the mouse
    mousepos : (i32, i32)
}

impl<'a> LogicArgs<'a>{
    /// Returns the delta value of the InfoPacket
    pub fn delta(&self) -> f64{
        self.delta
    }

    /// Returns the slice of key events
    pub fn keyevents(&self) -> &[(bool, u8)]{
        self.keyevents
    }

    /// Returns the mouse position
    pub fn mousepos(&self) -> (i32, i32){
        self.mousepos
    }

    /// Checks whether a key is pressed down
    pub fn is_down(&self, key: &u8) -> bool{
        self.down_keys.contains(key)
    }
}

/// Wraps everything needed to render together
pub struct RenderArgs<'a>{
    target: &'a mut glium::Frame,
    draw  : &'a Draw<'a>
}

impl<'a> RenderArgs<'a>{
    /// Returns the `glium::Frame` object, used to draw on the buffer
    pub fn get_target(&'a mut self) -> &'a mut glium::Frame {
        self.target
    }

    /// Returns the `Draw` instance
    pub fn get_draw(&mut self) -> &'a Draw {
        self.draw
    }

    /// Draws a slice of `Drawable`s to the screen using `Draw::texture()`
    pub fn draw_drawables<D: Drawable>(&mut self, drawables: &[D]) -> Result<()>{
        self.draw.draw_drawables(self.target, drawables)
    }
}

/// Macro for easily doing things if particular keys are down
/// # Examples
/// Basic usage:
///
/// ```ignore
/// #[macro_use]
/// extern crate korome;
///
/// use korome::{GameLogic, LogicArgs};
///
/// struct Logic {
///     player_y: f32
/// }
///
/// impl GameLogic for Logic{
///     fn logic(&self, l_args: LogicArgs){
///         is_down!(l_args;
///             80, 31 => { // 80 is the key code for W and 31 for the up arrow key
///                 self.player_y -= l_args.delta() as f32
///             },
///             72, 17 => { // 72 is the key code for S and 17 for the down arrow key
///                 self.player_y += l_args.delta() as f32
///             }
///         );
///     }
///     // the rest of implementation omitted
/// }
/// ```
#[macro_export]
macro_rules! is_down{
    ( $l_args:expr; $( $( $key:expr ),+ => $b:block ),+ ) => {
        $( if $( $l_args.is_down(&$key) )||* $b )*
    }
}

/// A trait for the functions a `Game` would need in its loop
pub trait GameLogic {
    /// A function that runs all the logic of the game
    fn logic (&mut self, LogicArgs);
    /// A function that handles the rendering
    fn render(&self, RenderArgs);
}

/// A struct to keep your "game" in
pub struct Game<'a, G: GameLogic> {
    g: G,
    draw: Draw<'a>,
}

impl<'a, G: GameLogic> Game<'a, G> {
    /// Creates a new `Game` with a `Draw` and `GameLogic`
    pub fn new(game_logic: G, draw: Draw<'a>) -> Self{
        Game{
            g   : game_logic,
            draw: draw,
        }
    }
    /// Runs the `Game` until the window is closed
    pub fn run_until_closed(mut self){
        let mut last = time_s();
        let mut down_keys: HashSet<u8> = HashSet::new();

        'main: loop {
            let mut mousepos = (0, 0);
            let mut keys     = Vec::new();

            for ev in self.draw.get_display().poll_events() {
                match ev {
                    Event::Closed => break 'main,
                    Event::KeyboardInput(es, u, _) => match es{
                        ElementState::Pressed  => {
                            down_keys.insert( u);
                            keys.push((true , u))
                        },
                        ElementState::Released => {
                            down_keys.remove(&u);
                            keys.push((false, u))
                        }
                    },
                    Event::MouseMoved(pos) => mousepos = pos,
                    _ => ()
                }
            }

            let now = time_s();

            let delta = now-last;

            last = now;

            self.render();
            self.g.logic(LogicArgs{
                delta    :  delta,
                keyevents: &keys,
                down_keys: &down_keys,
                mousepos :  mousepos
            });
        }
    }

    // A function for handling the the rendering initialisation and then do the rendering
    fn render(&self){
        let mut target = self.draw.get_display().draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        self.g.render(RenderArgs{
            target: &mut target,
            draw  : &self.draw
        });

        target.finish().unwrap()
    }
}
