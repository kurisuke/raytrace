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
mod transform;
mod vec3;

use rand::prelude::*;

use crate::camera::Camera;
use crate::hitable::HitableList;
use crate::material::Material;
use crate::rect::{Axes, Cuboid, Rect};
use crate::render::RenderParams;
use crate::sphere::Sphere;
use crate::texture::{Perlin, Texture};
use crate::transform::{RotateY, Translate};
use crate::vec3::Vec3;

fn main() {
    // command line argument
    let clap_matches = clap::App::new("raytrace")
        .version("0.1")
        .author("Peter Helbing <peter@abulafia.org>")
        .arg(
            clap::Arg::with_name("WIDTH")
                .help("width of the resulting image")
                .index(1),
        )
        .arg(
            clap::Arg::with_name("HEIGHT")
                .help("height of the resulting image")
                .index(2),
        )
        .arg(
            clap::Arg::with_name("SPP")
                .help("samples (rays) per pixel")
                .index(3),
        )
        .get_matches();

    // render parameters
    let params = RenderParams {
        nx: clap_matches
            .value_of("WIDTH")
            .unwrap_or("200")
            .parse::<u32>()
            .unwrap(),
        ny: clap_matches
            .value_of("HEIGHT")
            .unwrap_or("100")
            .parse::<u32>()
            .unwrap(),
        ns: clap_matches
            .value_of("SPP")
            .unwrap_or("64")
            .parse::<usize>()
            .unwrap(),
        filename: String::from("image.png"),
    };

    // define the camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        params.nx as f32 / params.ny as f32,
        0.1,
        dist_to_focus,
    );

    let cam_cornell = Camera::new(
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        params.nx as f32 / params.ny as f32,
        0.0,
        10.0,
    );

    // define the world
    // let world = two_perlin_spheres();
    let world = cornell_box_base() + cornell_box_balls();
    let world = HitableList {
        list: vec![Box::new(bvhnode::BvhNode::new(world.list))],
    };

    render::render(world, cam_cornell, params);
}

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();

    let mut hl = HitableList {
        list: vec![Box::new(Sphere {
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
        })],
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    hl.list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Diffuse {
                            albedo: Texture::Constant {
                                color: Vec3::new(
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                ),
                            },
                        },
                    }))
                } else if choose_mat < 0.95 {
                    // metal
                    hl.list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal {
                            albedo: Vec3::new(
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                            ),
                            fuzz: rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 0.5),
                        },
                    }))
                } else {
                    // glass
                    hl.list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric { ref_index: 1.5 },
                    }))
                }
            }
        }
    }

    hl.list.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric { ref_index: 1.5 },
    }));
    hl.list.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Diffuse {
            albedo: Texture::Constant {
                color: Vec3::new(0.4, 0.2, 0.1),
            },
        },
    }));
    hl.list.push(Box::new(Sphere {
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
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Diffuse { albedo: text1 },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, 2.0, 0.0),
                radius: 2.0,
                material: Material::Diffuse { albedo: text2 },
            }),
            Box::new(Rect {
                a: Axes::XY {
                    x: (3.0, 5.0),
                    y: (1.0, 3.0),
                    z: -2.0,
                },
                flip_normal: false,
                material: Material::DiffuseLight {
                    emit: Texture::Constant {
                        color: Vec3::new(4.0, 4.0, 4.0),
                    },
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, 7.0, 0.0),
                radius: 2.0,
                material: Material::DiffuseLight {
                    emit: Texture::Constant {
                        color: Vec3::new(4.0, 4.0, 4.0),
                    },
                },
            }),
        ],
    }
}

fn cornell_box_base() -> HitableList {
    let red = Material::Diffuse {
        albedo: Texture::Constant {
            color: Vec3::new(0.65, 0.05, 0.05),
        },
    };
    let white = Material::Diffuse {
        albedo: Texture::Constant {
            color: Vec3::new(0.73, 0.73, 0.73),
        },
    };
    let green = Material::Diffuse {
        albedo: Texture::Constant {
            color: Vec3::new(0.12, 0.45, 0.15),
        },
    };
    let light = Material::DiffuseLight {
        emit: Texture::Constant {
            color: Vec3::new(15.0, 15.0, 15.0),
        },
    };

    HitableList {
        list: vec![
            Box::new(Rect {
                a: Axes::YZ {
                    x: 555.0,
                    y: (0.0, 555.0),
                    z: (0.0, 555.0),
                },
                flip_normal: true,
                material: green,
            }),
            Box::new(Rect {
                a: Axes::YZ {
                    x: 0.0,
                    y: (0.0, 555.0),
                    z: (0.0, 555.0),
                },
                flip_normal: false,
                material: red,
            }),
            Box::new(Rect {
                a: Axes::XZ {
                    x: (213.0, 343.0),
                    y: 554.0,
                    z: (227.0, 332.0),
                },
                flip_normal: false,
                material: light,
            }),
            Box::new(Rect {
                a: Axes::XZ {
                    x: (0.0, 555.0),
                    y: 555.0,
                    z: (0.0, 555.0),
                },
                flip_normal: true,
                material: white.clone(),
            }),
            Box::new(Rect {
                a: Axes::XZ {
                    x: (0.0, 555.0),
                    y: 0.0,
                    z: (0.0, 555.0),
                },
                flip_normal: false,
                material: white.clone(),
            }),
            Box::new(Rect {
                a: Axes::XY {
                    x: (0.0, 555.0),
                    y: (0.0, 555.0),
                    z: 555.0,
                },
                flip_normal: true,
                material: white.clone(),
            }),
        ],
    }
}

fn cornell_box_blocks() -> HitableList {
    let white = Material::Diffuse {
        albedo: Texture::Constant {
            color: Vec3::new(0.73, 0.73, 0.73),
        },
    };

    HitableList {
        list: vec![
            Box::new(Translate::new(
                Box::new(RotateY::new(
                    Box::new(Cuboid::new(
                        Vec3::default(),
                        Vec3::new(165.0, 165.0, 165.0),
                        white.clone(),
                    )),
                    -18.0,
                )),
                Vec3::new(130.0, 0.0, 65.0),
            )),
            Box::new(Translate::new(
                Box::new(RotateY::new(
                    Box::new(Cuboid::new(
                        Vec3::default(),
                        Vec3::new(165.0, 330.0, 165.0),
                        white,
                    )),
                    15.0,
                )),
                Vec3::new(265.0, 0.0, 295.0),
            )),
        ],
    }
}

fn cornell_box_balls() -> HitableList {
    let glass = Material::Dielectric { ref_index: 1.5 };
    let metal = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    HitableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(212.5, 82.5, 147.5),
                radius: 82.5,
                material: glass.clone(),
            }),
            Box::new(Sphere {
                center: Vec3::new(212.5, 82.5, 147.5),
                radius: -72.5,
                material: glass,
            }),
            Box::new(Sphere {
                center: Vec3::new(347.5, 82.5, 377.5),
                radius: 82.5,
                material: metal,
            }),
        ],
    }
}
