#![warn(missing_docs)]
//! Abstraction over how a game can work

#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate toml;

/// Provides all rendering functionality
pub mod draw;
/// Provides useful mathematical functionality for vectors
pub mod vector;
/// Provides functionality for reading a settings file in TOML format
pub mod settings;

pub use draw::Draw as Draw;
pub use vector::Vector2 as Vector2;
pub use settings::Settings as Settings;

use glium::{Display, DisplayBuild, Frame, Surface};
use glium::glutin::{Event, ElementState, /*VirtualKeyCode*/};

use time::precise_time_s as time_s;

use std::collections::HashSet;

/// Creates a window with a title
pub fn create_window(title: &str, width: u32, height: u32) -> Display {
    glium::glutin::WindowBuilder::new()
        // .with_sync()
        .with_title(title.to_string())
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium().expect("Failed to build the window")
}

/// Contains all info about what has happened (events)
#[derive(Debug)]
pub struct InfoPacket<'a>{
    /// The delta time since last frame
    delta    : f64,
    /// A vector of all key events that happpened
    keyevents: &'a [(bool, u8)],
    /// A `HashSet` of all keys that are pressed down
    down_keys: &'a HashSet<u8>,
    /// The current position of the mouse
    mousepos : (i32, i32)
}

impl<'a> InfoPacket<'a>{
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

/// Macro for easily doing things if particular keys are down
/// # Examples
/// Basic usage:
///
/// ```ignore
/// #[macro_use]
/// extern crate korome;
///
/// use korome::GameLogic;
///
/// struct Logic {
///     player_y: f32
/// }
/// impl GameLogic for Logic{
///     fn logic(&self, ip: korome::InfoPacket){
///         is_down!(ip;
///             80, 31 => { // 80 is the key code for W and 31 for the up arrow key
///                 self.player_y -= ip.delta() as f32
///             },
///             72, 17 => { // 72 is the key code for S and 17 for the down arrow key
///                 self.player_y += ip.delta() as f32
///             }
///         );
///     }
///     // the rest of the implementation omitted
/// }
/// ```
#[macro_export]
macro_rules! is_down{
    ( $info_packet:expr; $( $( $key:expr ),+ => $b:block ),+ ) => {
        $( if $( $info_packet.is_down(&$key) )||* $b )*
    }
}

/// A trait for the functions a `Game` would need in its loop
pub trait GameLogic {
    /// A function that runs all the logic of the game
    fn logic (&mut self, infopacket: InfoPacket);
    /// A function that handles the rendering
    fn render(&self, target: &mut glium::Frame, draw: &Draw);
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
            self.g.logic(InfoPacket{
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

        self.g.render(&mut target, &self.draw);

        target.finish().unwrap()
    }
}
