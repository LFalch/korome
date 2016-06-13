use image;
use image::RgbaImage;

use glium::{DisplayBuild, VertexBuffer, Program, DrawParameters, Display, Surface};
use glium::{IndexBuffer, Frame, Blend};
use glium::draw_parameters::Smooth;
use glium::index::PrimitiveType;
use glium::texture::{Texture2d, RawImage2d};
use glium::glutin::WindowBuilder;

use std::path::Path;
use std::ops::{Deref, DerefMut};

use super::TextureResult;
use ::vertex::{TextureVertex, ColourVertex};

/// A 2D texture that is ready to be drawn
#[derive(Debug)]
pub struct Texture{
    tex: Texture2d,
    vertex_buffer: VertexBuffer<TextureVertex>,
}

impl Texture {
    #[inline]
    /// Creates a `Texture` from a PNG-encoded byte slice
    pub fn from_png_bytes(display: &Display, bytes: &[u8]) -> TextureResult{
        Texture::new(display,
            try!(
                image::load_from_memory_with_format(bytes, image::PNG)
            ).to_rgba()
        )
    }
    #[inline]
    /// Creates a `Texture` from a file
    pub fn from_file<P: AsRef<Path>>(display: &Display, path: P) -> TextureResult{
        Texture::new(display,
            try!(
                image::open(path)
            ).to_rgba()
        )
    }

    /// Creates a `Texture` from an `image::RgbaImage`
    pub fn new(display: &Display, image: RgbaImage) -> TextureResult{
        let (width, height) = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), (width, height));

        let (w, h) = (width as f32 / 2.0, height as f32 / 2.0);

        let vb = try!(VertexBuffer::new(display, &[
            TextureVertex::new([-w, -h], [0.0, 0.0]),
            TextureVertex::new([ w, -h], [1.0, 0.0]),
            TextureVertex::new([ w,  h], [1.0, 1.0]),
            TextureVertex::new([-w,  h], [0.0, 1.0])
        ]));

        Ok(Texture {
            tex: try!(Texture2d::new(display, image)),
            vertex_buffer: vb,
        })
    }
    /// Returns an object used for drawing the texture onto the screen with a `Drawer`
    pub fn drawer(&self) -> TextureDrawer{
        TextureDrawer{
            pos: (0., 0.),
            sin_cos: (0., 1.),
            colour: [1., 1., 1., 1.],
            texture: self
        }
    }
}

/// Loads a texture, by loading the bytes at compile-time
#[macro_export]
macro_rules! include_texture {
    ($graphics:expr, $texture:tt) => {
        $crate::Texture::from_png_bytes(&$graphics, include_bytes!($texture))
    };
}

quick_error! {
    /// Wraps together errors that could occur creating a `Graphics` context
    #[derive(Debug)]
    pub enum GraphicsCreationError{
        /// A `glium::GliumCreationError<::glium::glutin::CreationError>`
        WindowBuilderError(err: ::glium::GliumCreationError<::glium::glutin::CreationError>){
            from()
            cause(err)
            description(err.description())
        }
        /// This shouldn't occur often
        IndexBufferCreationError(err: ::glium::index::BufferCreationError){
            from()
            cause(err)
            description(err.description())
        }
    }
}

/// Contains the display and handles most of the graphics
pub struct Graphics<'a> {
    display: Display,
    program: Program,
    colour_program: Program,
    h_size : (f32, f32),
    params: DrawParameters<'a>,
    indices: IndexBuffer<u8>
}

impl<'a> Graphics<'a> {
    /// Creates a new `Graphics` from a `Display` made using the arguments
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, GraphicsCreationError> {
        WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(width, height)
            .with_vsync()
            .build_glium()
            .map_err(Into::into)
            .and_then(Self::from_display)
    }

    /// Creates a new `Graphics` instance using the given display
    pub fn from_display(display: Display) -> Result<Self, GraphicsCreationError> {
        let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();
        let (w, h) = (w as f32 / 2.0, h as f32 / 2.0);

        let params = DrawParameters{
            blend : Blend::alpha_blending(),
            smooth: Some(Smooth::Nicest),
            .. Default::default()
        };

        let indices = try!(IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[0u8, 1, 3, 2]));

        Ok(Graphics{
            // Unwrap should be safe
            program: Program::from_source(&display, include_str!("shaders/texture.vs"), include_str!("shaders/texture.fs"), None).unwrap(),
            colour_program: Program::from_source(&display, include_str!("shaders/colour.vs"), include_str!("shaders/colour.fs"), None).unwrap(),
            display: display,
            params : params,
            indices: indices,
            h_size : (w, h)
        })
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

impl<'a> Deref for Graphics<'a>{
    type Target = Display;

    #[inline]
    fn deref(&self) -> &Display{
        &self.display
    }
}

/// Provides functionality for drawing.
///
/// Can also be dereferenced into a `glium::Frame`.
pub struct Drawer<'a>{
    target: Frame,
    /// Reference to the `Graphics` object
    pub graphics: &'a Graphics<'a>
}

impl<'a> Drawer<'a>{
    #[inline]
    /// Creates a new `Drawer` to draw the next frame
    pub fn new(graphics: &'a Graphics) -> Self{
        Drawer{
            target: graphics.draw(),
            graphics: graphics
        }
    }

