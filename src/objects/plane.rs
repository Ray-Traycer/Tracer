use crate::{
    utils::{aabb::Aabb, material::Material},
    world::physics::{Intersection, Ray},
};

use super::object::{Bounded, Geometry, ObjectType};
use glam::Vec3;

pub enum PlaneType {
    YZ,
    ZX,
    XY,
}

pub struct Plane {
    plane_type: PlaneType,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: Material,
}

impl Plane {
    pub fn new(
        plane_type: PlaneType,
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        material: Material,
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
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
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
