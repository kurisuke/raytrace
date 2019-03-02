use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

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
                Some(HitRecord {
                    t,
                    p: r.point(t),
                    n: Vec3::div_s(&(r.point(t) - self.center), self.radius),
                    material: &self.material
                })
            }
            else {
                // second solution
                let t = (-b + d.sqrt()) / a;
                if t < t_max && t > t_min {
                    Some(HitRecord {
                        t,
                        p: r.point(t),
                        n: Vec3::div_s(&(r.point(t) - self.center), self.radius),
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
}
