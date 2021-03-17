use nalgebra::{Vector3, distance};

pub struct Camera {
    pub look_from: Vector3<f64>,
    pub look_at: Vector3<f64>,
    look_up: Vector3<f64>,
    pub fov: f64,
}

impl Camera {
    pub fn new(look_at: Vector3<f64>,
               look_from: Vector3<f64>,
               look_up: Vector3<f64>,
               fov: f64) -> Self {
        Camera {
            look_from,
            look_at,
            look_up,
            fov
        }
    }

    pub fn viewport_size(&self) -> f64 {
        (self.fov.to_radians()) * (self.look_from - self.look_at).z
    }
}
