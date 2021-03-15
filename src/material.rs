use nalgebra::{Vector3};

pub struct Material {
    pub diffuse_fac: f32,
    pub specular_fac: f32,
    pub ambient_fac: f32,
    pub diffuse_color: Vector3<f32>,
    pub specular_color: Vector3<f32>,
    pub gloss: f32,
}

impl Material {
    pub fn new(diffuse_fac: f32,
               specular_fac: f32,
               ambient_fac: f32,
               diffuse_color: Vector3<f32>,
               specular_color: Vector3<f32>,
               gloss: f32) -> Self {
        Material {
            diffuse_fac,
            specular_fac,
            ambient_fac,
            diffuse_color,
            specular_color,
            gloss
        }
    }
}
