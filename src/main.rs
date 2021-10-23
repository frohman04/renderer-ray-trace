#![forbid(unsafe_code)]

extern crate rand;

mod camera;
mod hittable;
mod material;
mod ray;
mod vec;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use material::{Dialectric, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use std::rc::Rc;
use vec::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let world = HittableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2))),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::new(Dialectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Rc::new(Dialectric::new(1.5)),
        )),
    ]);
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
    );
    let mut rng = rand::thread_rng();

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                // let p = r.point_at_parameter(2.0);
                c += color(r, &world, 0);
            }
            c /= ns as f32;
            c = Vec3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());

            let ir = (255.99 * c.x) as i32;
            let ig = (255.99 * c.y) as i32;
            let ib = (255.99 * c.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(&r, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&r, &hit) {
                attenuation * color(scattered, world, depth + 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
