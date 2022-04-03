use glam::Vec3;

use crate::utils::Color;

use super::{
    material::{Material, MaterialType},
    texture::Texture,
    PtrExtension, TexturePtr,
};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct EmissiveDiffuse {
    texture: TexturePtr,
}

impl EmissiveDiffuse {
    pub fn new(texture: TexturePtr) -> MaterialType {
        MaterialType::EmissiveDiffuse(EmissiveDiffuse { texture })
    }
}

impl Material for EmissiveDiffuse {
    fn emitted(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.deref().get_color_uv(uv, point)
    }
}
