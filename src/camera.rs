use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-2.0, -1.0, -1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
        )
    }
}
