#![forbid(unsafe_code)]

extern crate rand;
extern crate rayon;
extern crate time;

mod camera;
mod format;
mod hittable;
mod material;
mod ray;
mod vec;

use crate::camera::Camera;
use crate::format::{Format, Ppm};
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material::{Dialectric, Lambertian, Metal};
use crate::rand::Rng;
use crate::ray::Ray;
use crate::vec::{Color, Point3, Vec3};
use rayon::prelude::*;
use std::sync::Arc;
use time::OffsetDateTime;

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = 360;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 50;
    let mut image = Ppm::new(IMAGE_WIDTH, IMAGE_HEIGHT);

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

    let start_time = OffsetDateTime::now_local().unwrap();
    for j in (0..IMAGE_HEIGHT).rev() {
        let it_start_time = OffsetDateTime::now_local().unwrap();

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::thread_rng();

                let mut c = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f32 + rng.gen::<f32>()) / IMAGE_WIDTH as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / IMAGE_HEIGHT as f32;
                    let r = camera.get_ray(u, v);
                    // let p = r.point_at_parameter(2.0);
                    c += color(r, &world, MAX_DEPTH);
                }
                c /= SAMPLES_PER_PIXEL as f32;
                Color::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt())
            })
            .collect();
        for (i, pixel) in scanline.into_iter().enumerate() {
            image.set_pixel(i as u32, j, pixel);
        }

        let curr_time = OffsetDateTime::now_local().unwrap();
        let last_it_elapsed = curr_time - it_start_time;
        let elapsed = curr_time - start_time;
        let time_per_iteration = elapsed / (IMAGE_HEIGHT - j);
        let est_time_remaining = time_per_iteration * j;
        let est_time_of_completion = curr_time + time_per_iteration * IMAGE_HEIGHT;
        eprintln!(
            "Rendered scanline {} of {}\n\
            \tlast line: {}.{:0>3}s\n\
            \ttime/line: {}.{:0>3}s\n\
            \telapsed:   {}:{:0>2}:{:0>2}\n\
            \tremaining: {}:{:0>2}:{:0>2}\n\
            \tETA:       {}",
            IMAGE_HEIGHT - j,
            IMAGE_HEIGHT,
            last_it_elapsed.whole_seconds(),
            last_it_elapsed.whole_milliseconds() % 1000,
            time_per_iteration.whole_seconds(),
            time_per_iteration.whole_milliseconds() % 1000,
            elapsed.whole_hours(),
            elapsed.whole_minutes() % 60,
            elapsed.whole_seconds() % 60,
            est_time_remaining.whole_hours(),
            est_time_remaining.whole_minutes() % 60,
            est_time_remaining.whole_seconds() % 60,
            est_time_of_completion,
        );
    }

    image.save("image").expect("Unable to save image");
}

fn color(r: Ray, world: &dyn Hittable, depth: u32) -> Color {
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
            Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(Dialectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
        )),
        Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5))),
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
                        Arc::new(Lambertian::new(Color::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(Color::new(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        ))),
                    )));
                } else {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dialectric::new(1.5)),
                    )))
                }
            }
        }
    }

    HittableList::new(list)
}
