use crate::{ray::Ray, vector3::Vec3};
use rand::{self, Rng};
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
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
                    material: self.material.clone(),
                });
            }
            let temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp < tmax && temp > tmin {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ref_idx: f64 },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let target = rec.p + rec.normal + random_in_unit_sphere();
                let scattered = Ray {
                    a: rec.p,
                    b: target,
                };
                let hold = (*albedo, scattered);
                return Some(hold);
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(ray_in.direction().unit_vector(), rec.normal);
                let scattered = Ray {
                    a: rec.p,
                    b: reflected + random_in_unit_sphere() * *fuzz,
                };
                if scattered.direction().dot(&rec.normal) < 0.0 {
                    return None;
                } else {
                    let hold = (*albedo, scattered);
                    return Some(hold);
                }
            }
            Material::Dielectric { ref_idx } => {
                let mut rng = rand::rng();
                let outward_normal: Vec3;
                let reflected = reflect(ray_in.direction(), rec.normal);
                let mut refracted: Vec3  = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
                let scattered: Ray;
                let in_over_tn: f64;
                let attenuation = Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                };
                let reflect_prob: f64;
                let cosine: f64;
                if ray_in.direction().dot(&rec.normal) > 0.0 {
                    outward_normal = rec.normal * -1.0;
                    in_over_tn = *ref_idx;
                    cosine = *ref_idx * ray_in.direction().dot(&rec.normal)
                        / ray_in.direction().length();
                } else {
                    outward_normal = rec.normal;
                    in_over_tn = 1.0 / ref_idx;
                    cosine =
                        -1.0 * ray_in.direction().dot(&rec.normal) / ray_in.direction().length();
                }

                if let Some(refra) = refract(ray_in.direction(), outward_normal, in_over_tn) {
                    refracted = refra;
                    reflect_prob = schlick(cosine, *ref_idx);
                    
                } else {
                    reflect_prob = 1.0;
                }
                let frand: f64 = rng.random();
                if frand < reflect_prob {
                    scattered = Ray{a: rec.p, b: reflected};
                }else{
                    scattered = Ray{a: rec.p, b: refracted};
                }
                let result = (attenuation, scattered);
                return Some(result);
            }
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(&n)
}

fn refract(v: Vec3, n: Vec3, in_over_tn: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - in_over_tn * in_over_tn * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some((uv - n * dt) * in_over_tn - n * discriminant.sqrt());
    } else {
        return None;
    }
}
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::rng();
    loop {
        let p = Vec3 {
            x: rng.random_range(-1.0..1.0) as f64,
            y: rng.random_range(-1.0..1.0) as f64,
            z: rng.random_range(-1.0..1.0) as f64,
        };
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}
