use std::f32::consts::PI;

use arrayvec::ArrayVec;
use glam::Vec3;

use crate::utils::Color;
use image::{DynamicImage, GenericImageView};

use super::TexturePtr;

static mut TEXTURES: ArrayVec<TextureType, 1000> = ArrayVec::new_const();

pub enum TextureType {
    SolidColor(SolidColor),
    CheckerBoard(CheckerBoard),
    Image(Image),
}

impl TextureType {
    pub fn ptr(&self) -> TexturePtr {
        self as *const TextureType as usize
    }

    pub fn ref_ptr(self) -> TexturePtr {
        if let TextureType::Image(_) = self {
            panic!("Image Textures cannot use 'ref_ptr'!")
        }

        unsafe {
            TEXTURES.push(self);
            TEXTURES[TEXTURES.len() - 1].ptr()
        }
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
            TextureType::Image(tex) => tex.adjusted_normal(uv, normal),
            TextureType::CheckerBoard(_) => normal,
        }
    }
}

pub struct SolidColor {
    color: Color,
    bump_map: Option<PixelMap>,
}

impl SolidColor {
    pub fn new(color: Color, bump_map: Option<PixelMap>) -> TexturePtr {
        TextureType::SolidColor(SolidColor { color, bump_map }).ref_ptr()
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
    image: PixelMap,
    bump_map: Option<PixelMap>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(image: DynamicImage, bump_map: Option<PixelMap>) -> TextureType {
        let width = image.width();
        let height = image.height();

        TextureType::Image(Image {
            image: PixelMap::from_image(image),
            bump_map,
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
        self.image.get_pixel(clamp_uv(uv, self.width, self.height))
    }

    fn adjusted_normal(&self, uv: (f32, f32), normal: Vec3) -> Vec3 {
        if let Some(bp) = &self.bump_map {
            return bp.adjusted_normal(uv, normal);
        }

        normal
    }
}

pub struct CheckerBoard {
    color_1: Color,
    color_2: Color,
    scale: f32,
}

impl CheckerBoard {
    pub fn new(color_1: Color, color_2: Color, scale: f32) -> TexturePtr {
        TextureType::CheckerBoard(CheckerBoard {
            color_1,
            color_2,
            scale,
        })
        .ref_ptr()
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

pub struct PixelMap {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

impl PixelMap {
    pub fn from_image(image: DynamicImage) -> Self {
        Self {
            pixels: image
                .pixels()
                .map(|(_x, _y, pixel)| {
                    Vec3::new(
                        pixel[0] as f32 / 255.0,
                        pixel[1] as f32 / 255.0,
                        pixel[2] as f32 / 255.0,
                    )
                })
                .collect(),
            width: image.width(),
            height: image.height(),
        }
    }

    pub fn from_image_hdr(image: DynamicImage) -> Self {
        Self {
            pixels: image
                .as_rgb32f()
                .unwrap()
                .pixels()
                .map(|pixel| Vec3::new(pixel[0], pixel[1], pixel[2]))
                .collect(),
            width: image.width(),
            height: image.height(),
        }
    }

    fn adjusted_normal(&self, uv: (f32, f32), normal: Vec3) -> Vec3 {
        normal + self.get_normal(clamp_uv(uv, self.width, self.height))
    }

    fn get_normal(&self, pos: (u32, u32)) -> Vec3 {
        self.pixels[(pos.0 + self.width * pos.1) as usize]
    }

    fn get_pixel(&self, pos: (u32, u32)) -> Color {
        self.pixels[(pos.0 + self.width * pos.1) as usize]
    }

    pub fn get_pixel_uv(&self, uv: (f32, f32)) -> Color {
        self.get_pixel(clamp_uv(uv, self.width, self.height))
    }
}
