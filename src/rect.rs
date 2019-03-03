use crate::boundingbox::BoundingBox;
use crate::material::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct XYRect {
    pub x: (f64, f64),
    pub y: (f64, f64),
    pub k: f64,
    pub material: Material,
}

impl XYRect {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z()) / r.direction.z();
        if t < t_min || t > t_max {
            None
        } else {
            let x = r.origin.x() + t * r.direction.x();
            let y = r.origin.y() + t * r.direction.y();
            if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
                None
            } else {
                Some(HitRecord {
                    u: (x - self.x.0) / (self.x.1 - self.x.0),
                    v: (y - self.y.0) / (self.y.1 - self.y.0),
                    t,
                    material: &self.material,
                    p: r.point(t),
                    n: Vec3::new(0.0, 0.0, 1.0),
                })
            }
        }
    }

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox {
            min: Vec3::new(self.x.0, self.y.0, self.k-0.0001),
            max: Vec3::new(self.x.1, self.y.1, self.k+0.0001),
        })
    }
}