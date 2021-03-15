use nalgebra::{Point3};
use crate::{
    material::*,
    rays::{ray::*, hit::*}
};

pub struct Polygon {
    x: Point3<f32>,
    y: Point3<f32>,
    z: Point3<f32>,
    mat: Material,
}

impl Polygon {
    pub fn new(x: Point3<f32>,
               y: Point3<f32>,
               z: Point3<f32>,
               mat: Material) -> Self {
        Polygon {
            x,
            y,
            z,
            mat
        }
    }
}
