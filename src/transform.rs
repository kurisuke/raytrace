use crate::boundingbox::BoundingBox;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::collections::Bound;

pub struct Translate {
    pub offset: Vec3,
    pub h: Box<dyn Hitable>,
}

impl Translate {
    pub fn new(h: Box<dyn Hitable>, offset: Vec3) -> Translate {
        Translate { h, offset }
    }
}

impl Hitable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let r_moved = Ray::new(r.origin - self.offset, r.direction);
        if let Some(mut rec) = self.h.hit(&r_moved, t_min, t_max) {
            rec.p += self.offset;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        if let Some(mut bbox) = self.h.bounding_box() {
            Some(BoundingBox {
                min: bbox.min + self.offset,
                max: bbox.max + self.offset,
            })
        } else {
            None
        }
    }
}

pub struct RotateY {
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<BoundingBox>,
    h: Box<dyn Hitable>,
}

impl RotateY {
    pub fn new(h: Box<dyn Hitable>, angle: f64) -> RotateY {
        let radians = std::f64::consts::PI / 180.0 * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = match h.bounding_box() {
            Some(bbox) => {
                let mut min = Vec3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
                let mut max = Vec3::new(-std::f64::MAX, -std::f64::MAX, -std::f64::MAX);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f64 * bbox.max.x() + (1 - i) as f64 * bbox.min.x();
                            let y = j as f64 * bbox.max.y() + (1 - j) as f64 * bbox.min.y();
                            let z = k as f64 * bbox.max.z() + (1 - k) as f64 * bbox.min.z();
                            let x_new = cos_theta * x + sin_theta * z;
                            let z_new = -sin_theta * x + cos_theta * z;
                            let tester = Vec3::new(x_new, y, z_new);
                            for c in 0..3 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }
                Some(BoundingBox { min, max })
            }
            None => None,
        };
        RotateY {
            sin_theta,
            cos_theta,
            bbox,
            h,
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Vec3::new(
            self.cos_theta * r.origin.x() - self.sin_theta * r.origin.z(),
            r.origin.y(),
            self.sin_theta * r.origin.x() + self.cos_theta * r.origin.z(),
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x() - self.sin_theta * r.direction.z(),
            r.direction.y(),
            self.sin_theta * r.direction.x() + self.cos_theta * r.direction.z(),
        );
        let r_rotated = Ray::new(origin, direction);
        if let Some(mut rec) = self.h.hit(&r_rotated, t_min, t_max) {
            let p = Vec3::new(
                self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
                rec.p.y(),
                -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z(),
            );
            let n = Vec3::new(
                self.cos_theta * rec.n.x() + self.sin_theta * rec.n.z(),
                rec.n.y(),
                -self.sin_theta * rec.n.x() + self.cos_theta * rec.n.z(),
            );
            rec.p = p;
            rec.n = n;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        self.bbox.clone()
    }
}
