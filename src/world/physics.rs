use glam::Vec3;

use crate::utils::{BLACK, Color, material::Material};

use super::world::WorldObjects;
use crate::world::object::Geometry;

pub struct Intersection{
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub outward_normal: Vec3,
    pub uv: (f32, f32),
    pub material: Material
}

impl Intersection{
    pub fn new(distance: f32, point: Vec3, normal: Vec3, outward_normal: Vec3, material: Material, uv: (f32, f32)) -> Self{
        Self{
            distance,
            point,
            normal,
            outward_normal,
            uv,
            material
        }
    }
}

pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn front_face_normal(&self, normal: Vec3) -> bool {
       self.direction.dot(normal) < 0.0 
    }

    pub fn trace(&self, world_objects: &WorldObjects, t_min: f32, t_max: f32) -> Option<Intersection> {
        world_objects.iter()
        .filter_map(|obj| obj.intersect(self, t_min, t_max))
        .min_by(|inter_one, inter_two| inter_one.distance.partial_cmp(&inter_two.distance).unwrap_or(std::cmp::Ordering::Equal))
    }
    pub fn color(&self, world_objects: &WorldObjects, background_color: Color, depth: u32) -> Color {
        if depth <= 0{return BLACK;}
        match self.trace(&world_objects, 0.001, ::std::f32::MAX){
            Some(intersection) => {
                let material = &intersection.material;
                let emitted =  material.emitted(intersection.uv);
                return match material.scatter(self, &intersection) {
                    Some((attenuation, scattered)) => emitted + attenuation * scattered.color(world_objects, background_color, depth - 1),
                    None => emitted
                }
            },
            None => background_color
        }
    }

}