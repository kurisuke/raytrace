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
mod volume;

use rand::prelude::*;

use crate::camera::Camera;
use crate::hitable::HitableList;
use crate::material::Material;
use crate::rect::{Axes, Cuboid, Rect};
use crate::render::{Background, RenderParams, Scene};
use crate::sphere::Sphere;
use crate::texture::{Perlin, Texture};
use crate::transform::{RotateXYZ, Translate};
use crate::vec3::Vec3;
use crate::volume::ConstantMedium;

fn main() {
    // command line argument
    let clap_matches = clap::App::new("raytrace")
        .version("0.1")
        .author("Peter Helbing <peter@abulafia.org>")
        .arg(
            clap::Arg::with_name("SCENE")
                .help("scene to render")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("W")
                .help("width of the resulting image")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("H")
                .help("height of the resulting image")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("samples")
                .short("s")
                .long("samples")
                .value_name("S")
                .help("number of samples (rays) per pixel")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("out-filename")
                .short("o")
                .long("out-filename")
                .value_name("FILE")
                .help("name of the output file")
                .takes_value(true),
        )
        .get_matches();

    // render parameters
    let params = RenderParams {
        nx: clap_matches
            .value_of("width")
            .unwrap_or("320")
            .parse::<u32>()
            .unwrap(),
        ny: clap_matches
            .value_of("height")
            .unwrap_or("200")
            .parse::<u32>()
            .unwrap(),
        ns: clap_matches
            .value_of("samples")
            .unwrap_or("256")
            .parse::<usize>()
            .unwrap(),
        filename: clap_matches
            .value_of("out-filename")
            .unwrap_or("image.png")
            .to_string(),
    };

    // define the camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let cam_random_scene = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        params.nx as f32 / params.ny as f32,
        0.1,
        dist_to_focus,
    );

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let cam_two_spheres = Camera::new(
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

    let scene = match clap_matches.value_of("SCENE").unwrap() {
        "cornell_blocks" => Scene {
            world: (cornell_box_base() + cornell_box_blocks()).into_bvh(),
            cam: cam_cornell,
            background: Background::Color(Vec3::default()),
            params,
        },
        "cornell_blocks_volume" => Scene {
            world: (cornell_box_base() + cornell_box_blocks_volume()).into_bvh(),
            cam: cam_cornell,
            background: Background::Color(Vec3::default()),
            params,
        },
        "cornell_balls" => Scene {
            world: (cornell_box_base() + cornell_box_balls()).into_bvh(),
            cam: cam_cornell,
            background: Background::Color(Vec3::default()),
            params,
        },
        "earth_perlin" => Scene {
            world: (earth_perlin()).into_bvh(),
            cam: cam_two_spheres,
            background: Background::Color(Vec3::default()),
            params,
        },
        "random_scene" => Scene {
            world: (random_scene()).into_bvh(),
            cam: cam_random_scene,
            background: Background::BlendY(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0)),
            params,
        },
        _ => {
            panic!("Unknown scene: choose one from (cornell_blocks | cornell_blocks_volume | cornell_balls | earth_perlin | random_scene)");
        }
    };

    render::render(scene);
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

fn earth_perlin() -> HitableList {
    let text1 = Texture::PerlinNoise {
        perlin: Box::new(Perlin::new()),
        scale: 5.0,
    };

    let img = image::open("res/earth.jpg").unwrap();
    let text2 = Texture::Image {
        image: Box::new(img.as_rgb8().unwrap().clone()),
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
                material: white,
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
                Box::new(RotateXYZ::new(
                    Box::new(Cuboid::new(
                        Vec3::default(),
                        Vec3::new(165.0, 165.0, 165.0),
                        white.clone(),
                    )),
                    Vec3::new(0.0, -18.0, 0.0),
                )),
                Vec3::new(130.0, 0.0, 65.0),
            )),
            Box::new(Translate::new(
                Box::new(RotateXYZ::new(
                    Box::new(Cuboid::new(
                        Vec3::default(),
                        Vec3::new(165.0, 330.0, 165.0),
                        white,
                    )),
                    Vec3::new(0.0, 15.0, 0.0),
                )),
                Vec3::new(265.0, 0.0, 295.0),
            )),
        ],
    }
}

fn cornell_box_blocks_volume() -> HitableList {
    let white = Material::Diffuse {
        albedo: Texture::Constant {
            color: Vec3::new(0.73, 0.73, 0.73),
        },
    };

    let b1 = Box::new(Translate::new(
        Box::new(RotateXYZ::new(
            Box::new(Cuboid::new(
                Vec3::default(),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            )),
            Vec3::new(0.0, -18.0, 0.0),
        )),
        Vec3::new(130.0, 0.0, 65.0),
    ));
    let b2 = Box::new(Translate::new(
        Box::new(RotateXYZ::new(
            Box::new(Cuboid::new(
                Vec3::default(),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            )),
            Vec3::new(0.0, 15.0, 0.0),
        )),
        Vec3::new(265.0, 0.0, 295.0),
    ));

    HitableList {
        list: vec![
            Box::new(ConstantMedium::new(
                b1,
                0.01,
                Texture::Constant {
                    color: Vec3::new(1.0, 1.0, 1.0),
                },
            )),
            Box::new(ConstantMedium::new(
                b2,
                0.01,
                Texture::Constant {
                    color: Vec3::new(0.0, 0.0, 0.0),
                },
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
