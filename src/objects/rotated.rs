use std::f32::consts::PI;

use glam::Vec3;

use crate::{
    utils::{aabb::Aabb, sampling::PdfReady},
    world::physics::{Intersection, Ray},
};

use super::object::{Bounded, Geometry, ObjectType};

// yoinked from https://github.com/fralken/ray-tracing-the-rest-of-your-life/blob/master/src/rotate.rs
#[derive(Clone, Copy)]

pub enum Axis {
    X,
    Y,
    Z,
}

const X: (usize, usize, usize) = (0, 1, 2);
const Y: (usize, usize, usize) = (1, 2, 0);
const Z: (usize, usize, usize) = (2, 0, 1);

impl Axis {
    fn get_axis(&self) -> (usize, usize, usize) {
        match self {
            Axis::X => X,
            Axis::Y => Y,
            Axis::Z => Z,
        }
    }
}

#[derive(Clone)]
pub struct Rotated {
    axis: Axis,
    sin_theta: f32,
    cos_theta: f32,
    object: Box<ObjectType>,
    bbox: Option<Aabb>,
}

impl Rotated {
    pub fn new(axis: Axis, hitable: ObjectType, angle: f32) -> ObjectType {
        let (r_axis, a_axis, b_axis) = axis.get_axis();
        let radians = (PI / 180.0) * angle;
        let sin_theta = f32::sin(radians);
        let cos_theta = f32::cos(radians);
        let bbox = hitable.bounding_box().map(|mut b| {
            let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max = Vec3::new(-f32::MAX, -f32::MAX, -f32::MAX);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let r = k as f32 * b.max[r_axis] + (1 - k) as f32 * b.min[r_axis];
                        let a = i as f32 * b.max[a_axis] + (1 - i) as f32 * b.min[a_axis];
                        let b = j as f32 * b.max[b_axis] + (1 - j) as f32 * b.min[b_axis];
                        let new_a = cos_theta * a - sin_theta * b;
                        let new_b = sin_theta * a + cos_theta * b;

                        if new_a < min[a_axis] {
                            min[a_axis] = new_a
                        }
                        if new_b < min[b_axis] {
                            min[b_axis] = new_b
                        }
                        if r < min[r_axis] {
                            min[r_axis] = r
                        }

                        if new_a > max[a_axis] {
                            max[a_axis] = new_a
                        }
                        if new_b > max[b_axis] {
                            max[b_axis] = new_b
                        }
                        if r > max[r_axis] {
                            max[r_axis] = r
                        }
                    }
                }
            }
            b.min = min;
            b.max = max;
            b
        });

        ObjectType::Rotated(Self {
            axis,
            sin_theta,
            cos_theta,
            object: Box::new(hitable),
            bbox,
        })
    }
}

impl Bounded for Rotated {
    fn bounding_box(&self) -> Option<Aabb> {
        return self.bbox;
    }
}

impl Geometry for Rotated {
    fn intersects(
        &self,
        ray: &crate::world::physics::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        let (_, a_axis, b_axis) = self.axis.get_axis();
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin[a_axis] = self.cos_theta * ray.origin[a_axis] + self.sin_theta * ray.origin[b_axis];
        origin[b_axis] = -self.sin_theta * ray.origin[a_axis] + self.cos_theta * ray.origin[b_axis];
        direction[a_axis] =
            self.cos_theta * ray.direction[a_axis] + self.sin_theta * ray.direction[b_axis];
        direction[b_axis] =
            -self.sin_theta * ray.direction[a_axis] + self.cos_theta * ray.direction[b_axis];

        self.object
            .intersects(&Ray::new(origin, direction), t_min, t_max)
            .map(|mut hit| {
                let mut p = hit.point;
                let mut normal = hit.normal;
                p[a_axis] = self.cos_theta * hit.point[a_axis] - self.sin_theta * hit.point[b_axis];
                p[b_axis] = self.sin_theta * hit.point[a_axis] + self.cos_theta * hit.point[b_axis];
                normal[a_axis] =
                    self.cos_theta * hit.normal[a_axis] - self.sin_theta * hit.normal[b_axis];
                normal[b_axis] =
                    self.sin_theta * hit.normal[a_axis] + self.cos_theta * hit.normal[b_axis];
                hit.point = p;
                hit.normal = normal;
                hit
            })
    }

    fn surface_normal(&self, p: Vec3, r: &Ray) -> Vec3 {
        self.object.surface_normal(p, r)
    }

    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32) {
        self.object.surface_uv(outward_normal)
    }

    fn outward_normal(&self, p: Vec3) -> Vec3 {
        self.object.outward_normal(p)
    }
}

impl PdfReady for Rotated {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        self.object.pdf_value(o, v)
    }

    fn random(&self, o: Vec3) -> Vec3 {
        self.object.random(o)
    }
}
