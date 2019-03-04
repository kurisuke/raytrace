use crate::camera::Camera;
use crate::hitable::HitableList;
use crate::ray::Ray;
use crate::vec3::Vec3;

use image::ImageBuffer;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rand::Rng;

pub struct RenderParams {
    pub nx: u32,
    pub ny: u32,
    pub ns: usize,
    pub nt: usize,
    pub filename: String,
}

pub fn render(world: HitableList, cam: Camera, params: RenderParams) {
    // output
    let mut data = ImageBuffer::new(params.nx as u32, params.ny as u32);

    // RNG for anti-aliasing (average sampling)
    let mut rng = rand::thread_rng();

    let mut child_in_tx = vec![];
    let mut child_threads = vec![];
    let (out_tx, out_rx): (Sender<Vec3>, Receiver<Vec3>) = mpsc::channel();
    for _ in 0..params.nt {
        let (in_tx, in_rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let thread_out_tx = out_tx.clone();
        let world_c = world.clone();
        let child = thread::spawn(move || {
            render_job(world_c, in_rx, thread_out_tx);
        });
        child_threads.push(child);
        child_in_tx.push(in_tx);
    }

    let samples_per_thread = params.ns / params.nt;

    let mut pbr = pbr::ProgressBar::new((data.width() * data.height()) as u64);
    pbr.show_percent = true;
    pbr.show_time_left = true;
    pbr.show_counter = false;
    pbr.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));

    for (i, j, pixel) in data.enumerate_pixels_mut() {
        // invert y coordinate
        let j = params.ny - j - 1;

        let mut c = Vec3::new(0.0, 0.0, 0.0);
        for in_tx in &child_in_tx {
            let rs: Vec<_> = (0..samples_per_thread)
                .map(|_| {
                    let u = (i as f64 + rng.gen::<f64>()) / params.nx as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / params.ny as f64;
                    cam.get_ray(u, v)
                })
                .collect();
            in_tx.send(Job::Data(rs)).unwrap();
        }

        let mut count_output = 0;
        for cs in &out_rx {
            c += cs;
            count_output += 1;
            if count_output == params.nt {
                break;
            }
        }

        c /= params.ns as f64;
        *pixel = image::Rgb(convert_rgb_u8(&c, 2.0));
        pbr.inc();
    }

    for in_tx in &child_in_tx {
        in_tx.send(Job::End).unwrap();
    }

    for child in child_threads {
        let _ = child.join();
    }

    data.save(&params.filename).unwrap();
    pbr.finish_println("done");
}

enum Job {
    Data(Vec<Ray>),
    End,
}

fn render_job(world: HitableList, in_rx: mpsc::Receiver<Job>, out_tx: mpsc::Sender<Vec3>) {
    for j in in_rx {
        match j {
            Job::Data(rs) => {
                let c = rs.into_iter().map(|r| color(r, &world, 0)).sum();
                out_tx.send(c).unwrap();
            }
            Job::End => {
                break;
            }
        }
    }
}

fn color(r: Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f64::MAX) {
        if depth < 32 {
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

fn convert_rgb_u8(v: &Vec3, gamma: f64) -> [u8; 3] {
    let mut rgb = [0; 3];
    for i in 0..3 {
        rgb[i] = (255.99 * v[i].powf(1.0 / gamma)).min(255.0) as u8;
    }
    rgb
}
