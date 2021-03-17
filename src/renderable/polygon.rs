use nalgebra::{Point3, normalize, cross, Vector3, dot, approx_eq};
use crate::{
    material::*,
    rays::{ray::*, hit::*},
    renderable::RayTrace
};

pub struct Polygon {
    x: Point3<f64>,
    y: Point3<f64>,
    z: Point3<f64>,
    mat: Material,
}

impl Polygon {
    pub fn new(x: Point3<f64>,
               y: Point3<f64>,
               z: Point3<f64>,
               mat: Material) -> Self {
        Polygon {
            x,
            y,
            z,
            mat
        }
    }

    fn calc_normal(&self) -> Vector3<f64> {
        let v1 = self.x-self.y;
        let v2 = self.z-self.y;
        normalize(&cross(&v1, &v2))
    }
}

impl RayTrace for Polygon {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let epsilon = 0.000000001;
        let w0: Vector3<f64> = ray.origin() - self.y;
        let num = -(dot(&self.calc_normal(), &w0));
        let den = dot(&self.calc_normal(), &ray.dir());

        if den.abs() < epsilon {
            return None;
        }

        let intersect_dist = num / den;

        if intersect_dist >= 0.0 {
            let point = ray.origin() + ray.dir() * intersect_dist;

            let u = self.x-self.y;
            let v = self.z-self.y;

            let uu = dot(&u, &u);
            let uv = dot(&u, &v);
            let vv = dot(&v, &v);
            let w = point - self.y;
            let wu = dot(&w, &u);
            let wv = dot(&w, &v);
            let den = (uv * uv) - (uu * vv);

            let s = ((uv * wv) - (vv * wu)) / den;

            if s < 0.0 || s > 1.0 {
                return None;
            }

            let t = ((uv * wu) - (uu * wv)) / den;
            if t < 0.0 || (s+t) > 1.0 {
                return None;
            }

            let norm = self.calc_normal();
            return Some(Hit::new(0.0, point, norm));
        }

        None
    }

    fn get_mat(&self) -> &Material {
        &self.mat
    }
}
