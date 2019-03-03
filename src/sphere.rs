use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;
use crate::boundingbox::BoundingBox;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let d = b * b - a * c;

        if d > 0.0 {
            // first solution
            let t = (-b - d.sqrt()) / a;
            if t < t_max && t > t_min {
                let n = Vec3::div_s(&(r.point(t) - self.center), self.radius);
                let (u, v) = sphere_uv(&n);
                Some(HitRecord {
                    t,
                    p: r.point(t),
                    n,
                    u,
                    v,
                    material: &self.material
                })
            }
            else {
                // second solution
                let t = (-b + d.sqrt()) / a;
                if t < t_max && t > t_min {
                    let n = Vec3::div_s(&(r.point(t) - self.center), self.radius);
                    let (u, v) = sphere_uv(&n);
                    Some(HitRecord {
                        t,
                        p: r.point(t),
                        n,
                        u,
                        v,
                        material: &self.material,
                    })
                }
                else {
                    None
                }
            }
        }
        else {
            None
        }
    }

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

fn sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
    let v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;
    (u, v)
}