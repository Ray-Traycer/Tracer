use glam::Vec3;

use crate::{
    random::{random_distribution, random_sphere_distribution},
    utils::{Color, Vec3Extension, BLACK},
    world::physics::{Intersection, Ray},
};

use super::{
    material::{Material, MaterialType},
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
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        let normal = inter.normal;
        let mut scatter_dir = inter.point + normal + random_sphere_distribution().normalize();
        let uv = inter.uv;
        let unit_direction = ray.direction.normalize();
        let color = self.texture.deref().get_color_uv(uv, inter.point);

        if random_distribution() > self.sheen {
            let normal = inter.normal;
            let reflected = Metal::reflect(unit_direction, normal);
            Some((
                color,
                Ray::new(
                    inter.point,
                    reflected + self.roughness * random_sphere_distribution().normalize(),
                ),
            ))
        } else {
            if scatter_dir.near_zero() {
                scatter_dir = normal;
            }

            Some((color, Ray::new(inter.point, scatter_dir - inter.point)))
        }
    }

    fn emitted(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        BLACK
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }
}
