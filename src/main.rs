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
use vec::{Color, Point3, Vec3};

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_HEIGHT: i32 = 720;
    const IMAGE_WIDTH: i32 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let world = random_scene();

    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );
    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut c = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / IMAGE_WIDTH as f32;
                let v = (j as f32 + rng.gen::<f32>()) / IMAGE_HEIGHT as f32;
                let r = camera.get_ray(u, v);
                // let p = r.point_at_parameter(2.0);
                c += color(r, &world, MAX_DEPTH);
            }
            c /= SAMPLES_PER_PIXEL as f32;
            c = Color::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());

            let ir = (255.99 * c.x) as i32;
            let ig = (255.99 * c.y) as i32;
            let ib = (255.99 * c.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    if let Some(hit) = world.hit(&r, 0.001, f32::MAX) {
        if depth > 0 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&r, &hit) {
                attenuation * color(scattered, world, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut list: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Rc::new(Dialectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
        )),
        Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5))),
        )),
    ];
    let cmp = Vec3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Point3::new(
                a as f32 + 0.9f32 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9f32 * rng.gen::<f32>(),
            );
            if (center - cmp).len() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Color::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(Color::new(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        ))),
                    )));
                } else {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dialectric::new(1.5)),
                    )))
                }
            }
        }
    }

    HittableList::new(list)
}
