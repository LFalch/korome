extern crate glium;
extern crate time;

use std::collections::HashSet;

use super::{Graphics, Drawer};
use time::precise_time_s as time_s;

use glium::glutin::{Event, ElementState};

pub use glium::glutin::VirtualKeyCode;

/// Manages events and frames
pub struct GameManager<'a>{
    graphics: Graphics<'a>,
    last: f64,
    down_keys: HashSet<VirtualKeyCode>,
    mousepos: (f32, f32),
}

impl<'a> GameManager<'a>{
    #[inline]
    /// Creates a new `Game` from a `Graphics` object
    pub fn new(graphics: Graphics<'a>) -> Self {
        GameManager{
            graphics: graphics,
            last: time_s(),
            mousepos: (0., 0.),
            down_keys: HashSet::new()
        }
    }

    /// Returns some tuple of a `FrameInfo` and a ´Drawer` or None if the window has been closed
    pub fn next_frame(&mut self) -> Option<(FrameInfo, Drawer)>{
        let mut keys = Vec::new();

        for ev in self.graphics.poll_events() {
            match ev {
                Event::Closed => return None,
                Event::KeyboardInput(es, _, Some(vkc)) => match es{
                    ElementState::Pressed  => {
                        self.down_keys.insert( vkc);
                        keys.push((true , vkc));
                    },
                    ElementState::Released => {
                        self.down_keys.remove(&vkc);
                        keys.push((false, vkc));
                    }
                },
                Event::MouseMoved(pos) => {
                    let (w, h) = self.graphics.get_h_size();
                    let (x, y) = pos;
                    let (x, y) = (x as f32 - w, h - y as f32);

                    self.mousepos = (x, y);
                }
                _ => ()
            }
        }

        let now = time_s();
        let delta = now - self.last;
        self.last = now;

        let update = FrameInfo{
            delta    : delta,
            keyevents: keys,
            down_keys: &self.down_keys,
            mousepos : self.mousepos
        };

        Some((update, Drawer::new(&self.graphics)))
    }
}

/// Wraps together useful data about what has happened (e.g. events)
#[derive(Debug)]
pub struct FrameInfo<'a>{
    /// The time that has passed since last frame
    pub delta: f64,
    /// The last position of the mouse on the screen
    pub mousepos: (f32, f32),
    keyevents: Vec<(bool, VirtualKeyCode)>,

    // All keys that are pressed down
    down_keys: &'a HashSet<VirtualKeyCode>
}

impl<'a> FrameInfo<'a>{
    #[inline]
    /// Returns a slice of all key events that have happened
    pub fn get_key_events(&self) -> &[(bool, VirtualKeyCode)]{
        &self.keyevents
    }
    #[inline]
    /// Checks whether a key is pressed down
    pub fn is_down(&self, key: &VirtualKeyCode) -> bool{
        self.down_keys.contains(key)
    }
}

/// Describes objects that can change because of events
pub trait Update{
    /// Changes the object depending on what has happened
    fn update(&mut self, &FrameInfo);
}

/// Macro for easily doing things if particular keys are down
/// # Example
///
/// ```rust
/// # macro_rules! is_down{($l_args:ident; $($($key:ident),+ => $b:block),+) => {}}
/// fn logic(player_y: &mut f32, l_args: korome::LogicArgs){
///     is_down!{l_args;
///         W, Up => {
///             player_y -= l_args.delta() as f32;
///         },
///         S, Down => {
///             player_y += l_args.delta() as f32;
///         }
///     };
/// }
/// ```
#[macro_export]
macro_rules! is_down{
    ( $info:ident; $( $( $key:ident ),+ => $b:block ),+ ) => {{
        $( if $( $info.is_down(&korome::VirtualKeyCode::$key) )||+ $b )+
    }}
}