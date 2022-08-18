use crate::math::{self, Float, Ray, Vec3};

#[derive(Clone, Copy)]
/// The hit attribute of a ray-object intersection. 
/// t is the distance from the ray origin to the intersection point.
pub struct HitAttr {
    pub t: Float,
    pub ray: Ray,
    pub hitkind: HitKind,
}

#[derive(Clone, Copy)]
/// The kind of a ray-object intersection. 
/// NormalHit indicates that the ray bounces off or penetrates the object. 
/// LastHit indicates that the ray terminates there. 
/// Used in HitAttr.
pub enum HitKind {
    NormalHit,
    LastHit,
}

/// Similar to HitAttr, but used to pass information within the reflection object
pub enum Hit {
    NormalHit(Ray),
    LastHit(Ray),
}

/// Needs to be implemented to be rendered by the raytracer.
pub trait Reflection {
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit;
}

pub fn simple_specular_reflection(color: &Vec3, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit {
    Hit::NormalHit(Ray {
        origin: p,
        direction: normal * -2.0 * normal.dot(&h.ray.direction) + h.ray.direction,
        color: h.ray.color.mul(color),
    })
}

/// Diffuses incoming ray uniformly over all directions.
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
        Hit::NormalHit(Ray {
            origin: p,
            color: h.ray.color.mul(&self.color),
            direction: normal.random_diffusion(),
        })
    }
}

/// A mirror reflection.
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

/// A Light Source that emits light in all directions uniformly.
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

/// Refracts incoming ray in the direction of the normal. 
/// Implementation is based on the Schlick approximation. 
/// The incoming ray decays exponentially depending on the depth.
pub struct Glass {
    color: Vec3,
    refraction_index: Float,
    r_0: Float,
}

impl Glass {
    pub fn new(color: Vec3, refraction_index: Float) -> Self {
        Self {
            color,
            refraction_index,
            r_0: ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2),
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
    let det = cos_incidence_angle.powi(2) + refraction_index.powi(2) - 1.0;
    if det > 0.0 {
        let cos_refraction_angle = det.sqrt() / refraction_index;
        let incidence_horizontal = h.ray.direction - normal * h.ray.direction.dot(&normal);
        let refraction_direction =
            normal * (-cos_refraction_angle) + incidence_horizontal * (1.0 / refraction_index);
        Hit::NormalHit(Ray {
            origin: p,
            direction: refraction_direction,
            color: *color,
        })
    } else {
        simple_specular_reflection(color, p, normal, h)
    }
}

impl Reflection for Glass {
    fn get_reflection(&self, p: Vec3, normal: Vec3, h: &HitAttr) -> Hit {
        let cos_incidence_angle = -normal.dot(&h.ray.direction);
        // Reflects randomly based on the Schlick approximation.
        if math::rand() < self.r_0 + (1.0 - self.r_0) * (1.0 - cos_incidence_angle.abs()).powi(5) {
            if cos_incidence_angle > 0.0 {
                // Light gets reflected without entering the object.
                simple_specular_reflection(&self.color, p, normal, h)
            } else {
                // Light gets reflected inside the object.
                simple_specular_reflection(&h.ray.color.exp_decay(h.t, &self.color), p, -normal, h)
            }
        } else {
            if cos_incidence_angle > 0.0 {
                // Light enters the glass.
                snells_law(
                    normal,
                    p,
                    h,
                    &h.ray.color,
                    self.refraction_index,
                    cos_incidence_angle,
                )
            } else {
                // Light leaves the glass.
                snells_law(
                    -normal,
                    p,
                    h,
                    &h.ray.color.exp_decay(h.t, &self.color),
                    1.0 / self.refraction_index,
                    -cos_incidence_angle,
                )
            }
        }    
    }
}
