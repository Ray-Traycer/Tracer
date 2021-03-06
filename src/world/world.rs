use std::path::Path;

use glam::Vec3;

use crate::{
    materials::{material::MaterialType, texture::PixelMap},
    objects::{
        obj::{load_obj, load_obj_spec},
        object::ObjectType,
        rotated::{Axis, Rotated},
    },
    utils::{Color, RenderedImage},
};

use super::{camera::Camera, WorldLights, WorldObjects};

pub struct World {
    pub objects: WorldObjects,
    pub lights: WorldLights,
    pub background: Color,
    pub width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub skybox: PixelMap,
}

impl World {
    pub fn new(skybox: PixelMap) -> Self {
        World {
            objects: vec![],
            lights: vec![],
            background: Color::new(0.3, 0.3, 0.35),
            width: 800,
            samples_per_pixel: 128,
            max_depth: 50,
            skybox,
        }
    }
    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }
    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn add(&mut self, object: ObjectType) {
        self.objects.push(object);
    }

    pub fn add_object_rot(
        &mut self,
        path: &Path,
        origin: Vec3,
        scale: f32,
        axis: Axis,
        angle: f32,
        material: MaterialType,
    ) {
        load_obj(path, origin, scale, material)
            .into_iter()
            .for_each(|tri| self.objects.push(Rotated::new(axis, tri, angle)));
    }

    pub fn add_object(&mut self, path: &Path, origin: Vec3, scale: f32, material: MaterialType) {
        load_obj(path, origin, scale, material)
            .into_iter()
            .for_each(|tri| self.objects.push(tri));
    }

    pub fn add_light(&mut self, object: ObjectType) {
        self.objects.push(object.clone());
        self.lights.push(object);
    }

    pub fn render(mut self, camera: Camera) -> RenderedImage {
        camera.render_threaded(&mut self)
    }
}
