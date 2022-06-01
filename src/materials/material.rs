use glam::Vec3;

use crate::{
    utils::{sampling::PDF, Color},
    world::physics::{Intersection, Ray},
};

use super::{
    dielectric::Dielectric, emissivediffuse::EmissiveDiffuse, glossy::Glossy,
    lambertian::Lambertian, metal::Metal,
};

pub enum ScatterType<'a> {
    Specular {
        specular: Ray,
        attenuation: Vec3,
    },
    Scatter {
        pdf: PDF<'a>,
        attenuation: Vec3,
    },
    Glossy {
        pdf: PDF<'a>,
        attenuation: Vec3,
        specular: Ray,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    EmissiveDiffuse(EmissiveDiffuse),
    Glossy(Glossy),
}

pub trait Material {
    fn scatter(&self, _ray: &Ray, _inter: &Intersection) -> Option<ScatterType> {
        None
    }
    fn emitted(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        Color::ZERO
    }
    fn albedo(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        Color::ZERO
    }
    fn scattering_pdf(&self, _inter: &Intersection, _scattered: &Ray) -> f32 {
        1.0
    }
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<ScatterType> {
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

    fn scattering_pdf(&self, inter: &Intersection, scattered: &Ray) -> f32 {
        match self {
            MaterialType::Lambertian(mat) => mat.scattering_pdf(inter, scattered),
            MaterialType::Metal(mat) => mat.scattering_pdf(inter, scattered),
            MaterialType::Dielectric(mat) => mat.scattering_pdf(inter, scattered),
            MaterialType::EmissiveDiffuse(mat) => mat.scattering_pdf(inter, scattered),
            MaterialType::Glossy(mat) => mat.scattering_pdf(inter, scattered),
        }
    }
}
