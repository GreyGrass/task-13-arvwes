use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length_squared(&self) -> f64 {
       self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn cross(&self, rhs: &Self) -> Self{
        Vec3{
            x: self.y * rhs.z - self.z*rhs.y,
            y: -self.x * rhs.z + self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn unit_vector(self) -> Self{
        let length = self.length();
        self / length
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, skalar: f64) -> Self::Output {
        Vec3 {
            x: self.x * skalar,
            y: self.y * skalar,
            z: self.z * skalar,
        }
    }
}

impl Mul<Vec3> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, skalar: f64) {
        self.x *= skalar;
        self.y *= skalar;
        self.z *= skalar;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, skalar: f64) -> Self::Output {
        Vec3 {
            x: self.x / skalar,
            y: self.y / skalar,
            z: self.z / skalar,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, skalar: f64) {
        self.x /= skalar;
        self.y /= skalar;
        self.z /= skalar;
    }
}
