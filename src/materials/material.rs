use glam::Vec3;

use crate::{
    utils::Color,
    world::physics::{Intersection, Ray},
};

use super::{
    dielectric::Dielectric, emissivediffuse::EmissiveDiffuse, glossy::Glossy,
    lambertian::Lambertian, metal::Metal,
};

#[derive(Copy, Clone, Debug)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    EmissiveDiffuse(EmissiveDiffuse),
    Glossy(Glossy),
}

pub trait Material {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)>;
    fn emitted(&self, uv: (f32, f32), point: Vec3) -> Color;
    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color;
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        match self {
            MaterialType::Lambertian(mat) => mat.scatter(ray, inter),
            MaterialType::Metal(mat) => mat.scatter(ray, inter),
            MaterialType::Dielectric(mat) => mat.scatter(ray, inter),
            MaterialType::EmissiveDiffuse(mat) => mat.scatter(ray, inter),
            MaterialType::Glossy(mat) => mat.scatter(ray, inter),
        }
    }

    fn emitted(&self, uv: (f32, f32), point: Vec3) -> Color {
        match self {
            MaterialType::Lambertian(mat) => mat.emitted(uv, point),
            MaterialType::Metal(mat) => mat.emitted(uv, point),
            MaterialType::Dielectric(mat) => mat.emitted(uv, point),
            MaterialType::EmissiveDiffuse(mat) => mat.emitted(uv, point),
            MaterialType::Glossy(mat) => mat.emitted(uv, point),
        }
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        match self {
            MaterialType::Lambertian(mat) => mat.albedo(uv, point),
            MaterialType::Metal(mat) => mat.albedo(uv, point),
            MaterialType::Dielectric(mat) => mat.albedo(uv, point),
            MaterialType::EmissiveDiffuse(mat) => mat.albedo(uv, point),
            MaterialType::Glossy(mat) => mat.albedo(uv, point),
        }
    }
}
