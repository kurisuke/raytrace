use crate::boundingbox::BoundingBox;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;

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
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let r_moved = Ray::new(r.origin - self.offset, r.direction);
        if let Some(mut rec) = self.h.hit(&r_moved, t_min, t_max) {
            rec.p += self.offset;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        if let Some(bbox) = self.h.bounding_box() {
            Some(BoundingBox {
                min: bbox.min + self.offset,
                max: bbox.max + self.offset,
            })
        } else {
            None
        }
    }
}

struct RotationMatrix {
    pub r: [Vec3; 3],
}

impl RotationMatrix {
    pub fn rotate(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            Vec3::dot(v, self.r[0]),
            Vec3::dot(v, self.r[1]),
            Vec3::dot(v, self.r[2]),
        )
    }

    pub fn invert(&self) -> RotationMatrix {
        RotationMatrix {
            r: [
                Vec3::new(self.r[0].x(), self.r[1].x(), self.r[2].x()),
                Vec3::new(self.r[0].y(), self.r[1].y(), self.r[2].y()),
                Vec3::new(self.r[0].z(), self.r[1].z(), self.r[2].z()),
            ],
        }
    }
}

pub struct RotateXYZ {
    rot_matrix: RotationMatrix,
    inv_rot_matrix: RotationMatrix,
    bbox: Option<BoundingBox>,
    h: Box<dyn Hitable>,
}

impl RotateXYZ {
    pub fn new(h: Box<dyn Hitable>, angles: Vec3) -> RotateXYZ {
        let radians = std::f32::consts::PI / 180.0 * angles;
        let sin_phi = radians.x().sin();
        let cos_phi = radians.x().cos();
        let sin_theta = radians.y().sin();
        let cos_theta = radians.y().cos();
        let sin_psi = radians.z().sin();
        let cos_psi = radians.z().cos();

        let rot_matrix = RotationMatrix {
            r: [
                Vec3::new(
                    cos_theta * cos_psi,
                    -cos_phi * sin_psi + sin_phi * sin_theta * cos_psi,
                    sin_phi * sin_psi + cos_phi * sin_theta * cos_psi,
                ),
                Vec3::new(
                    cos_theta * sin_psi,
                    cos_phi * cos_psi + sin_phi * sin_theta * sin_psi,
                    -sin_phi * cos_psi + cos_phi * sin_theta * sin_psi,
                ),
                Vec3::new(-sin_theta, sin_phi * cos_theta, cos_phi * cos_theta),
            ],
        };
        let inv_rot_matrix = rot_matrix.invert();

        let bbox = match h.bounding_box() {
            Some(bbox) => {
                let mut min = Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
                let mut max = Vec3::new(-std::f32::MAX, -std::f32::MAX, -std::f32::MAX);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let v = Vec3::new(
                                i as f32 * bbox.max.x() + (1 - i) as f32 * bbox.min.x(),
                                j as f32 * bbox.max.y() + (1 - j) as f32 * bbox.min.y(),
                                k as f32 * bbox.max.z() + (1 - k) as f32 * bbox.min.z(),
                            );

                            let tester = rot_matrix.rotate(v);

                            for c in 0..3 {
                                if tester.i(c) > max.i(c) {
                                    max.set_i(c, tester.i(c));
                                }
                                if tester.i(c) < min.i(c) {
                                    min.set_i(c, tester.i(c));
                                }
                            }
                        }
                    }
                }
                Some(BoundingBox { min, max })
            }
            None => None,
        };

        RotateXYZ {
            rot_matrix,
            inv_rot_matrix,
            bbox,
            h,
        }
    }
}

impl Hitable for RotateXYZ {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let r_inv_rotated = Ray::new(
            self.inv_rot_matrix.rotate(r.origin),
            self.inv_rot_matrix.rotate(r.direction),
        );

        if let Some(mut rec) = self.h.hit(&r_inv_rotated, t_min, t_max) {
            rec.p = self.rot_matrix.rotate(rec.p);
            rec.n = self.rot_matrix.rotate(rec.n);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        self.bbox.clone()
    }
}
