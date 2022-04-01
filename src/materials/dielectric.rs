use glam::Vec3;

use crate::{
    random::random_distribution,
    utils::{Color, BLACK, WHITE},
    world::physics::{Intersection, Ray},
};

use super::{
    material::{Material, MaterialType},
    metal::Metal,
};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> MaterialType {
        MaterialType::Dielectric(Dielectric {
            ir: index_of_refraction,
        })
    }

    pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-uv).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
        let r_our_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
        r_out_perp + r_our_parallel
    }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        let outward_norm = inter.outward_normal;
        let normal;

        let attenuation = WHITE;
        let refraction_r;

        if ray.front_face(outward_norm) {
            refraction_r = 1.0 / self.ir;
            normal = outward_norm;
        } else {
            refraction_r = self.ir;
            normal = -outward_norm;
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_r * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_r) > random_distribution()
        {
            Metal::reflect(unit_direction, normal)
        } else {
            Dielectric::refract(unit_direction, normal, refraction_r)
        };
        Some((attenuation, Ray::new(inter.point, direction)))
    }

    fn emitted(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        BLACK
    }

    fn albedo(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        BLACK
    }
}
