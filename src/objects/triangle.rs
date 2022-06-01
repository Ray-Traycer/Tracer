use crate::{
    materials::material::MaterialType,
    utils::{aabb::Aabb, sampling::PdfReady},
    world::physics::{Intersection, Ray},
};

use glam::Vec3;
use rand::Rng;

use super::object::{Bounded, Geometry, ObjectType};

#[derive(Clone)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    normal: Vec3,
    material: MaterialType,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: MaterialType) -> ObjectType {
        ObjectType::Triangle(Triangle {
            v0,
            v1,
            v2,
            normal: (v1 - v0).cross(v2 - v0).normalize(),
            material,
        })
    }
}

impl Bounded for Triangle {
    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(
                self.v0.x.min(self.v1.x.min(self.v2.x)),
                self.v0.y.min(self.v1.y.min(self.v2.y)),
                self.v0.z.min(self.v1.z.min(self.v2.z)),
            ),
            max: Vec3::new(
                self.v0.x.max(self.v1.x.max(self.v2.x)),
                self.v0.y.max(self.v1.y.max(self.v2.y)),
                self.v0.z.max(self.v1.z.max(self.v2.z)),
            ),
        })
    }
}

impl Geometry for Triangle {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = ray.direction.cross(v0v2);
        let det = v0v1.dot(pvec);

        if det.abs() < 1e-4 {
            return None;
        }
        let inv_det = 1. / det;

        let tvec = ray.origin - self.v0;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0. || u > 1. {
            return None;
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = v0v2.dot(qvec) * inv_det;

        if t < t_min || t > t_max {
            return None;
        }

        let p = ray.at(t);

        return Some(Intersection {
            distance: t,
            point: p,
            normal: self.normal,
            outward_normal: self.outward_normal(p),
            uv: (u, v),
            material: self.material,
        });
    }

    fn surface_normal(&self, _p: Vec3, _ray: &Ray) -> Vec3 {
        self.normal
    }

    fn outward_normal(&self, p: Vec3) -> Vec3 {
        self.normal
    }

    fn surface_uv(&self, _outward_normal: Vec3) -> (f32, f32) {
        (0.0, 0.0)
    }
}

impl PdfReady for Triangle {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        if let Some(_) = self.intersects(&Ray::new(o, v), 0.001, f32::MAX) {
            // from https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=4121581
            let r1 = self.v0 - o;
            let r2 = self.v1 - o;
            let r3 = self.v2 - o;

            let r1_l = r1.length();
            let r2_l = r2.length();
            let r3_l = r3.length();

            let n = r1.dot(r2.cross(r3));
            let d = (r1_l * r2_l * r3_l)
                + (r1.dot(r2) * r3_l)
                + (r1.dot(r3) * r2_l)
                + (r2.dot(r3) * r3_l);

            1.0 / n.atan2(d)
        } else {
            0.0
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        // From https://math.stackexchange.com/questions/18686/uniform-random-point-in-triangle-in-3d
        let mut rng = rand::thread_rng();
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();

        let ca = 1.0 - r1.sqrt();
        let cb = r1.sqrt() * (1.0 - r2);
        let cc = r2 * r1.sqrt();

        (self.v0 * ca + self.v1 * cb + self.v2 * cc) - o
    }
}
