use crate::camera::Camera;
use crate::hitable::{Hitable, HitableList};
use crate::ray::Ray;
use crate::vec3::Vec3;

use image::ImageBuffer;
use std::time::{Duration, Instant};

use rand::Rng;

use rayon::prelude::*;

pub struct RenderParams {
    pub nx: u32,
    pub ny: u32,
    pub ns: usize,
    pub filename: String,
}

pub enum Background {
    Color(Vec3),
    BlendY(Vec3, Vec3),
}

pub struct Scene {
    pub world: HitableList,
    pub cam: Camera,
    pub background: Background,
    pub params: RenderParams,
}

impl Background {
    pub fn color(&self, r: &Ray) -> Vec3 {
        match self {
            Background::Color(c) => *c,
            Background::BlendY(c1, c2) => {
                let unique_direction = r.direction.normalize();
                let t = 0.5 * (unique_direction.y() + 1.0);
                *c1 * (1. - t) + *c2 * t
            }
        }
    }
}

pub fn render(scene: Scene) {
    let begin_time = Instant::now();
    // output
    let mut data = ImageBuffer::new(scene.params.nx as u32, scene.params.ny as u32);

    // RNG for anti-aliasing (average sampling)
    let mut rng = rand::thread_rng();

    // let samples_per_thread = params.ns / params.nt;

    let mut pbr = pbr::ProgressBar::new(u64::from(data.width() * data.height()));
    pbr.show_percent = true;
    pbr.show_time_left = true;
    pbr.show_counter = false;
    pbr.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));

    for (i, j, pixel) in data.enumerate_pixels_mut() {
        // invert y coordinate
        let j = scene.params.ny - j - 1;

        let work: Vec<(f32, f32)> = (0..scene.params.ns)
            .map(|_| {
                (
                    (i as f32 + rng.gen::<f32>()) / scene.params.nx as f32,
                    (j as f32 + rng.gen::<f32>()) / scene.params.ny as f32,
                )
            })
            .collect();

        let c = work
            .par_iter()
            .map(|(u, v)| {
                let r = scene.cam.get_ray(*u, *v);
                color(r, &scene.world, &scene.background, 0)
            })
            .sum::<Vec3>()
            / scene.params.ns as f32;

        *pixel = image::Rgb(convert_rgb_u8(&c, 2.0));
        pbr.inc();
    }

    data.save(&scene.params.filename).unwrap();
    pbr.finish_println(&format!(
        "Done in {}\n",
        humantime::format_duration(Duration::from_secs(begin_time.elapsed().as_secs()))
    ));
}

fn color(r: Ray, world: &HitableList, background: &Background, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        if depth < 64 {
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            if let Some(s) = rec.material.scatter(&r, &rec) {
                emitted + color(s.ray, world, background, depth + 1) * s.att
            } else {
                emitted
            }
        } else {
            Vec3::default()
        }
    } else {
        background.color(&r)
    }
}

fn convert_rgb_u8(v: &Vec3, gamma: f32) -> [u8; 3] {
    let mut rgb = [0; 3];
    for i in 0..3 {
        rgb[i] = (255.99 * v.i(i).powf(1.0 / gamma)).min(255.0) as u8;
    }
    rgb
}
