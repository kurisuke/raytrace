use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList {
    pub list: Vec<Box<Hitable>>
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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