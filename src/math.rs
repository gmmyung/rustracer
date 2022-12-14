pub type Float = f32;

pub use fastrand::f32 as rand;

pub const SKY_COLOR: (Float, Float, Float) = (0.8, 0.8, 1.0);
// Used to prevent shadow acne.
pub const EPSILON: Float = 0.0001;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vec3 { x, y, z }
    }

    pub fn sky_color() -> Self {
        Vec3 {
            x: SKY_COLOR.0,
            y: SKY_COLOR.0,
            z: SKY_COLOR.0,
        }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn mag(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.mag();
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    pub fn sqrt(&self) -> Self {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn random() -> Self {
        let x = rand() * 2.0 - 1.0;
        let y = rand() * 2.0 - 1.0;
        let z = rand() * 2.0 - 1.0;
        Self { x, y, z }
    }

    pub fn exp_decay(&self, t: Float, color: &Self) -> Self {
        Vec3 {
            x: color.x.powf(t) * self.x,
            y: color.y.powf(t) * self.y,
            z: color.z.powf(t) * self.z,
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random();
            if v.mag() < 1.0 {
                return v.normalize();
            }
        }
    }

    pub fn random_diffusion(&self) -> Self {
        loop {
            let v = Self::random_in_unit_sphere();
            if v.dot(&self) > 0.0 {
                return v;
            }
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Float> for Vec3 {
    type Output = Self;
    fn mul(self, other: Float) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub color: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
            color: Vec3::sky_color(),
        }
    }

    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + self.direction * t
    }
}