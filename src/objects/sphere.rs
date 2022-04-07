use crate::{
    materials::material::MaterialType,
    utils::{
        aabb::Aabb,
        sampling::{PdfReady, ONB, UVW},
    },
    world::physics::{Intersection, Ray},
};

use glam::Vec3;
use rand::Rng;
use std::f32::consts::PI;

use super::object::{Bounded, Geometry, ObjectType};

#[derive(Clone)]
pub struct Sphere {
    material: MaterialType,
    center: Vec3,
    radius: f32,
    node_index: usize,
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
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
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
        (p - self.center) / self.radius
    }

    fn surface_uv(&self, outward_normal: Vec3) -> (f32, f32) {
        let phi = (-outward_normal.z).atan2(outward_normal.x) + PI;
        let theta = (-outward_normal.y).acos();
        (phi / (2.0 * PI), theta / PI)
    }
}

impl PdfReady for Sphere {
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        if let Some(_) = self.intersects(&Ray::new(o, v), 0.001, f32::MAX) {
            let cos_theta_max =
                (1.0 - self.radius.powi(2) / (self.center - o).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let direction = self.center - o;
        let uvw = ONB::build_from_w(direction);
        uvw.local(random_to_sphere(self.radius, direction.length_squared()))
    }
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
    let y = phi.sin() * (1.0 - z.powi(2)).sqrt();
    Vec3::new(x, y, z)
}
