use crate::math::{Ray, Vec3, Float};

pub trait Hittable {
    fn hit(&self, h: &HitAttr) -> Option<HitAttr>{
        if let Some(t) = self.intersect(h) {
            Some(HitAttr{
                t, 
                ray: Ray { 
                    origin: h.ray.at(t), 
                    direction: self.normal(h).random_diffusion(), 
                    color: self.get_color().mul(&h.ray.color)
                },
                prev_hit_index: h.prev_hit_index,
            })
        } else {
            None
        }
    }

    fn normal(&self, h: &HitAttr) -> Vec3;
    fn intersect(&self, h: &HitAttr) -> Option<Float>;
    fn get_color(&self) -> Vec3;
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
    fn intersect(&self, h: &HitAttr) -> Option<Float> {
        let oc = h.ray.origin - self.center;
        let a = h.ray.direction.dot(&h.ray.direction);
        let b = oc.dot(&h.ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > 0.0 {
                Some(temp)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn normal(&self, h: &HitAttr) -> Vec3 {
        (h.ray.origin - self.center) * (1.0 / self.radius)
    }

    fn get_color(&self) -> Vec3 {
        self.color
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

    fn intersect(&self, h: &HitAttr) -> Option<Float> {
        let t = (self.height - h.ray.origin.z) / h.ray.direction.z;
        if t > 0.0 {
            Some(t)
        } else {
            None
        }
    }

    fn normal(&self, _h: &HitAttr) -> Vec3 {
        if self.upwards {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        }
    }

    fn get_color(&self) -> Vec3 {
        self.color
    }
}