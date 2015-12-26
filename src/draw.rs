extern crate glium;
extern crate image;

use std::collections::HashMap;

use glium::{VertexBuffer, Program, DrawParameters, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::Texture2d;
use glium::index::NoIndices;

use std::io::{Read, Cursor, Result as IOResult};
use std::fs::File;

#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub struct Vertex {
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

/// A type for having two `VertexBuffer`s as an array
pub type VertexBuffers = [VertexBuffer<Vertex>; 2];

/// Struct for storing a 2D texture
pub struct Texture{
    tex: Texture2d,
    size: (f32, f32),
    vertex_buffers: VertexBuffers,
}

impl Texture {
    /// Creates a new instance of `Texture` with the given bytes
    pub fn new(display: &GlutinFacade, bytes: &[u8], width: u32, height: u32) -> Texture{
        let (dis_width, dis_height) = display.get_window().unwrap().get_inner_size().unwrap();

        let image = image::load(Cursor::new(bytes),
            image::PNG).unwrap().to_rgba();

        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

        let (w, h) = ((width as f32/dis_width as f32)/2.0, (height as f32/dis_height as f32)/2.0);

        let v1 = Vertex::new((-w, -h), (0.0, 0.0));
        let v2 = Vertex::new(( w, -h), (1.0, 0.0));
        let v3 = Vertex::new(( w,  h), (1.0, 1.0));
        let v4 = Vertex::new((-w,  h), (0.0, 1.0));

        Texture {
            tex: Texture2d::new(display, image).unwrap(),
            size: (w, h),
            vertex_buffers: [VertexBuffer::new(display, &vec![v1, v2, v4]).unwrap(), VertexBuffer::new(display, &vec![v2, v3, v4]).unwrap()],
        }
    }

    /// Reads the bytes of a texture from a file and creates a `Texture` instance
    pub fn new_from_file(display: &GlutinFacade, str_path: &str, width: u32, height: u32) -> IOResult<Texture>{
        let mut f = try!(File::open(str_path));
        let mut bytes = Vec::new();
        try!(f.read_to_end(&mut bytes));

        Ok(Self::new(display, &bytes, width, height))
    }

    /// Gets the array of `VertexBuffer`s
    pub fn get_vertex_bufffers(&self) -> &VertexBuffers {
        &self.vertex_buffers
    }

    /// Gets the given size of this `Texture`
    pub fn get_size(&self) -> (f32, f32){
        self.size
    }
}

const INDENTIY_MATRIX: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0]];


type Textures<'a> = HashMap<&'a str, Texture>;

/// Functionality for rendering
pub struct Draw<'a> {
    display: GlutinFacade,
    program: Program,
    indices: NoIndices,
    params : DrawParameters<'a>,
    size   : (u32, u32),
    textures: Textures<'a>
}

impl<'a> Draw<'a> {
    /// Creates a new `Draw` instance using the given display
    pub fn new(display: GlutinFacade) -> Draw<'a> {
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;

            uniform mat4 scale_matrix;
            uniform mat4 rotation_matrix;
            uniform mat4 translation_matrix;

            void main() {
                v_tex_coords = tex_coords;
                gl_Position = translation_matrix * rotation_matrix * scale_matrix * vec4(position, 0.0, 1.0); // has to be in this order!
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        let size = display.get_window().unwrap().get_inner_size().unwrap();

        let params: DrawParameters = DrawParameters{
            blend : glium::Blend::alpha_blending(),
            smooth: Some(glium::Smooth::Nicest),
            .. Default::default()
        };

        Draw{
            program: Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            indices: NoIndices(glium::index::PrimitiveType::TrianglesList),
            textures: HashMap::new(),
            display: display,
            size: size,
            params: params,
        }
    }

    /// Loads a texture from a byte slice into the texture cache
    pub fn load_texture_from_bytes(&mut self, identifier: &'a str, bytes: &[u8], width: u32, height: u32) {
        let texture = Texture::new(&self.display, bytes, width, height);

        self.textures.insert(identifier, texture);
    }

    /// Loads a texture from a file into the texture cache
    pub fn load_texture(&mut self, identifier: &'a str, width: u32, height: u32) -> IOResult<()> {
        let texture = try!(Texture::new_from_file(&self.display, &format!("{}.png", identifier), width, height));
        self.textures.insert(identifier, texture);

        Ok(())
    }

    /// Returns a `TextureDrawer` for a texture if it exists in the cache
    pub fn texture(&self, tex: &str) -> Option<TextureDrawer> {
        self.textures.get(tex).map(|texture| TextureDrawer{
            texture: texture,
            scale_matrix      : INDENTIY_MATRIX,
            rotation_matrix   : INDENTIY_MATRIX,
            translation_matrix: INDENTIY_MATRIX,
            program: &self.program,
            indices: &self.indices,
            params : &self.params,
        })
    }

    /// Returns the size of the window
    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// Returns the inner `GlutinFacade`
    pub fn get_display(&self) -> &GlutinFacade {
        &self.display
    }
}

/// An interface for drawing a texture on the screen using
#[must_use = "`TextureDrawer` does nothing until drawn"]
pub struct TextureDrawer<'a> {
    scale_matrix      : [[f32; 4]; 4],
    rotation_matrix   : [[f32; 4]; 4],
    translation_matrix: [[f32; 4]; 4],
    texture: &'a Texture,
    program: &'a glium::Program,
    indices: &'a NoIndices,
    params : &'a glium::DrawParameters<'a>,
}

impl<'a> TextureDrawer<'a> {
    /// Scales the texture to be drawn
    pub fn scale(self, scale_x: f32, scale_y: f32) -> TextureDrawer<'a> {
        TextureDrawer{
            scale_matrix: [
                [scale_x, 0.0, 0.0, 0.0],
                [0.0, scale_y, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            ..
            self
        }
    }

    // TODO fix rotate
    // NOTE rotate works fine when the window is square
    /// Rotates the texture, though it seems in a bit of a skewed manner
    pub fn rotate(self, rotation: f32) -> TextureDrawer<'a> {
        TextureDrawer{
            rotation_matrix: [
                [rotation.cos(), -rotation.sin(), 0.0, 0.0],
                [rotation.sin(),  rotation.cos(), 0.0, 0.0],
                [           0.0,             0.0, 1.0, 0.0],
                [           0.0,             0.0, 0.0, 1.0]
            ],
            ..
            self
        }
    }

    /// Translates the texture on the screen
    pub fn translate(self, translate_x: f32, translate_y: f32) -> TextureDrawer<'a> {
        TextureDrawer{
            translation_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [translate_x, translate_y, 0.0, 1.0],
            ],
            ..
            self
        }
    }

    /// Finally draws the texture
    pub fn draw(self, frame: &mut glium::Frame){
        let uniforms = uniform! {
            tex               : &self.texture.tex,
            scale_matrix      :  self.scale_matrix,
            rotation_matrix   :  self.rotation_matrix,
            translation_matrix:  self.translation_matrix,
        };

        for vertex_buffer in self.texture.get_vertex_bufffers(){
            frame.draw(vertex_buffer, self.indices, self.program, &uniforms, self.params).unwrap();
        }
    }
}
