use nalgebra::{Vector3};

pub struct Material {
    pub diffuse_fac: f64,
    pub specular_fac: f64,
    pub ambient_fac: f64,
    pub diffuse_color: Vector3<f64>,
    pub specular_color: Vector3<f64>,
    pub gloss: f64,
}

impl Material {
    pub fn new(diffuse_fac: f64,
               specular_fac: f64,
               ambient_fac: f64,
               diffuse_color: Vector3<f64>,
               specular_color: Vector3<f64>,
               gloss: f64) -> Self {
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
