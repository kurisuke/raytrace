use crate::camera::Camera;
use crate::ray::Ray;
use crate::hitable::HitableList;
use crate::vec3::Vec3;

use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use rand::Rng;

pub struct RenderParams {
    pub nx: usize,
    pub ny: usize,
    pub ns: usize,
    pub nt: usize,
}

pub fn render(world: HitableList, cam: Camera, params: RenderParams) -> Vec<u8>
{
    // output
    let mut data: Vec<u8> = vec![];

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

    for j in (0..params.ny).rev() {
        for i in 0..params.nx {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for in_tx in &child_in_tx {
                for _ in 0..samples_per_thread {
                    let u = (i as f64 + rng.gen::<f64>()) / params.nx as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / params.ny as f64;
                    let r = cam.get_ray(u, v);
                    in_tx.send(Job::Data(r)).unwrap();
                }
            }

            let mut count_output = 0;
            for cs in &out_rx {
                c += cs;
                count_output += 1;
                if count_output == params.ns {
                    break;
                }
            }

            let c = Vec3::div_s(&c, params.ns as f64);
            data.extend(convert_rgb_u8(&c, 2.0));
        }
    }

    for in_tx in &child_in_tx {
        in_tx.send(Job::End).unwrap();
    }

    for child in child_threads {
        let _ = child.join();
    }

    data
}

enum Job {
    Data(Ray),
    End,
}

fn render_job(world: HitableList, in_rx: mpsc::Receiver<Job>, out_tx: mpsc::Sender<Vec3>) {
    for j in in_rx {
        match j {
            Job::Data(r) => {
                let c = color(r, &world, 0);
                out_tx.send(c).unwrap();
            },
            Job::End => { break; }
        }
    }
}

fn color(r: Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f64::MAX) {
        if depth < 32 {
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
