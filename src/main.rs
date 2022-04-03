#![allow(dead_code)]
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
    sphere::Sphere,
};
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

fn color(r: f32, g: f32, b: f32) -> Color {
    return Color::new(r, g, b);
}

fn main() {
    let mut world = World::new(PixelMap::from_image(load_image(
        "christmas_photo_studio_02_4k.exr",
        Rotate::None,
    )));

    world.add(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(CheckerBoard::new(
            color(1.0, 1.0, 1.0),
            color(0.05, 0.05, 0.05),
            5.0,
        )),
    ));

    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));

    let earth_texture = Image::new(load_image("images/earthmap.jpeg", Rotate::None), None);
    world.add(Sphere::new(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(earth_texture.ptr()),
    ));

    world.add(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Metal::new(
            SolidColor::new(
                color(0.7, 0.6, 0.5),
                None
                // Some(PixelMap::from_image(load_image(
                //     "images/bricks.jpeg",
                //     Rotate::R90,
                // ))),
            ),
            0.0,
        ),
    ));

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat = random_distribution();
    //         let center = vec3(
    //             a as f32 + 0.9 * random_distribution(),
    //             0.2,
    //             b as f32 + 0.9 * random_distribution(),
    //         );

    //         if (center - vec3(3.0, 0.2, 0.0)).length() > 0.9 {
    //             let material;

    //             if choose_mat < 0.75 {
    //                 material = Lambertian::new(SolidColor::new(
    //                     color(
    //                         random_distribution(),
    //                         random_distribution(),
    //                         random_distribution(),
    //                     ),
    //                     None,
    //                 ))
    //             } else if choose_mat < 0.90 {
    //                 material = Metal::new(
    //                     SolidColor::new(
    //                         color(
    //                             random_distribution(),
    //                             random_distribution(),
    //                             random_distribution(),
    //                         ),
    //                         None,
    //                     ),
    //                     random_distribution(),
    //                 )
    //             } else if choose_mat < 0.95 {
    //                 material = EmissiveDiffuse::new(SolidColor::new(
    //                     color(
    //                         random_distribution(),
    //                         random_distribution(),
    //                         random_distribution(),
    //                     ),
    //                     None,
    //                 ))
    //             } else {
    //                 material = Dielectric::new(1.5);
    //             }

    //             world.add(Sphere::new(center, 0.2, material));
    //         }
    //     }
    // }

    let camera = Camera::new(
        vec3(13.0, 2.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        40.0,
        16.0 / 9.0,
        0.0,
        100.0,
    );

    let image = world.samples_per_pixel(256).max_depth(50).render(camera);
    image.save("test.png").unwrap();
}
