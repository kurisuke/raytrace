use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(&vup, &w).normalize();
        let v = Vec3::cross(&w, &u);

        Camera {
            origin: look_from,
            lower_left: look_from - Vec3::mul_s(&u, half_width) - Vec3::mul_s(&v, half_height) - w,
            horizontal: Vec3::mul_s(&u, 2.0 * half_width),
            vertical: Vec3::mul_s(&v, 2.0 * half_height),
        }
    }

    pub fn default(aspect: f64) -> Camera {
        Camera::new(Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, -1.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    90.0, aspect)
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + Vec3::mul_s(&self.horizontal, s)
                + Vec3::mul_s(&self.vertical, t) - self.origin,
        }
    }
}
