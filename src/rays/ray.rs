use nalgebra::{Point3, Vector3};

pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn origin_as_tuple(&self) -> (f32, f32, f32) {
        (self.origin.x, self.origin.y, self.origin.z)
    }

    pub fn origin(&self) -> Point3<f32> {
        self.origin
    }

    pub fn dir_as_tuple(&self) -> (f32, f32, f32) {
        (self.direction.x, self.direction.y, self.direction.z)
    }

    pub fn dir(&self) -> Vector3<f32> {
        self.direction
    }
}
