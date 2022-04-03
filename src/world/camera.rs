use std::time::Instant;

use glam::Vec3;
use image::{imageops::flip_vertical, RgbImage};
use indicatif::ProgressBar;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use super::{physics::Ray, world::World};
use crate::{
    random::{random_distribution, random_in_unit_disk},
    utils::{bvh::BvhTree, RenderedImage, Vec3Extension, BLACK},
};
use indicatif::ProgressStyle;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    aspect_ratio: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        let origin = lookfrom;
        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            aspect_ratio,
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }

    pub fn render_threaded(&self, world: &mut World) -> RenderedImage {
        let width = world.width;
        let samples_per_pixel = world.samples_per_pixel;
        let max_depth = world.max_depth;
        let world_objects = BvhTree::new(&mut world.objects);
        let light_objects = &world.lights;

        let height = (width as f32 / self.aspect_ratio) as u32;

        let mut img = RgbImage::new(width, height);

        let bar = ProgressBar::new((height * width) as u64 + 1);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} pixel"),
        );

        println!(
            "Rendering {}x{} at {} samples per pixel with a max depth of {}",
            width, height, samples_per_pixel, max_depth
        );
        let t1 = Instant::now();

        img.par_chunks_mut(3).enumerate().for_each(|(i, slab)| {
            let mut final_color = BLACK;

            (0..samples_per_pixel).for_each(|_| {
                let u = (random_distribution() + (i as u32 % width) as f32) / (width - 1) as f32;
                let v = (random_distribution() + (i as u32 / width) as f32) / (height - 1) as f32;

                let r = self.get_ray(u, v);

                final_color = final_color
                    + r.color(&world_objects, &light_objects, &world.skybox, max_depth)
                        .max(Vec3::ZERO)
                        .min(Vec3::ONE);
            });
            slab.copy_from_slice(
                &(final_color / samples_per_pixel as f32)
                    .powf(0.5)
                    .to_slice_u8(),
            );

            bar.inc(1);
        });

        img = flip_vertical(&img);

        bar.finish();
        println!("Took {:?}", t1.elapsed());
        img
    }
}
