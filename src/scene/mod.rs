use crate::renderable::Object;
use nalgebra::{Vector3, distance};
use crate::rays::{ray::Ray, hit::Hit};
use std::intrinsics::discriminant_value;

pub mod light;
pub mod camera;

pub struct Scene {
    camera: camera::Camera,
    lights: Vec<light::Light>,
    ambient: Vector3<f32>,
    background: Vector3<f32>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn new(camera: camera::Camera,
               lights: Vec<light::Light>,
               ambient: Vector3<f32>,
               background: Vector3<f32>,
               objects: Vec<Object>) -> Self {
        Scene {
            camera,
            lights,
            ambient,
            background,
            objects
        }
    }
}

impl Scene {
    pub fn trace_ray(&self, ray: &Ray, depth: u8) -> Vector3<f32> {
        let mut closest_hit: Hit;

        for obj in &self.objects {
            if closest_hit == None || closest_hit.did_hit == false {
                closest_hit = obj.intersect(ray);
                continue;
            }

            let new_test = obj.intersect(ray);
            if new_test.did_hit && distance(new_test.point, &ray.origin()) <
                    distance(&closest_hit.point, &ray.origin()) {
                closest_hit = new_test
            }
        }

        if closest_hit == None || closest_hit.did_hit == false {
            return self.background
        }
        return self.ambient
    }
}
