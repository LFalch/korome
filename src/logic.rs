use std::collections::HashSet;

use super::{Graphics, Drawer};
use draw::resize;
use time::precise_time_s as time_s;

use glium::glutin::{Event, ElementState};

pub use glium::glutin::{VirtualKeyCode, MouseButton};

/// Describes a way to handle an update
pub trait State{
    /// Function that gets called each frame update, when this `State` is being run
    ///
    /// Returns whether to do something after the update has been handled
    fn frame(&mut self, closed: bool, FrameInfo, Drawer) -> StateAction;
}

impl<F> State for F where F: FnMut(bool, FrameInfo, Drawer) -> StateAction{
    fn frame(&mut self, c: bool, i: FrameInfo, d: Drawer) -> StateAction{
        (self)(c, i, d)
    }
}

/// Describes what to do after a frame update
pub enum StateAction{
    /// Says to continue without changing the state
    Continue,
    /// Says to change the state to the one specified
    ChangeTo(Box<State>),
    /// Says to close the game
    Close
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
    /// Creates a new `GameManager` from a `Graphics` object
    pub fn new(graphics: Graphics<'a>) -> Self {
        GameManager{
            graphics: graphics,
            last: time_s(),
            mousepos: (0., 0.),
            down_keys: HashSet::new()
        }
    }

<<<<<<< HEAD
    /// Returns an optional tuple of a `FrameInfo` and a `Drawer` or `None`, if the window has been closed.
    ///
    /// The window doesn't actually close when `None` is returned; you can just
    /// ignore it, and it will just keep running: The window only closes when the `GameManager` object is dropped.
    pub fn next_frame(&mut self) -> Option<(FrameInfo, Drawer)>{
=======
    /// Runs the game with the specified initial state
    pub fn run(&mut self, initial_state: Box<State>){
        let mut current_state = initial_state;

        loop{
            let (c, fi, d) = self.next_frame();

            match current_state.frame(c, fi, d){
                StateAction::ChangeTo(new) => current_state = new,
                StateAction::Continue      => (),
                StateAction::Close         => break,
            }
        }
    }

    /// Runs the game until the user tries to close the window
    pub fn run_until_closed<F: FnMut(FrameInfo, Drawer)>(&mut self, mut f: F){
        while let (false, frame_info, drawer) = self.next_frame() {
            f(frame_info, drawer)
        }
    }

    /// Returns a tuple of a `bool` (which is `true` if the user tried
    /// to close the window), a `FrameInfo` and a `Drawer`
    pub fn next_frame(&mut self) -> (bool, FrameInfo, Drawer){
>>>>>>> feauture-game-states
        let mut keys = Vec::new();
        let mut mouses = Vec::new();
        let mut closed = false;

        let mut resized = None::<(u32, u32)>;

        for ev in self.graphics.poll_events() {
            match ev {
                Event::Closed => closed = true,
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

                    self.mousepos = (x as f32 - w, h - y as f32);
                },
                // This is only neccessary because `graphics` gets immutably borrowed for this for-loop
                Event::Resized(w, h) => resized = Some((w, h)),
                Event::MouseInput(state, button) => mouses.push((state == ElementState::Pressed, button)),
                _ => ()
            }
        }

        if let Some((w, h)) = resized{
            resize(&mut self.graphics, w, h);
        }

        let now = time_s();
        let delta = now - self.last;
        self.last = now;

        let update = FrameInfo{
            delta    : delta,
            key_events: keys,
            mouse_events: mouses,
            down_keys: &self.down_keys,
            mousepos : self.mousepos
        };

        (closed, update, Drawer::new(&self.graphics))
    }
}

/// Wraps together useful data about what has happened (e.g. events)
#[derive(Debug)]
pub struct FrameInfo<'a>{
    /// The amount of time passed since last frame
    pub delta: f64,
    /// The last position of the mouse on the screen
    pub mousepos: (f32, f32),
    mouse_events: Vec<(bool, MouseButton)>,
    key_events: Vec<(bool, VirtualKeyCode)>,

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
