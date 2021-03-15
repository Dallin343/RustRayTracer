use crate::rays::{ray::Ray, hit::Hit};

pub mod sphere;
pub mod polygon;

pub enum Object {
    Sphere(sphere::Sphere),
    Polygon(polygon::Polygon),
}

pub trait RayTrace {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
}



