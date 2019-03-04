use crate::boundingbox;
use crate::boundingbox::BoundingBox;
use crate::bvhnode::BvhNode;
use crate::material::Material;
use crate::ray::Ray;
use crate::rect::{Cuboid, Rect};
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub u: f64,
    pub v: f64,
    pub material: &'a Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<BoundingBox>;
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

unsafe impl Sync for HitableList {}
unsafe impl Send for HitableList {}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for item in &self.list {
            if let Some(temp_record) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }
        hit_record
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        let mut bbox: Option<BoundingBox> = None;
        for item in &self.list {
            if let Some(temp_bbox) = item.bounding_box() {
                if bbox.is_some() {
                    bbox = Some(boundingbox::surrounding_box(&bbox.unwrap(), &temp_bbox));
                } else {
                    bbox = Some(temp_bbox);
                }
            } else {
                return None;
            }
        }

        bbox
    }
}
