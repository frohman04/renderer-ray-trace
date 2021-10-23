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

#[derive(Debug, Copy, Clone)]
pub struct Dialectric {
    ref_idx: f32,
}

impl Dialectric {
    pub fn new(ref_idx: f32) -> Dialectric {
        Dialectric { ref_idx }
    }
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut rng = rand::thread_rng();

        let reflected = reflect(&r_in.direction, &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(&rec.normal) > 0.0 {
            (
                -1.0 * rec.normal,
                self.ref_idx,
                self.ref_idx * r_in.direction.dot(&rec.normal) / r_in.direction.len(),
            )
        } else {
            (
                rec.normal,
                1.0 / self.ref_idx,
                -1.0 * r_in.direction.dot(&rec.normal) / r_in.direction.len(),
            )
        };
        let refracted = refract(&r_in.direction, &outward_normal, ni_over_nt);
        let reflect_prob = if refracted.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };
        if rng.gen::<f32>() < reflect_prob {
            Some((Ray::new(rec.p, reflected), attenuation))
        } else {
            Some((Ray::new(rec.p, refracted.unwrap()), attenuation))
        }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0_2 = r0 * r0;
    r0_2 + (1.0 - r0_2) * (1.0 - cosine).powi(5)
}