    #[inline]
    /// Clears the screen with the specified colour
    pub fn clear(&mut self, red: f32, green: f32, blue: f32){
        self.clear_color(red, green, blue, 1.)
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

macro_rules! set {
    ($(#[$m:meta])* fn $d:ident; $f:ident: $T:ty) => (
        #[inline]
        $(#[$m])*
        pub fn $f(self, $f: $T) -> Self{
            $d{
                $f: $f,
                .. self
            }
        }
    );
    ($(#[$m:meta])* fn $d:ident; $f:ident: $T:ty => $field:ident: $e:expr) => (
        #[inline]
        $(#[$m])*
        pub fn $f(self, $f: $T) -> Self{
            $d{
                $field: $e,
                .. self
            }
        }
    );
}

/// Object for drawing textures to the screen using the builder pattern
#[must_use = "drawers are lazy and do nothing until consumed"]
#[derive(Debug, Clone)]
pub struct TextureDrawer<'a>{
    /// The position on the screen where the texture will be drawn
    pub pos: (f32, f32),
    sin_cos: (f32, f32),
    /// The colour the texture will drawn with
    pub colour: [f32; 4],
    texture: &'a Texture
}

impl<'a> TextureDrawer<'a> {
    set!{/// Sets the position the texture will be drawn at
        fn TextureDrawer; pos: (f32, f32)}
    set!{/// Sets the colours the texture will be drawn with
        fn TextureDrawer; colour: [f32; 4]}
    set!{/// Sets the rotation of the texture to be drawn on the screen
        fn TextureDrawer; rotation: f32 => sin_cos: rotation.sin_cos()}
    /// Consumes self and draws the texture to the screen with the given options
    pub fn draw(self, drawer: &mut Drawer){
        let TextureDrawer{pos: (x, y), sin_cos: (sin, cos), colour, texture} = self;

        let uniforms = uniform! {
            h_size: drawer.graphics.h_size,
            tex   : &texture.tex,
            colour: colour,
            matrix: [
                [ cos, sin, 0., 0.],
                [-sin, cos, 0., 0.],
                [  0.,  0., 1., 0.],
                [  x ,  y , 0., 1.],
            ]
        };

        drawer.draw(&texture.vertex_buffer, &drawer.graphics.indices, &drawer.graphics.program, &uniforms, &drawer.graphics.params)
            .expect("draw failed")
    }
}

/// A simple rectangle that can be drawn on the screen
#[derive(Debug)]
pub struct Quad{
    vertex_buffer: VertexBuffer<ColourVertex>
}

impl Quad {
    /// Creates a new quad from the vertices with one colour for the whole quad
    pub fn new(display: &Display, colour: [f32; 4], vertices: [[f32; 2]; 4]) -> Result<Self, ::glium::vertex::BufferCreationError>{
        VertexBuffer::new(display, &[
            ColourVertex::new(vertices[0], colour),
            ColourVertex::new(vertices[1], colour),
            ColourVertex::new(vertices[2], colour),
            ColourVertex::new(vertices[3], colour)
        ]).map(|vb| Quad{
            vertex_buffer: vb
        })
    }
    /// Creates a new rectangular quad from the width and height with one colour
    pub fn new_rect(display: &Display, colour: [f32; 4], width: f32, height: f32) -> Result<Self, ::glium::vertex::BufferCreationError>{
        let (w, h) = (width/2., height/2.);
        VertexBuffer::new(display, &[
            ColourVertex::new([-w, -h], colour),
            ColourVertex::new([ w, -h], colour),
            ColourVertex::new([ w,  h], colour),
            ColourVertex::new([-w,  h], colour)
        ]).map(|vb| Quad{
            vertex_buffer: vb
        })
    }
    /// Creates a new quad from the vertices with each with vertice having its own colour
    pub fn with_colours(display: &Display, vertices: [[f32; 2]; 4], colours: [[f32; 4]; 4]) -> Result<Self, ::glium::vertex::BufferCreationError>{
        VertexBuffer::new(display, &[
            ColourVertex::new(vertices[0], colours[0]),
            ColourVertex::new(vertices[1], colours[1]),
            ColourVertex::new(vertices[2], colours[2]),
            ColourVertex::new(vertices[3], colours[3]),
        ]).map(|vb| Quad{
            vertex_buffer: vb
        })
    }
    /// Returns an object used for drawing the quad onto the screen with a `Drawer`
    pub fn drawer(&self) -> QuadDrawer{
        QuadDrawer{
            quad: self,
            pos: (0., 0.),
            sin_cos: (0., 1.)
        }
    }
}

/// Object for drawing polygons to the screen using the builder pattern
#[must_use = "drawers are lazy and do nothing until consumed"]
#[derive(Debug, Clone)]
pub struct QuadDrawer<'a>{
    quad: &'a Quad,
    /// The position on the screen where the it will be drawn
    pub pos: (f32, f32),
    sin_cos: (f32, f32)
}

impl<'a> QuadDrawer<'a>{
    set!{/// Sets the position the rectangle will be drawn at
        fn QuadDrawer; pos: (f32, f32)}
    set!{/// Sets the rotation of the rectangle to be drawn on the screen
        fn QuadDrawer; rotation: f32 => sin_cos: rotation.sin_cos()}
    /// Consumes self and draws the rectangle to the screen with the given options
    pub fn draw(self, drawer: &mut Drawer){
        let QuadDrawer{quad, pos: (x, y), sin_cos: (sin, cos)} = self;

        let uniforms = uniform! {
            h_size : drawer.graphics.h_size,
            matrix: [
                [ cos, sin, 0., 0.],
                [-sin, cos, 0., 0.],
                [  0.,  0., 1., 0.],
                [  x ,  y , 0., 1.],
            ]
        };

        // If this panics, it is a problem with korome
        drawer.draw(&quad.vertex_buffer, &drawer.graphics.indices, &drawer.graphics.colour_program, &uniforms, &drawer.graphics.params)
            .expect("draw failed")
    }
}
