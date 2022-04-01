use glam::Vec3;

use crate::{
    utils::{aabb::Aabb, material::Material},
    world::physics::{Intersection, Ray},
};

use super::{plane::Plane, sphere::Sphere};

pub enum ObjectType {
    Sphere(Sphere),
    Plane(Plane),
}

pub trait Geometry {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
    fn surface_normal(&self, p: Vec3, r: &Ray) -> Vec3;
    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32);
    fn outward_normal(&self, p: Vec3) -> Vec3;
}

pub trait Bounded {
    fn bounding_box(&self) -> Option<Aabb>;
}

impl Geometry for ObjectType {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        match self {
            ObjectType::Sphere(obj) => obj.intersect(ray, t_min, t_max),
            ObjectType::Plane(obj) => obj.intersect(ray, t_min, t_max),
        }
    }

    fn surface_normal(&self, p: Vec3, r: &Ray) -> Vec3 {
        match self {
            ObjectType::Sphere(obj) => obj.surface_normal(p, r),
            ObjectType::Plane(obj) => obj.surface_normal(p, r),
        }
    }

    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32) {
        match self {
            ObjectType::Sphere(obj) => obj.surface_uv(outward_normal),
            ObjectType::Plane(obj) => obj.surface_uv(outward_normal),
        }
    }

    fn outward_normal(&self, p: Vec3) -> Vec3 {
        match self {
            ObjectType::Sphere(obj) => obj.outward_normal(p),
            ObjectType::Plane(obj) => obj.outward_normal(p),
        }
    }
}

impl Bounded for ObjectType {
    fn bounding_box(&self) -> Option<Aabb> {
        match self {
            ObjectType::Sphere(obj) => obj.bounding_box(),
            ObjectType::Plane(obj) => obj.bounding_box(),
        }
    }
}
