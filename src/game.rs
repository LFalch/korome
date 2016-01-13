extern crate glium;
extern crate time;

use std::collections::HashSet;

use super::{Result, Draw, Drawable, Texture};
use time::precise_time_s as time_s;

use glium::{Frame, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

/// A struct to keep your "game" in
pub struct Game<'a, S, L, R>  where L: FnMut(&mut S, LogicArgs), R: FnMut(&S, RenderArgs){
    /// An object that will parsed into the logic and render methods
    pub shared: S,
    logic : L,
    render: R,
    draw  : Draw<'a>,
}

impl<'a, S, L, R> Game<'a, S, L, R> where L: FnMut(&mut S, LogicArgs), R: FnMut(&S, RenderArgs){
    /// Creates a new `Game` with a `Draw` and two closures
    pub fn new(draw: Draw<'a>, logic: L, render: R) -> Game<'a, (), L, R>
    where L: FnMut(&mut (), LogicArgs), R: FnMut(&(), RenderArgs){
        Game{
            shared: (),
            logic : logic,
            render: render,
            draw  : draw,
        }
    }

    /// Creates a new `Game` with a `Draw`, a shared value, and two closures
    pub fn with_shared(draw: Draw<'a>, shared: S, logic: L, render: R) -> Self{
        Game{
            shared: shared,
            logic : logic,
            render: render,
            draw  : draw,
        }
    }

    /// Runs the `Game` until the window is closed
    pub fn run_until_closed(mut self){
        let mut last = time_s();
        let mut down_keys: HashSet<VirtualKeyCode> = HashSet::new();
        let mut mousepos = (0, 0);

        'main: loop {
            let mut keys = Vec::new();

            for ev in self.draw.get_display().poll_events() {
                match ev {
                    Event::Closed => break 'main,
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
                    Event::MouseMoved(pos) => mousepos = pos,
                    _ => ()
                }
            }

            let now = time_s();

            let delta = now-last;

            last = now;

            (self.logic)(&mut self.shared, LogicArgs{
                delta    :  delta,
                keyevents: &keys,
                down_keys: &down_keys,
                mousepos :  mousepos
            });

            // Do rendering
            let mut target = self.draw.get_display().draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);

            (self.render)(&self.shared, RenderArgs{
                target: &mut target,
                draw  : &self.draw
            });

            target.finish().unwrap()
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

use std::ops::Deref;

/// Wraps everything needed to render together
pub struct RenderArgs<'a>{
    /// Object used to draw on the buffer.
    /// Generally, you shouldn't have to access this directly.
    pub target: &'a mut glium::Frame,
    /// Reference to the `Draw` instance
    /// Generally, you shouldn't have to access this directly.
    pub draw  : &'a Draw<'a>
}

impl<'a> RenderArgs<'a>{
    /// Uses `Draw` to draw an iterator of `Drawable`s onto the screen
    pub fn draw_drawables<'d, D: 'd + Drawable, I: IntoIterator<Item = &'d D>>(&mut self, drawables: I) -> Result<()>{
        self.draw.draw_drawables(self.target, drawables)
    }

    /// Uses `Draw` to draw a texture onto the screen
    pub fn draw_texture(&mut self, texture: &Texture, rotation: f32, x: f32, y: f32) -> Result<()>{
        self.draw.draw_texture(self.target, texture, rotation, x, y)
    }
}

impl<'a> Deref for RenderArgs<'a>{
    type Target = Draw<'a>;

    fn deref(&self) -> &Draw<'a>{
        self.draw
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
