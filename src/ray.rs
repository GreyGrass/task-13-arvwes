use crate::vector3::Vec3;

pub struct Ray{
    pub a: Vec3,
    pub b: Vec3,
}

impl Ray{
    pub fn origin(&self) -> Vec3{
        self.a
    }
    pub fn direction(&self) -> Vec3{
        self.b
    }
    pub fn point_at_parameter(&self, t: f64) -> Vec3{
        self.a + self.b*t
    }
}