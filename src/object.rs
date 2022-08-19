use crate::math::{Float, Vec3};
use crate::reflection::{Hit, HitAttr, HitKind, Reflection};

pub trait Hittable {
    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr) -> HitAttr;
    fn get_normal(&self, h: &HitAttr, p: Vec3) -> Vec3;
    fn get_intersect(&self, h: &HitAttr) -> Option<Float>;
}

pub struct Sphere<R: Reflection> {
    center: Vec3,
    radius: Float,
    reflection: R,
}


/// A sphere with a given center and radius, and a given reflection(e.g. Diffuse, Mirror)
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
    fn get_intersect(&self, h: &HitAttr) -> Option<Float> {
        let oc = h.ray.origin - self.center;
        let a = h.ray.direction.dot(&h.ray.direction);
        let b = oc.dot(&h.ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // If discriminant is positive, the ray intersects the sphere.
            let d_sqrt = discriminant.sqrt();
            if -b - d_sqrt > 0.0 {
                // The ray starts from outside the sphere.
                let t = (-b - d_sqrt) / a;
                return Some(t);
            } else if -b + d_sqrt > 0.0 {
                // The ray starts from inside the sphere.
                let t = (-b + d_sqrt) / a;
                return Some(t);
            }
        }
        // If discriminant is negative, the ray misses the sphere
        None
    }

    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr) -> HitAttr {
        match self.reflection.get_reflection(p, normal, h) {
            // If the hit is a normal hit, (e.g. Diffusion, Mirror, Glass, etc.), return the hit.
            Hit::NormalHit(r) => HitAttr {
                t: t,
                ray: r,
                hitkind: HitKind::NormalHit,
            },
            // If the hit is a last hit, (e.g. DiffusedLightSource), return the hit.
            Hit::LastHit(r) => HitAttr {
                t: t,
                ray: r,
                hitkind: HitKind::LastHit,
            },
        }
    }

    fn get_normal(&self, _h: &HitAttr, p: Vec3) -> Vec3 {
        (p - self.center) * (1.0 / self.radius)
    }
}

/// Horizontal plane with a certain height, and a given reflection. Glass reflection doesn't work well with this, since the ray doesn't exit the floor.
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
where
    R: Reflection,
{
    fn get_intersect(&self, h: &HitAttr) -> Option<Float> {
        let t = (self.height - h.ray.origin.z) / h.ray.direction.z;
        if t > 0.0 {
            let p = h.ray.at(t);
            // The ray only reflects towards the upward direction.
            if h.ray.direction.dot(&self.get_normal(h, p)) < 0.0 {
                return Some(t);
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

    fn reflect(&self, t: Float, p: Vec3, normal: Vec3, h: &HitAttr) -> HitAttr {
        match self.reflection.get_reflection(p, normal, h) {
            // If the hit is a normal hit, (e.g. Diffusion, Mirror, etc.), return the hit.
            Hit::NormalHit(r) => HitAttr {
                t,
                ray: r,
                hitkind: HitKind::NormalHit,
            },
            // If the hit is a last hit, (e.g. DiffusedLightSource), return the hit.
            Hit::LastHit(r) => HitAttr {
                t,
                ray: r,
                hitkind: HitKind::LastHit,
            },
        }
    }
}
