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

pub fn render(world: HitableList, cam: Camera, params: RenderParams) {
    let begin_time = Instant::now();
    // output
    let mut data = ImageBuffer::new(params.nx as u32, params.ny as u32);

    // RNG for anti-aliasing (average sampling)
    let mut rng = rand::thread_rng();

    // let samples_per_thread = params.ns / params.nt;

    let mut pbr = pbr::ProgressBar::new((data.width() * data.height()) as u64);
    pbr.show_percent = true;
    pbr.show_time_left = true;
    pbr.show_counter = false;
    pbr.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));

    for (i, j, pixel) in data.enumerate_pixels_mut() {
        // invert y coordinate
        let j = params.ny - j - 1;

        let work: Vec<(f32, f32)> = (0..params.ns)
            .map(|_| {
                (
                    (i as f32 + rng.gen::<f32>()) / params.nx as f32,
                    (j as f32 + rng.gen::<f32>()) / params.ny as f32,
                )
            })
            .collect();

        let c = work
            .par_iter()
            .map(|(u, v)| {
                let r = cam.get_ray(*u, *v);
                color(r, &world, 0)
            })
            .sum::<Vec3>()
            / params.ns as f32;

        *pixel = image::Rgb(convert_rgb_u8(&c, 2.0));
        pbr.inc();
    }

    data.save(&params.filename).unwrap();
    pbr.finish_println(&format!(
        "Done in {}\n",
        humantime::format_duration(Duration::from_secs(begin_time.elapsed().as_secs()))
    ));
}

fn color(r: Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        if depth < 64 {
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            if let Some(s) = rec.material.scatter(&r, &rec) {
                emitted + color(s.ray, world, depth + 1) * s.att
            } else {
                emitted
            }
        } else {
            Vec3::default()
        }
    } else {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

fn convert_rgb_u8(v: &Vec3, gamma: f32) -> [u8; 3] {
    let mut rgb = [0; 3];
    for i in 0..3 {
        rgb[i] = (255.99 * v.i(i).powf(1.0 / gamma)).min(255.0) as u8;
    }
    rgb
}
