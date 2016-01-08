extern crate glium;
extern crate image;

use std::collections::HashMap;

use glium::{DisplayBuild, VertexBuffer, Program, DrawParameters, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::Texture2d;
use glium::index::NoIndices;

use std::io::{Read, Cursor};
use std::fs::File;

use super::{Result};

#[derive(Copy, Clone)]
struct Vertex {
    position  : [f32; 2],
    tex_coords: [f32; 2]
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex{
    fn new(position: (f32, f32), tex_coords: (f32, f32)) -> Self{
        Vertex{
            position  : [ position .0,  position .1],
            tex_coords: [tex_coords.0, tex_coords.1]
         }
    }
}

type VertexBuffers = [VertexBuffer<Vertex>; 2];

/// Struct for storing a 2D texture
// NOTE Size: 1 696 bytes
pub struct Texture{
    tex: Texture2d,
    vertex_buffers: VertexBuffers,
}

impl Texture {
    /// Creates a new instance of `Texture` with the given bytes
    fn new(display: &GlutinFacade, bytes: &[u8], width: u32, height: u32) -> Result<Texture>{
        //let (dis_width, dis_height) = display.get_window().unwrap().get_inner_size().unwrap();

        let image = try!(image::load(Cursor::new(bytes),
            image::PNG)).to_rgba();

        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

        let (w, h) = (width as f32 / 2.0, height as f32 / 2.0);

        let v1 = Vertex::new((-w, -h), (0.0, 0.0));
        let v2 = Vertex::new(( w, -h), (1.0, 0.0));
        let v3 = Vertex::new(( w,  h), (1.0, 1.0));
        let v4 = Vertex::new((-w,  h), (0.0, 1.0));

        Ok(Texture {
            tex: try!(Texture2d::new(display, image)),
            vertex_buffers: [try!(VertexBuffer::new(display, &[v1, v2, v4])), try!(VertexBuffer::new(display, &[v2, v3, v4]))],
        })
    }

    /// Reads the bytes of a texture from a file and creates a `Texture` instance
    fn new_from_file(display: &GlutinFacade, str_path: &str, width: u32, height: u32) -> Result<Texture>{
        let mut f = try!(File::open(str_path));
        let mut bytes = Vec::new();
        try!(f.read_to_end(&mut bytes));

        Self::new(display, &bytes, width, height)
    }

    /// Gets the array of `VertexBuffer`s
    fn get_vertex_bufffers(&self) -> &VertexBuffers {
        &self.vertex_buffers
    }

    /// Returns a `TextureDrawer` to draw the `Texture1`
    pub fn drawer(&self) -> TextureDrawer{
        TextureDrawer{
            texture: &self,
            rotation   : 0.0,
            translation: (0.0, 0.0),
        }
    }
}

/*
const INDENTIY_MATRIX: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0]];
*/

type Textures<'a> = HashMap<&'a str, Texture>;

/// Trait for objects that can be drawn to the screen
pub trait Drawable {
    /// Returns the position on the screen it should be drawn
    fn get_pos(&self) -> (f32, f32);
    /// Returns the rotation it should be drawn with
    fn get_rotation(&self) -> f32;
    /// Returns the `Texture`
    fn get_texture(&self) -> &Texture;
}

/// Functionality for rendering
pub struct Draw<'a> {
    display: GlutinFacade,
    program: Program,
    indices: NoIndices,
    params : DrawParameters<'a>
}

impl<'a> Draw<'a> {
    /// Creates a new `Draw` from a `Display` made using the arguments
    pub fn new(title: &str, width: u32, height: u32) -> Draw<'a> {
        Self::new_from_display(
            glium::glutin::WindowBuilder::new()
                .with_title(title.to_string())
                .with_dimensions(width, height)
                .with_vsync()
                .build_glium().expect("Failed to build the window")
        )
    }

    /// Creates a new `Draw` instance using the given display
    pub fn new_from_display(display: GlutinFacade) -> Draw<'a> {
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

        let params: DrawParameters = DrawParameters{
            blend : glium::Blend::alpha_blending(),
            .. Default::default()
        };

        Draw{
            program: Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            indices: NoIndices(glium::index::PrimitiveType::TrianglesList),
            display: display,
            params : params,
            // halfsize: (w, h),
        }
    }

    /// Returns a `Texture` created from a byte slice
    pub fn load_texture_from_bytes(&self, bytes: &[u8], width: u32, height: u32) -> Result<Texture> {
        Texture::new(&self.display, bytes, width, height)
    }

    /// Returns a `Texture` created from a file
    pub fn load_texture(&self, identifier: &'a str, width: u32, height: u32) -> Result<Texture> {
        Texture::new_from_file(&self.display, &format!("{}.png", identifier), width, height)
    }

    /// Returns a `DrawablesDrawer` for drawing `Drawable`s to the screen
    pub fn draw_drawables<D: Drawable>(&'a self, target: &'a mut glium::Frame) -> DrawablesDrawer<D>{
        DrawablesDrawer{
            drawables: Vec::new(),
            target: target,
            draw: self
        }
    }

    /// Returns the inner `GlutinFacade`
    pub fn get_display(&self) -> &GlutinFacade {
        &self.display
    }
}

