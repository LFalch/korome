use std::ops::{Deref, DerefMut};

use super::{Game, GameUpdate, FrameInfo, Drawer, DrawResult};

/// Describes objects that can change because of events and be drawn to the screen
pub trait Obj{
    /// Changes the object depending on what has happened
    fn update(&mut self, &FrameInfo);
    /// Draw the object to the screen
    fn draw(&self, &mut Drawer) -> DrawResult;
}

/// An easy to use implementation of a game
pub struct EasyGame<T: Obj>{
    objects: Vec<T>
}

impl<T: Obj> EasyGame<T>{
    #[inline(always)]
    /// Create a new empty `EasyGame`
    pub fn new() -> Self{
        EasyGame{
            objects: Vec::new()
        }
    }
    #[inline(always)]
    /// Create new `EasyGame` with the given objects
    pub fn with_vec(objs: Vec<T>) -> Self{
        EasyGame{
            objects: objs
        }
    }
}

impl<T: Obj> Deref for EasyGame<T>{
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T>{
        &self.objects
    }
}

impl<T: Obj> DerefMut for EasyGame<T>{
    fn deref_mut(&mut self) -> &mut Vec<T>{
        &mut self.objects
    }
}

impl<T: Obj> Game for EasyGame<T>{
    fn frame(&mut self, info: FrameInfo, mut drawer: Drawer) -> GameUpdate{
        drawer.clear(0., 0., 1.);

        for obj in &mut self.objects{
            obj.update(&info);
            obj.draw(&mut drawer).unwrap();
        }

        GameUpdate::nothing()
    }
}
