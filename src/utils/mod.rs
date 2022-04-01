use glam::Vec3;
use image::{ImageBuffer, Rgb};

pub mod aabb;
pub mod bvh;

pub mod material;

pub type Color = glam::Vec3;
pub type RenderedImage = ImageBuffer<image::Rgb<u8>, Vec<u8>>;
pub const BLACK: glam::Vec3 = Color::ZERO;
pub const WHITE: glam::Vec3 = Color::ONE;

pub trait Vec3Extension {
    fn near_zero(&self) -> bool;
    fn to_rgb(&self) -> Rgb<u8>;
    fn to_slice_u8(&self) -> [u8; 3];
}

impl Vec3Extension for Vec3 {
    fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
    fn to_rgb(&self) -> Rgb<u8> {
        Rgb([
            (self.x * 255.99) as u8,
            (self.y * 255.99) as u8,
            (self.z * 255.99) as u8,
        ])
    }

    fn to_slice_u8(&self) -> [u8; 3] {
        [
            (self.x * 255.99) as u8,
            (self.y * 255.99) as u8,
            (self.z * 255.99) as u8,
        ]
    }
}
