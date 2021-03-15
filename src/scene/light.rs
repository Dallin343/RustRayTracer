use nalgebra::Vector3;

pub struct Light {
    color: Vector3<f32>,
    direction_to: Vector3<f32>,
}


impl Light {
    pub fn new(direction_to: Vector3<f32>, color: Vector3<f32>) -> Self {
        Light {
            direction_to,
            color
        }
    }
}
