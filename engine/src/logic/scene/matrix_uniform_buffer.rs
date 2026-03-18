#[derive(Debug, Clone, Copy)]
pub struct MatrixUniformBuffer {
    pub projection_matrix: [[f32; 4]; 4],
    pub view_matrix: [[f32; 4]; 4],
}
