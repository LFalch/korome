extern crate glium;
extern crate image;

use glium::{DisplayBuild, VertexBuffer, Program, DrawParameters, Display, Surface};
use glium::texture::Texture2d;
use glium::index::NoIndices;

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

type VertexBuffers = [VertexBuffer<Vertex>; 2];

/// A 2D texture that is ready to be drawn
// NOTE Size: 1 696 bytes
pub struct Texture{
    tex: Texture2d,
    vertex_buffers: VertexBuffers,
}

impl Texture {
    #[inline]
    fn from_png_bytes(display: &Display, bytes: &[u8]) -> TextureResult<Texture>{
        Texture::new(display,
            try!(
                image::load_from_memory_with_format(bytes, image::PNG)
            ).to_rgba()
        )
    }
    #[inline]
    fn from_file<P: AsRef<Path>>(display: &Display, path: P) -> TextureResult<Texture>{
        Texture::new(display,
            try!(
                image::open(path)
            ).to_rgba()
        )
    }

    fn new(display: &Display, image: image::RgbaImage) -> TextureResult<Texture>{
        let (width, height) = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), (width, height));

        let (w, h) = (width as f32 / 2.0, height as f32 / 2.0);

        let v1 = Vertex::new([-w, -h], [0.0, 0.0]);
        let v2 = Vertex::new([ w, -h], [1.0, 0.0]);
        let v3 = Vertex::new([ w,  h], [1.0, 1.0]);
        let v4 = Vertex::new([-w,  h], [0.0, 1.0]);

        Ok(Texture {
            tex: try!(Texture2d::new(display, image)),
            vertex_buffers: [try!(VertexBuffer::new(display, &[v1, v2, v4])), try!(VertexBuffer::new(display, &[v2, v3, v4]))],
        })
    }
}

/// Loads a texture, by loading the bytes at compile-time
#[macro_export]
macro_rules! include_texture {
    ($draw:expr, $texture:tt) => {
        $draw.load_texture_from_bytes(include_bytes!($texture))
    };
}

/// Contains the display and handles most of the graphics
pub struct Draw<'a> {
    display: Display,
    program: Program,
    params : DrawParameters<'a>
}

const INDICES: NoIndices = NoIndices(glium::index::PrimitiveType::TrianglesList);

impl<'a> Draw<'a> {
    /// Creates a new `Draw` from a `Display` made using the arguments
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self::from_display(
            glium::glutin::WindowBuilder::new()
                .with_title(title.to_string())
                .with_dimensions(width, height)
                .with_vsync()
                .build_glium().expect("Failed to build the window")
        )
    }

    /// Creates a new `Draw` instance using the given display
    pub fn from_display(display: Display) -> Self {
        let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();
        let (w, h) = (w as f32 / 2.0, h as f32 / 2.0);

        let vertex_shader_src = &r#"
            #version 140

            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;

            uniform mat4 matrix;

            void main() {
                v_tex_coords = tex_coords;

                vec4 pos = matrix * vec4(position, 0.0, 1.0);

                pos.x /= $width;
                pos.y /= $height;

                gl_Position = pos;
            }
        "#.replace("$width", &w.to_string()).replace("$height", &h.to_string());
        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        let params = DrawParameters{
            blend : glium::Blend::alpha_blending(),
            .. Default::default()
        };

        Draw{
            // Unwrap should be safe
            program: Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            display: display,
            params : params,
        }
    }

    #[inline]
    /// Returns a `Texture` created from a PNG-encoded byte slice
    pub fn load_texture_from_bytes(&self, bytes: &[u8]) -> TextureResult<Texture> {
        Texture::from_png_bytes(&self.display, bytes)
    }

    #[inline]
    /// Returns a `Texture` created from a file
    pub fn load_texture_from_file<P: AsRef<Path>>(&self, path: P) -> TextureResult<Texture> {
        Texture::from_file(&self.display, path)
    }

    /// Draws a texture onto the screen
    pub fn draw_texture(&self, target: &mut glium::Frame, texture: &Texture, rotation: f32, x: f32, y: f32) -> DrawResult<()>{
        let (sin, cos)  = rotation.sin_cos();

        let uniforms = uniform! {
            tex   : &texture.tex,
            matrix: [
                [ cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [ 0.0, 0.0, 1.0, 0.0],
                [   x,   y, 0.0, 1.0],
            ],
        };

        for vertex_buffer in &texture.vertex_buffers{
            try!(target.draw(vertex_buffer, INDICES, &self.program, &uniforms, &self.params));
        }

        Ok(())
    }

    /// Draws a texture onto the screen without rotation
    pub fn draw_texture_rigid(&self, target: &mut glium::Frame, texture: &Texture, x: f32, y: f32) -> DrawResult<()>{
        let uniforms = uniform! {
            tex: &texture.tex,
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [  x,   y, 0.0, 1.0],
            ],
        };

        for vertex_buffer in &texture.vertex_buffers{
            try!(target.draw(vertex_buffer, INDICES, &self.program, &uniforms, &self.params));
        }

        Ok(())
    }
}

impl<'a> Deref for Draw<'a>{
    type Target = Display;

    #[inline]
    fn deref(&self) -> &Display{
        &self.display
    }
}

/// Provides functionality for drawing.
/// Can also be dereferenced into a `glium::Frame`.
pub struct Drawer<'a>{
    target: glium::Frame,
    /// Reference to the draw instance
    pub draw  : &'a Draw<'a>
}

impl<'a> Drawer<'a>{
    #[inline]
    /// Creates a new `Drawer` to draw next frame
    pub fn new(draw: &'a Draw<'a>) -> Self{
        let target = draw.draw();
        Drawer{
            target: target,
            draw: draw
        }
    }

    #[inline]
    /// Clears the screen with the specified colour
    pub fn clear(&mut self, red: f32, green: f32, blue: f32){
        self.clear_color(red, green, blue, 1.)
    }

    #[inline]
    /// Uses `Draw` to draw a texture onto the screen
    pub fn draw_texture(&mut self, texture: &Texture, rotation: f32, x: f32, y: f32) -> DrawResult<()>{
        self.draw.draw_texture(self, texture, rotation, x, y)
    }

    /// Draws an iterator of `Sprite`s onto the screen
    pub fn draw_sprites<'b, D: 'b + Sprite, I: IntoIterator<Item = &'b D>>(&mut self, sprites: I) -> DrawResult<()>{
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

impl<'a> Deref for Drawer<'a>{
    type Target = glium::Frame;
    #[inline]
    fn deref(&self) -> &glium::Frame{
        &self.target
    }
}

impl<'a> DerefMut for Drawer<'a>{
    #[inline]
    fn deref_mut(&mut self) -> &mut glium::Frame{
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
pub trait Sprite {
    /// Returns the position on the screen it should be drawn
    fn get_pos(&self) -> (f32, f32);
    /// Returns the rotation it should be drawn with
    fn get_rotation(&self) -> f32;
    /// Returns the `Texture`
    fn get_texture(&self) -> &Texture;
}
