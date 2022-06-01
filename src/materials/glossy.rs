use glam::Vec3;

use crate::{
    random::{random_distribution, random_in_unit_disk, random_sphere_distribution},
    utils::{sampling::PDF, Color},
    world::physics::{Intersection, Ray},
};

use super::{
    material::{Material, MaterialType, ScatterType},
    metal::Metal,
    texture::Texture,
    PtrExtension, TexturePtr,
};

#[derive(Copy, Clone, Debug)]
pub struct Glossy {
    texture: TexturePtr,
    sheen: f32,
    roughness: f32,
}

impl Glossy {
    pub fn new(texture: TexturePtr, sheen: f32, roughness: f32) -> MaterialType {
        MaterialType::Glossy(Self {
            texture,
            sheen,
            roughness,
        })
    }
}

impl Material for Glossy {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<ScatterType> {
        let texture = self.texture.deref();
        let normal = texture.adjusted_normal(inter.uv, inter.normal);

        let reflected = Metal::reflect(ray.direction.normalize(), normal);
        let uv = inter.uv;

        if reflected.dot(normal) > 0.0 {
            let reflected =
                reflected + (1.0 - self.sheen) * random_sphere_distribution().normalize();
            let scattered = Ray::new(
                inter.point,
                reflected + self.roughness * random_in_unit_disk(),
            );

            Some(ScatterType::Glossy {
                specular: scattered,
                pdf: PDF::cosine(normal),
                attenuation: texture.get_color_uv(uv, inter.point),
            })
        } else {
            None
        }
    }

    fn scattering_pdf(&self, inter: &Intersection, scattered: &Ray) -> f32 {
        inter.normal.dot(scattered.direction.normalize()).max(0.0) / std::f32::consts::PI
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }
}
