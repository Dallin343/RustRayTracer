use nalgebra::{Vector3, Point3};

pub enum RayType {
    Camera,
    Illumination
}

pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
    pub ray_type: RayType
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>, ray_type: RayType) -> Self {
        Ray {
            origin,
            direction,
            ray_type
        }
    }

    pub fn origin_as_tuple(&self) -> (f64, f64, f64) {
        (self.origin.x, self.origin.y, self.origin.z)
    }

    pub fn origin(&self) -> Point3<f64> {
        self.origin
    }

    pub fn dir_as_tuple(&self) -> (f64, f64, f64) {
        (self.direction.x, self.direction.y, self.direction.z)
    }

    pub fn dir(&self) -> Vector3<f64> {
        self.direction
    }
}
