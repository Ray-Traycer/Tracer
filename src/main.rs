#![allow(dead_code)]
use glam::Vec3;
use utils::{material::Material, Color, WHITE};
use world::{world::World, object::Sphere};


mod utils;
mod random;

mod world;



fn main(){
    let mut world = World::new();
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::new(Color::new(0.0, 0.0, 0.8))).into());
    
    for i in 0..10{
        for k in 0..10{
            let pos = Vec3::new(i as f32 * 5., 0.0, k as f32 * 5. );
            world.add(Sphere::new(pos, 0.2, Material::new(Color::new(0.9, 0.9, 0.9))).into());    
        }
    }

    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::new(Color::new(0.8, 0.8, 0.8))).into());
    
    let image = world.samples_per_pixel(128).max_depth(50).render();
    image.save("test.png").unwrap();
}