use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

use rand::Rng;

pub struct Scatter {
    pub att: Vec3,
    pub ray: Ray,
}

#[derive(Clone)]
pub enum Material {
    Diffuse {albedo: Texture},
    Metal {albedo: Vec3, fuzz: f64},
    Dielectric {ref_index: f64},
    DiffuseLight {emit: Texture},
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>
    {
        match self {
            Material::Diffuse {albedo} =>
                scatter_diffuse(rec, &albedo),
            Material::Metal {albedo, fuzz} =>
                scatter_metal(r_in, rec, &albedo, *fuzz),
            Material::Dielectric {ref_index} =>
                scatter_dielectric(r_in, rec, *ref_index),
            Material::DiffuseLight {emit: _} =>
                None
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Material::DiffuseLight {emit} =>
                emit.value(u, v, p),
            _ =>
                Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

fn scatter_diffuse(rec: &HitRecord, albedo: &Texture) -> Option<Scatter> {
    let target = rec.p + rec.n + random_in_unit_sphere();
    Some(Scatter {
        att: albedo.value(rec.u, rec.v, &rec.p),
        ray: Ray {
            origin: rec.p,
            direction: target - rec.p,
        }
    })
}

fn scatter_metal(r_in: &Ray, rec: &HitRecord, albedo: &Vec3, fuzz: f64) -> Option<Scatter> {
    let reflected = reflect(&r_in.direction.normalize(), &rec.n) + Vec3::mul_s(&random_in_unit_sphere(), fuzz);
    if Vec3::dot(&reflected, &rec.n) > 0.0 {
        Some(Scatter {
            att: albedo.clone(),
            ray: Ray {
                origin: rec.p,
                direction: reflected,
            }
        })
    }
    else {
        None
    }
}

fn scatter_dielectric(r_in: &Ray, rec: &HitRecord, ref_index: f64) -> Option<Scatter> {
    let din = Vec3::dot(&r_in.direction, &rec.n);
    let outward_normal = if din > 0.0 { -rec.n } else { rec.n };
    let ni_over_nt = if din > 0.0 { ref_index } else { 1.0 / ref_index };
    let cosine = if din > 0.0 {
        ref_index * din / r_in.direction.len()
    } else {
        -din / r_in.direction.len()
    };

    let refracted_opt = refract(&r_in.direction, &outward_normal, ni_over_nt);
    let reflect_prob = if refracted_opt.is_some() {
        schlick(cosine, ref_index)
    } else {
        1.0
    };

    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() < reflect_prob {
        let reflected = reflect(&r_in.direction, &rec.n);
        Some(Scatter {
            att: Vec3::new(1.0, 1.0, 1.0),
            ray: Ray {
                origin: rec.p,
                direction: reflected,
            },
        })
    } else {
        Some(Scatter {
            att: Vec3::new(1.0, 1.0, 1.0),
            ray: Ray {
                origin: rec.p,
                direction: refracted_opt.unwrap(),
            },
        })
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0),
                          rng.gen_range(-1.0, 1.0),
                          rng.gen_range(-1.0, 1.0));
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.clone() - Vec3::mul_s(&n, 2.0 * Vec3::dot(&v, &n))
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = Vec3::dot(&uv, n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        Some(Vec3::mul_s(&(uv - Vec3::mul_s(n, dt)), ni_over_nt) -
             Vec3::mul_s(&n, d.sqrt())
        )
    }
    else {
        None
    }
}

fn schlick(cosine: f64, ref_index: f64) -> f64 {
    let r0 = (1.0 - ref_index) / (1.0 + ref_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
