use super::{object::ObjectType, triangle::Triangle};
use crate::materials::material::MaterialType;
use glam::Vec3;
use std::path::Path;

extern crate tobj;

pub fn load_obj(path: &Path, origin: Vec3, scale: f32, material: MaterialType) -> Vec<ObjectType> {
    let obj = tobj::load_obj(path, &tobj::LoadOptions::default());
    let (models, _mtls) = obj.unwrap();
    let mut tris: Vec<ObjectType> = Vec::new();

    for m in models.iter() {
        let mesh = &m.mesh;
        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;
            let v0 = Vec3::new(
                (mesh.positions[i0 * 3] * scale) + origin.x,
                (mesh.positions[i0 * 3 + 1] * scale) + origin.y,
                (mesh.positions[i0 * 3 + 2] * scale) + origin.z,
            );
            let v1 = Vec3::new(
                (mesh.positions[i1 * 3] * scale) + origin.x,
                (mesh.positions[i1 * 3 + 1] * scale) + origin.y,
                (mesh.positions[i1 * 3 + 2] * scale) + origin.z,
            );
            let v2 = Vec3::new(
                (mesh.positions[i2 * 3] * scale) + origin.x,
                (mesh.positions[i2 * 3 + 1] * scale) + origin.y,
                (mesh.positions[i2 * 3 + 2] * scale) + origin.z,
            );

            tris.push(Triangle::new(v0, v1, v2, material));
        }
    }

    tris
}

pub fn load_obj_spec(
    path: &Path,
    origin: Vec3,
    scale: f32,
    material: MaterialType,
) -> Vec<ObjectType> {
    let obj = tobj::load_obj(path, &tobj::LoadOptions::default());
    let (models, _mtls) = obj.unwrap();
    let mut tris: Vec<ObjectType> = Vec::new();

    for m in models.iter() {
        let mesh = &m.mesh;
        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;
            let v0 = Vec3::new(
                (mesh.positions[i0 * 3] * scale) + origin.x,
                (mesh.positions[i0 * 3 + 2] * scale) + origin.z,
                (mesh.positions[i0 * 3 + 1] * scale) + origin.y,
            );
            let v1 = Vec3::new(
                (mesh.positions[i1 * 3] * scale) + origin.x,
                (mesh.positions[i1 * 3 + 2] * scale) + origin.z,
                (mesh.positions[i1 * 3 + 1] * scale) + origin.y,
            );
            let v2 = Vec3::new(
                (mesh.positions[i2 * 3] * scale) + origin.x,
                (mesh.positions[i2 * 3 + 2] * scale) + origin.z,
                (mesh.positions[i2 * 3 + 1] * scale) + origin.y,
            );

            tris.push(Triangle::new(v0, v1, v2, material));
        }
    }

    tris
}
