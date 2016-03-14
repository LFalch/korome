use image;
use image::RgbaImage;

use glium::{DisplayBuild, VertexBuffer, Program, DrawParameters, Display, Surface};
use glium::{IndexBuffer, Frame, Blend};
use glium::index::PrimitiveType;
use glium::texture::{Texture2d, RawImage2d};
use glium::glutin::WindowBuilder;

use std::path::Path;
use std::ops::{Deref, DerefMut};

use super::{DrawResult, TextureResult};

#[derive(Copy, Clone)]
struct Vertex {
    position  : [f32; 2],
    tex_coords: [f32; 2]
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex{
    #[inline]
    fn new(position: [f32; 2], tex_coords: [f32; 2]) -> Self{
        Vertex{
            position  : position,
            tex_coords: tex_coords,
        }
    }
}

/// A 2D texture that is ready to be drawn
pub struct Texture{
    tex: Texture2d,
    vertex_buffer: VertexBuffer<Vertex>,
}

impl Texture {
    #[inline]
    /// Creates a `Texture` from a PNG-encoded byte slice
    pub fn from_png_bytes(display: &Display, bytes: &[u8]) -> TextureResult<Texture>{
        Texture::new(display,
            try!(
                image::load_from_memory_with_format(bytes, image::PNG)
            ).to_rgba()
        )
    }
    #[inline]
    /// Creates a `Texture` from a file
    pub fn from_file<P: AsRef<Path>>(display: &Display, path: P) -> TextureResult<Texture>{
        Texture::new(display,
            try!(
                image::open(path)
            ).to_rgba()
        )
    }

    /// Creates a `Texture` from an `image::RgbaImage`
    pub fn new(display: &Display, image: RgbaImage) -> TextureResult<Texture>{
        let (width, height) = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), (width, height));

        let (w, h) = (width as f32 / 2.0, height as f32 / 2.0);

        let vb = try!(VertexBuffer::new(display, &[
            Vertex::new([-w, -h], [0.0, 0.0]),
            Vertex::new([ w, -h], [1.0, 0.0]),
            Vertex::new([ w,  h], [1.0, 1.0]),
            Vertex::new([-w,  h], [0.0, 1.0])
        ]));

        Ok(Texture {
            tex: try!(Texture2d::new(display, image)),
            vertex_buffer: vb,
        })
    }
}

/// Loads a texture, by loading the bytes at compile-time
#[macro_export]
macro_rules! include_texture {
    ($graphics:expr, $texture:tt) => {
        $crate::Texture::from_png_bytes(&$graphics, include_bytes!($texture))
    };
}

/// Contains the display and handles most of the graphics
pub struct Graphics<'a> {
    display: Display,
    program: Program,
    h_size : (f32, f32),
    indices: IndexBuffer<u8>,
    params : DrawParameters<'a>
}

impl<'a> Graphics<'a> {
    /// Creates a new `Graphics` from a `Display` made using the arguments
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self::from_display(
            WindowBuilder::new()
                .with_title(title.to_string())
                .with_dimensions(width, height)
                .with_vsync()
                .build_glium().expect("Failed to build the window")
        )
    }

    /// Creates a new `Graphics` instance using the given display
    pub fn from_display(display: Display) -> Self {
        let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();
        let (w, h) = (w as f32 / 2.0, h as f32 / 2.0);

        let params = DrawParameters{
            blend : Blend::alpha_blending(),
            .. Default::default()
        };

        let indices = IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[0u8, 1, 3, 2]).unwrap();

        Graphics{
            // Unwrap should be safe
            program: Program::from_source(&display, include_str!("shaders/texture.vs"), include_str!("shaders/texture.fs"), None).unwrap(),
            display: display,
            params : params,
            indices: indices,
            h_size : (w, h)
        }
    }

    #[inline]
    /// Returns half of the size of the window
    pub fn get_h_size(&self) -> (f32, f32){
        self.h_size
    }
}

#[inline(always)]
// This function is only used inside `FrameInfo` when Event::Resized occurs
pub fn resize(graphics: &mut Graphics, width: u32, height: u32){
    graphics.h_size = (width as f32 / 2.0, height as f32 / 2.0);
}

fn draw(target: &mut Frame, graphics: &Graphics, texture: &Texture, matrix: [[f32; 4]; 4]) -> DrawResult<()>{
    let uniforms = uniform! {
        h_size: graphics.h_size,
        tex   : &texture.tex,
        matrix: matrix
    };

    target.draw(&texture.vertex_buffer, &graphics.indices, &graphics.program, &uniforms, &graphics.params)
}

impl<'a> Deref for Graphics<'a>{
    type Target = Display;

    #[inline]
    fn deref(&self) -> &Display{
        &self.display
    }
}

/// Provides functionality for drawing.
/// Can also be dereferenced into a `glium::Frame`.
pub struct Drawer<'a>{
    target: Frame,
    /// Reference to the draw instance
    pub graphics: &'a Graphics<'a>
}

impl<'a> Drawer<'a>{
    #[inline]
    /// Creates a new `Drawer` to draw next frame
    pub fn new(graphics: &'a Graphics<'a>) -> Self{
        let target = graphics.draw();
        Drawer{
            target: target,
            graphics: graphics
        }
    }

    #[inline]
    /// Clears the screen with the specified colour
    pub fn clear(&mut self, red: f32, green: f32, blue: f32){
        self.clear_color(red, green, blue, 1.)
    }

    /// Draws a texture onto the screen
    pub fn draw_texture(&mut self, texture: &Texture, x: f32, y: f32, rotation: f32) -> DrawResult<()>{
        let (sin, cos)  = rotation.sin_cos();

        let matrix = [
            [ cos, sin, 0.0, 0.0],
            [-sin, cos, 0.0, 0.0],
            [ 0.0, 0.0, 1.0, 0.0],
            [   x,   y, 0.0, 1.0],
        ];

        draw(self, self.graphics, texture, matrix)
    }

    /// Draws a texture onto the screen without rotation
    pub fn draw_texture_rigid(&mut self, texture: &Texture, x: f32, y: f32) -> DrawResult<()>{
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [  x,   y, 0.0, 1.0],
        ];

        draw(self, self.graphics, texture, matrix)
    }
}

impl<'a> Deref for Drawer<'a>{
    type Target = Frame;
    #[inline]
    fn deref(&self) -> &Frame{
        &self.target
    }
}

impl<'a> DerefMut for Drawer<'a>{
    #[inline]
    fn deref_mut(&mut self) -> &mut Frame{
        &mut self.target
    }
}

impl<'a> Drop for Drawer<'a>{
    #[inline]
    fn drop(&mut self){
        self.target.set_finish().unwrap()
    }
}

/// Descibes objects that can be drawn to the screen
pub trait Draw {
    /// Draws itself to the screen
    fn draw(&self, &mut Drawer) -> DrawResult<()>;
}
