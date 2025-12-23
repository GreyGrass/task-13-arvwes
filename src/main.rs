mod ray;
mod scene;
mod shapes;
mod vector3;
use crate::ray::Ray;
use crate::scene::{Camera, World};
use crate::shapes::{Material, Sphere};
use crate::vector3::Vec3;
use rand::{self, Rng};

use std::f64::INFINITY;
use std::fs::File;
use std::io::Write;

fn color(ray: Ray, hitables: &World, depth: usize) -> Vec3 {
    let possible_hit = hitables.hit_anything(0.001, INFINITY, &ray);
    if let Some(rec) = possible_hit {
        let possible_scatter = rec.material.scatter(&ray, &rec);
        if depth < 50
            && let Some(scatter) = possible_scatter
        {
            return scatter.0 * color(scatter.1, &hitables, depth + 1);
        } else {
            return Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
    } else {
        let u_direction = ray.direction().unit_vector();
        let t: f64 = 0.5 * (u_direction.y + 1.0);
        let start_value = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let end_value = Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };
        start_value * (1.0 - t) + end_value * t
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;

    let nx = 200;
    let ny = 100;
    let ns = 100;
    let nxf = nx as f64;
    let nyf = ny as f64;
    let nsf = ns as f64;
    writeln!(&mut file, "P3\n{} {}\n255", nx, ny)?;
    let list: Vec<Sphere> = vec![
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: Material::Lambertian {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.3,
                    z: 0.3,
                },
            },
        },
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            material: Material::Lambertian {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.8,
                    z: 0.0,
                },
            },
        },
        Sphere {
            center: Vec3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: Material::Metal {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.6,
                    z: 0.2,
                },
                fuzz: 0.0,
            },
        },
        Sphere {
            center: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
            material: Material::Dielectric { ref_idx: 1.5 },
        },
        Sphere {
            center: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.45,
            material: Material::Dielectric { ref_idx: 1.5 },
        },
    ];
    let world = World { hitables: list };
    let camera = Camera::new(
        Vec3 {
            x: -2.0,
            y: 2.0,
            z: 1.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        45.0,
        nxf / nyf,
    );
    let mut rng = rand::rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _s in 0..ns {
                let jfl = j as f64;
                let ifl = i as f64;

                let randfl: f64 = rng.random();
                let u = (ifl + randfl) / nxf;
                let v = (jfl + randfl) / nyf;

                let ray = camera.get_ray(u, v);
                let _p = ray.point_at_parameter(2.0);
                col += color(ray, &world, 1);
            }
            col /= nsf;
            col = Vec3 {
                x: col.x.sqrt(),
                y: col.y.sqrt(),
                z: col.z.sqrt(),
            };
            let ir: u8 = (255.99 * col.x) as u8;
            let ig: u8 = (255.99 * col.y) as u8;
            let ib: u8 = (255.99 * col.z) as u8;
            writeln!(&mut file, "{} {} {}", ir, ig, ib);
        }
    }
    Ok(())
}
