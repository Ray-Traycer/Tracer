use glam::Vec3;

use crate::{
    utils::{sampling::PDF, Color},
    world::physics::{Intersection, Ray},
};

use super::{
    material::{Material, MaterialType, ScatterType},
    texture::Texture,
    PtrExtension, TexturePtr,
};

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    texture: TexturePtr,
}

impl Lambertian {
    pub fn new(texture: TexturePtr) -> MaterialType {
        MaterialType::Lambertian(Self { texture })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, inter: &Intersection) -> Option<ScatterType> {
        let texture = self.texture.deref();
        let normal = texture.adjusted_normal(inter.uv, inter.normal);

        Some(ScatterType::Scatter {
            pdf: PDF::cosine(normal),
            attenuation: texture.get_color_uv(inter.uv, inter.point),
        })
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }

    fn scattering_pdf(&self, inter: &Intersection, scattered: &Ray) -> f32 {
        inter.normal.dot(scattered.direction.normalize()).max(0.0) / std::f32::consts::PI
    }
}
