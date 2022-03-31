use bvh::bvh::BVH;
use glam::Vec3;

use crate::utils::{Color, RenderedImage};

use super::camera::Camera;
use super::object::ObjectType;

pub type WorldObjects = Vec<ObjectType>;

pub struct World{
    pub camera: Camera,
    pub objects: WorldObjects,
    pub background: Color,
    pub width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32
}

impl World{
    pub fn new()-> Self{
        World{
            camera: Camera::new(
                 Vec3::new(5.0, 20.0, 20.0),
                   Vec3::new(0.0, 0.5, 0.0),
                     Vec3::new(0.0, 1.0, 0.0),
                40.0,
                16.0 / 9.0,
                0.0,
                100.0
            ),
            objects: vec![],
            background: Color::new(0.5, 0.5, 0.5),
            width:  800,
            samples_per_pixel: 128,
            max_depth: 50,
        }
    }   
    pub fn background(mut self , color: Color)-> Self{
        self.background = color;
        self
    }
    pub fn width(mut self , width: u32)-> Self{
        self.width = width;
        self
    }
    pub fn samples_per_pixel(mut self , samples_per_pixel: u32)-> Self{
        self.samples_per_pixel = samples_per_pixel;
        self
    }
    pub fn max_depth(mut self , max_depth: u32)-> Self{
        self.max_depth = max_depth;
        self
    }

    pub fn add(&mut self, object: ObjectType){
        self.objects.push(object);
    }

    pub fn render(&self) -> RenderedImage{
        let bvh = BVH::build(&mut self.objects);
        self.camera.render_threaded(self)
    }
}