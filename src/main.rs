#![allow(dead_code)]
use glam::Vec3;
use materials::{
    lambertian::Lambertian,
    material::MaterialType,
    metal::Metal,
    texture::{CheckerBoard, Image, SolidColor, TextureType},
};
use objects::sphere::Sphere;
use random::random_distribution;
use utils::{Color, WHITE};
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
pub fn load_image(image_path: &str, rotation: Rotate) -> image::DynamicImage {
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    match rotation {
        Rotate::R90 => img.rotate90(),
        Rotate::R180 => img.rotate180(),
        Rotate::R270 => img.rotate270(),
        _ => img,
    }
}

fn main() {
    let mut world = World::new();

    let texture1 = Image::new(load_image("earthmap.jpeg", Rotate::None));
    let texture1_ptr = texture1.ptr();

    let texture2 = CheckerBoard::new(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0), 8.0);
    let texture2_ptr = texture2.ptr();

    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Lambertian::new(texture1_ptr),
    ));

    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Metal::new(texture2_ptr, 0.9),
    ));

    let camera = Camera::new(
        Vec3::new(0.0, 1.0, 5.0),
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        16.0 / 9.0,
        0.0,
        100.0,
    );

    let image = world.samples_per_pixel(128).max_depth(50).render(camera);
    image.save("test.png").unwrap();
}
