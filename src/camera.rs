use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(&vup, &w).normalize();
        let v = Vec3::cross(&w, &u);

        Camera {
            origin: look_from,
            lower_left: look_from
                - Vec3::mul_s(&u, half_width * focus_dist)
                - Vec3::mul_s(&v, half_height * focus_dist)
                - Vec3::mul_s(&w, focus_dist),
            horizontal: Vec3::mul_s(&u, 2.0 * half_width * focus_dist),
            vertical: Vec3::mul_s(&v, 2.0 * half_height * focus_dist),
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn default(aspect: f64) -> Camera {
        Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            aspect,
            2.0,
            1.0,
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::mul_s(&random_in_unit_disk(), self.lens_radius);
        let offset = Vec3::mul_s(&self.u, rd.x()) + Vec3::mul_s(&self.v, rd.y());
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left
                + Vec3::mul_s(&self.horizontal, s)
                + Vec3::mul_s(&self.vertical, t)
                - self.origin
                - offset,
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        if Vec3::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}
