use std::collections::HashSet;

use super::{Graphics, Drawer};
use draw::resize;
use time::precise_time_s as time_s;

use glium::glutin::{Event, ElementState};

pub use glium::glutin::{VirtualKeyCode, MouseButton};

pub trait State{
    fn frame(&mut self, Option<(FrameInfo, Drawer)>) -> StateChange;
}

pub enum StateChange{
    No, New(Box<State>), Close
}

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

    /// Runs the game with the specified initial state
    pub fn run(&mut self, initial_state: Box<State>){
        let mut current_state = initial_state;

        loop{
            match current_state.frame(self.next_frame()){
                StateChange::No => (),
                StateChange::Close => break,
                StateChange::New(new_state) => current_state = new_state,
            }
        }
    }

    /// Returns some tuple of a `FrameInfo` and a ´Drawer` or None if the window has been closed
    pub fn next_frame(&mut self) -> Option<(FrameInfo, Drawer)>{
        let mut keys = Vec::new();
        let mut mouses = Vec::new();

        let mut resized = None::<(u32, u32)>;

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
                },
                // This is neccessary because `graphics` gets immutably borrowed for this for loop
                Event::Resized(w, h) => resized = Some((w, h)),
                Event::MouseInput(state, button) => mouses.push((state == ElementState::Pressed, button)),
                _ => ()
            }
        }

        if let Some((w, h)) = resized{
            resize(&mut self.graphics, w, h);
        }drop(resized);

        let now = time_s();
        let delta = now - self.last;
        self.last = now;

        let update = FrameInfo{
            delta    : delta,
            keyevents: keys,
            mouseevents: mouses,
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
    mouseevents: Vec<(bool, MouseButton)>,
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
    /// Returns a slice of all key events that have happened
    pub fn get_mouse_events(&self) -> &[(bool, MouseButton)]{
        &self.mouseevents
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
/// fn logic(player_y: &mut f32, info: korome::FrameInfo){
///     is_down!{info;
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
        $( if $( $info.is_down(&$crate::VirtualKeyCode::$key) )||+ $b )+
    }}
}
