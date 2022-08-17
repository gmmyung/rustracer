use crate::math::{Float, Vec3};
use crate::reflection::{Reflection, HitAttr, HitKind, Hit};

pub trait Hittable{
    fn hit(&self, h: &HitAttr) -> HitAttr {
        if let Some((t, p)) = self.get_intersect(h) {
            self.reflect(t, p, self.get_normal(h, p), h)
        } else {
            HitAttr {
                t: Float::INFINITY,
                ray: h.ray,
                prev_hit_index: None,
                hitkind: HitKind::LastHit
            }
        }
    }

    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr) -> HitAttr ;
    fn get_normal(&self, h: &HitAttr, p: Vec3) -> Vec3;
    fn get_intersect(&self, h: &HitAttr) -> Option<(Float, Vec3)>;
}

pub struct Sphere<R: Reflection> {
    center: Vec3,
    radius: Float,
    reflection: R,
}

impl<R> Sphere<R>
where
    R: Reflection,
{
    pub fn new(center: Vec3, radius: Float, reflection: R) -> Self {
        Sphere {
            center,
            radius,
            reflection,
        }
    }
}

impl<R> Hittable for Sphere<R>
where R: Reflection
{
    fn get_intersect(&self, h: &HitAttr) -> Option<(Float, Vec3)> {
        let oc = h.ray.origin - self.center;
        let a = h.ray.direction.dot(&h.ray.direction);
        let b = oc.dot(&h.ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            let p = h.ray.at(temp);
            if temp > 0.0 {
                if h.ray.direction.dot(&self.get_normal(h, p)) < 0.0 {
                    return Some((temp, p));
                }
            }
        }
        None
    }

    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr) -> HitAttr{
        match self.reflection.get_reflection(p, normal, h) {
            Hit::NormalHit(r) => HitAttr { 
                t: t, 
                ray: r, 
                prev_hit_index: h.prev_hit_index, 
                hitkind: HitKind::NormalHit 
            },
            Hit::LastHit(r) => HitAttr { 
                t: t, 
                ray: r, 
                prev_hit_index: h.prev_hit_index, 
                hitkind: HitKind::LastHit 
            }
        }
    }

    fn get_normal(&self, _h: &HitAttr, p: Vec3) -> Vec3 {
        (p - self.center) * (1.0 / self.radius)
    }
}

pub struct Floor<R: Reflection> {
    pub height: Float,
    pub upwards: bool,
    pub reflection: R,
}

impl<R> Floor<R>
where
    R: Reflection,
{
    pub fn new(height: Float, upwards: bool, reflection: R) -> Self {
        Floor {
            height,
            upwards,
            reflection,
        }
    }
}

impl<R> Hittable for Floor<R>
where R: Reflection
{
    fn get_intersect(&self, h: &HitAttr) -> Option<(Float, Vec3)> {
        let t = (self.height - h.ray.origin.z) / h.ray.direction.z;
        if t > 0.0 {
            let p = h.ray.at(t);
            if h.ray.direction.dot(&self.get_normal(h, p)) < 0.0 {
                return Some((t, p));
            }
        }
        None
    }

    fn get_normal(&self, _h: &HitAttr, _p: Vec3) -> Vec3 {
        if self.upwards {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        }
    }

    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr) -> HitAttr{
        match self.reflection.get_reflection(p, normal, h) {
            Hit::NormalHit(r) => HitAttr { 
                t: t, 
                ray: r, 
                prev_hit_index: h.prev_hit_index, 
                hitkind: HitKind::NormalHit 
            },
            Hit::LastHit(r) => HitAttr { 
                t: t, 
                ray: r, 
                prev_hit_index: h.prev_hit_index, 
                hitkind: HitKind::LastHit 
            }
        }
    }
}
