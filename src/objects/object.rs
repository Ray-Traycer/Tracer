use glam::Vec3;

use crate::{
    utils::{aabb::Aabb, sampling::PdfReady},
    world::physics::{Intersection, Ray},
};

use super::{plane::Plane, rotated::Rotated, sphere::Sphere, triangle::Triangle};

#[derive(Clone)]
pub enum ObjectType {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
    Rotated(Rotated),
}

pub trait Geometry {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
    fn surface_normal(&self, p: Vec3, r: &Ray) -> Vec3;
    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32);
    fn outward_normal(&self, p: Vec3) -> Vec3;
}

pub trait Bounded {
    fn bounding_box(&self) -> Option<Aabb>;
}

impl Geometry for ObjectType {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        match self {
            ObjectType::Sphere(obj) => obj.intersects(ray, t_min, t_max),
            ObjectType::Plane(obj) => obj.intersects(ray, t_min, t_max),
            ObjectType::Triangle(obj) => obj.intersects(ray, t_min, t_max),
            ObjectType::Rotated(obj) => obj.intersects(ray, t_min, t_max),
        }
    }

    fn surface_normal(&self, p: Vec3, r: &Ray) -> Vec3 {
        match self {
            ObjectType::Sphere(obj) => obj.surface_normal(p, r),
            ObjectType::Plane(obj) => obj.surface_normal(p, r),
            ObjectType::Triangle(obj) => obj.surface_normal(p, r),
            ObjectType::Rotated(obj) => obj.surface_normal(p, r),
        }
    }

    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32) {
        match self {
            ObjectType::Sphere(obj) => obj.surface_uv(outward_normal),
            ObjectType::Plane(obj) => obj.surface_uv(outward_normal),
            ObjectType::Triangle(obj) => obj.surface_uv(outward_normal),
            ObjectType::Rotated(obj) => obj.surface_uv(outward_normal),
        }
    }

    fn outward_normal(&self, p: Vec3) -> Vec3 {
        match self {
            ObjectType::Sphere(obj) => obj.outward_normal(p),
            ObjectType::Plane(obj) => obj.outward_normal(p),
            ObjectType::Triangle(obj) => obj.outward_normal(p),
            ObjectType::Rotated(obj) => obj.outward_normal(p),
        }
    }
}

impl Bounded for ObjectType {
    fn bounding_box(&self) -> Option<Aabb> {
        match self {
            ObjectType::Sphere(obj) => obj.bounding_box(),
            ObjectType::Plane(obj) => obj.bounding_box(),
            ObjectType::Triangle(obj) => obj.bounding_box(),
            ObjectType::Rotated(obj) => obj.bounding_box(),
        }
    }
}

impl PdfReady for ObjectType {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        match self {
            ObjectType::Sphere(obj) => obj.pdf_value(o, v),
            ObjectType::Plane(obj) => obj.pdf_value(o, v),
            ObjectType::Triangle(obj) => obj.pdf_value(o, v),
            ObjectType::Rotated(obj) => obj.pdf_value(o, v),
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        match self {
            ObjectType::Sphere(obj) => obj.random(o),
            ObjectType::Plane(obj) => obj.random(o),
            ObjectType::Triangle(obj) => obj.random(o),
            ObjectType::Rotated(obj) => obj.random(o),
        }
    }
}
