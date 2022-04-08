#![allow(dead_code)]
use std::path::Path;

use glam::{vec3, Vec3};
use materials::{
    dielectric::Dielectric,
    emissivediffuse::EmissiveDiffuse,
    glossy::Glossy,
    lambertian::Lambertian,
    material::MaterialType,
    metal::Metal,
    texture::{CheckerBoard, Image, PixelMap, SolidColor, TextureType},
};
use objects::{
    plane::{Plane, PlaneType},
    rotated::Axis,
    sphere::Sphere,
};
use random::random_distribution;
use utils::{Color, BLACK, WHITE};
use world::{camera::Camera, world::World};

mod random;
mod utils;

mod materials;
mod objects;
mod world;

#[allow(dead_code)]
pub enum Rotate {
    None,
    R90,
    R180,
    R270,
}
use image::{io::Reader as ImageReader, DynamicImage};

#[allow(dead_code)]
pub fn load_image(image_path: &str, rotation: Rotate) -> DynamicImage {
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    match rotation {
        Rotate::R90 => img.rotate90(),
        Rotate::R180 => img.rotate180(),
        Rotate::R270 => img.rotate270(),
        _ => img,
    }
}

fn color(r: f32, g: f32, b: f32) -> Color {
    return Color::new(r, g, b);
}

fn main() {
    // let mut world = World::new(PixelMap::from_image(load_image(
    //     "christmas_photo_studio_02_4k.exr",
    //     Rotate::None,
    // )));
    let mut world = World::new(PixelMap::from_color(color(0.0, 0.0, 0.0)));
    world.add_object(
        Path::new("./objs/dragon.obj"),
        vec3(0.0, 0.0, -1.52),
        0.3,
        Axis::X,
        -90.0,
        // Metal::new(
        //     SolidColor::new(color(183.0 / 255.0, 202.0 / 255.0, 121.0 / 255.0), None),
        //     0.8,
        // ),
        // Glossy::new(
        //     SolidColor::new(color(183.0 / 255.0, 202.0 / 255.0, 121.0 / 255.0), None),
        //     0.2,
        //     0.8,
        // ),
        Dielectric::new(1.333),
    );

    world.add(Plane::new(
        PlaneType::ZX,
        -25.0,
        25.0,
        -25.0,
        25.0,
        0.0,
        Lambertian::new(SolidColor::new(
            color(216.0 / 255.0, 202.0 / 255.0, 168.0 / 255.0),
            None,
        )),
    ));

    // world.add(Sphere::new(
    //     vec3(-1.7, 3.3, 0.0),
    //     0.14,
    //     EmissiveDiffuse::new(SolidColor::new(color(8.0, 2.8, 0.04), None)),
    // ))

    world.add_light(Sphere::new(
        vec3(0.0, 10.0, 0.0),
        4.0,
        EmissiveDiffuse::new(SolidColor::new(color(5.0, 5.0, 5.0), None)),
    ));

    let camera = Camera::new(
        vec3(-2.8, 7.0, 9.0),
        vec3(-0.5, 2.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        30.0,
        4.0 / 3.0,
        0.0,
        100.0,
    );

    let image = world
        .samples_per_pixel(128)
        .max_depth(50)
        .width(800)
        .render(camera);
    image.save("renders/test.png").unwrap();
}
