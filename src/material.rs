use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

use rand::Rng;

pub trait Material: core::fmt::Debug {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);
        if p.square_len() < 1.0 {
            return p;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&r_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * n
}
