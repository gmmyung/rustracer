use crate::math::{Float, Vec3};
use crate::reflection::{Hit, HitAttr, HitKind, Reflection};

pub trait Hittable {
    fn hit(&self, h: &HitAttr, i: usize) -> HitAttr {
        if let Some((t, p, previously_hit_reject)) = self.get_intersect(h, i) {
            if previously_hit_reject {
                HitAttr {
                    t,
                    ray: h.ray,
                    prev_hit_index: None,
                    hitkind: HitKind::LastHit,
                }
            } else {
                self.reflect(t, p, self.get_normal(h, p), h, i)
            }
            
        } else {
            HitAttr {
                t: Float::INFINITY,
                ray: h.ray,
                prev_hit_index: Some(i),
                hitkind: HitKind::LastHit,
            }
        }
    }

    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr, i: usize) -> HitAttr;
    fn get_normal(&self, h: &HitAttr, p: Vec3) -> Vec3;
    fn get_intersect(&self, h: &HitAttr, i:usize) -> Option<(Float, Vec3, bool)>;
}

pub struct Sphere<R> {
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
where
    R: Reflection,
{ 
    fn get_intersect(&self, h: &HitAttr, i:usize) -> Option<(Float, Vec3, bool)> {
        if let Some(pi) = h.prev_hit_index {
            if i == pi {
                return Some((0.0, Vec3::zero(), true));
            }
        }
        let oc = h.ray.origin - self.center;
        let a = h.ray.direction.dot(&h.ray.direction);
        let b = oc.dot(&h.ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let d_sqrt = discriminant.sqrt();
            if -b > d_sqrt {
                let t = (-b - d_sqrt) / a;
                let p = h.ray.at(t);
                return Some((t, p, false));
            } else {
                let t = (-b + d_sqrt) / a;
                let p = h.ray.at(t);
                if t > 0.0 {
                    return Some((t, p, false));
                }
            }
        }
        None
    }
    //     if discriminant > 0.0 {
    //         let temp = (-b - discriminant.sqrt()) / a;
    //         let p = h.ray.at(temp);
    //         if temp > 0.0 {
    //             if h.ray.direction.dot(&self.get_normal(h, p)) < 0.0 {
    //                 return Some((temp, p, false));
    //             }
    //         }
    //     }
    //     None
    // }
    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr, i: usize) -> HitAttr {
        match self.reflection.get_reflection(p, normal, h) {
            Hit::NormalHit(r) => HitAttr {
                t: t,
                ray: r,
                prev_hit_index: Some(i),
                hitkind: HitKind::NormalHit,
            },
            Hit::LastHit(r) => HitAttr {
                t: t,
                ray: r,
                prev_hit_index: Some(i),
                hitkind: HitKind::LastHit,
            },
        }
    }

    fn get_normal(&self, _h: &HitAttr, p: Vec3) -> Vec3 {
        (p - self.center) * (1.0 / self.radius)
    }
}


pub struct Floor<R> {
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
where
    R: Reflection,
{
    fn get_intersect(&self, h: &HitAttr, i: usize) -> Option<(Float, Vec3, bool)> {
        if let Some(pi) = h.prev_hit_index {
            if i == pi {
                return Some((0.0, Vec3::zero(), true));
            }
        }
        let t = (self.height - h.ray.origin.z) / h.ray.direction.z;
        if t > 0.0 {
            let p = h.ray.at(t);
            if h.ray.direction.dot(&self.get_normal(h, p)) < 0.0 {
                return Some((t, p, false));
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

    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr, i: usize) -> HitAttr {
        match self.reflection.get_reflection(p, normal, h) {
            Hit::NormalHit(r) => HitAttr {
                t: t,
                ray: r,
                prev_hit_index: Some(i),
                hitkind: HitKind::NormalHit,
            },
            Hit::LastHit(r) => HitAttr {
                t: t,
                ray: r,
                prev_hit_index: Some(i),
                hitkind: HitKind::LastHit,
            },
        }
    }
}
