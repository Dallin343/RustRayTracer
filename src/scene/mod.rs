use crate::renderable::{Object, RayTrace};
use nalgebra::{Vector3, Point3, approx_eq, dot, clamp, normalize, abs};
use crate::rays::{ray::{Ray, RayType}, hit::Hit};

pub mod light;
pub mod camera;

pub struct Scene {
    camera: camera::Camera,
    lights: Vec<light::Light>,
    ambient: Vector3<f64>,
    background: Vector3<f64>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn new(camera: camera::Camera,
               lights: Vec<light::Light>,
               ambient: Vector3<f64>,
               background: Vector3<f64>,
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
    pub fn render(&self, screen_width: usize, screen_height: usize) -> Vec<Vec<Vector3<f64>>> {
        let mut image = vec![vec![Vector3::<f64> {x:0.0, y: 0.0, z: 0.0}; screen_width]; screen_height];

        let from = self.camera.look_from;
        let cam_pos = Point3::new(from.x, from.y, from.z);

        for col in 0..screen_width {
            for row in 0..screen_height {
                let dir = self.get_world_space_coord(col, row, screen_width, screen_height);

                let ray = Ray::new(cam_pos, normalize(&(dir - cam_pos)), RayType::Camera);
                let color = self.trace_ray(&ray);
                image[row][col] = color;
            }
        }
        return image;
    }

    fn get_world_space_coord(&self, i: usize, j: usize, width: usize, height: usize) -> Point3<f64> {
        let viewport = self.camera.viewport_size();
        let i_step = (viewport * 2.0) / width as f64;
        let j_step = (viewport * 2.0) / height as f64;
        let u = i as f64 * i_step + (i_step / 2.0) - viewport;
        let v = j as f64 * j_step + (j_step / 2.0) - viewport;

        Point3::new(u, v, 0.0)
    }

    fn trace_ray(&self, ray: &Ray) -> Vector3<f64> {
        match ray.ray_type {
            RayType::Camera => self.trace_camera_ray(ray, 0),
            RayType::Illumination => Vector3::new(0.0, 0.0, 0.0)
        }
    }

    fn in_shadow(&self, ray: &Ray) -> bool {
        for obj in &self.objects {
            match obj.intersect(ray) {
                None => (),
                Some(_) => return true,
            }
        }
        false
    }

    fn trace_camera_ray(&self, ray: &Ray, depth: u8) -> Vector3<f64> {
        let hits: Vec<(&Object, Hit)> = {
            let mut v: Vec<(&Object, Hit)> = Vec::new();
            for obj in &self.objects {
                match obj.intersect(ray) {
                    None => (),
                    Some(h) => v.push((obj, h))
                }
            }
            v
        };

        let closest_hit: Option<(&Object, Hit)> = hits.into_iter().fold(None, |min, (obj_x, hit_x)| match min {
            None => Some((obj_x, hit_x)),
            Some((obj_y, hit_y)) => Some(if hit_x.distance_to(ray) < hit_y.distance_to(ray) {(obj_x,hit_x)} else {(obj_y,hit_y)})
        });


        let (hit_object, hit): (&Object, Hit) = match closest_hit {
            None => return self.background,
            Some(hit) => hit
        };

        let mat = hit_object.get_mat();

        let mut final_color: Vector3<f64> = Vector3::new(0.0,0.0,0.0);

        let light = &self.lights[0];
        let light_dir = light.direction_to;
        let e = 0.000001;
        let adj_origin = hit.point + (light_dir*e);
        let illumination_ray = Ray::new(adj_origin, light_dir, RayType::Illumination);
        match self.in_shadow(&illumination_ray) {
            true => (),
            false => {
                let reflection: Vector3<f64> = normalize(&(2.0*hit.norm*dot(&hit.norm, &light_dir)-light_dir));

                let mut angle: f64 = dot(&hit.norm, &light_dir).abs();
                let diffuse = mat.diffuse_fac *
                    mat.diffuse_color;
                    angle.max(0.0);


                let v = Point3::new(self.camera.look_from.x,self.camera.look_from.y,self.camera.look_from.z);
                // let v: Point3<f64> = hit.point - self.camera.look_from;
                let v: Vector3<f64> = v - hit.point;
                angle = (dot(&ray.dir(), &reflection)).abs();

                let specular = mat.specular_fac *
                     mat.specular_color *
                     angle.max(0.0).powf(mat.gloss);


                final_color = light.color * diffuse + light.color * specular;
            }
        };

        let ambient = mat.ambient_fac * self.ambient * mat.diffuse_color;
        final_color += ambient;

        let (x, y, z) = clamp((final_color.x, final_color.y, final_color.z), (0.0, 0.0, 0.0), (1.0, 1.0, 1.0));

        return Vector3::new(x,y,z);
    }
}
