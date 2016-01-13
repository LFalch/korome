extern crate glium;
extern crate time;

use std::collections::HashSet;

use super::{Result, Draw, Texture};
use time::precise_time_s as time_s;

use glium::{Frame, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

/// Manages a main game loop with two functions and a shared object
pub struct Game<'a, S, L, R>  where L: FnMut(&mut S, LogicArgs), R: FnMut(&S, RenderArgs){
    /// An object that will be parsed to the logic and render functions
    pub shared: S,
    logic : L,
    render: R,
    draw  : Draw<'a>,
}

impl<'a, S, L, R> Game<'a, S, L, R> where L: FnMut(&mut S, LogicArgs), R: FnMut(&S, RenderArgs){
    #[inline]
    /// Creates a new `Game` with a `Draw` and two closures
    pub fn new(draw: Draw<'a>, logic: L, render: R) -> Game<'a, (), L, R>
    where L: FnMut(&mut (), LogicArgs), R: FnMut(&(), RenderArgs){
        Game::with_shared(draw, (), logic, render)
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

/// Wraps together useful data about what has happened (e.g. events)
#[derive(Debug)]
pub struct LogicArgs<'a>{
    /// The time that has passed since last update
    pub delta    : f64,
    /// The current position of the mouse
    pub mousepos : (i32, i32),
    /// A slice of all key events that have happened
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

/// Wraps together everything needed to render and
/// also provides functions for actually drawing
pub struct RenderArgs<'a>{
    /// Object used to draw on the buffer.
    /// Generally, you shouldn't have to access this directly.
    pub target: &'a mut glium::Frame,
    /// Reference to the `Draw` instance.
    /// Generally, you shouldn't have to access this directly.
    pub draw  : &'a Draw<'a>
}

impl<'a> RenderArgs<'a>{
    /// Uses `Draw` to draw a texture onto the screen
    pub fn draw_texture(&mut self, texture: &Texture, rotation: f32, x: f32, y: f32) -> Result<()>{
        self.draw.draw_texture(self.target, texture, rotation, x, y)
    }

    /// Draws an iterator of `Sprite`s onto the screen
    pub fn draw_sprites<'b, D: 'b + Sprite, I: IntoIterator<Item = &'b D>>(&mut self, sprites: I) -> Result<()>{
        for sprite in sprites{
            let (x, y) = sprite.get_pos();

            try!(
                self.draw_texture(sprite.get_texture(),
                sprite.get_rotation(), x, y)
            );
        }

        Ok(())
    }
}

/// Descibes objects that can be drawn to the screen
pub trait Sprite {
    /// Returns the position on the screen it should be drawn
    fn get_pos(&self) -> (f32, f32);
    /// Returns the rotation it should be drawn with
    fn get_rotation(&self) -> f32;
    /// Returns the `Texture`
    fn get_texture(&self) -> &Texture;
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
    ( $l_args:ident; $( $( $key:ident ),+ => $b:block ),+ ) => {{
        $( if $( $l_args.is_down(&glium::glutin::VirtualKeyCode::$key) )||+ $b )+
    }}
}
