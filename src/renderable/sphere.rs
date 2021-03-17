use nalgebra::{Vector3, Point3, normalize, dot};

use crate::{
    material::*,
    rays::{ray::*, hit::*},
    renderable::RayTrace
};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>,
               radius: f64,
               mat: Material) -> Self {
        Sphere {
            center,
            radius,
            mat
        }
    }
}

impl RayTrace for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let oc: Vector3<f64> = self.center - ray.origin();
        let adj = dot(&oc, &ray.dir());

        if adj < 0.0 {
            return None;
        }

        let d2 = dot(&oc, &oc) - (adj * adj);

        if d2 > self.radius*self.radius {
            return None;
        }

        let thc = (self.radius*self.radius - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 <= 0.0 && t1 <= 0.0 {
            return None;
        }

        let t = if t0 < t1 {t0} else {t1};

        let point = ray.origin() + ray.dir() * t;
        let norm: Vector3<f64> = (point - self.center) / self.radius;

        Some(Hit::new(t, point, norm))
    }

    fn get_mat(&self) -> &Material {
        &self.mat
    }
}
