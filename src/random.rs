use std::f32::consts::PI;

use glam::Vec3;
use rand::Rng;

pub fn random_int(i: u32, j: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(i..j)
}

pub fn random_float(i: f32, j: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(i..j)
}

pub fn random_distribution() -> f32 {
    rand::random()
    // let mut rng = rand::thread_rng();
    // BETWEEN.sample(&mut rng)
}

pub fn random_sphere_distribution() -> Vec3 {
    let r1 = random_distribution();
    let r2 = random_distribution();

    return Vec3::new(
        (2.0 * PI * r1).cos() * 2.0 * (r2 * (1.0 - r2)).sqrt(),
        (2.0 * PI * r1).sin() * 2.0 * (r2 * (1.0 - r2)).sqrt(),
        1.0 - (2.0 * r2),
    );

    // loop {
    //     let p = Vec3::new(
    //         random_float(-1.0, 1.0),
    //         random_float(-1.0, 1.0),
    //         random_float(-1.0, 1.0),
    //     );
    //     if p.length_squared() >= 1.0 {
    //         continue;
    //     }
    //     return p;
    // }
}

pub fn random_hemisphere_distribution(normal: Vec3) -> Vec3 {
    let us = random_sphere_distribution();
    if us.dot(normal) > 0.0 {
        us
    } else {
        -us
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        };
        return p;
    }
}
