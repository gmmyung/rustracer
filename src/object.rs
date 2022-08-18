use crate::math::{Float, Vec3};
use crate::reflection::{Hit, HitAttr, HitKind, Reflection};

pub trait Hittable {
    fn hit(&self, h: &HitAttr, i: usize) -> HitAttr {
        if let Some((t, p)) = self.get_intersect(h) {
            self.reflect(t, p, self.get_normal(h, p), h, i)
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
    fn get_intersect(&self, h: &HitAttr) -> Option<(Float, Vec3)>;
}

pub struct Sphere<R: Reflection> {
    center: Vec3,
    radius: Float,
    color: Vec3,
}

impl Sphere{
    pub fn new(center: Vec3, radius: Float, color: Vec3) -> Self {
        Sphere{center, radius, color}
    }
}

impl<R> Hittable for Sphere<R>
where
    R: Reflection,
{
    fn get_intersect(&self, h: &HitAttr) -> Option<(Float, Vec3)> {
        let oc = h.ray.origin - self.center;
        let a = h.ray.direction.dot(&h.ray.direction);
        let b = oc.dot(&h.ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        let normal = (h.ray.origin - self.center) * (1.0 / self.radius);
        if discriminant > 0.0 {
            let d_sqrt = discriminant.sqrt();
            if -b > d_sqrt {
                let t = (-b - d_sqrt) / a;
                let p = h.ray.at(t);
                return Some((t, p));
            } else {
                let t = (-b + d_sqrt) / a;
                let p = h.ray.at(t);
                if t > 0.0 {
                    return Some((t, p));
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
    //                 return Some((temp, p));
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

impl<R> Hittable for Floor<R>
where
    R: Reflection,
{
    fn get_intersect(&self, h: &HitAttr) -> Option<(Float, Vec3)> {
        let t = (self.height - h.ray.origin.z) / h.ray.direction.z;
        let normal = if self.upwards {
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
