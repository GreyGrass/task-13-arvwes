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
    pub fn new() -> Self {
        Camera {
            lower_left_corner: Vec3 {
                x: -2.0,
                y: -1.0,
                z: -1.0,
            },
            vertical: Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            horizontal: Vec3 {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
    pub fn get_ray(&self, u:f64, v:f64) -> Ray{
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        Ray { a: self.origin, b: direction }
    }
}
