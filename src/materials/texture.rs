use glam::Vec3;

use crate::utils::Color;
use image::{DynamicImage, GenericImageView};

use super::TexturePtr;

pub enum TextureType {
    SolidColor(SolidColor),
    CheckerBoard(CheckerBoard),
    Image(Image),
}

impl TextureType {
    pub fn ptr(&self) -> TexturePtr {
        self as *const TextureType as usize
    }
}

pub trait Texture {
    fn get_color_uv(&self, uv: (f32, f32), point: Vec3) -> Color;
}

impl Texture for TextureType {
    fn get_color_uv(&self, uv: (f32, f32), point: Vec3) -> Color {
        match self {
            TextureType::SolidColor(tex) => tex.get_color_uv(uv, point),
            TextureType::Image(tex) => tex.get_color_uv(uv, point),
            TextureType::CheckerBoard(tex) => tex.get_color_uv(uv, point),
        }
    }
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> TextureType {
        TextureType::SolidColor(SolidColor { color })
    }
}

impl Texture for SolidColor {
    fn get_color_uv(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        self.color
    }
}

pub struct Image {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(image: DynamicImage) -> TextureType {
        let width = image.width();
        let height = image.height();

        TextureType::Image(Image {
            image,
            width,
            height,
        })
    }
}

pub fn clamp(value: f32, lower: f32, upper: f32) -> f32 {
    value.min(upper).max(lower)
}

impl Texture for Image {
    fn get_color_uv(&self, uv: (f32, f32), _point: Vec3) -> Color {
        let u = clamp(uv.0, 0.0, 1.0);
        let v = 1.0 - clamp(uv.1, 0.0, 1.0);

        let mut i = (u * self.width as f32) as u32;
        let mut j = (v * self.height as f32) as u32;

        i = if i >= self.width { self.width - 1 } else { i };
        j = if j >= self.height { self.height - 1 } else { j };

        let pixel = self.image.get_pixel(i, j);

        Color::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        )
    }
}

pub struct CheckerBoard {
    color_1: Color,
    color_2: Color,
    scale: f32,
}

impl CheckerBoard {
    pub fn new(color_1: Color, color_2: Color, scale: f32) -> TextureType {
        TextureType::CheckerBoard(CheckerBoard {
            color_1,
            color_2,
            scale,
        })
    }
}

impl Texture for CheckerBoard {
    fn get_color_uv(&self, _uv: (f32, f32), point: Vec3) -> Color {
        let sin_v = (self.scale * point.x).sin()
            * (self.scale * point.y).sin()
            * (self.scale * point.z).sin();
        if sin_v < 0.0 {
            self.color_1
        } else {
            self.color_2
        }
    }
}
