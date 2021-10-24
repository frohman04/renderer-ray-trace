use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Point3, normal: Vec3, material: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                temp_rec = Some(hit)
            }
        }
        temp_rec
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.point_at_parameter(temp);
                Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    self.material.clone(),
                ))
            } else {
                let temp = (-b + discriminant.sqrt()) / a;
                if t_min < temp && temp < t_max {
                    let p = r.point_at_parameter(temp);
                    Some(HitRecord::new(
                        temp,
                        p,
                        (p - self.center) / self.radius,
                        self.material.clone(),
                    ))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
