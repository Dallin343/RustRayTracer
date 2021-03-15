use nalgebra::Vector3;

pub struct Camera {
    look_from: Vector3<f32>,
    look_at: Vector3<f32>,
    look_up: Vector3<f32>,
    fov: f32,
}

impl Camera {
    pub fn new(look_from: Vector3<f32>,
               look_at: Vector3<f32>,
               look_up: Vector3<f32>,
               fov: f32) -> Self {
        Camera {
            look_from,
            look_at,
            look_up,
            fov
        }
    }
}
