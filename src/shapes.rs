use crate::{ray::Ray, vector3::Vec3};

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let off_center = ray.origin() - self.center;
        //dot((A​ + t*B ​- C)​ ,(A​ + t*B​ - C​)) = R*R
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&off_center);
        let c = off_center.dot(&off_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if temp < tmax && temp > tmin {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                });
            }
            let temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp < tmax && temp > tmin {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                });
            }
        }
        None
    }


}
