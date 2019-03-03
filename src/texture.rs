use crate::vec3::Vec3;

use rand::{Rng, thread_rng};

#[derive(Clone)]
pub enum Texture {
    Constant {color: Vec3},
    Checker {odd: Box<Texture>, even: Box<Texture>},
    PerlinNoise {perlin: Perlin, scale: f64},
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Texture::Constant {color} => color.clone(),
            Texture::Checker {odd, even} => {
                let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
                if sines < 0.0 {
                    odd.value(u, v, p)
                } else {
                    even.value(u, v, p)
                }
            },
            Texture::PerlinNoise {perlin, scale} => {
                Vec3::mul_s(&Vec3::new(1.0, 1.0, 1.0),
                            0.5 * (1.0 + (scale * p.z() + 10.0 * perlin.turb(p, 7)).sin()))
            }
        }
    }
}

#[derive(Clone)]
pub struct Perlin {
    pub values: [Vec3; 256],
    pub perm_x: [usize; 256],
    pub perm_y: [usize; 256],
    pub perm_z: [usize; 256],
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            values: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.values[
                        self.perm_x[((i + di) & 255) as usize] ^
                        self.perm_y[((j + dj) & 255) as usize] ^
                        self.perm_z[((k + dk) & 255) as usize]];
                }
            }
        }
        perlin_interpolate(c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: usize) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            acc += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = Vec3::mul_s(&temp_p, 2.0);
        }
        acc.abs()
    }
}

fn perlin_generate() -> [Vec3; 256] {
    let mut rng = rand::thread_rng();
    let v = (0..256).map(|_| Vec3::new(rng.gen_range(-1.0, 1.0),
                                       rng.gen_range(-1.0, 1.0),
                                       rng.gen_range(-1.0, 1.0)).normalize()).collect::<Vec<Vec3>>();
    let mut p = [Vec3::default(); 256];
    p.copy_from_slice(&v);
    p
}

fn perlin_generate_perm() -> [usize; 256] {
    let mut rng = rand::thread_rng();
    let mut v: Vec<usize> = (0..256).collect();
    rng.shuffle(&mut v);
    let mut p = [0 as usize; 256];
    p.copy_from_slice(&v);
    p
}

fn perlin_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                acc += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu)) *
                    (j as f64 * vv + (1 - j) as f64 * (1.0 - vv)) *
                    (k as f64 * ww + (1 - k) as f64 * (1.0 - ww)) *
                    Vec3::dot(&c[i][j][k], &weight_v);
            }
        }
    }
    acc
}