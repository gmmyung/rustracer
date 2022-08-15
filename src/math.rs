use indicatif::{ProgressBar, ProgressStyle};

pub type Float = f32;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        vec3 { x, y, z }
    }
    pub fn mag(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let length = self.mag();
        vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Add for vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Float> for vec3 {
    type Output = Self;
    fn mul(self, other: Float) -> Self {
        vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

pub struct ray {
    pub origin: vec3,
    pub direction: vec3,
}

impl ray {
    pub fn new(origin: vec3, direction: vec3) -> Self {
        ray { origin, direction }
    }
    pub fn at(&self, t: Float) -> vec3 {
        self.origin + self.direction * t
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let v = vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.dot(&vec3::new(1.0, 2.0, 3.0)), 14.0);
        assert_eq!(v.cross(&vec3::new(1.0, 2.0, 3.0)), vec3::new(0.0, 0.0, 0.0));
    }
}