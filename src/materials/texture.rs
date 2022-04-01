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
    fn adjusted_normal(&self, uv: (f32, f32), normal: Vec3) -> Vec3;
}

impl Texture for TextureType {
    fn get_color_uv(&self, uv: (f32, f32), point: Vec3) -> Color {
        match self {
            TextureType::SolidColor(tex) => tex.get_color_uv(uv, point),
            TextureType::Image(tex) => tex.get_color_uv(uv, point),
            TextureType::CheckerBoard(tex) => tex.get_color_uv(uv, point),
        }
    }

    fn adjusted_normal(&self, uv: (f32, f32), normal: Vec3) -> Vec3 {
        match self {
            TextureType::SolidColor(tex) => tex.adjusted_normal(uv, normal),
            TextureType::CheckerBoard(_) => normal,
            TextureType::Image(_) => normal,
        }
    }
}

pub struct SolidColor {
    color: Color,
    bump_map: Option<BumpMap>,
}

impl SolidColor {
    pub fn new(color: Color, bump_map: Option<BumpMap>) -> TextureType {
        TextureType::SolidColor(SolidColor { color, bump_map })
    }
}

impl Texture for SolidColor {
    fn get_color_uv(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        self.color
    }

    fn adjusted_normal(&self, uv: (f32, f32), normal: Vec3) -> Vec3 {
        if let Some(bp) = &self.bump_map {
            return bp.adjusted_normal(uv, normal);
        }

        normal
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

pub fn clamp_uv(uv: (f32, f32), w: u32, h: u32) -> (u32, u32) {
    let u = clamp(uv.0, 0.0, 1.0);
    let v = 1.0 - clamp(uv.1, 0.0, 1.0);

    let mut i = (u * w as f32) as u32;
    let mut j = (v * h as f32) as u32;

    i = if i >= w { w - 1 } else { i };
    j = if j >= h { h - 1 } else { j };

    (i, j)
}

impl Texture for Image {
    fn get_color_uv(&self, uv: (f32, f32), _point: Vec3) -> Color {
        let (i, j) = clamp_uv(uv, self.width, self.height);
        let pixel = self.image.get_pixel(i, j);

        Color::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        )
    }

    fn adjusted_normal(&self, _uv: (f32, f32), normal: Vec3) -> Vec3 {
        normal
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

    fn adjusted_normal(&self, _uv: (f32, f32), normal: Vec3) -> Vec3 {
        normal
    }
}

pub struct BumpMap {
    bp: Vec<Vec3>,
    width: u32,
    height: u32,
}

impl BumpMap {
    pub fn from_image(image: DynamicImage) -> Option<Self> {
        let bp: Vec<Vec3> = image
            .pixels()
            .map(|(_x, _y, pixel)| {
                Vec3::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                )
            })
            .collect();

        Some(Self {
            bp,
            width: image.width(),
            height: image.height(),
        })
    }

    fn adjusted_normal(&self, uv: (f32, f32), normal: Vec3) -> Vec3 {
        normal + self.get_normal(clamp_uv(uv, self.width, self.height))
    }

    fn get_normal(&self, pos: (u32, u32)) -> Vec3 {
        self.bp[(pos.0 + self.width * pos.1) as usize]
    }
}
