use crate::vec2::Vec2;

pub fn create_model_matrix(position: &Vec2, rotation: &f32, scale: &f32) -> [[f32; 4]; 4] {
    let translation_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position.x, position.y, 0.0, 1.0]
    ];
    
    let rotation_matrix = [
        [rotation.cos(), -rotation.sin(), 0.0, 0.0],
        [rotation.sin(), rotation.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ];

    let scale_matrix = [
        [*scale, 0.0, 0.0, 0.0],
        [0.0, *scale, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ];

    let mut model_matrix = translation_matrix;
    multiply_matrix(&mut model_matrix, &rotation_matrix);
    multiply_matrix(&mut model_matrix, &scale_matrix);

    model_matrix
}

pub fn create_view_matrix(position: &Vec2) -> [[f32; 4]; 4] {
    let translation_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-position.x, -position.y, 0.0, 1.0]
    ];
    
    translation_matrix
}

fn multiply_matrix(result: &mut [[f32; 4]; 4], other: &[[f32; 4]; 4]) {
    let mut tmp = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                tmp[i][j] += result[i][k] * other[k][j];
            }
        }   
    }
    *result = tmp;
}

pub fn create_ortographic_projection_matrix(width :u32, height: u32/*left: f32, right: f32, bottom: f32, top: f32*/, near: f32, far: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = (width as f32) / (height as f32);
    
    let right = aspect_ratio / 2.0;
    let left = -aspect_ratio / 2.0;
    let top = 0.5;
    let bottom = -0.5;
    
    let r_minus_l = right - left;
    let t_minus_b = top - bottom;
    let f_minus_n = far - near;

    let tx = -(right + left) / r_minus_l;
    let ty = -(top + bottom) / t_minus_b;
    let tz = -(far + near) / f_minus_n;
    
    [
        [2.0 / r_minus_l, 0.0, 0.0, tx],
        [0.0, 2.0 / t_minus_b, 0.0, ty],
        [0.0, 0.0, -2.0 / f_minus_n, tz],
        [0.0, 0.0, 0.0, 1.0]
    ]
}
