use nalgebra::{Point3, Vector3, normalize};

use crate::{
    material::*,
    rays::{ray::*, hit::*}
};
use crate::renderable::RayTrace;

pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Point3<f32>,
               radius: f32,
               mat: Material) -> Self {
        Sphere {
            center,
            radius,
            mat
        }
    }
}

impl RayTrace for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let (x_o, y_o, z_o) : (f32,f32,f32) = ray.origin_as_tuple();
        let (x_d, y_d, z_d) : (f32,f32,f32) = ray.dir_as_tuple();
        let (x_c, y_c, z_c) : (f32,f32,f32) = {
            (self.center.x, self.center.y, self.center.z)
        };

        let r = self.radius;

        let b = 2.0 * (x_d*x_o - x_d*x_c + y_d*y_o - y_d*y_c + z_d*z_o - z_d*z_c);
        let c = x_o.powi(2) - 2.0*x_o*x_c + x_c.powi(2)
            + y_o.powi(2) - 2.0*y_o*y_c + y_c.powi(2)
            + z_o.powi(2) - 2.0*z_o*z_c + z_c.powi(2) - r.powi(2);

        let discriminant = b.powi(2) - 4.0* c;

        let mut did_hit: bool = false;
        let mut t: f32 = 0.0;
        let mut point: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

        if discriminant >= 0.0 {
            let t_0 = (-b - discriminant.sqrt()) / 2.0;

            if t_0 <= 0.0 {
                let t_1 = (-b + discriminant.sqrt()) / 2.0;
                if t_1 <= 0.0 {
                    return None
                } else {
                    did_hit = true;
                    t = t_1;
                }
            }

            let ray_o = ray.origin();
            let ray_d = ray.dir();
            point = ray_o + ray_d * t_0;

        } else {
            did_hit = true;
        }

        let norm: Vector3<f32> = normalize(point - self.center);

        Some(Hit::new(
            did_hit,
            t,
            point,
            norm,
            self.mat.diffuse_color
        ))
    }
}
