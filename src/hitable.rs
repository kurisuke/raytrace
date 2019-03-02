use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;
use crate::sphere::Sphere;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub material: &'a Material,
}

#[derive(Clone)]
pub enum Hitable {
    Sphere(Sphere),
}

impl Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hitable::Sphere(sphere) => sphere.hit(r, t_min, t_max)
        }
    }
}

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Hitable>,
}

impl HitableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record : Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for item in &self.list {
            if let Some(temp_record) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }
        hit_record
    }
}