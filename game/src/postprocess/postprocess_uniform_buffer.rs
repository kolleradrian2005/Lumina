#[derive(Debug, Clone, Copy)]
pub struct PostProcessUniformBuffer {
    pub saturation: f32,
    pub tint_intensity: f32,
    pub darkening_factor: f32,
    pub focal_radius: f32,
    pub tint_color: [f32; 3], // Placing is important because of padding
    pub smooth_factor: f32,
    pub vignette_intensity: f32,
}
