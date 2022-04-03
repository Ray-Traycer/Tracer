use std::f32::consts::PI;

use glam::Vec3;

use crate::{materials::texture::PixelMap, utils::Color};
pub type HDRI = PixelMap;

impl HDRI {
    fn surface_uv(&self, point: Vec3) -> (f32, f32) {
        let phi = (-point.z).atan2(point.x) + PI;
        let theta = (-point.y).acos();
        (phi / (2.0 * PI), theta / PI)
    }

    fn dir_color(&self, dir: Vec3) -> Color {
        self.get_pixel_uv(self.surface_uv(dir.normalize()))
    }

    pub fn value(&self, dir: Vec3) -> f32 {
        0.0
    }

    pub fn generate(&self) -> Vec3 {
        Vec3::ZERO
    }
}

struct SkyMap {
    hdri: HDRI,
}
