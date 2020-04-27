use crate::boundingbox;
use crate::boundingbox::BoundingBox;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

use rand::Rng;

pub struct BvhNode {
    left: Box<dyn Hitable>,
    right: Option<Box<dyn Hitable>>,
    bbox: BoundingBox,
}

impl BvhNode {
    pub fn new(items: Vec<Box<dyn Hitable>>) -> BvhNode {
        let mut rng = rand::thread_rng();
        let mut sorted_items = items;

        let axis: usize = rng.gen_range(0, 3);
        if axis == 0 {
            sorted_items.sort_by(|a, b| {
                let box_left = a.bounding_box().unwrap();
                let box_right = b.bounding_box().unwrap();
                box_left.min.x().partial_cmp(&box_right.min.x()).unwrap()
            });
        } else if axis == 1 {
            sorted_items.sort_by(|a, b| {
                let box_left = a.bounding_box().unwrap();
                let box_right = b.bounding_box().unwrap();
                box_left.min.y().partial_cmp(&box_right.min.y()).unwrap()
            });
        } else {
            sorted_items.sort_by(|a, b| {
                let box_left = a.bounding_box().unwrap();
                let box_right = b.bounding_box().unwrap();
                box_left.min.z().partial_cmp(&box_right.min.z()).unwrap()
            });
        }

        if sorted_items.len() == 1 {
            let left = sorted_items.pop().unwrap();
            let bbox = left.bounding_box().unwrap();
            BvhNode {
                left,
                right: None,
                bbox,
            }
        } else if sorted_items.len() == 2 {
            let right = sorted_items.pop().unwrap();
            let left = sorted_items.pop().unwrap();
            let bbox = boundingbox::surrounding_box(
                &left.bounding_box().unwrap(),
                &right.bounding_box().unwrap(),
            );
            BvhNode {
                left,
                right: Some(right),
                bbox,
            }
        } else {
            let right = sorted_items.split_off(sorted_items.len() / 2);
            let left = sorted_items;

            let left = BvhNode::new(left);
            let right = BvhNode::new(right);
            let bbox = boundingbox::surrounding_box(
                &left.bounding_box().unwrap(),
                &right.bounding_box().unwrap(),
            );
            BvhNode {
                left: Box::new(left),
                right: Some(Box::new(right)),
                bbox,
            }
        }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            if self.right.is_none() {
                self.left.hit(&r, t_min, t_max)
            } else {
                let hit_left = self.left.hit(&r, t_min, t_max);
                let hit_right = self.right.as_ref().unwrap().hit(&r, t_min, t_max);
                if hit_left.is_some() && hit_right.is_some() {
                    if hit_left.as_ref().unwrap().t < hit_right.as_ref().unwrap().t {
                        hit_left
                    } else {
                        hit_right
                    }
                } else if hit_left.is_some() {
                    hit_left
                } else if hit_right.is_some() {
                    hit_right
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(self.bbox.clone())
    }
}
