#![forbid(unsafe_code)]

extern crate rand;

mod camera;
mod hittable;
mod ray;
mod vec;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use rand::Rng;
use ray::Ray;
use vec::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    let camera = Camera::default();
    let mut rng = rand::thread_rng();

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                // let p = r.point_at_parameter(2.0);
                c += color(r, &world);
            }
            c /= ns as f32;

            let ir = (255.99 * c.x) as i32;
            let ig = (255.99 * c.y) as i32;
            let ib = (255.99 * c.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color<T: Hittable>(r: Ray, world: &T) -> Vec3 {
    if let Some(hit) = world.hit(&r, 0.0, f32::MAX) {
        0.5 * Vec3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0)
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
