use std::f32::consts::PI;

use crate::model::sprite;

fn connecting_indices(num: u32) -> Vec<u32> {
    let mut indices = Vec::new();
    for i in 0..num - 1 {
        indices.push(i);
        indices.push(i + 1);
    }
    indices.push(num - 1);
    indices.push(0);
    indices
}
pub fn rectangle(width: f32, height: f32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    (
        sprite::rectangle_vertices(width / 2.0, height / 2.0).to_vec(),
        connecting_indices(4),
        vec![],
    )
}

pub fn capsule(width: f32, height: f32, segments: u32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let radius = width / 2.0;
    let half_spine = (height - width) / 2.0;

    let mut vertices = Vec::new();

    // Top semicircle
    for i in 0..=segments {
        let theta = (i as f32 / segments as f32) * PI;
        let x = radius * theta.cos();
        let y = radius * theta.sin() + half_spine;
        vertices.push(x);
        vertices.push(y);
        vertices.push(sprite::Z_DEFAULT);
    }

    // Bottom semicircle
    for i in 0..=segments {
        let theta = (i as f32 / segments as f32) * PI;
        let x = -radius * theta.cos();
        let y = -radius * theta.sin() - half_spine;
        vertices.push(x);
        vertices.push(y);
        vertices.push(sprite::Z_DEFAULT);
    }

    let total = (segments + 1) * 2;
    let indices = connecting_indices(total);

    (vertices, indices, vec![])
}