/// An interface for drawing `Drawable`s on the screen
#[must_use = "`DrawablesDrawer` does nothing until drawn"]
pub struct DrawablesDrawer<'a, D: 'a + Drawable>{
    drawables: Vec<&'a D>,
    target: &'a mut glium::Frame,
    draw  : &'a Draw<'a>
}

impl<'a, D: Drawable> DrawablesDrawer<'a, D>{
    /// Adds another `Drawble` to be drawn
    pub fn add(mut self, drawable: &'a D) -> Self{
        self.drawables.push(drawable);

        self
    }

    /// Adds an `Iterator` of `Drawable`s to be drawn
    pub fn add_vec<I: IntoIterator<Item = &'a D>>(mut self, drawables: I) -> Self{
        for d in drawables.into_iter(){
            self = self.add(d);
        }

        self
    }

    /// Finally draws the `Drawable`s
    pub fn draw(self) -> Result<()>{
        for drawable in self.drawables{
            let (x, y) = drawable.get_pos();

            try!(
                drawable.get_texture().drawer()
                    .rotate(drawable.get_rotation())
                    .translate(x, y)
                    .draw(self.target, self.draw)
            );
        }

        Ok(())
    }
}

/// An interface for drawing a texture on the screen
#[must_use = "`TextureDrawer` does nothing until drawn"]
pub struct TextureDrawer<'a> {
    translation: (f32, f32),
    rotation   :  f32,
    texture: &'a Texture,
}

impl<'a> TextureDrawer<'a> {
    /// Rotates the texture
    pub fn rotate(self, rotation: f32) -> TextureDrawer<'a> {
        TextureDrawer{
            rotation: rotation,
            ..
            self
        }
    }

    /// Translates the texture
    pub fn translate(self, translate_x: f32, translate_y: f32) -> TextureDrawer<'a> {
        TextureDrawer{
            translation: (translate_x, translate_y),
            ..
            self
        }
    }

    /// Finally draws the texture
    pub fn draw(self, frame: &mut glium::Frame, draw: &Draw) -> Result<()>{
        let rotation    = self.rotation;
        let translation = self.translation;

        let uniforms = uniform! {
            tex   : &self.texture.tex,
            matrix: [
                [rotation.cos(), -rotation.sin(), 0.0, 0.0],
                [rotation.sin(),  rotation.cos(), 0.0, 0.0],
                [           0.0,             0.0, 1.0, 0.0],
                [ translation.0,   translation.1, 0.0, 1.0],
            ],
        };

        for vertex_buffer in self.texture.get_vertex_bufffers(){
            try!(frame.draw(vertex_buffer, &draw.indices, &draw.program, &uniforms, &draw.params));
        }

        Ok(())
    }
}
