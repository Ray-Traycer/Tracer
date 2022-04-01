use glam::Vec3;

use crate::{
    random::random_sphere_distribution,
    utils::{Color, Vec3Extension, BLACK},
    world::physics::{Intersection, Ray},
};

use super::{
    material::{Material, MaterialType},
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
    fn scatter(&self, _ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        let normal = inter.normal;
        let mut scatter_dir = inter.point + normal + random_sphere_distribution().normalize();

        if scatter_dir.near_zero() {
            scatter_dir = normal;
        }
        let texture = self.texture.deref();

        Some((
            texture.get_color_uv(inter.uv, inter.point),
            Ray::new(inter.point, scatter_dir - inter.point),
        ))
    }

    fn emitted(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        BLACK
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }
}
