use crate::boundingbox;
use crate::boundingbox::BoundingBox;
use crate::bvhnode;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::ops::{Add, AddAssign};

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub u: f32,
    pub v: f32,
    pub material: &'a Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<BoundingBox>;
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

unsafe impl Sync for HitableList {}
unsafe impl Send for HitableList {}

impl HitableList {
    pub fn into_bvh(self) -> HitableList {
        HitableList {
            list: vec![Box::new(bvhnode::BvhNode::new(self.list))],
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

impl Add for HitableList {
    type Output = HitableList;

    fn add(self, rhs: HitableList) -> HitableList {
        let mut list = self.list;
        list.extend(rhs.list);
        HitableList { list }
    }
}

impl AddAssign for HitableList {
    fn add_assign(&mut self, rhs: HitableList) {
        self.list.extend(rhs.list);
    }
}
