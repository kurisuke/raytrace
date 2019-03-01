use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + Vec3::mul_s(self.horizontal, u)
                + Vec3::mul_s(self.vertical, v)
        }
    }
}
