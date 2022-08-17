use crate::math::{Float, Ray, Vec3};

#[derive(Clone, Copy)]
pub struct HitAttr {
    pub t: Float,
    pub ray: Ray,
    pub hitkind: HitKind,
}

#[derive(Clone, Copy)]
pub enum HitKind {
    NormalHit,
    LastHit,
}

pub enum Hit {
    NormalHit(Ray),
    LastHit(Ray),
}

pub trait Reflection {
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit;
}

pub fn simple_specular_reflection(color: &Vec3, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit {

    if normal.dot(&h.ray.direction) > 0.0 {
        panic!("ray is not pointing towards the normal, Try Increasing Epsilon");
    }

    let direction = normal * -2.0 * normal.dot(&h.ray.direction) + h.ray.direction;
    Hit::NormalHit(Ray {
        origin: p,
        direction,
        color: h.ray.color.mul(color),
    })
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
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit {
        let direction = normal.random_diffusion();
        Hit::NormalHit(Ray {
            origin: p,
            color: h.ray.color.mul(&self.color),
            direction,
        })
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
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit {
        simple_specular_reflection(&self.color, p, normal, h)
    }
}

pub struct DiffusedLightSource {
    color: Vec3,
}

impl DiffusedLightSource {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Reflection for DiffusedLightSource {
    fn get_reflection(&self, p: Vec3, _normal: Vec3, h: &HitAttr) -> Hit {
        Hit::LastHit(Ray {
            origin: p,
            direction: Vec3::new(0.0, 0.0, 0.0),
            color: h.ray.color.div(&Vec3::sky_color()).mul(&self.color),
        })
    }
}

pub struct Glass {
    color: Vec3,
    refraction_index: Float,
}

impl Glass {
    pub fn new(color: Vec3, refraction_index: Float) -> Self {
        Self {
            color,
            refraction_index,
        }
    }
}

fn snells_law(
    normal: Vec3,
    p: Vec3,
    h: &HitAttr,
    color: &Vec3,
    refraction_index: Float,
    cos_incidence_angle: Float,
) -> Hit {
    let cos_refraction_angle_squared =
        1.0 - (cos_incidence_angle / refraction_index).powi(2);
    if cos_refraction_angle_squared > 0.0 {
        let cos_refraction_angle = cos_refraction_angle_squared.sqrt();
        let incidence_horizontal = h.ray.direction - normal * h.ray.direction.dot(&normal);
        let refraction_direction = normal * (-cos_refraction_angle)
            + incidence_horizontal * (1.0 / refraction_index);
        // Hit::LastHit(Ray {
        //     origin: p,
        //     direction: Vec3::zero(),
        //     color: Vec3::zero(),
        // })
        Hit::NormalHit(Ray {
            origin: p,
            direction: refraction_direction,
            color: h.ray.color,
        })
    } else {
        simple_specular_reflection(&Vec3::one(), p, normal, h)

    }
}

impl Reflection for Glass {
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit {
        let cos_incidence_angle = -normal.dot(&h.ray.direction);
        if cos_incidence_angle > 0.0 {
            Hit::NormalHit(Ray {
                origin: p,
                direction: h.ray.direction,
                color: h.ray.color,
            })
            // snells_law(normal, p, h, &self.color, self.refraction_index, cos_incidence_angle)
        } else {    
            // Hit::NormalHit(Ray {
            //     origin: p,
            //     direction: h.ray.direction,
            //     color: h.ray.color,
            // })
            snells_law(-normal, p, h, &self.color, 1.0 / self.refraction_index, -cos_incidence_angle)
        }
    }
}
