extern crate glium;
extern crate image;

use glium::{DisplayBuild, VertexBuffer, Program, DrawParameters, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::Texture2d;
use glium::index::NoIndices;

use std::io::{Read, Cursor};
use std::fs::File;

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
    /// Creates a new instance of `Texture` with the given bytes
    fn new(display: &GlutinFacade, bytes: &[u8], width: u32, height: u32) -> TextureResult<Texture>{
        //let (dis_width, dis_height) = display.get_window().unwrap().get_inner_size().unwrap();

        let image = try!(image::load(Cursor::new(bytes),
            image::PNG)).to_rgba();

        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

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

    /// Reads the bytes of a texture from a file and creates a `Texture` instance
    fn new_from_file(display: &GlutinFacade, str_path: &str, width: u32, height: u32) -> TextureResult<Texture>{
        let mut f = try!(File::open(str_path));
        let mut bytes = Vec::new();
        try!(f.read_to_end(&mut bytes));

        Self::new(display, &bytes, width, height)
    }
}

/// Loads a texture, by loading the bytes at compile-time
#[macro_export]
macro_rules! include_texture {
    ($draw:expr, $texture:tt, $width:expr, $height:expr) => {
        $draw.load_texture_from_bytes(include_bytes!($texture), $width, $height)
    };
}

/// Contains the display and handles most of the graphics
pub struct Draw<'a> {
    display: GlutinFacade,
    program: Program,
    params : DrawParameters<'a>
}

const INDICES: NoIndices = NoIndices(glium::index::PrimitiveType::TrianglesList);

impl<'a> Draw<'a> {
    /// Creates a new `Draw` from a `Display` made using the arguments
    pub fn new(title: &str, width: u32, height: u32) -> Draw<'a> {
        Self::from_display(
            glium::glutin::WindowBuilder::new()
                .with_title(title.to_string())
                .with_dimensions(width, height)
                .with_vsync()
                .build_glium().expect("Failed to build the window")
        )
    }

    /// Creates a new `Draw` instance using the given display
    pub fn from_display(display: GlutinFacade) -> Draw<'a> {
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
            display: display,
            params : params,
            // halfsize: (w, h),
        }
    }

    /// Returns a `Texture` created from a byte slice
    pub fn load_texture_from_bytes(&self, bytes: &[u8], width: u32, height: u32) -> TextureResult<Texture> {
        Texture::new(&self.display, bytes, width, height)
    }

    /// Returns a `Texture` created from a file
    pub fn load_texture(&self, identifier: &'a str, width: u32, height: u32) -> TextureResult<Texture> {
        Texture::new_from_file(&self.display, &format!("{}.png", identifier), width, height)
    }

    /// Draws a texture onto the screen
    pub fn draw_texture(&self, target: &mut glium::Frame, texture: &Texture, rotation: f32, x: f32, y: f32) -> DrawResult<()>{
        let (sin, cos)  = rotation.sin_cos();

        let uniforms = uniform! {
            tex   : &texture.tex,
            matrix: [
                [cos, -sin, 0.0, 0.0],
                [sin,  cos, 0.0, 0.0],
                [0.0,  0.0, 1.0, 0.0],
                [  x,    y, 0.0, 1.0],
            ],
        };

        for vertex_buffer in &texture.vertex_buffers{
            try!(target.draw(vertex_buffer, INDICES, &self.program, &uniforms, &self.params));
        }

        Ok(())
    }

    /// Returns the inner `GlutinFacade`
    pub fn get_display(&self) -> &GlutinFacade {
        &self.display
    }
}
