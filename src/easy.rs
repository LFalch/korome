#![allow(deprecated)]

use std::ops::{Deref, DerefMut};

use super::{Game, FrameInfo, Drawer};

/// Describes objects that can change because of events and be drawn to the screen
pub trait Obj{
    /// Changes the object depending on what has happened
    fn update(&mut self, &FrameInfo){
        // Do nothing as a default
    }
    /// Draw the object to the screen
    fn draw(&self, &mut Drawer);
}

/// An easy to use implementation of a game
pub struct EasyGame<T: Obj>{
    objects: Vec<T>,
    clear_colour: (f32, f32, f32),
}

/// Used for making an `EasyGame` object
pub struct EasyGameBuilder<T: Obj>{
    objects: Option<Vec<T>>,
    clear_colour: (f32, f32, f32),
}

impl<T: Obj> EasyGameBuilder<T>{
    #[inline]
    /// Create a new `EasyGameBuilder`
    pub fn new() -> Self{
        EasyGameBuilder{
            objects: None,
            clear_colour: (0., 0., 0.)
        }
    }

    #[inline]
    /// Set an initial vector of objects
    pub fn with_vec(self, objs: Vec<T>) -> Self{
        EasyGameBuilder{
            objects: Some(objs),
            .. self
        }
    }
    #[inline]
    /// Set the colour that the screen is cleared with every frame
    pub fn with_clear_colour(self, r: f32, g: f32, b: f32) -> Self{
        EasyGameBuilder{
            clear_colour: (r, g, b),
            .. self
        }
    }

    #[inline]
    /// Consume the builder and return the EasyGame object
    pub fn build(self) -> EasyGame<T>{
        let EasyGameBuilder{objects, clear_colour} = self;
        EasyGame{
            objects: objects.unwrap_or_default(),
            clear_colour: clear_colour,
        }
    }
}

impl<T: Obj> EasyGame<T>{
    #[inline]
    /// Consumes the object and returns the inner vector
    pub fn into_inner(self) -> Vec<T>{
        let EasyGame{objects, ..} = self;
        objects
    }
    /// Change the colour that the screen is cleared with every frame
    pub fn set_clear_colour(&mut self, r: f32, g: f32, b: f32){
        self.clear_colour = (r, g, b);
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
    type ReturnType = ();
    fn frame(&mut self, info: &FrameInfo, drawer: &mut Drawer) {
        let (r, g, b) = self.clear_colour;
        drawer.clear(r, g, b);

        for obj in &mut self.objects{
            obj.update(info);
            obj.draw(drawer);
        }
    }
}
