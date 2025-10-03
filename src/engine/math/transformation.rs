use crate::engine::{
    model::{model::Model, model_group::ModelGroup},
    scene::world::component::transform_component::TransformComponent,
    transformable::Transformable,
};

use super::{vec2::Vec2, vec3::Vec3};

pub fn create_model_matrix(model: &Model, model_group: Option<&ModelGroup>) -> [[f32; 4]; 4] {
    let mut position = model.get_position();
    let mut rotation = model.get_rotation();
    let mut scale = model.get_scale();

    if let Some(m_group) = model_group {
        let m_scale = m_group.get_scale();
        let m_rotation = m_group.get_rotation();

        position = Vec3::from_vec2(position.xy().rotated(m_rotation), position.z);
        position += m_group.get_position();
        rotation += m_rotation;
        scale.x *= m_scale.x;
        scale.y *= m_scale.y;
    }

    let translation_matrix = create_translation_matrix(position);
    let rotation_matrix = create_rotation_matrix(rotation);
    let scale_matrix = create_scale_matrix(scale);

    let mut model_matrix = scale_matrix;
    multiply_matrix(&mut model_matrix, &rotation_matrix);
    multiply_matrix(&mut model_matrix, &translation_matrix);

    model_matrix
}

pub fn create_translation_matrix(position: Vec3) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position.x, position.y, position.z, 1.0],
    ]
}

pub fn create_rotation_matrix(rotation: f32) -> [[f32; 4]; 4] {
    [
        [rotation.cos(), -rotation.sin(), 0.0, 0.0],
        [rotation.sin(), rotation.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn create_scale_matrix(scale: Vec2) -> [[f32; 4]; 4] {
    [
        [scale.x, 0.0, 0.0, 0.0],
        [0.0, scale.y, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn create_view_matrix(position: Vec3) -> [[f32; 4]; 4] {
    create_translation_matrix(-position)
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

pub fn create_ortographic_projection_matrix(
    aspect_ratio: f32,
    near: f32,
    far: f32,
) -> [[f32; 4]; 4] {
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
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn create_transform_matrix(
    model: &TransformComponent,
    parent_component: Option<&TransformComponent>,
) -> [[f32; 4]; 4] {
    let inherited = calc_intherited_transform(model, parent_component);

    let translation_matrix = create_translation_matrix(inherited.position);
    let rotation_matrix = create_rotation_matrix(inherited.rotation);
    let scale_matrix = create_scale_matrix(inherited.scale);

    let mut model_matrix = scale_matrix;
    multiply_matrix(&mut model_matrix, &rotation_matrix);
    multiply_matrix(&mut model_matrix, &translation_matrix);

    model_matrix
}

pub fn calc_intherited_transform(
    model: &TransformComponent,
    parent_component: Option<&TransformComponent>,
) -> TransformComponent {
    let mut position = model.position;
    let mut rotation = model.rotation;
    let mut scale = model.scale;

    if let Some(parent_transform) = parent_component {
        let m_scale = parent_transform.scale;
        let m_rotation = parent_transform.rotation;
        let flip_mul = -((parent_transform.is_flipped as i32) * 2 - 1) as f32;

        position = Vec3::from_vec2(position.xy().rotated(m_rotation * flip_mul), position.z);
        position.x *= flip_mul;
        position += parent_transform.position;

        rotation *= -flip_mul;
        rotation += m_rotation;
        scale.x *= m_scale.x;
        scale.y *= m_scale.y;
    }

    TransformComponent {
        position,
        rotation,
        scale,
        is_flipped: false,
    }
}
