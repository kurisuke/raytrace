mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;
use rand::prelude::*;

use crate::camera::Camera;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitableList};
use crate::sphere::Sphere;
use crate::material::Material;

fn color(r: Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f64::MAX) {
        if depth < 50 {
            if let Some(s) = rec.material.scatter(&r, &rec) {
                Vec3::mul_v(&color(s.ray, world, depth + 1), &s.att)
            }
            else {
                Vec3::default()
            }
        }
        else {
            Vec3::default()
        }
    } else {
        // background

        let unit_direction = r.direction.normalize();
        // get y component, scale to 0..1
        let t = 0.5 * (unit_direction.y() + 1.0);
        // get a color blend between white and light blue
        Vec3::mul_s(&Vec3::new(1.0, 1.0, 1.0), 1.0 - t) + Vec3::mul_s(&Vec3::new(0.5, 0.7, 1.0), t)
    }
}

fn convert_rgb_u8(v: &Vec3, gamma: f64) -> Vec<u8> {
    vec![(255.99 * v[0].powf(1.0 / gamma)) as u8,
         (255.99 * v[1].powf(1.0 / gamma)) as u8,
         (255.99 * v[2].powf(1.0 / gamma)) as u8]
}

fn main() {
    let nx = 1000;
    let ny = nx / 2;
    let ns = 100;

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, nx, ny);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data: Vec<u8> = vec![];

    // define the camera
    let cam = Camera::default();

    // define the world
    let world = HitableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Diffuse {
                    albedo: Vec3::new(0.1, 0.2, 0.5),
                    },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Diffuse {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                    },
            }),
            Box::new(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    },
            }),
            Box::new(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Dielectric {
                    ref_index: 1.5,
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: -0.45,
                material: Material::Dielectric {
                    ref_index: 1.5,
                },
            }),
        ]
    };

    // RNG for anti-aliasing (average sampling)
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                c += color(r, &world, 0);
            }
            let c = Vec3::div_s(&c, ns as f64);
            data.extend(convert_rgb_u8(&c, 2.0));
        }
    }

    writer.write_image_data(&data).unwrap(); // Save
}
