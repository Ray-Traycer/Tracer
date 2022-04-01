use self::texture::TextureType;

pub mod emissivediffuse;
pub mod glossy;
pub mod lambertian;
pub mod material;
pub mod metal;
pub mod texture;
pub type TexturePtr = usize;
pub mod dielectric;

pub trait PtrExtension {
    fn deref(self) -> &'static TextureType;
}

impl PtrExtension for TexturePtr {
    fn deref(self) -> &'static TextureType {
        unsafe { &(*(self as *const TextureType)) }
    }
}
