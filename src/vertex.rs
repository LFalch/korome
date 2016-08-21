#[derive(Debug, Copy, Clone)]
pub struct TextureVertex {
    position  : [f32; 2],
    tex_coords: [f32; 2]
}

// Implement the glium::vertex::Vertex trait for `TextureVertex`.
implement_vertex!(TextureVertex, position, tex_coords);

impl TextureVertex{
    #[inline]
    pub fn new(position: [f32; 2], tex_coords: [f32; 2]) -> Self{
        TextureVertex{
            position  : position,
            tex_coords: tex_coords,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColourVertex {
    position: [f32; 2],
    colour  : [f32; 4]
}

// Implement the glium::vertex::Vertex trait for `ColourVertex`.
implement_vertex!(ColourVertex, position, colour);

impl ColourVertex{
    #[inline]
    pub fn new(position: [f32; 2], colour: [f32; 4]) -> Self{
        ColourVertex{
            position: position,
            colour: colour
        }
    }
}
