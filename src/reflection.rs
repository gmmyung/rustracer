use crate::math::{Vec3, Ray, Float};

#[derive(Clone, Copy)]
pub struct HitAttr {
    pub t: Float,
    pub ray: Ray,
    pub prev_hit_index: Option<usize>,
}

pub trait Reflection {
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Ray;
}

pub struct Diffuse {
    color: Vec3,
}

impl Diffuse {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Reflection for Diffuse {
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Ray {
        Ray{
            origin: p,
            color: h.ray.color.mul(&self.color),
            direction: normal.random_diffusion()
        }
    }
}

pub struct Mirror {
    color: Vec3,
}

impl Mirror {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Reflection for Mirror {
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Ray {
        Ray{
            origin: p,
            direction: normal * -2.0 * normal.dot(&h.ray.direction) + h.ray.direction,
            color: h.ray.color.mul(&self.color)
        }
    }
}

pub struct DiffusedLightSource {
    color: Vec3
}

impl DiffusedLightSource {
    pub fn new(color: Vec3) -> Self{
        Self { color }
    }
}

impl Reflection for DiffusedLightSource {
    fn get_reflection(&self, p: Vec3, _normal: Vec3, h: &HitAttr) -> Ray {
        Ray { origin: p, direction: Vec3::zero(), color: h.ray.color.div(&Vec3::sky_color()).mul(&self.color) }
    }
}