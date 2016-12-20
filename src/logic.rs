use std::collections::HashSet;

use super::{Graphics, Drawer};
use draw::resize;
use std::time::Instant;

use glium::glutin::ElementState;

pub use glium::glutin::{VirtualKeyCode, MouseButton, Event};

/// Methods `run_until_closed()` will call
pub trait Game{
    /// Method that gets called each frame from `run_until_closed()`.
    ///
    /// Should return a `GameUpdate` specifying things the game should do.
    fn frame(&mut self, &FrameInfo, &mut Drawer) -> GameUpdate;
}

impl<F: FnMut(&FrameInfo, &mut Drawer) -> GameUpdate> Game for F{
    fn frame(&mut self, info: &FrameInfo, drawer: &mut Drawer) -> GameUpdate{
        (self)(info, drawer)
    }
}

/// This is returned each frame from an object implementing `Game`.
///
/// It describes anything the game should do, e.g. closing the game.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameUpdate{
    /// Tells the game to close
    Close,
    /// Tells it do nothing
    Nothing
}

impl Default for GameUpdate{
    fn default() -> Self{GameUpdate::Nothing}
}

/// Runs the game until the window is closed
pub fn run_until_closed<G: Game>(mut graphics: Graphics, mut game: G){
    let mut last = Instant::now();
    let mut mousepos = (0., 0.);
    let mut down_keys = HashSet::new();

    'game: loop{
        let mut keys = Vec::new();
        // mice?
        let mut mouses = Vec::new();
        let mut misc_events = Vec::new();

        let mut resized = None;

        for ev in graphics.poll_events() {
            match ev {
                Event::Closed => break 'game,
                Event::KeyboardInput(es, _, Some(vkc)) => match es{
                    ElementState::Pressed  => {
                        down_keys.insert( vkc);
                        keys.push((true , vkc));
                    },
                    ElementState::Released => {
                        down_keys.remove(&vkc);
                        keys.push((false, vkc));
                    }
                },
                Event::MouseMoved(x, y) => {
                    let (w, h) = graphics.get_h_size();

                    mousepos = (x as f32 - w, h - y as f32);
                },
                // This is only neccessary because `graphics` gets immutably borrowed for this for-loop
                Event::Resized(w, h) => resized = Some((w, h)),
                Event::MouseInput(state, button) => mouses.push((state == ElementState::Pressed, button)),
                // Put any other event in the misc_events vector in case the game needs them.
                misc_event => misc_events.push(misc_event)
            }
        }

        if let Some((w, h)) = resized{
            resize(&mut graphics, w, h);
        }

        let dur = last.elapsed();
        let delta = dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1e9;
        last = Instant::now();

        let update = FrameInfo{
            delta    : delta,
            key_events: keys,
            mouse_events: mouses,
            misc_events: misc_events,
            down_keys: &down_keys,
            mousepos : mousepos
        };

        let update = game.frame(&update, &mut Drawer::new(&graphics));

        if let GameUpdate::Close = update {
            break
        }
    }
}

/// Wraps together useful data about what has happened (e.g. events)
#[derive(Debug)]
pub struct FrameInfo<'a>{
    /// The amount of time passed since last frame
    pub delta: f32,
    /// The last position of the mouse on the screen
    pub mousepos: (f32, f32),
    mouse_events: Vec<(bool, MouseButton)>,
    key_events: Vec<(bool, VirtualKeyCode)>,
    misc_events: Vec<Event>,

    // All keys that are pressed down
    down_keys: &'a HashSet<VirtualKeyCode>
}

impl<'a> FrameInfo<'a>{
    #[inline]
    /// Returns a slice of all key events that have happened
    pub fn get_key_events(&self) -> &[(bool, VirtualKeyCode)]{
        &self.key_events
    }
    #[inline]
    /// Returns a slice of all key events that have happened
    pub fn get_mouse_events(&self) -> &[(bool, MouseButton)]{
        &self.mouse_events
    }
    #[inline]
    /// Returns a slice of all unhandled events
    pub fn get_misc_events(&self) -> &[Event]{
        &self.misc_events
    }
    #[inline]
    /// Checks whether a key is pressed down
    pub fn is_down(&self, key: &VirtualKeyCode) -> bool{
        self.down_keys.contains(key)
    }
}

/// Macro for easily doing things if particular keys are down
/// # Example
///
/// ```rust
/// # #[macro_use]
/// # extern crate korome;
/// # fn main(){}
/// fn logic(player_y: &mut f32, info: korome::FrameInfo){
///     is_down!{info;
///         W, Up => {
///             *player_y -= info.delta
///         },
///         S, Down => {
///             *player_y += info.delta
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
