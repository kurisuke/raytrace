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

#[derive(Clone)]
pub enum Hitable {
    BvhNode(BvhNode),
    Sphere(Sphere),
    Rect(Rect),
    Cuboid(Cuboid),
}

impl Hitable {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hitable::BvhNode(bvh_node) => bvh_node.hit(r, t_min, t_max),
            Hitable::Sphere(sphere) => sphere.hit(r, t_min, t_max),
            Hitable::Rect(rect) => rect.hit(r, t_min, t_max),
            Hitable::Cuboid(cuboid) => cuboid.hit(r, t_min, t_max),
        }
    }

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        match self {
            Hitable::BvhNode(bvh_node) => bvh_node.bounding_box(),
            Hitable::Sphere(sphere) => sphere.bounding_box(),
            Hitable::Rect(rect) => rect.bounding_box(),
            Hitable::Cuboid(cuboid) => cuboid.bounding_box(),
        }
    }
}

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Hitable>,
}

impl HitableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

    pub fn bounding_box(&self) -> Option<BoundingBox> {
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
