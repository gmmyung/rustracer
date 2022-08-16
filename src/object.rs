use crate::math::{Float, Ray, Vec3};

pub trait Hittable {
    fn hit_dist(&self, r: &Ray) -> Option<Float>;

    fn normal_vec(&self, r: &Ray, t: Float) -> Vec3;

    fn get_color(&self) -> Vec3;

    fn hit(&self, h: &Hit) -> Hit {
        let r = &h.ray;
        match self.hit_dist(r) {
            Some(t) => {
                return Hit {
                    t,
                    ray: Ray {
                        origin: r.at(t),
                        direction: self.normal_vec(r, t).random_diffusion(),
                        color: self.get_color().mul(&r.color),
                    },
                    prev_hit_index: None,
                    hitattr: HitAttr::MidHit
                }
            }
            None => {
                return Hit {
                    t: Float::INFINITY,
                    ray: Ray {
                        origin: Vec3::new(0.0,0.0,0.0),
                        direction: Vec3::new(0.0,0.0,0.0),
                        color: self.get_color().mul(&r.color),
                    },
                    prev_hit_index: None,
                    hitattr: HitAttr::LastHit
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: Float,
    pub ray: Ray,
    pub prev_hit_index: Option<usize>,
    pub hitattr: HitAttr
}

#[derive(Clone, Copy)]
pub enum HitAttr {
    FirstHit,
    MidHit,
    LastHit,
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
    fn hit_dist(&self, r: &Ray) -> Option<Float> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
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

    fn normal_vec(&self, r: &Ray, t: Float) -> Vec3 {
        (r.origin - self.center) * (1.0 / self.radius)
    }

    fn get_color(&self) -> Vec3 {
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
    fn hit_dist(&self, r: &Ray) -> Option<Float> {
        Some((self.height - r.origin.z) / r.direction.z)
    }

    fn normal_vec(&self, _r: &Ray, _t: Float) -> Vec3 {
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
