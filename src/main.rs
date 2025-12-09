mod ray;
mod vector3;
use crate::ray::Ray;
use crate::vector3::Vec3;

use std::fs::File;
use std::io::Write;

fn color(ray: Ray) -> Vec3 {
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

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;

    let nx = 200;
    let ny = 100;
    writeln!(&mut file, "P3\n{} {}\n255", nx, ny)?;
    let lower_left_corner = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let jfl = j as f64;
            let ifl = i as f64;
            let nxf = nx as f64;
            let nyf = ny as f64;
            let u = ifl / nxf;
            let v = jfl / nyf;
            let direction = lower_left_corner + horizontal*u + vertical*v;
            let ray = Ray{a: origin, b:direction};
            let color = color(ray);
            let ir: u8 = (255.99 * color.x) as u8;
            let ig: u8 = (255.99 * color.y) as u8;
            let ib: u8 = (255.99 * color.z) as u8;
            writeln!(&mut file, "{} {} {}", ir, ig, ib);
        }
    }
    Ok(())
}
