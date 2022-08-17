use crate::math::{Ray, Vec3, Float};

pub trait Hittable {
    fn hit(&self, h: &HitAttr) -> Option<HitAttr>;
}

#[derive(Clone, Copy)]
pub struct HitAttr {
    pub t: Float,
    pub ray: Ray,
    pub prev_hit_index: Option<usize>,
}

pub struct Sphere{
    center: Vec3,
    radius: Float,
    color: Vec3,
}

impl Sphere{
    pub fn new(center: Vec3, radius: Float, color: Vec3) -> Self {
        Sphere{center, radius, color}
    }
}

impl Hittable for Sphere {
    fn hit(&self, h: &HitAttr) -> Option<HitAttr> {
        let oc = h.ray.origin - self.center;
        let a = h.ray.direction.dot(&h.ray.direction);
        let b = oc.dot(&h.ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        let normal = (h.ray.origin - self.center) * (1.0 / self.radius);
        if discriminant > 0.0 {
            let t = (- b - discriminant.sqrt()) / a;
            if t < 0.0 {
                None
            } else {
                Some(HitAttr{
                    t, 
                    ray: Ray { 
                        origin: h.ray.at(t), 
                        direction: normal.random_diffusion(), 
                        color: self.color.mul(&h.ray.color)
                    },
                    prev_hit_index: h.prev_hit_index,
                })
            }
        } else {
            None
        }
    }
}

pub struct Floor {
    pub height: Float,
    pub color: Vec3,
    pub upwards: bool
}

impl Floor {
    pub fn new(height: Float, color: Vec3, upwards: bool) -> Self {
        Floor {height, color, upwards}
    }
}

impl Hittable for Floor {
    fn hit(&self, h: &HitAttr) -> Option<HitAttr> {
        let t = (self.height - h.ray.origin.z) / h.ray.direction.z;
        let normal = if self.upwards {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        };
        if t < 0.0 {
            None
        } else {
            Some(HitAttr{
                t, 
                ray: Ray { 
                    origin: h.ray.at(t), 
                    direction: normal.random_diffusion(), 
                    color: h.ray.color.mul(&self.color)
                },
                prev_hit_index: h.prev_hit_index
            })
        }
    }
}