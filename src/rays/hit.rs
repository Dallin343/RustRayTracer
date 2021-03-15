use nalgebra::{Point3, Vector3};

pub struct Hit {
    pub did_hit: bool,
    pub t: f32,
    pub point: Point3<f32>,
    pub norm: Vector3<f32>,
    pub color: Vector3<f32>,
}

impl Hit {
    pub fn new(did_hit: bool, t: f32, point: Point3<f32>, norm: Vector3<f32>, color: Vector3<f32>) -> Self {
        Hit {
            did_hit,
            t,
            point,
            norm,
            color
        }
    }
}

