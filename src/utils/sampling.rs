use std::f32::consts::PI;

use arrayvec::ArrayVec;

use glam::Vec3;
use rand::Rng;

use crate::{materials::texture::PixelMap, world::WorldLights};

pub type ONB = ArrayVec<Vec3, 3>;

pub trait UVW {
    fn build_from_w(n: Vec3) -> Self;
    fn u(&self) -> Vec3;
    fn v(&self) -> Vec3;
    fn w(&self) -> Vec3;
    fn local(&self, a: Vec3) -> Vec3;
}

impl UVW for ONB {
    fn build_from_w(n: Vec3) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);

        ArrayVec::<Vec3, 3>::from([u, v, w])
    }

    #[inline(always)]
    fn u(&self) -> Vec3 {
        self[0]
    }

    #[inline(always)]
    fn v(&self) -> Vec3 {
        self[1]
    }

    #[inline(always)]
    fn w(&self) -> Vec3 {
        self[2]
    }

    fn local(&self, a: Vec3) -> Vec3 {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}

pub trait PdfReady {
    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f32 {
        0.0
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub enum PDF<'a> {
    Cosine {
        uvw: ONB,
    },
    Lights {
        origin: Vec3,
        objects: &'a WorldLights,
    },
    Mixture {
        p: &'a PDF<'a>,
        q: &'a PDF<'a>,
    }, // Image {
       //     map: &'a PixelMap,
       //     pUDist: Vec<f32>,
       // }
}

impl<'a> PDF<'a> {
    #[inline(always)]
    pub fn cosine(w: Vec3) -> Self {
        PDF::Cosine {
            uvw: ONB::build_from_w(w),
        }
    }

    #[inline(always)]
    pub fn lights(objects: &'a WorldLights, origin: Vec3) -> Self {
        PDF::Lights { origin, objects }
    }

    #[inline(always)]
    pub fn mixture(p: &'a PDF, q: &'a PDF) -> Self {
        PDF::Mixture { p, q }
    }

    // #[inline(always)]
    // // function generates a PDF from an image
    // pub fn image(map: &'a PixelMap) -> Self {
    //     // generate a marginal cdf
    //     let mut pUDist = Vec::with_capacity(map.width as usize);
    //     let mut pBuffer = Vec::with_capacity(map.width as usize);
    //     let mut m_pData = Vec::with_capacity(map.width as usize * map.height as usize);

    //     let mut sum = 0.0;
    //     for y in 0..map.height {
    //         for x in 0..map.width {
    //             let pixel = map.get_pixel((x, y));
    //             let luminance = 0.2126 * pixel.x + 0.7152 * pixel.y + 0.0722 * pixel.z;
    //             sum += luminance;
    //             m_pData.push(luminance);
    //         }
    //     }

    //     for i in 0..map.width as usize {
    //         pBuffer.push(m_pData[i]);
    //     }

    //     for i in 0..map.width as usize {
    //         pUDist.push(pBuffer[i] / sum);
    //     }

    //     PDF::Image { map, pUDist }
    // }

    pub fn value(&self, direction: Vec3) -> f32 {
        match self {
            PDF::Cosine { uvw } => {
                let cosine = direction.normalize().dot(uvw.w());
                if cosine > 0.0 {
                    cosine / std::f32::consts::PI
                } else {
                    1.0
                }
            }
            PDF::Lights { origin, objects } => objects.pdf_value(*origin, direction),
            PDF::Mixture { p, q } => 0.5 * p.value(direction) + 0.5 * q.value(direction), // PDF::Image { map, pUDist } => {
                                                                                          //     let (u, v) = map.surface_uv(direction.normalize());
                                                                                          //     let uIndex = (u * (map.width - 1) as f32) as usize;
                                                                                          //     let vIndex = (v * (map.height - 1) as f32) as usize;
                                                                                          //     let uDist = pUDist[uIndex];
                                                                                          //     let vDist = pUDist[vIndex];
                                                                                          //     uDist * vDist
                                                                                          // }
        }
    }

    pub fn generate(&self) -> Vec3 {
        match self {
            PDF::Cosine { uvw } => uvw.local(random_cosine_direction()),
            PDF::Lights { origin, objects } => objects.random(*origin),
            PDF::Mixture { p, q } => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() {
                    p.generate()
                } else {
                    q.generate()
                }
            } // PDF::Image { map, pUDist } => {
              //     let mut rng = rand::thread_rng();
              //     let u = rng.gen_range(0.0..1.0);
              //     let v = rng.gen_range(0.0..1.0);
              //     let uIndex = (u * (map.width - 1) as f32) as usize;
              //     let vIndex = (v * (map.height - 1) as f32) as usize;
              //     let uDist = pUDist[uIndex];
              //     let vDist = pUDist[vIndex];
              //     let u = u * (map.width - 1) as f32;
              //     let v = v * (map.height - 1) as f32;
              //     let u = u / (map.width - 1) as f32;
              //     let v = v / (map.height - 1) as f32;
              //     let u = u * uDist;
              //     let v = v * vDist;
              //     let u = u * (map.width - 1) as f32;
              //     let v = v * (map.height - 1) as f32;
              //     let u = u as usize;
              //     let v = v as usize;
              //     let pixel = map.get_pixel((u as u32, v as u32));
              //     Vec3::new(pixel.x, pixel.y, pixel.z)
              // }
        }
    }
}

fn random_cosine_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * 2.0 * r2.sqrt();
    let y = phi.sin() * 2.0 * r2.sqrt();
    Vec3::new(x, y, z)
}

fn from_spherical_uv(u: f32, v: f32) -> Vec3 {
    let phi = u * 2.0 * std::f32::consts::PI;
    let theta = (v * 0.5 + 0.5) * std::f32::consts::PI;
    let sin_theta = theta.sin();
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), theta.cos())
}
