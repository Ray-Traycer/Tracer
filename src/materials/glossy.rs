use glam::Vec3;

use crate::{
    random::{random_distribution, random_sphere_distribution},
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
        let uv = inter.uv;
        let normal = texture.adjusted_normal(uv, inter.normal);
        let unit_direction = ray.direction.normalize();
        let color = texture.get_color_uv(uv, inter.point);

        if random_distribution() < self.sheen {
            let reflected = Metal::reflect(unit_direction, normal);
            Some(ScatterType::Specular {
                specular: Ray::new(
                    inter.point,
                    reflected + self.roughness * random_sphere_distribution().normalize(),
                ),
                attenuation: color,
            })
        } else {
            Some(ScatterType::Scatter {
                pdf: PDF::cosine(normal),
                attenuation: color,
            })
        }
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }
}
