mod camera;
mod hitable;
mod material;
mod ray;
mod render;
mod sphere;
mod vec3;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;
use rand::prelude::*;

use crate::camera::Camera;
use crate::vec3::Vec3;
use crate::hitable::{Hitable,HitableList};
use crate::sphere::Sphere;
use crate::material::Material;
use crate::render::{RenderParams};

fn main() {
    // render parameters
    let params = RenderParams {
        nx: 200,
        ny: 100,
        ns: 64,
        nt: 4,
    };

    // define the camera
    let look_from = Vec3::new(8.0, 1.5, 2.5);
    let look_at = Vec3::new(0.0, -0.5, -2.5);
    let dist_to_focus = (look_from - look_at).len();

    let cam = Camera::new(look_from,
                          look_at,
                          Vec3::new(0.0, 1.0, 0.0),
                          45.0, params.nx as f64 / params.ny as f64,
                          0.125, dist_to_focus);

    // define the world
    let world = random_scene();

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, params.nx as u32, params.ny as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = render::render(world, cam, params);

    writer.write_image_data(&data).unwrap(); // Save
}

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();

    let mut hl = HitableList {
        list: vec![
            Hitable::Sphere( Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Diffuse {
                    albedo: Vec3::new(0.5, 0.5, 0.5),
                },
            })
        ]
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2,
                                   b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    hl.list.push(Hitable::Sphere(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: Material::Diffuse {
                                albedo: Vec3::new(rng.gen::<f64>() * rng.gen::<f64>(),
                                                  rng.gen::<f64>() * rng.gen::<f64>(),
                                                  rng.gen::<f64>() * rng.gen::<f64>()),
                            },
                        }
                    ))
                } else if choose_mat < 0.95 {
                    // metal
                    hl.list.push(Hitable::Sphere(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: Material::Metal {
                                albedo: Vec3::new(rng.gen_range(0.5, 1.0),
                                                  rng.gen_range(0.5, 1.0),
                                                  rng.gen_range(0.5, 1.0)),
                            },
                        }
                    ))
                } else {
                    // glass
                    hl.list.push(Hitable::Sphere(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: Material::Dielectric {
                                ref_index: 1.5,
                            },
                        }
                    ))
                }
            }
        }
    }

    hl.list.push(Hitable::Sphere(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric {
            ref_index: 1.5,
        },
    }));
    hl.list.push(Hitable::Sphere(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Diffuse {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    }));
    hl.list.push(Hitable::Sphere(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
        },
    }));

    hl
}