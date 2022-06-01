use crate::{
    materials::{
        material::{Material, MaterialType, ScatterType},
        texture::PixelMap,
    },
    utils::{bvh::BvhTree, sampling::PDF, Color, BLACK},
};
use glam::Vec3;

use super::WorldLights;

pub struct Intersection {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub outward_normal: Vec3,
    pub uv: (f32, f32),
    pub material: MaterialType,
}

impl Intersection {
    pub fn new(
        distance: f32,
        point: Vec3,
        normal: Vec3,
        outward_normal: Vec3,
        material: MaterialType,
        uv: (f32, f32),
    ) -> Self {
        Self {
            distance,
            point,
            normal,
            outward_normal,
            uv,
            material,
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn front_face(&self, normal: Vec3) -> bool {
        self.direction.dot(normal) < 0.0
    }

    pub fn color(
        &self,
        world_objects: &BvhTree,
        light_objects: &WorldLights,
        skybox: &PixelMap,
        depth: u32,
    ) -> Color {
        if depth <= 0 {
            return BLACK;
        }

        match world_objects.hit(self, 0.001, ::std::f32::MAX) {
            Some(intersection) => {
                let emitted = intersection
                    .material
                    .emitted(intersection.uv, intersection.point);

                match intersection.material.scatter(self, &intersection) {
                    Some(scatter_type) => match scatter_type {
                        ScatterType::Specular {
                            specular,
                            attenuation,
                        } => {
                            return attenuation
                                * specular.color(world_objects, light_objects, skybox, depth - 1)
                        }
                        ScatterType::Scatter { pdf, attenuation } => {
                            let geo_pdf = PDF::lights(light_objects, intersection.point);
                            let pdf_func = PDF::mixture(&geo_pdf, &pdf);
                            let scattered = Ray::new(intersection.point, pdf_func.generate());
                            let pdf_val = pdf_func.value(scattered.direction);
                            let scattering_pdf = intersection
                                .material
                                .scattering_pdf(&intersection, &scattered);
                            return emitted
                                + attenuation
                                    * scattering_pdf
                                    * scattered.color(
                                        world_objects,
                                        light_objects,
                                        skybox,
                                        depth - 1,
                                    )
                                    / pdf_val;
                        }
                        ScatterType::Glossy {
                            pdf,
                            attenuation,
                            specular,
                        } => {
                            let geo_pdf = PDF::lights(light_objects, intersection.point);
                            let pdf_func = PDF::mixture(&geo_pdf, &pdf);
                            let scattered = Ray::new(intersection.point, pdf_func.generate());
                            let pdf_val = pdf_func.value(scattered.direction);
                            let scattering_pdf = intersection
                                .material
                                .scattering_pdf(&intersection, &scattered);
                            return emitted
                                + attenuation
                                    * scattering_pdf
                                    * scattered.color(
                                        world_objects,
                                        light_objects,
                                        skybox,
                                        depth - 1,
                                    )
                                    / pdf_val
                                + specular.color(world_objects, light_objects, skybox, depth - 1);
                        }
                    },
                    None => emitted,
                }
            }
            None => skybox.dir_color(self.direction),
        }
    }
}
