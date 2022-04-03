use std::f32::consts::PI;

use glam::Vec3;
use rand::prelude::SliceRandom;

use crate::{
    objects::object::ObjectType, random::random_sphere_distribution, utils::sampling::PdfReady,
};

pub mod camera;
pub mod physics;
pub mod world;

pub type WorldObjects = Vec<ObjectType>;
pub type WorldLights = Vec<ObjectType>;

impl PdfReady for WorldLights {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        let n = self.len();

        if n == 0 {
            return 0.25 / PI;
        } else {
            let mut sum = 0.0_f32;
            for light in self {
                sum += light.pdf_value(o, v)
            }

            return sum / (n as f32);
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let n = self.len();

        return if n == 0 {
            random_sphere_distribution().normalize()
        } else {
            self.choose(&mut rand::thread_rng()).unwrap().random(o)
        };
    }
}
