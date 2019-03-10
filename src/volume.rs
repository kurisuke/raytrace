use crate::boundingbox::BoundingBox;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

use rand::Rng;

pub struct ConstantMedium {
    pub boundary: Box<Hitable>,
    pub density: f32,
    pub phase_function: Box<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hitable>, density: f32, albedo: Texture) -> ConstantMedium {
        ConstantMedium {
            boundary,
            density,
            phase_function: Box::new(Material::Isotropic { albedo }),
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // check if our ray hits the boundary (e.g. enters the volume)
        let opt_rec1 = self.boundary.hit(r, -std::f32::MAX, std::f32::MAX);
        if let Some(mut rec1) = opt_rec1 {
            // check if there is a second point where the ray exits the volume
            let opt_rec2 = self.boundary.hit(r, rec1.t + 0.0001, std::f32::MAX);
            if let Some(mut rec2) = opt_rec2 {
                // limit entry / exit point to our range (t_min .. t_max)
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);
                if rec1.t >= rec2.t {
                    None
                } else {
                    // ignore the part of the ray behind its origin (t1 < 0)
                    rec1.t = rec1.t.max(0.0);
                    // now we can calculate the travel distance in the volume boundary (t2 - t1)
                    let distance_inside_boundary = (rec2.t - rec1.t) * r.direction.len();

                    // generate random hit distance depending on the density
                    let mut rng = rand::thread_rng();
                    let hit_distance = (-1. / self.density) * rng.gen::<f32>().ln();

                    // if the hit distance is smaller than our ray travel distance, we have a hit
                    if hit_distance < distance_inside_boundary {
                        let t = rec1.t + hit_distance / r.direction.len();
                        Some(HitRecord {
                            t,
                            p: r.point(t),
                            n: Vec3::new(1.0, 0.0, 0.0),
                            u: 0.0,
                            v: 0.0,
                            material: &self.phase_function,
                        })
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        self.boundary.bounding_box()
    }
}
