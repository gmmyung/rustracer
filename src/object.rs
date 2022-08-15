use crate::math::{ray, vec3, Float};

pub struct sphere{
    center: vec3,
    radius: Float,
}

impl sphere{
    pub fn new(center: vec3, radius: Float) -> Self {
        sphere{center, radius}
    }
    pub fn hit_sphere(&self, ray: &ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}