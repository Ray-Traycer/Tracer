use crate::{
    materials::{
        material::{self, MaterialType},
        texture,
    },
    utils::aabb::Aabb,
    world::physics::{Intersection, Ray},
};

use glam::Vec3;
use std::f32::consts::PI;

use super::object::{Bounded, Geometry, ObjectType};

pub struct Sphere {
    pub material: MaterialType,
    pub center: Vec3,
    pub radius: f32,
    pub node_index: usize,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialType) -> ObjectType {
        ObjectType::Sphere(Sphere {
            center,
            radius,
            material,
            node_index: 0,
        })
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();

        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = self.outward_normal(point);

        // let texture = self.material.

        Some(Intersection::new(
            root,
            point,
            self.surface_normal(point, ray),
            outward_normal,
            self.material,
            self.surface_uv(outward_normal),
        ))
    }

    fn surface_normal(&self, p: Vec3, _ray: &Ray) -> Vec3 {
        (p - self.center).normalize()
    }

    fn outward_normal(&self, p: Vec3) -> Vec3 {
        (p - self.center) / self.radius
    }

    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32) {
        let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;
        let theta = (-outward_normal.y).acos();
        (phi / (2.0 * PI), theta / PI)
    }
}
