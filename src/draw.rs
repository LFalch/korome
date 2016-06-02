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

#[derive(Copy, Clone)]
struct RectVertex {
    index: u32,
    position: [f32; 2]
}

implement_vertex!(RectVertex, index, position);

impl RectVertex{
    #[inline]
    fn new(i: u32, x: f32, y: f32) -> Self{
        RectVertex{
            index: i,
            position: [x, y]
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
    rect_program: Program,
    h_size : (f32, f32),
    indices: IndexBuffer<u8>,
    params : DrawParameters<'a>
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
            rect_program: Program::from_source(&display, include_str!("shaders/rect.vs"), include_str!("shaders/rect.fs"), None).unwrap(),
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
    pub fn new(graphics: &'a Graphics<'a>) -> Self{
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
    /// Draws a texture onto the screen
    #[deprecated(since = "0.11.1", note="use Drawer::texture() instead")]
    #[inline]
    pub fn draw_texture(&mut self, texture: &Texture, x: f32, y: f32, rotation: f32) -> DrawResult{
        self.texture(texture).pos((x, y)).rotation(rotation).draw()
    }
    /// Draws a texture onto the screen without rotation
    #[deprecated(since = "0.11.1", note="use Drawer::texture() instead")]
    #[inline]
    pub fn draw_texture_rigid(&mut self, texture: &Texture, x: f32, y: f32) -> DrawResult{
        self.texture(texture).pos((x, y)).draw()
    }
    /// Returns an object for drawing texture to the screen
    pub fn texture<'b>(&'b mut self, texture: &'b Texture) -> TextureDrawer<'b>{
        TextureDrawer::new(self, self.graphics, texture)
    }
    /// Returns an object for drawing a rectangle to the screen
    pub fn rect<'b>(&'b mut self, rect: &'b Rect) -> RectDrawer<'b>{
        RectDrawer::new(self, self.graphics, rect)
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

/// Object for drawing textures to the screen using the builder pattern
#[must_use = "drawers are lazy and do nothing until consumed"]
pub struct TextureDrawer<'a>{
    /// The position on the screen where the texture will be drawn
    pub pos: (f32, f32),
    sin_cos: (f32, f32),
    /// The colour the texture will drawn with
    pub colour: [f32; 4],
    target: &'a mut Frame,
    graphics: &'a Graphics<'a>,
    texture: &'a Texture
}

impl<'a> TextureDrawer<'a> {
    #[inline(always)]
    fn new(target: &'a mut Frame, graphics: &'a Graphics, texture: &'a Texture) -> Self {
        TextureDrawer{
            pos: (0., 0.),
            sin_cos: (0., 1.),
            colour: [1., 1., 1., 1.],
            target: target,
            graphics: graphics,
            texture: texture
        }
    }
    /// Sets the position the texture will be drawn at
    #[inline]
    pub fn pos(self, pos: (f32, f32)) -> Self{
        TextureDrawer{
            pos: pos,
            .. self
        }
    }
    /// Sets the colours the texture will be drawn with
    #[inline]
    pub fn colour(self, colour: [f32; 4]) -> Self{
        TextureDrawer{
            colour: colour,
            .. self
        }
    }
    #[inline]
    /// Sets the rotation of the texture to be drawn on the screen
    pub fn rotation(self, rot: f32) -> Self{
        TextureDrawer{
            sin_cos: rot.sin_cos(),
            .. self
        }
    }
    /// Consumes self and draws the texture to the screen with the given options
    pub fn draw(self) -> Result<(), ::glium::DrawError>{
        let TextureDrawer{pos: (x, y), sin_cos: (sin, cos), colour, target, graphics, texture} = self;

        let uniforms = uniform! {
            h_size: graphics.h_size,
            tex   : &texture.tex,
            colour: colour,
            matrix: [
                [ cos, sin, 0., 0.],
                [-sin, cos, 0., 0.],
                [  0.,  0., 1., 0.],
                [  x ,  y , 0., 1.],
            ]
        };

        target.draw(&texture.vertex_buffer, &graphics.indices, &graphics.program, &uniforms, &graphics.params)
    }
}

/// A simple rectangle that can be drawn on the screen
pub struct Rect{
    vertex_buffer: VertexBuffer<RectVertex>
}

impl Rect {
    pub fn new(display: &Display, width: f32, height: f32) -> Result<Self, ::glium::vertex::BufferCreationError>{
        let (w, h) = (width/2., height/2.);
        VertexBuffer::new(display, &[
            RectVertex::new(0, -w, -h),
            RectVertex::new(1,  w, -h),
            RectVertex::new(2,  w,  h),
            RectVertex::new(3, -w,  h)
        ]).map(|vb| Rect{
            vertex_buffer: vb
        })
    }
}

/// Object for drawing rectangles to the screen using the builder pattern
#[must_use = "drawers are lazy and do nothing until consumed"]
pub struct RectDrawer<'a>{
    rect: &'a Rect,
    /// The position on the screen where the it will be drawn
    pub pos: (f32, f32),
    sin_cos: (f32, f32),
    /// The colours the rectangle will drawn with
    pub colours: [[f32; 4]; 4],
    target: &'a mut Frame,
    graphics: &'a Graphics<'a>
}

impl<'a> RectDrawer<'a>{
    #[inline(always)]
    fn new(target: &'a mut Frame, graphics: &'a Graphics, rect: &'a Rect) -> Self {
        RectDrawer{
            rect: rect,
            pos: (0., 0.),
            sin_cos: (0., 1.),
            colours: [[1.; 4]; 4],
            target: target,
            graphics: graphics
        }
    }
    /// Sets the position the rectangle will be drawn at
    #[inline]
    pub fn pos(self, pos: (f32, f32)) -> Self{
        RectDrawer{
            pos: pos,
            .. self
        }
    }
    #[inline]
    /// Sets the rotation of the rectangle to be drawn on the screen
    pub fn rotation(self, rot: f32) -> Self{
        RectDrawer{
            sin_cos: rot.sin_cos(),
            .. self
        }
    }
    /// Sets the colour on all corners of the rectangle
    #[inline]
    pub fn colour_whole(self, colour: [f32; 4]) -> Self{
        RectDrawer{
            colours: [colour; 4],
            .. self
        }
    }
    /// Sets the colours on each corner of the rectangle
    #[inline]
    pub fn colours(self, colours: [[f32; 4]; 4]) -> Self{
        RectDrawer{
            colours: colours,
            .. self
        }
    }
    /// Consumes self and draws the rectangle to the screen with the given options
    pub fn draw(self) -> Result<(), ::glium::DrawError>{
        let RectDrawer{rect, pos: (x, y), sin_cos: (sin, cos), colours, target, graphics} = self;

        let uniforms = uniform! {
            h_size : graphics.h_size,
            colours: colours,
            matrix: [
                [ cos, sin, 0., 0.],
                [-sin, cos, 0., 0.],
                [  0.,  0., 1., 0.],
                [  x ,  y , 0., 1.],
            ]
        };

        target.draw(&rect.vertex_buffer, &graphics.indices, &graphics.rect_program, &uniforms, &graphics.params)
    }
}
