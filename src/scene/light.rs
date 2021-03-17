use nalgebra::Vector3;

pub struct Light {
    pub color: Vector3<f64>,
    pub direction_to: Vector3<f64>,
}


impl Light {
    pub fn new(direction_to: Vector3<f64>, color: Vector3<f64>) -> Self {
        Light {
            direction_to,
            color
        }
    }
}
