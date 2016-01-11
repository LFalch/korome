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
pub use vector::{Vector2, FloatVector};
pub use settings::Settings;

use draw::{Drawable, DrawablesDrawer};

use glium::{Frame, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};
use glium::texture::*;

use time::precise_time_s as time_s;

use std::error::Error;
use std::collections::HashSet;

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

/// Wraps all useful info about what has happened (e.g. events) together
#[derive(Debug)]
pub struct LogicArgs<'a>{
    /// The delta time since last frame
    pub delta    : f64,
    /// The current position of the mouse
    pub mousepos : (i32, i32),
    /// A vector of all key events that happened
    pub keyevents: &'a [(bool, VirtualKeyCode)],
    
    /// A `HashSet` of all keys that are pressed down
    down_keys: &'a HashSet<VirtualKeyCode>
}

impl<'a> LogicArgs<'a>{
    /// Checks whether a key is pressed down
    pub fn is_down(&self, key: &VirtualKeyCode) -> bool{
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

    /// Returns a `DrawablesDrawer` for drawing `Drawable`s to the screen
    pub fn draw_drawables<D: Drawable>(&mut self) -> DrawablesDrawer<D>{
        self.draw.draw_drawables(self.target)
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
///             W, Up => {
///                 self.player_y -= l_args.delta() as f32
///             },
///             S, Down => {
///                 self.player_y += l_args.delta() as f32
///             }
///         );
///     }
///     // the rest of implementation omitted
/// }
/// ```
#[macro_export]
macro_rules! is_down{
    ( $l_args:ident; $( $( $key:ident ),+ => $b:block ),+ ) => {{
        $( if $( $l_args.is_down(&glium::glutin::VirtualKeyCode::$key) )||+ $b )+
    }}
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
        let mut down_keys: HashSet<VirtualKeyCode> = HashSet::new();

        'main: loop {
            let mut mousepos = (0, 0);
            let mut keys     = Vec::new();

            for ev in self.draw.get_display().poll_events() {
                match ev {
                    Event::Closed => break 'main,
                    Event::KeyboardInput(es, _, vkc) => match es{
                        ElementState::Pressed  => {
                            if let Some(vkc) = vkc{
                                down_keys.insert( vkc);
                                keys.push((true , vkc));
                            }
                        },
                        ElementState::Released => {
                            if let Some(vkc) = vkc{
                                down_keys.remove(&vkc);
                                keys.push((false, vkc));
                            }
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
