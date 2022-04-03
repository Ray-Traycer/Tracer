use glam::Vec3;

use crate::{
    random::random_sphere_distribution,
    utils::Color,
    world::physics::{Intersection, Ray},
};

use super::{
    material::{Material, MaterialType, ScatterType},
    texture::Texture,
    PtrExtension, TexturePtr,
};

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub texture: TexturePtr,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(texture: TexturePtr, fuzz: f32) -> MaterialType {
        MaterialType::Metal(Metal { texture, fuzz })
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<ScatterType> {
        let texture = self.texture.deref();
        let normal = texture.adjusted_normal(inter.uv, inter.normal);

        let reflected = Metal::reflect(ray.direction.normalize(), normal);
        let uv = inter.uv;

        if reflected.dot(normal) > 0.0 {
            Some(ScatterType::Specular {
                specular: Ray::new(
                    inter.point,
                    reflected + self.fuzz * random_sphere_distribution().normalize(),
                ),
                attenuation: texture.get_color_uv(uv, inter.point),
            })
        } else {
            None
        }
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }
}
