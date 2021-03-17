use nalgebra::{Vector3, Point3, distance, normalize};
use crate::rays::ray::Ray;

pub struct Hit {
    pub t: f64,
    pub point: Point3<f64>,
    pub norm: Vector3<f64>,
}

impl Hit {
    pub fn new(t: f64, point: Point3<f64>, norm: Vector3<f64>) -> Self {
        Hit {
            t,
            point,
            norm,
        }
    }

    pub fn distance_to(&self, ray: &Ray) -> f64 {
        distance(&self.point, &ray.origin())
    }

    pub fn vector_to(&self, v: Point3<f64>) -> Vector3<f64> {
        normalize(&(v - self.point))
    }
}

