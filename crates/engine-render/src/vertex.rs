use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[repr(C)]
#[derive(BufferContents, Vertex, Clone, Copy, Debug)]
pub struct Vertex3D {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}

impl Vertex3D {
    pub const fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}
