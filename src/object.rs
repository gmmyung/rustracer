use crate::math::{Float, Ray, Vec3};

pub trait Hittable {
    fn hit_dist(&self, h: &HitAttr) -> Option<Float>;

    fn normal_vec(&self, h: &HitAttr, t: Float) -> Vec3;

    fn get_color(&self, h: &HitAttr) -> Vec3;

    fn hit(&self, h: &HitAttr) -> Option<HitAttr> {
        match self.hit_dist(h) {
            Some(t) => {
                Some(HitAttr{
                    t,
                    ray: Ray { origin: h.ray.at(t), direction: self.normal_vec(h, t).random_diffusion(), color: self.get_color(h).mul(&h.ray.color) },
                    prev_hit_index: h.prev_hit_index,
                })
            }
            None => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct HitAttr {
    pub t: Float,
    pub ray: Ray,
    pub prev_hit_index: Option<usize>,
}

pub struct Sphere {
    center: Vec3,
    radius: Float,
    color: Vec3,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, color: Vec3) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl Hittable for Sphere {
    fn hit_dist(&self, h: &HitAttr) -> Option<Float> {
        let oc = h.ray.origin - self.center;
        let a = h.ray.direction.dot(&h.ray.direction);
        let b = oc.dot(&h.ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t > 0.0 {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn normal_vec(&self, h: &HitAttr, _t: Float) -> Vec3 {
        (h.ray.origin - self.center) * (1.0 / self.radius)
    }

    fn get_color(&self, _h: &HitAttr) -> Vec3 {
        self.color
    }
}

pub struct Floor {
    pub height: Float,
    pub color: Vec3,
    pub upwards: bool,
}

impl Floor {
    pub fn new(height: Float, color: Vec3, upwards: bool) -> Self {
        Floor {
            height,
            color,
            upwards,
        }
    }
}

impl Hittable for Floor {
    fn hit_dist(&self, h: &HitAttr) -> Option<Float> {
        Some((self.height - h.ray.origin.z) / h.ray.direction.z)
    }

    fn normal_vec(&self, _h: &HitAttr, _t: Float) -> Vec3 {
        if self.upwards {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        }
    }

    fn get_color(&self, _h: &HitAttr) -> Vec3 {
        self.color
    }
}
