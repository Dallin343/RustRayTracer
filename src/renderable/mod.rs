use crate::rays::{ray::Ray, hit::Hit};
use crate::material::Material;

pub mod sphere;
pub mod polygon;

pub enum Object {
    Sphere(sphere::Sphere),
    Polygon(polygon::Polygon),
}

pub trait RayTrace {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn get_mat(&self) -> &Material;
}

impl RayTrace for Object {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        match &self {
            Object::Sphere(sphere) => sphere.intersect(ray),
            Object::Polygon(polygon) => polygon.intersect(ray)
        }
    }

    fn get_mat(&self) -> &Material {
        match &self {
            Object::Sphere(sphere) => sphere.get_mat(),
            Object::Polygon(polygon) => polygon.get_mat()
        }
    }
}



