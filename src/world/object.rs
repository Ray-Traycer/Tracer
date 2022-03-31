use std::f32::consts::PI;

use bvh::{bounding_hierarchy::BHShape, aabb::{Bounded, AABB}};
use enum_dispatch::enum_dispatch;
use glam::Vec3;

use crate::utils::material::Material;

use super::physics::{Ray, Intersection};


#[enum_dispatch(ObjectType)]
pub trait Geometry{
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
    fn surface_normal(&self, p: Vec3, r: &Ray) -> Vec3;
    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32);
    fn outward_normal(&self, p: Vec3) -> Vec3;
}

pub struct Sphere{
    pub material: Material,
    pub center: Vec3,
    pub radius: f32,
    pub node_index: usize
}

impl Sphere{
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self{
        Sphere{
            center,
            radius,
            material,
            node_index: 0,
        }
    }
}
impl Bounded for Sphere{
    fn aabb(&self) -> AABB{
        let min = self.center - Vec3::new(self.radius, self.radius, self.radius);
        let max = self.center + Vec3::new(self.radius, self.radius, self.radius);
        AABB::with_bounds(min, max)
    }
}
impl BHShape for Sphere{
    fn set_bh_node_index(&mut self, i: usize) {
        self.node_index = i;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Geometry for Sphere{
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
        (p - self.center)/self.radius
    }

    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32) {
        let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;
        let theta = (- outward_normal.y).acos();
        (phi / (2.0 * PI), theta / PI)
    }
}

#[enum_dispatch]
pub enum ObjectType {
    Sphere
}