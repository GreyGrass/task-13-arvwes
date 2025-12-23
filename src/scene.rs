use std::f64::consts::PI;

use crate::{
    ray::Ray,
    shapes::{HitRecord, Hitable, Sphere}, vector3::Vec3,
};

pub struct World {
    pub hitables: Vec<Sphere>,
}

impl World {
    pub fn hit_anything(&self, tmin: f64, tmax: f64, ray: &Ray) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_sofar = tmax;
        for object in &self.hitables {
            let rec = object.hit(ray, tmin, closest_sofar);
            if let Some(hit) = rec {
                closest_sofar = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
}

pub struct Camera {
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from:Vec3, look_at:Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let theta = vfov*PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        
        Camera {
            lower_left_corner: look_from - u*half_width - v*half_height - w,
            vertical: v*2.0*half_height,
            horizontal: u*2.0*half_width,
            origin: look_from,
        }
    }
    pub fn get_ray(&self, u:f64, v:f64) -> Ray{
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray { a: self.origin, b: direction }
    }
}
