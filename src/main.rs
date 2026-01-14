mod ray;
mod scene;
mod shapes;
mod vector3;
use crate::ray::Ray;
use crate::scene::{Camera, World};
use crate::shapes::{Material, Sphere};
use crate::vector3::Vec3;
use rand::{self, Rng, rand_core};

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

    let nx = 1280;
    let ny = 720;
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
                    x: 0.3,
                    y: 0.3,
                    z: 0.8,
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
    //let world = World { hitables: list };
    let world = World { hitables: random_scene() };
    let lookfrom = Vec3 {
        x: 7.0,
        y: 1.6,
        z: 2.2,
    };
    let lookat = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        40.0,
        nxf as f64 / nyf as f64,
        aperture,
        dist_to_focus,
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

                let u_rand: f64 = rng.random();
                let v_rand: f64 = rng.random();

                let u = (ifl + u_rand) / nxf;
                let v = (jfl + v_rand) / nyf;

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

fn random_scene() -> Vec<Sphere> {
    let n = 500;
    let mut list: Vec<Sphere> = Vec::new();
    let mut rng = rand::rng();
    list.push(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: -1.0,
        },
        radius: 1000.0,
        material: Material::Lambertian {
            albedo: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        },
    });

    let i = 1;
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.random();
            let randx: f64 = rng.random();
            let randy: f64 = rng.random();
            let randz: f64 = rng.random();
            let center = Vec3 {
                x: a as f64 + 0.9 * randx,
                y: 0.2,
                z: b as f64 + 0.9 * randz,
            };
            if (center
                - Vec3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .length()
                > 0.9
            {
                if choose_mat < 0.8 {
                    //diffuse
                    list.push(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Material::Lambertian {
                            albedo: Vec3 {
                                x: rng.random::<f64>() * rng.random::<f64>(),
                                y: rng.random::<f64>() * rng.random::<f64>(),
                                z: rng.random::<f64>() * rng.random::<f64>(),
                            },
                        },
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    list.push(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Material::Metal {
                            albedo: Vec3 {
                                x: 0.5 * (1.0 + rng.random::<f64>()),
                                y: 0.5 * (1.0 + rng.random::<f64>()),
                                z: 0.5 * (1.0 + rng.random::<f64>()),
                            },
                            fuzz: 0.5 * rng.random::<f64>(),
                        },
                    });
                } else {
                    list.push(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Material::Dielectric { ref_idx: 1.5 },
                    });
                }
            }
        }
    }
    list.push(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::Dielectric { ref_idx: 1.5 },
    });
    list.push(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::Lambertian {
            albedo: Vec3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        },
    });
    list.push(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::Metal {
            albedo: Vec3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        },
    });

    return list;
}
