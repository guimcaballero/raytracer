use crate::{hit_record::*, onb::*, pdf::*, ray::*, texture::*, vec3::*};
use rand::*;
use std::f32::consts::PI;

#[derive(Clone)]
#[allow(dead_code)]
pub enum Material {
    Normal,
    Lambertian(Color),
    LambertianTexture(Texture),
    Metal(Color, f32),
    Dielectric(f32),
    DiffuseLight(Color),
    DiffuseLightTexture(Texture),
    Isotropic(Color),
}

impl Material {
    pub fn old_scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        albedo: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f32,
    ) -> bool {
        match self {
            Self::Normal => false,
            Self::Lambertian(alb) => {
                let onb = ONB::build_from_w(hit_record.normal);
                // let direction = (hit_record.normal + Vec3::random_unit_vector()).normalize();
                let direction = onb.local(Vec3::random_cosine_direction());
                *scattered = Ray {
                    origin: hit_record.point,
                    direction,
                    time: ray_in.time,
                };
                *albedo = *alb;
                *pdf = onb.w.dot(&scattered.direction) / PI;

                true
            }
            Self::LambertianTexture(alb) => {
                let direction = (hit_record.normal + Vec3::random_unit_vector()).normalize();
                *scattered = Ray {
                    origin: hit_record.point,
                    direction,
                    time: ray_in.time,
                };
                *albedo = alb(hit_record.u, hit_record.v, hit_record.point);
                *pdf = hit_record.normal.dot(&scattered.direction) / PI;
                true
            }
            Self::Metal(alb, fuzz) => {
                let reflected = ray_in.direction.normalize().reflect(&hit_record.normal);
                let ray = Ray {
                    origin: hit_record.point,
                    direction: reflected + fuzz.min(1.0) * Vec3::random_in_unit_sphere(),
                    time: ray_in.time,
                };
                *scattered = ray;
                *albedo = *alb;
                reflected.dot(&hit_record.normal) > 0.0
            }
            Self::Dielectric(ref_idx) => {
                *albedo = Color::ones();
                let eta_over_etai = if hit_record.front_face {
                    1.0 / *ref_idx
                } else {
                    *ref_idx
                };
                let unit = ray_in.direction.normalize();

                let cos_theta = (-unit).dot(&hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                if eta_over_etai * sin_theta > 1.0 {
                    let reflected = unit.reflect(&hit_record.normal);
                    *scattered = Ray {
                        origin: hit_record.point,
                        direction: reflected,
                        time: ray_in.time,
                    };
                    return true;
                }
                let reflect_prob = schlick(cos_theta, eta_over_etai);
                if rand::thread_rng().gen::<f32>() < reflect_prob {
                    let reflected = unit.reflect(&hit_record.normal);
                    *scattered = Ray {
                        origin: hit_record.point,
                        direction: reflected,
                        time: ray_in.time,
                    };
                    return true;
                }

                let refracted = unit.refract(&hit_record.normal, eta_over_etai);
                *scattered = Ray {
                    origin: hit_record.point,
                    direction: refracted,
                    time: ray_in.time,
                };

                true
            }
            Self::DiffuseLight(_) => false,
            Self::DiffuseLightTexture(_) => false,
            Self::Isotropic(alb) => {
                *scattered = Ray {
                    origin: hit_record.point,
                    direction: Vec3::random_in_unit_sphere(),
                    time: ray_in.time,
                };
                *albedo = *alb;
                true
            }
        }
    }

    #[allow(unused_variables)]
    pub fn emitted(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        u: f32,
        v: f32,
        point: Point,
    ) -> Color {
        if hit_record.front_face {
            match self {
                Self::Normal => hit_record.normal,
                Self::DiffuseLight(emit) => *emit,
                Self::DiffuseLightTexture(emit) => emit(u, v, point),
                _ => Color::zeros(),
            }
        } else {
            Color::zeros()
        }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian(albedo) => Some(ScatterRecord::Scatter {
                attenuation: *albedo,
                pdf: PDF::Cosine(ONB::build_from_w(hit_record.normal)),
            }),
            Self::LambertianTexture(albedo) => Some(ScatterRecord::Scatter {
                attenuation: albedo(hit_record.u, hit_record.v, hit_record.point),
                pdf: PDF::Cosine(ONB::build_from_w(hit_record.normal)),
            }),
            Self::Metal(albedo, fuzz) => {
                let reflected = ray_in.direction.normalize().reflect(&hit_record.normal);

                Some(ScatterRecord::Specular {
                    specular_ray: Ray {
                        origin: hit_record.point,
                        direction: reflected + *fuzz * Vec3::random_in_unit_sphere(),
                        time: 0.,
                    },
                    attenuation: *albedo,
                })
            }
            Self::Dielectric(ref_idx) => {
                let attenuation = Color::ones();
                let eta_over_etai = if hit_record.front_face {
                    1.0 / *ref_idx
                } else {
                    *ref_idx
                };
                let unit = ray_in.direction.normalize();

                let cos_theta = (-unit).dot(&hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                if eta_over_etai * sin_theta > 1.0 {
                    let specular_ray = Ray {
                        origin: hit_record.point,
                        direction: unit.reflect(&hit_record.normal),
                        time: ray_in.time,
                    };
                    return Some(ScatterRecord::Specular {
                        specular_ray,
                        attenuation,
                    });
                }
                let reflect_prob = schlick(cos_theta, eta_over_etai);
                if rand::thread_rng().gen::<f32>() < reflect_prob {
                    let specular_ray = Ray {
                        origin: hit_record.point,
                        direction: unit.reflect(&hit_record.normal),
                        time: ray_in.time,
                    };
                    return Some(ScatterRecord::Specular {
                        specular_ray,
                        attenuation,
                    });
                }

                let specular_ray = Ray {
                    origin: hit_record.point,
                    direction: unit.refract(&hit_record.normal, eta_over_etai),
                    time: ray_in.time,
                };
                return Some(ScatterRecord::Specular {
                    specular_ray,
                    attenuation,
                });
            }
            Self::Isotropic(albedo) => {
                let specular_ray = Ray {
                    origin: hit_record.point,
                    direction: Vec3::random_in_unit_sphere(),
                    time: ray_in.time,
                };
                Some(ScatterRecord::Specular {
                    specular_ray,
                    attenuation: *albedo,
                })
            }
            _ => None,
        }
    }

    #[allow(unused_variables)]
    pub fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        match self {
            Self::Lambertian(_albedo) => {
                let cosine = hit_record.normal.dot(&scattered.direction.normalize());
                if cosine < 0. {
                    0.
                } else {
                    cosine / PI
                }
            }
            _ => 0.,
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub enum ScatterRecord<'a> {
    Specular {
        specular_ray: Ray,
        attenuation: Color,
    },
    Scatter {
        pdf: PDF<'a>,
        attenuation: Color,
    },
}
