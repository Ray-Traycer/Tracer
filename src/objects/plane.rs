use crate::{
    materials::material::MaterialType,
    utils::{aabb::Aabb, sampling::PdfReady},
    world::physics::{Intersection, Ray},
};

use super::object::{Bounded, Geometry, ObjectType};
use glam::Vec3;
use rand::Rng;

#[derive(Clone)]

pub enum PlaneType {
    YZ,
    ZX,
    XY,
}

#[derive(Clone)]
pub struct Plane {
    plane_type: PlaneType,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: MaterialType,
}

impl Plane {
    pub fn new(
        plane_type: PlaneType,
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        material: MaterialType,
    ) -> ObjectType {
        ObjectType::Plane(Plane {
            plane_type,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        })
    }

    pub fn get_axis(plane_type: &PlaneType) -> (usize, usize, usize) {
        match plane_type {
            PlaneType::YZ => (0, 1, 2),
            PlaneType::ZX => (1, 2, 0),
            PlaneType::XY => (2, 0, 1),
        }
    }
}

impl Geometry for Plane {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let (k_axis, a_axis, b_axis) = Plane::get_axis(&self.plane_type);
        let t = (self.k - ray.origin[k_axis]) / ray.direction[k_axis];

        if t < t_min || t > t_max {
            None
        } else {
            let a = ray.origin[a_axis] + t * ray.direction[a_axis];
            let b = ray.origin[b_axis] + t * ray.direction[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let point = ray.at(t);
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);

                Some(Intersection::new(
                    t,
                    point,
                    self.surface_normal(point, ray),
                    point,
                    self.material,
                    (u, v),
                ))
            }
        }
    }

    fn surface_normal(&self, _point: Vec3, ray: &Ray) -> Vec3 {
        let (k_axis, _a_axis, _b_axis) = Plane::get_axis(&self.plane_type);
        let mut normal = Vec3::ZERO;
        normal[k_axis] = 1.0;

        if ray.origin[k_axis] > self.k {
            normal[k_axis] = 1.0;
        } else {
            normal[k_axis] = -1.0;
        }

        normal
    }

    fn outward_normal(&self, point: Vec3) -> Vec3 {
        point
    }

    fn surface_uv(&self, point: Vec3) -> (f32, f32) {
        let (_k_axis, a_axis, b_axis) = Plane::get_axis(&self.plane_type);
        (
            (point[a_axis] - self.a0) / (self.a1 - self.a0),
            (point[b_axis] - self.b0) / (self.b1 - self.b0),
        )
    }
}

impl Bounded for Plane {
    fn bounding_box(&self) -> Option<Aabb> {
        match self.plane_type {
            PlaneType::YZ => Some(Aabb {
                min: Vec3::new(self.k - 1e-4, self.a0, self.b0),
                max: Vec3::new(self.k + 1e-4, self.a1, self.b1),
            }),
            PlaneType::ZX => Some(Aabb {
                min: Vec3::new(self.b0, self.k - 1e-4, self.a0),
                max: Vec3::new(self.b1, self.k + 1e-4, self.a1),
            }),
            PlaneType::XY => Some(Aabb {
                min: Vec3::new(self.a0, self.b0, self.k - 1e-4),
                max: Vec3::new(self.a1, self.b1, self.k + 1e-4),
            }),
        }
    }
}

impl PdfReady for Plane {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        if let Some(hit) = self.intersects(&Ray::new(o, v), 0.001, f32::MAX) {
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);
            let distance_squared = hit.distance.powi(2) * v.length_squared();
            let cosine = v.dot(hit.normal).abs() / v.length();
            if cosine != 0.0 {
                distance_squared / (cosine * area)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let (k_axis, a_axis, b_axis) = Plane::get_axis(&self.plane_type);
        let mut random_point = Vec3::ZERO;
        random_point[a_axis] = rng.gen_range(self.a0..self.a1);
        random_point[b_axis] = rng.gen_range(self.b0..self.b1);
        random_point[k_axis] = self.k;
        random_point - o
    }
}
