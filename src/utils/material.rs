use glam::Vec3;

use crate::{
    random::random_sphere_distribution,
    world::physics::{Intersection, Ray},
};

use super::{Color, Vec3Extension, BLACK};

#[derive(Clone, Copy)]
pub struct Material {
    color: Color,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn scatter(&self, _ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        let normal = inter.normal;
        let mut scatter_dir = inter.point + normal + random_sphere_distribution().normalize();
        // let outward_normal = inter.outward_normal;
        // let uv = inter.uv; #NOTE: uv is not used currently, make sure to implement texture mapping

        if scatter_dir.near_zero() {
            scatter_dir = normal;
        }

        Some((self.color, Ray::new(inter.point, scatter_dir - inter.point)))
    }

    pub fn emitted(&self, _uv: (f32, f32)) -> Color {
        BLACK
    }

    pub fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.color
    }
}
