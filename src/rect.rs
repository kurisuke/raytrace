use crate::boundingbox::BoundingBox;
use crate::hitable::{HitRecord, Hitable, HitableList};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Rect {
    pub a: Axes,
    pub flip_normal: bool,
    pub material: Material,
}

#[derive(Clone)]
pub enum Axes {
    XY {
        x: (f64, f64),
        y: (f64, f64),
        z: f64,
    },
    XZ {
        x: (f64, f64),
        y: f64,
        z: (f64, f64),
    },
    YZ {
        x: f64,
        y: (f64, f64),
        z: (f64, f64),
    },
}

impl Rect {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // check if ray intersects the rect plane
        let t = match self.a {
            Axes::XY { x: _, y: _, z } => (z - r.origin.z()) / r.direction.z(),
            Axes::XZ { x: _, y, z: _ } => (y - r.origin.y()) / r.direction.y(),
            Axes::YZ { x, y: _, z: _ } => (x - r.origin.x()) / r.direction.x(),
        };
        if t < t_min || t > t_max {
            None
        } else {
            // check if intersect point is in rect calc and the (u, v) coordinates
            let uv = match self.a {
                Axes::XY { x, y, z: _ } => {
                    let xt = r.origin.x() + t * r.direction.x();
                    let yt = r.origin.y() + t * r.direction.y();
                    if xt < x.0 || xt > x.1 || yt < y.0 || yt > y.1 {
                        None
                    } else {
                        Some(((xt - x.0) / (x.1 - x.0), (yt - y.0) / (y.1 - y.0)))
                    }
                }
                Axes::XZ { x, y: _, z } => {
                    let xt = r.origin.x() + t * r.direction.x();
                    let zt = r.origin.z() + t * r.direction.z();
                    if xt < x.0 || xt > x.1 || zt < z.0 || zt > z.1 {
                        None
                    } else {
                        Some(((xt - x.0) / (x.1 - x.0), (zt - z.0) / (z.1 - z.0)))
                    }
                }
                Axes::YZ { x: _, y, z } => {
                    let yt = r.origin.y() + t * r.direction.y();
                    let zt = r.origin.z() + t * r.direction.z();
                    if yt < y.0 || yt > y.1 || zt < z.0 || zt > z.1 {
                        None
                    } else {
                        Some(((yt - y.0) / (y.1 - y.0), (zt - z.0) / (z.1 - z.0)))
                    }
                }
            };
            if uv.is_none() {
                None
            } else {
                let (u, v) = uv.unwrap();
                let n = match self.a {
                    Axes::XY { x: _, y: _, z: _ } => Vec3::new(0.0, 0.0, 1.0),
                    Axes::XZ { x: _, y: _, z: _ } => Vec3::new(0.0, 1.0, 0.0),
                    Axes::YZ { x: _, y: _, z: _ } => Vec3::new(1.0, 0.0, 0.0),
                };
                let n = if self.flip_normal { -n } else { n };
                Some(HitRecord {
                    t,
                    p: r.point(t),
                    n,
                    u,
                    v,
                    material: &self.material,
                })
            }
        }
    }

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        match self.a {
            Axes::XY { x, y, z } => Some(BoundingBox {
                min: Vec3::new(x.0, y.0, z - 0.0001),
                max: Vec3::new(x.1, y.1, z + 0.0001),
            }),
            Axes::XZ { x, y, z } => Some(BoundingBox {
                min: Vec3::new(x.0, y - 0.0001, z.0),
                max: Vec3::new(x.1, y + 0.0001, z.1),
            }),
            Axes::YZ { x, y, z } => Some(BoundingBox {
                min: Vec3::new(x - 0.0001, y.0, z.0),
                max: Vec3::new(x + 0.0001, y.1, z.1),
            }),
        }
    }
}

#[derive(Clone)]
pub struct Cuboid {
    p_min: Vec3,
    p_max: Vec3,
    faces: Box<HitableList>,
}

impl Cuboid {
    pub fn new(p_min: Vec3, p_max: Vec3, material: Material) -> Cuboid {
        Cuboid {
            p_min,
            p_max,
            faces: Box::new(HitableList {
                list: vec![
                    Hitable::Rect(Rect {
                        a: Axes::XY {
                            x: (p_min.x(), p_max.x()),
                            y: (p_min.y(), p_max.y()),
                            z: p_max.z(),
                        },
                        flip_normal: false,
                        material: material.clone(),
                    }),
                    Hitable::Rect(Rect {
                        a: Axes::XY {
                            x: (p_min.x(), p_max.x()),
                            y: (p_min.y(), p_max.y()),
                            z: p_min.z(),
                        },
                        flip_normal: true,
                        material: material.clone(),
                    }),
                    Hitable::Rect(Rect {
                        a: Axes::XZ {
                            x: (p_min.x(), p_max.x()),
                            y: p_max.y(),
                            z: (p_min.z(), p_max.z()),
                        },
                        flip_normal: false,
                        material: material.clone(),
                    }),
                    Hitable::Rect(Rect {
                        a: Axes::XZ {
                            x: (p_min.x(), p_max.x()),
                            y: p_min.y(),
                            z: (p_min.z(), p_max.z()),
                        },
                        flip_normal: true,
                        material: material.clone(),
                    }),
                    Hitable::Rect(Rect {
                        a: Axes::YZ {
                            x: p_max.x(),
                            y: (p_min.y(), p_max.y()),
                            z: (p_min.z(), p_max.z()),
                        },
                        flip_normal: false,
                        material: material.clone(),
                    }),
                    Hitable::Rect(Rect {
                        a: Axes::YZ {
                            x: p_min.x(),
                            y: (p_min.y(), p_max.y()),
                            z: (p_min.z(), p_max.z()),
                        },
                        flip_normal: true,
                        material,
                    }),
                ],
            }),
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox {
            min: self.p_min,
            max: self.p_max,
        })
    }
}
