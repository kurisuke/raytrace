mod boundingbox;
mod bvhnode;
mod camera;
mod hitable;
mod material;
mod ray;
mod rect;
mod render;
mod sphere;
mod texture;
mod vec3;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use rand::prelude::*;

use crate::camera::Camera;
use crate::vec3::Vec3;
use crate::hitable::{Hitable,HitableList};
use crate::sphere::Sphere;
use crate::material::Material;
use crate::render::{RenderParams};
use crate::texture::{Perlin, Texture};
use crate::rect::{Axes,Rect};

fn main() {
    // command line argument
    let clap_matches = clap::App::new("raytrace")
        .version("0.1")
        .author("Peter Helbing <peter@abulafia.org>")
        .arg(clap::Arg::with_name("WIDTH")
            .help("width of the resulting image")
            .index(1))
        .arg(clap::Arg::with_name("HEIGHT")
            .help("height of the resulting image")
            .index(2))
        .arg(clap::Arg::with_name("SPP")
            .help("samples (rays) per pixel")
            .index(3))
        .get_matches();

    // render parameters
    let params = RenderParams {
        nx: clap_matches.value_of("WIDTH").unwrap_or("200").parse::<u32>().unwrap(),
        ny: clap_matches.value_of("HEIGHT").unwrap_or("100").parse::<u32>().unwrap(),
        ns: clap_matches.value_of("SPP").unwrap_or("64").parse::<usize>().unwrap(),
        nt: num_cpus::get(),
        filename: String::from("image.png"),
    };

    // define the camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(look_from,
                          look_at,
                          Vec3::new(0.0, 1.0, 0.0),
                          40.0, params.nx as f64 / params.ny as f64,
                          0.1, dist_to_focus);

    // define the world
    // let world = random_scene();
    let world = two_perlin_spheres();
    let world = HitableList {
        list: vec![Hitable::BvhNode(bvhnode::BvhNode::new(world.list))]
    };

    let nx = params.nx as u32;
    let ny = params.ny as u32;
    render::render(world, cam, params);
}

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();

    let mut hl = HitableList {
        list: vec![
            Hitable::Sphere( Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Diffuse {
                    albedo: Texture::Checker {
                        odd: Box::new(Texture::Constant {
                            color: Vec3::new(0.2, 0.3, 0.1),
                        }),
                        even: Box::new(Texture::Constant {
                            color: Vec3::new(0.9, 0.9, 0.9),
                        }),
                    },
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
                                albedo: Texture::Constant
                                    { color: Vec3::new(rng.gen::<f64>() * rng.gen::<f64>(),
                                                       rng.gen::<f64>() * rng.gen::<f64>(),
                                                       rng.gen::<f64>() * rng.gen::<f64>()), },
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
                                fuzz: rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 0.5),
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
            albedo: Texture::Constant { color: Vec3::new(0.4, 0.2, 0.1) },
        },
    }));
    hl.list.push(Hitable::Sphere(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.1,
        },
    }));

    hl
}

fn two_perlin_spheres() -> HitableList {
    let text1 = Texture::PerlinNoise {
        perlin: Perlin::new(),
        scale: 5.0,
    };

    let img = image::open("res/earth.jpg").unwrap();
    let text2 = Texture::Image {
        image: img.as_rgb8().unwrap().clone(),
    };

    HitableList {
        list : vec![
            Hitable::Sphere(Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Diffuse {
                    albedo: text1,
                }
            }),
            Hitable::Sphere(Sphere {
                center: Vec3::new(0.0, 2.0, 0.0),
                radius: 2.0,
                material: Material::Diffuse {
                    albedo: text2,
                }
            }),
            Hitable::Rect(Rect {
                a: Axes::XY {
                    x: (3.0, 5.0),
                    y: (1.0, 3.0),
                    z: -2.0,
                },
                flip_normal: false,
                material: Material::DiffuseLight {
                    emit: Texture::Constant { color: Vec3::new(4.0, 4.0, 4.0) },
                },
            }),
            Hitable::Sphere(Sphere {
                center: Vec3::new(0.0, 7.0, 0.0),
                radius: 2.0,
                material: Material::DiffuseLight {
                    emit: Texture::Constant { color: Vec3::new(4.0, 4.0, 4.0) },
                }
            }),
        ]
    }
}