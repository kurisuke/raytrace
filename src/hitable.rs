use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::boundingbox::BoundingBox;
use crate::boundingbox;
use crate::bvhnode::BvhNode;

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
}

impl Hitable {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hitable::Sphere(sphere) => sphere.hit(r, t_min, t_max),
            Hitable::BvhNode(bvh_node) => bvh_node.hit(r, t_min, t_max)
        }
    }

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        match self {
            Hitable::Sphere(sphere) => sphere.bounding_box(),
            Hitable::BvhNode(bvh_node) => bvh_node.bounding_box()
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

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        let mut bbox : Option<BoundingBox> = None;
        for item in &self.list {
            if let Some(temp_bbox) = item.bounding_box() {
                if bbox.is_some() {
                    bbox = Some(boundingbox::surrounding_box(&bbox.unwrap(), &temp_bbox));
                }
                else {
                    bbox = Some(temp_bbox);
                }
            }
            else {
                return None;
            }
        }

        bbox
    }
}