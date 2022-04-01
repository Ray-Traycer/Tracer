#![allow(dead_code)]
use glam::Vec3;
use objects::sphere::Sphere;
use random::random_distribution;
use utils::{material::Material, Color, WHITE};
use world::{camera::Camera, world::World};

mod random;
mod utils;

mod objects;
mod world;

fn main() {
    let mut world = World::new();
    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::new(Color::new(0.0, 0.0, 0.8)),
    ));

    for i in 0..1000 {
        for k in 0..1000 {
            let pos = Vec3::new(i as f32 * 5., 1.0, k as f32 * 5.);
            world.add(
                Sphere::new(
                    pos,
                    1.0,
                    Material::new(Color::new(
                        random_distribution(),
                        random_distribution(),
                        random_distribution(),
                    )),
                )
                .into(),
            );
        }
    }

    world.add(
        Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::new(Color::new(0.8, 0.8, 0.8)),
        )
        .into(),
    );

    let camera = Camera::new(
        Vec3::new(5.0, 1000.0, 500.0),
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
