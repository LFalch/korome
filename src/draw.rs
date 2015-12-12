extern crate glium;
extern crate image;

use std::collections::HashMap;

use glium::{VertexBuffer, Program, DrawParameters};

use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::Texture2d;

use std::io;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

pub type VertexBuffers = [VertexBuffer<Vertex>; 2];

pub struct Texture{
    tex: Texture2d,
    width: u32,
    height: u32,
    vertex_buffers: VertexBuffers,
}

impl Texture {
    pub fn new<'a, 'b>(display: &'a GlutinFacade, bytes: &'b [u8], width: u32, height: u32) -> Texture{
        use std::io::Cursor;

        let (dis_width, dis_height) = display.get_window().unwrap().get_inner_size().unwrap();

        let image = image::load(Cursor::new(bytes),
            image::PNG).unwrap().to_rgba();

        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

        let (w, h) = ((width as f32/dis_width as f32)/2.0, (height as f32/dis_height as f32)/2.0);

        let v1 = Vertex{ position: [-w, -h], tex_coords: [0.0, 0.0] };
        let v2 = Vertex{ position: [ w, -h], tex_coords: [1.0, 0.0] };
        let v3 = Vertex{ position: [ w,  h], tex_coords: [1.0, 1.0] };
        let v4 = Vertex{ position: [-w,  h], tex_coords: [0.0, 1.0] };

        Texture {
            tex: Texture2d::new(display, image).unwrap(),
            width: width,
            height: height,
            vertex_buffers: [VertexBuffer::new(display, &vec![v1, v2, v4]).unwrap(), VertexBuffer::new(display, &vec![v2, v3, v4]).unwrap()],
        }
    }

    pub fn new_from_file<'a, 'b>(display: &'a GlutinFacade, str_path: &'b str, width: u32, height: u32) -> io::Result<Texture>{
        use std::fs::File;
        use std::io::Read;

        let mut f = try!(File::open(str_path));
        let mut bytes = Vec::new();
        try!(f.read_to_end(&mut bytes));

        Ok(Self::new(display, &*bytes, width, height))
    }

    pub fn get_vertex_bufffers(&self) -> &VertexBuffers {
        &self.vertex_buffers
    }

    pub fn get_width(&self) -> u32{
        self.width
    }

    pub fn get_height(&self) -> u32{
        self.height
    }
}

const INDENTIY_MATRIX: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0]];


type Textures<'a> = HashMap<&'a str, Texture>;

pub struct Draw<'a> {
    program: Program,
    indices: glium::index::NoIndices,
    params : DrawParameters<'a>,
    width : u32,
    height: u32,
    textures: Textures<'a>
}

impl<'a> Draw<'a> {
    pub fn new(display: &GlutinFacade) -> Draw {
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

        let (width, height) = display.get_window().unwrap().get_inner_size().unwrap();

        let params: DrawParameters = DrawParameters{
            blend : glium::Blend::alpha_blending(),
            smooth: Some(glium::Smooth::Nicest),
            .. Default::default()
        };

        Draw{
            program: Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            width: width,
            height: height,
            params: params,
            textures: HashMap::new(),
        }
    }

    pub fn load_texture_from_bytes(&mut self, display: &GlutinFacade,
            identifier: &'a str, bytes: &[u8], width: u32, height: u32) {

        let texture = Texture::new(display, bytes, width, height);

        self.textures.insert(identifier, texture);
    }

    pub fn load_texture(&mut self, display: &GlutinFacade,
            identifier: &'a str, width: u32, height: u32) -> io::Result<()> {

        let texture = try!(Texture::new_from_file(display, &format!("{}.png",identifier), width, height));
        self.textures.insert(identifier, texture);

        Ok(())
    }

    pub fn texture(&self, tex: &str) -> Option<TextureDrawer> {
        if let Some(texture) = self.textures.get(tex){
            Some(TextureDrawer{
                texture: texture,
                scale_matrix      : INDENTIY_MATRIX,
                rotation_matrix   : INDENTIY_MATRIX,
                translation_matrix: INDENTIY_MATRIX,
                program: &self.program,
                indices: &self.indices,
                params : &self.params,
            })
        }else{
            None
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub struct TextureDrawer<'a> {
    scale_matrix      : [[f32; 4]; 4],
    rotation_matrix   : [[f32; 4]; 4],
    translation_matrix: [[f32; 4]; 4],
    texture: &'a Texture,
    program: &'a glium::Program,
    indices: &'a glium::index::NoIndices,
    params : &'a glium::DrawParameters<'a>,
}

impl<'a> TextureDrawer<'a> {
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

    pub fn rotate(self, rotation: f32) -> TextureDrawer<'a> {
        TextureDrawer{
            rotation_matrix: [
                [ rotation.cos(), rotation.sin(), 0.0, 0.0],
                [-rotation.sin(), rotation.cos(), 0.0, 0.0],
                [            0.0,            0.0, 1.0, 0.0],
                [            0.0,            0.0, 0.0, 1.0]
            ],
            ..
            self
        }
    }

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

    pub fn draw(self, frame: &mut glium::Frame){
        use glium::Surface;

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
