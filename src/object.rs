use crate::math::{Ray, Vec3, Float};

pub enum Hitten {
    Hit(HitAttr),
    Miss,
}

pub trait Hittable {
    fn hit(&self, r: &Ray) -> Hitten;
}

pub struct HitAttr {
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
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
    fn hit(&self, r: &Ray) -> Hitten {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (- b - discriminant.sqrt()) / a;
            if t < 0.0 {
                Hitten::Miss
            } else {
                Hitten::Hit(HitAttr{
                    t, 
                    p: r.at(t),
                    normal: (r.at(t)-self.center).normalize(),
                    color: r.color.mul(&self.color),
                })
            }
        } else {
            Hitten::Miss
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
    fn hit(&self, r: &Ray) -> Hitten {
        let t = (self.height - r.origin.z) / r.direction.z;
        if t < 0.0 {
            Hitten::Miss
        } else {
            Hitten::Hit(HitAttr{
                t, 
                p: r.at(t),
                normal:Vec3::new(0.0, 0.0, if self.upwards{1.0}else{-1.0}),
                color: r.color.mul(&self.color)
            })
        }
    }
}